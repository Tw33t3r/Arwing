use std::{
    ffi::OsStr,
    fs::canonicalize,
    io::{Error, ErrorKind},
    path::PathBuf,
};

use clap::{Arg, ArgAction, arg, command, value_parser};
use glob::glob;

use arwing_core::{
    ParsedGame, characters::internal_character_from_str, check_players, create_json,
    interaction::Interaction, parse_game, read_game,
};

use ssbm_data::character::External;

fn parse_internal_character(env: &str) -> Result<External, Error> {
    match internal_character_from_str(env) {
        Some(character) => Ok(character),
        None => Err(Error::new(ErrorKind::Other, "Character does not exist")),
    }
}

fn main() {
    let matches = command!()
        .arg(
            arg!(-n --name <SlippiName>)
                .required(false)
                .help("Slippi name to search for")
                .action(ArgAction::Set),
        )
        .arg(
            arg!(-e --export <File>)
                .required(false)
                .help("Location to export file to")
                .action(ArgAction::Set)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(-d --directory <Path>)
                .required(true)
                .help("Directory to parse files in")
                .action(ArgAction::Set)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(-p --player <Character>)
                .required(true)
                .help("Player character to search for")
                .action(ArgAction::Set)
                .value_parser(clap::builder::ValueParser::new(parse_internal_character)),
        )
        .arg(
            arg!(-o --opponent <Character>)
                .required(true)
                .help("Opponent character to search for")
                .action(ArgAction::Set)
                .value_parser(clap::builder::ValueParser::new(parse_internal_character)),
        )
        .arg(
            Arg::new("interaction")
                .short('i')
                .value_names(["Character", "ActionState", "NumFrames"])
                .num_args(3)
                .required(true)
                .help("Character, ActionState, Num of frames until this state")
                .action(ArgAction::Append),
        )
        .get_matches();

    let _name: Option<&String> = matches.get_one("name");
    let export_option: Option<&PathBuf> = matches.get_one("export");

    let player: External = *matches.get_one("player").unwrap();
    let opponent: External = *matches.get_one("opponent").unwrap();
    let path: PathBuf = matches
        .get_one::<PathBuf>("directory")
        .unwrap()
        .to_path_buf();

    let interactions: Vec<Interaction> = matches
        .get_many("interaction")
        .unwrap()
        .collect::<Vec<&String>>()
        .chunks(3)
        .map(|interaction| {
            let from_player_result = internal_character_from_str(&interaction[0][..]);
            let from_player = match from_player_result {
                Some(from_player) => from_player,
                None => {
                    panic!("Couldn't match the from_player in interactions")
                }
            };
            Interaction {
                action: interaction[1].parse().unwrap(),
                from_player: from_player,
                //TODO(Tweet): Figure out how to fix the 1st input low number of within frames bug.
                within: match interaction[2].as_str() {
                    "None" => None,
                    other => Some(other.parse().unwrap()),
                },
            }
        })
        .collect();

    let now = std::time::Instant::now();

    let mut parsed_games: Vec<ParsedGame> = Vec::new();

    if path.is_dir() {
        //glob-match advertises that it might be a faster library for this use case
        for entry in glob(
            //TODO(Tweet): Excessive casting
            &(path
                .as_path()
                .to_str()
                .expect("Could not convert directory to string, does the OS use utf-8?")
                .to_owned()
                + "/**/*.slp"),
        )
        .expect("Failed to read glob pattern")
        {
            match entry {
                Ok(path) => {
                    //TODO(Tweet): spawn a new thread for each game
                    if let Ok(game) = read_game(path.as_path()) {
                        let players_result = check_players(&game, player, opponent);
                        if let Some(players) = players_result {
                            let parsed = parse_game(game, &interactions, players).unwrap();
                            parsed_games.push(ParsedGame {
                                query_result: parsed,
                                loc: canonicalize(path).unwrap(),
                            });
                        }
                    };
                }
                Err(e) => println!("{:?}", e),
            }
        }
    } else if path.extension() == Some(OsStr::new("slp")) {
        //Speed of single file parsing .252057s on dev machine
        let game = read_game(path.as_path()).unwrap();
        let players = check_players(&game, player, opponent).unwrap();
        let parsed = parse_game(game, &interactions, players).unwrap();
        parsed_games.push(ParsedGame {
            query_result: parsed,
            loc: canonicalize(path).unwrap(),
        });
    }

    match export_option {
        Some(export) => create_json(parsed_games, export.to_path_buf()),
        None => {
            for game in parsed_games {
                for parse in game.query_result.result {
                    println!("Found frames: {:?}, in {:?}", parse, game.loc)
                }
            }
        }
    }
    println!("parsed in {} μs", now.elapsed().as_micros());
}

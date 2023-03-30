use std::{
    io::{Error, ErrorKind},
    path::{Path, PathBuf},
    process,
};

use clap::{arg, command, value_parser, Arg, ArgAction};

use arwing::{check_players, parse_game, read_game, Interaction};
use peppi::model::enums::{action_state::State, character::Internal};

fn parse_internal_character(env: &str) -> Result<Internal, Error> {
    match Internal::try_from(env) {
        Ok(character) => Ok(character),
        Err(_) => Err(Error::new(ErrorKind::Other, "Character does not exist")),
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

    let name: Option<&String> = matches.get_one("name");
    let export: Option<&String> = matches.get_one("export");

    let player: Internal = *matches.get_one("player").unwrap();
    let opponent: Internal = *matches.get_one("opponent").unwrap();
    let path: &PathBuf = matches.get_one("directory").unwrap();

    let interactions: Vec<Interaction> = matches
        .get_many("interaction")
        .unwrap()
        .into_iter()
        .collect::<Vec<&String>>()
        .chunks(3)
        .map(|interaction| {
            let from_player_result = Internal::try_from(&interaction[1][..]);
            let from_player = match from_player_result {
                Ok(Internal(from_player)) => from_player,
                Err(error) => {
                    panic!("Couldn't match the from_player in interactions {:?}", error)
                }
            };
            Interaction {
                action: State::from(interaction[0].parse().unwrap(), Internal(from_player)),
                from_player: Internal(from_player),
                //TODO(Tweet): Figure out how to fix the 1st input low number of within frames bug.
                within: match interaction[2].as_str() {
                    "None" => None,
                    other => Some(other.parse().unwrap()),
                },
            }
        })
        .collect();

    let now = std::time::Instant::now();
    let path = Path::new("test.slp");
    let game = read_game(path).unwrap();
    let players = check_players(&game, player, opponent).unwrap();
    let parsed = parse_game(game, interactions, players).unwrap();
    //create_json(parsed, path, query.export);
    println!("parsed replay in {} μs", now.elapsed().as_micros());
    println!("{:#?}", parsed.result);
}

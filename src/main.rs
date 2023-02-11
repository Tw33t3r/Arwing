use std::{
    io::{Error, ErrorKind},
    path::{Path, PathBuf},
    process,
};

use clap::{arg, command, value_parser, Arg, ArgAction};

use arwing::{check_players, parse_game, read_game, Query, Interaction};
use peppi::model::enums::character::Internal;

fn parse_internal_character(env: &str) -> Result<Internal, Error> {
    match Internal::try_from(env) {
        Ok(character) => Ok(character),
        Err(_) => Err(Error::new(
            ErrorKind::Other,
            "Character does not exist",
        )),
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
    
    let player: &Internal = matches.get_one("player").unwrap();
    let opponent: &Internal = matches.get_one("opponent").unwrap();
    let path: &PathBuf = matches.get_one("directory").unwrap();
    //TODO(Tweet): begin to parse matches
    let interactions: Vec<Interaction> = matches.get_many("interaction");

    //    let query = query::build(&args).unwrap_or_else(|err| {
    //        println!("couldn't parse arguments: {err}");
    //        process::exit(1);
    //    });
    //
    //    let now = std::time::Instant::now();
    //    let path = path::new("test.slp");
    //    let game = read_game(path).unwrap();
    //    let players = check_players(&game, &query).unwrap();
    //    let parsed = parse_game(game, query, players).unwrap();
    //    //create_json(parsed, path, query.export);
    //    println!("parsed replay in {} μs", now.elapsed().as_micros());
    //    println!("{:#?}", parsed.result);
}

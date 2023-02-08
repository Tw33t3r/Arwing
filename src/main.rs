use std::{env, process, error::Error, fs::File, path::Path};

use peppi::model::{
    enums::{
        action_state::{self, State},
        character::External,
    },
    primitives::Port,
};

use arwing::{read_game, parse_game, Query, QueryResult, Interaction};



fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let query = Query::build(&args).unwrap_or_else(|err| {
        println!("Couldn't parse arguments: {err}");
        process::exit(1);
    });

    let now = std::time::Instant::now();
    let path = Path::new("test.slp");
    let game = read_game(path).unwrap();
    let parsed = parse_game(game, query).unwrap();
    println!("Parsed replay in {} μs", now.elapsed().as_micros());
    println!("{:#?}", parsed.frame_indices);
}

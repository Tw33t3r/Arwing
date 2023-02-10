use std::{env, error::Error, fs::File, path::Path, process};

use arwing::{check_players, parse_game, read_game, Query};

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = Query::build(&args).unwrap_or_else(|err| {
        println!("Couldn't parse arguments: {err}");
        process::exit(1);
    });

    let now = std::time::Instant::now();
    let path = Path::new("test.slp");
    let game = read_game(path).unwrap();
    let players = check_players(&game, &query).unwrap();
    let parsed = parse_game(game, query, players).unwrap();
    println!("Parsed replay in {} μs", now.elapsed().as_micros());
    println!("{:#?}", parsed.result);
}

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{ffi::OsStr, fs::canonicalize, path::PathBuf};

use arwing_core::{
    check_players, create_json, interaction::Interaction, parse_game, read_game, ParsedGame,
};
use glob::glob;
use peppi::model::enums::character::Internal;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            scan_for_interactions,
            export_to_json
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn scan_for_interactions(
    path_string: String,
    player: u8,
    opponent: u8,
    interactions: Vec<Interaction>,
) -> Result<Vec<ParsedGame>, String> {
    let mut parsed_games: Vec<ParsedGame> = Vec::new();
    let player_char = Internal(player);
    let opponent_char = Internal(opponent);

    let path = PathBuf::from(&path_string);
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
                         let players_result = check_players(&game, player_char, opponent_char);
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
        let players = check_players(&game, player_char, opponent_char).unwrap();
        let parsed = parse_game(game, &interactions, players).unwrap();
        parsed_games.push(ParsedGame {
            query_result: parsed,
            loc: canonicalize(path).unwrap(),
        });
    }
    Ok(parsed_games)
}

#[tauri::command]
fn export_to_json(export_location: String, parsed_games: Vec<ParsedGame>) {
    print!("{:?}", export_location);
    create_json(parsed_games, PathBuf::from(export_location));
}

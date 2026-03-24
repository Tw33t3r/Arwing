// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{ffi::OsStr, fs::canonicalize, path::PathBuf};

use arwing_core::{
    ParsedGame, check_players, create_json, interaction::Interaction, interaction::InteractionCond,
    parse_game, read_game,
};
use glob::glob;
use ssbm_data::character::External;

fn main() {
    //TEMP https://github.com/tauri-apps/tauri/issues/10702
    if std::env::var("WAYLAND_DISPLAY").is_ok() {
        unsafe {
            std::env::set_var("__NV_DISABLE_EXPLICIT_SYNC", "1");
        }
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            scan_for_interactions,
            export_to_json
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command(async)]
fn scan_for_interactions(
    path_string: String,
    player: u8,
    opponent: u8,
    interactions: Vec<Interaction>,
) -> Result<Vec<ParsedGame>, String> {
    let mut parsed_games: Vec<ParsedGame> = Vec::new();
    let player_char = External::try_from(player).unwrap();
    let opponent_char = External::try_from(opponent).unwrap();
    let interaction_conds: Vec<InteractionCond> = interactions
        .into_iter()
        .map(InteractionCond::Single)
        .collect();

    let path = PathBuf::from(&path_string);
    if path.is_dir() {
        for entry in glob(
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
                        //TODO: Pass charater into here
                        let players_result =
                            check_players(&game, player_char, opponent_char, None, None);
                        if let Some(players) = players_result {
                            let parsed = parse_game(game, &interaction_conds, players).unwrap();
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
        let game = read_game(path.as_path()).unwrap();
        //TODO: Pass charater into here instead of inintializing to None
        let players = check_players(&game, player_char, opponent_char, None, None).unwrap();
        let parsed = parse_game(game, &interaction_conds, players).unwrap();
        parsed_games.push(ParsedGame {
            query_result: parsed,
            loc: canonicalize(path).unwrap(),
        });
    }
    Ok(parsed_games)
}

#[tauri::command]
fn export_to_json(export_location: String, parsed_games: Vec<ParsedGame>) {
    create_json(parsed_games, PathBuf::from(export_location));
}

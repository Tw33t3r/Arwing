#![allow(non_snake_case)]
// Non snake case is allowed in order for serde to export proper json to clippi
// It seems like allowing for specific lines is broken in this case
use std::{
    error::Error,
    fs, io,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use peppi::frame::immutable::Frame;
use peppi::game::immutable::Game;
use peppi::io::slippi::read;

use ssbm_data::character::External;

pub mod characters;
pub mod interaction;

#[derive(Serialize, Deserialize)]
pub struct QueryResult {
    pub result: Vec<Vec<usize>>,
}

#[derive(Serialize, Deserialize)]
pub struct ParsedGame {
    pub query_result: QueryResult,
    pub loc: PathBuf,
}

pub struct Characters {
    pub p1: External,
    pub p2: External,
}

#[derive(Serialize)]
struct ClippiJson {
    pub queue: Vec<DolphinEntry>,
}

#[derive(Serialize)]
struct DolphinEntry {
    path: PathBuf,
    startFrame: usize,
    endFrame: usize,
}

//TODO(Tweet): We can change the return of this from character to port numbers, then we don't need
//to worry about wrong character errors later on
pub fn check_players(game: &Game, player: External, opponent: External) -> Option<Characters> {
    let players = &game.start.players;
    if players.len() != 2 {
        return None;
    }
    let p1: External;
    let p2: External;
    if players[0].character == player as u8 && players[1].character == opponent as u8 {
        p1 = player;
        p2 = opponent;
    } else if players[1].character == player as u8 && players[0].character == opponent as u8 {
        p1 = opponent;
        p2 = player;
    } else {
        return None;
    }
    Some(Characters { p1, p2 })
}

pub fn read_game(infile: &Path) -> Result<Game, Box<dyn Error>> {
    let mut buf = io::BufReader::new(
        fs::File::open(infile)
            .map_err(|e| format!("couldn't open `{}`: {}", infile.display(), e))?,
    );
    let game = read(&mut buf, None)?;
    Ok(game)
}

pub fn parse_game(
    game: Game,
    interactions: &[interaction::InteractionCond],
    players: Characters,
) -> Result<QueryResult, Box<dyn Error>> {
    if game.frames.ports.len() != 2 {
        panic!("Only 2 player games are supported at this moment.");
    }
    let result: QueryResult = parse_frames(game.frames, interactions, players).unwrap();
    Ok(result)
}

// Frame is in struct-of-arrays format. Deeply nested values each have their own array
// ranging from beginning to end of frame.id<i32>.
pub fn parse_frames(
    frames: Frame,
    interactions: &[interaction::InteractionCond],
    players: Characters,
) -> Result<QueryResult, Box<dyn Error>> {
    let mut active_matches: Vec<interaction::MatchState> = Vec::new();
    let mut results = Vec::new();
    let mut previous_frame = [u16::MAX, u16::MAX];

    for index in 0..frames.id.len() {
        //With this the way it is we will end up checking a lot of unnecessary frames in non-mirror
        //matchups.
        for (port_index, port) in frames.ports.iter().enumerate() {
            let port_character = match port_index {
                0 => players.p1,
                1 => players.p2,
                _ => panic!("Attempting to parse a game with more than 2 players"),
            };

            let post_frame = port.leader.post.state.get(index).unwrap();

            if post_frame == previous_frame[port_index] {
                continue;
            }

            let l_cancel_state = port
                .leader
                .post
                .l_cancel
                .as_ref()
                .expect("l_cancel")
                .get(index);

            //advance matches
            active_matches.retain_mut(|state| {
                let cond = &interactions[state.step];
                match cond.matches(
                    post_frame,
                    l_cancel_state,
                    &mut state.remaining,
                    port_character,
                ) {
                    interaction::InteractionResult::Target => {
                        state.indices.push(index);
                        state.step += 1;
                        println!("state {:?}", state);
                        println!("results {:?}", results);
                        if state.step == interactions.len() {
                            println!("pushing results at index: {:?}", index);
                            results.push(state.indices.clone());
                            return false;
                        }

                        println!("setting remaining at index: {:?}", index);
                        state.remaining = interactions[state.step].within();
                    }

                    interaction::InteractionResult::TimeOut => {
                        return false;
                    }

                    interaction::InteractionResult::WrongCharacter
                    | interaction::InteractionResult::NonContiguous
                    | interaction::InteractionResult::GameStateMismatch => {}
                }

                true
            });

            //start new match if no branch is in at state 0
            if interactions[0].matches(post_frame, l_cancel_state, &mut None, port_character)
                == interaction::InteractionResult::Target
            {
                if interactions.len() == 1 {
                    results.push(vec![index]);
                } else {
                    active_matches.push(interaction::MatchState {
                        step: 0,
                        remaining: interactions[0].within(),
                        indices: vec![index],
                    })
                };
            }

            //update previous frame
            previous_frame[port_index] = post_frame;
        }
    }
    Ok(QueryResult { result: results })
}

pub fn create_json(games: Vec<ParsedGame>, output_loc: PathBuf) {
    let mut clippi: ClippiJson = ClippiJson { queue: vec![] };

    let game_iter = games.iter();
    for (_, game) in game_iter.enumerate() {
        //TODO(Tweet): game.query_result.result is pretty ugly here, maybe refactor the structs
        let occurrence_iter = game.query_result.result.iter();
        for (_, occurrence) in occurrence_iter.enumerate() {
            // slp files start at index -123. User input starts at 0
            let first_frame = {
                if occurrence[0] < 400 {
                    0
                } else {
                    occurrence[0] - 400
                }
            };
            let last_frame: usize = match occurrence.last() {
                Some(frame) => *frame + 200,
                None => panic!("No value found in matched frames"),
            };
            clippi.queue.push(DolphinEntry {
                path: game.loc.to_path_buf(),
                startFrame: first_frame,
                endFrame: last_frame,
            });
        }
    }

    let json = serde_json::to_string(&clippi);
    match json {
        Ok(json) => {
            //TODO(Tweet): Let's not panic here
            fs::write(output_loc, json).unwrap_or_else(|_| panic!("Could not write to file"));
        }
        Err(_) => panic!("Could not serialize the gathered query data"),
    }
}

//TODO If it's possible, try avoiding reading all of the game into memory at a time
#[cfg(test)]
mod tests {
    use super::*;
    use ssbm_data::{
        action_state::{Common, Fox},
        character::External,
    };

    #[test]
    fn special_moves() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("test/test.slp");
        let game = read_game(path.as_path()).unwrap();
        let character = External::Fox;
        let opponent = External::Pikachu;
        let interactions = vec![interaction::InteractionCond::Single(
            interaction::Interaction {
                action: Fox::BlasterAirLoop as u16,
                from_player: External::Fox,
                failed_l_cancel: None,
                within: None,
            },
        )];
        let players = check_players(&game, character, opponent).unwrap();
        let parsed = parse_game(game, &interactions, players).unwrap();
        assert_eq!(
            parsed.result,
            vec![
                [1161,],
                [1181,],
                [2489,],
                [2702,],
                [2895,],
                [4874,],
                [5633,],
                [7174,],
                [11075,],
                [11095,],
                [11124,],
                [14038,],
                [14059,],
                [14395,],
            ]
        );
    }

    #[test]
    fn dairs() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("test/test.slp");
        let game = read_game(path.as_path()).unwrap();
        let character = External::Fox;
        let opponent = External::Pikachu;
        let interactions = vec![interaction::InteractionCond::Single(
            interaction::Interaction {
                action: Common::AttackAirLw as u16,
                from_player: External::Fox,
                failed_l_cancel: None,
                within: None,
            },
        )];
        let players = check_players(&game, character, opponent).unwrap();
        let parsed = parse_game(game, &interactions, players).unwrap();
        assert_eq!(
            parsed.result,
            [
                [605],
                [1345],
                [1667],
                [3261],
                [3645],
                [4081],
                [4907],
                [5174],
                [5834],
                [5877],
                [6943],
                [7512],
                [8387],
                [8444],
                [8725],
                [9038],
                [10230],
                [10702],
                [10808],
                [10954],
                [11015],
                [11190],
                [12272],
                [12984],
                [13251]
            ]
        )
    }

    #[test]
    fn dairs_that_hit() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("test/test.slp");
        let game = read_game(path.as_path()).unwrap();
        let character = External::Fox;
        let opponent = External::Pikachu;
        let interactions = vec![
            interaction::InteractionCond::Single(interaction::Interaction {
                action: Common::AttackAirLw as u16,
                from_player: External::Fox,
                failed_l_cancel: None,
                within: None,
            }),
            interaction::InteractionCond::Single(interaction::Interaction {
                action: Common::DamageAir2 as u16,
                failed_l_cancel: None,
                from_player: External::Pikachu,
                within: Some(200),
            }),
        ];
        let players = check_players(&game, character, opponent).unwrap();
        let parsed = parse_game(game, &interactions, players).unwrap();
        assert_eq!(parsed.result, [[3645, 3661], [6943, 6947], [12272, 12276]])
    }

    #[test]
    fn dairs_that_miss_l_cancel() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("test/test.slp");
        let game = read_game(path.as_path()).unwrap();
        let character = External::Pikachu;
        let opponent = External::Fox;
        let interactions = vec![interaction::InteractionCond::Single(
            interaction::Interaction {
                action: Common::LandingAirB as u16,
                failed_l_cancel: Some(true),
                from_player: External::Pikachu,
                within: None,
            },
        )];
        let players = check_players(&game, character, opponent).unwrap();
        let parsed = parse_game(game, &interactions, players).unwrap();
        assert_eq!(parsed.result, [[463], [7756], [9757]])
    }
}

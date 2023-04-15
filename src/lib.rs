#![allow(non_snake_case)]
// Non snake case is allowed in order for serde to export proper json to clippi
// It seems like allowing for specific lines is broken in this case
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use peekread::BufPeekReader;
use serde::Serialize;

use peppi::model::{
    enums::{
        action_state::{Common, State},
        character::Internal,
    },
    frame::{Frame, PortData},
    game::{Frames, Game},
};

#[derive(Debug)]
pub struct Interaction {
    pub action: State,
    pub from_player: Internal,
    pub within: Option<u32>,
}

pub struct QueryResult {
    pub result: Vec<Vec<usize>>,
}

pub struct ParsedGame {
    pub query_result: QueryResult,
    pub loc: PathBuf,
}

pub struct Characters {
    pub p1: Internal,
    pub p2: Internal,
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

enum InteractionResult {
    TimeOut,
    WrongCharacter,
    NonContiguous,
    Target,
}

pub fn check_players(game: &Game, player: Internal, opponent: Internal) -> Option<Characters> {
    match &game.metadata.players {
        Some(players) => {
            if players.len() != 2 {
                return None;
            }
            let p1;
            let p2;
            let char1_map = &players[0].characters.as_ref()?;
            let char2_map = &players[1].characters.as_ref()?;
            // This is ugly, but it saves us from iterating over keys
            // Might break on zelda
            if char1_map.contains_key(&player) {
                p1 = player;
                if char2_map.contains_key(&opponent) {
                    p2 = opponent;
                } else {
                    return None;
                };
            } else if char1_map.contains_key(&opponent) {
                p1 = opponent;
                if char2_map.contains_key(&player) {
                    p2 = player;
                } else {
                    return None;
                };
            } else {
                return None;
            }
            return Some(Characters { p1, p2 });
        }
        None => return None,
    }
}

pub fn read_game(infile: &Path) -> Result<Game, Box<dyn Error>> {
    let mut buf = BufPeekReader::new(
        fs::File::open(infile)
            .map_err(|e| format!("couldn't open `{}`: {}", infile.display(), e))?,
    );
    buf.set_min_read_size(8192);
    let game = peppi::game(&mut buf, None, None)?;
    Ok(game)
}

pub fn parse_game(
    game: Game,
    interactions: &Vec<Interaction>,
    players: Characters,
) -> Result<QueryResult, Box<dyn Error>> {
    let result: QueryResult;
    match game.frames {
        Frames::P2(frames) => {
            result = parse_frames(frames, &interactions, players).unwrap();
        }
        _ => panic!("Only 2 player games are supported at this moment."),
    }
    Ok(result)
}

pub fn parse_frames(
    frames: Vec<Frame<2>>,
    interactions: &Vec<Interaction>,
    players: Characters,
) -> Result<QueryResult, Box<dyn Error>> {
    let mut target_indices: Vec<usize> = Vec::new();
    let mut result = Vec::new();

    //Some state that doesn't exist in non-crazy-hand games
    let mut previous_frame = [
        State::Common(Common::CAPTURE_CRAZY_HAND),
        State::Common(Common::CAPTURE_CRAZY_HAND),
    ];

    let mut interaction_iter = interactions.iter();
    let mut target_interaction = match interaction_iter.next() {
        Some(interaction) => interaction,
        None => panic!("There were no interactions listed when parsing frames"),
    };

    let mut remaining = target_interaction.within;

    let iter = frames.iter();

    for (index, frame) in iter.enumerate() {
        //With this the way it is we will end up checking a lot of unnecessary frames in non-mirror
        //matchups.
        //TODO way too many indentations
        for (port_index, port) in frame.ports.iter().enumerate() {
            let port_character = match port_index {
                0 => players.p1,
                1 => players.p2,
                _ => panic!("Attempting to parse a game with more than 2 players"),
            };
            match check_interaction(port, target_interaction, &mut remaining, port_character) {
                InteractionResult::WrongCharacter => (),
                InteractionResult::TimeOut => {
                    //reset
                    interaction_iter = interactions.iter();
                    let reset_interaction = match interaction_iter.next() {
                        Some(next_interaction) => next_interaction,
                        None => panic!(
                            "When resetting internal interactions no interactions were found"
                        ),
                    };
                    target_indices = Vec::new();
                    remaining = reset_interaction.within;
                    target_interaction = reset_interaction;
                }
                InteractionResult::NonContiguous => (),
                InteractionResult::Target => {
                    if previous_frame[port_index] != port.leader.post.state {
                        //TODO Excessively moving memory around in this block
                        target_interaction = match interaction_iter.next() {
                            Some(interaction) => {
                                target_indices.push(index);
                                remaining = interaction.within;
                                interaction
                            }
                            None => {
                                interaction_iter = interactions.iter();
                                let reset_interaction = match interaction_iter.next(){
                                    Some(next_interaction) => next_interaction,
                                    None => panic!(
                                        "When resetting internal interactions no interactions were found"
                                        ),
                                };
                                target_indices.push(index);
                                result.push(target_indices);
                                target_indices = Vec::new();
                                remaining = reset_interaction.within;
                                reset_interaction
                            }
                        };
                    }
                }
            };
            previous_frame[port_index] = port.leader.post.state;
        }
    }
    Ok(QueryResult { result })
}

fn check_interaction(
    port: &PortData,
    target: &Interaction,
    remaining: &mut Option<u32>,
    character: Internal,
) -> InteractionResult {
    if let Some(amount) = remaining {
        if amount == &0 {
            return InteractionResult::TimeOut;
        }
        *remaining = Some(*amount - 1);
    }

    let post_frame = port.leader.post;
    if character != target.from_player {
        return InteractionResult::WrongCharacter;
    }
    if post_frame.state == target.action {
        return InteractionResult::Target;
    }
    return InteractionResult::NonContiguous;
}

pub fn create_json(games: Vec<ParsedGame>, output_loc: PathBuf) {
    let mut clippi: ClippiJson = ClippiJson { queue: vec![] };

    let game_iter = games.iter();
    for (_, game) in game_iter.enumerate() {
        //TODO(Tweet): Test speed of this vs copying game.loc in the clippi.queue.push
        let loc = &game.loc;
        //TODO(Tweet): game.query_result.result is pretty ugly here, maybe refactor the structs
        let occurrence_iter = game.query_result.result.iter();
        for (_, occurrence) in occurrence_iter.enumerate() {
            //TODO(Tweet): Offset first frame, by the -frames found in slippi,
            //TODO(Tweet): add buffers so players have context and can see results more
            let first_frame = occurrence[0];
            let last_frame: usize = match occurrence.last() {
                Some(frame) => *frame,
                None => panic!("No value found in matched frames"),
            };
            clippi.queue.push(DolphinEntry {
                path: loc.to_path_buf(),
                startFrame: first_frame,
                endFrame: last_frame,
            });
        }
    }

    let json = serde_json::to_string(&clippi);
    match json {
        Ok(json) => {
            fs::write(output_loc, json).unwrap_or_else(|_| panic!("Could not write to file"));
        }
        Err(_) => panic!("Could not serialize the gathered query data"),
    }
}

//TODO If it's possible, try avoiding reading all of the game into memory at a time
//TODO Parse through multiple files
#[cfg(test)]
mod tests {
    use peppi::model::enums::action_state::{Common, Fox};

    use super::*;

    #[test]
    fn special_moves() {
        let path = Path::new("test.slp");
        let game = read_game(path).unwrap();
        let character = Internal::FOX;
        let opponent = Internal::PIKACHU;
        let interactions = vec![Interaction {
            action: State::Fox(Fox::BLASTER_AIR_LOOP),
            from_player: Internal::FOX,
            within: None,
        }];
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
        let path = Path::new("test.slp");
        let game = read_game(path).unwrap();
        let character = Internal::FOX;
        let opponent = Internal::PIKACHU;
        let interactions = vec![Interaction {
            action: State::Common(Common::ATTACK_AIR_LW),
            from_player: Internal::FOX,
            within: None,
        }];
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
        let path = Path::new("test.slp");
        let game = read_game(path).unwrap();
        let character = Internal::FOX;
        let opponent = Internal::PIKACHU;
        let interactions = vec![
            Interaction {
                action: State::Common(Common::ATTACK_AIR_LW),
                from_player: Internal::FOX,
                within: None,
            },
            Interaction {
                action: State::Common(Common::DAMAGE_AIR_2),
                from_player: Internal::PIKACHU,
                within: Some(200),
            },
        ];
        let players = check_players(&game, character, opponent).unwrap();
        let parsed = parse_game(game, &interactions, players).unwrap();
        assert_eq!(parsed.result, [[3645, 3661], [6943, 6947], [12272, 12276]])
    }
}

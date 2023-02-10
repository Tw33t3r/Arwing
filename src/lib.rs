use std::{error::Error, fs::File, path::Path};

use peekread::BufPeekReader;

use peppi::model::{
    enums::{action_state::State, character::Internal},
    frame::{Frame, PortData},
    game::{Frames, Game},
};

#[derive(Debug)]
pub struct Interaction {
    pub action: State,
    pub from_player: Internal,
    pub within: Option<u32>,
}

pub struct Query {
    pub player_name: Option<String>,
    pub character: Internal,
    pub opponent: Internal,
    pub interactions: Vec<Interaction>,
}

impl Query {
    //TODO learn clap, and try to use it instead of this
    pub fn build(mut args: &[String]) -> Result<Query, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let mut name = None;
        if args[1] == "--name" {
            name = Some(&args[2]);
            args = &args[3..];
        }

        let character_result = Internal::try_from(&args[1][..]);
        let character = match character_result {
            Ok(Internal(character)) => character,
            Err(error) => panic!("Couldn't match the player's character {:?}", error),
        };

        let opponent_result = Internal::try_from(&args[2][..]);
        let opponent = match opponent_result {
            Ok(Internal(opponent)) => opponent,
            Err(error) => panic!("Couldn't match the opponent's character {:?}", error),
        };

        args = &args[3..];
        println!("{:?}", args);
        let interactions: Vec<Interaction> = args
            .chunks(3)
            .map(|interaction| {
                let from_player_result = Internal::try_from(&args[1][..]);
                let from_player = match from_player_result {
                    Ok(Internal(from_player)) => from_player,
                    Err(error) => {
                        panic!("Couldn't match the from_player in interactions {:?}", error)
                    }
                };
                Interaction {
                    //TODO Double check whether I want to use STATE names or state numbers slippi uses
                    action: State::from(interaction[0].parse().unwrap(), Internal(character)),
                    //TODO Refactor to use the input opponent, or character to handle dittos better
                    from_player: Internal(from_player),
                    within: match interaction[2].as_str() {
                        "None" => None,
                        other => Some(other.parse().unwrap()),
                    },
                }
            })
            .collect();
        Ok(Query {
            //TODO I think cloned is slow here
            player_name: name.cloned(),
            character: Internal(character),
            opponent: Internal(opponent),
            interactions,
        })
    }
}

pub struct QueryResult {
    pub result: Vec<Vec<usize>>,
}

pub struct Characters {
    pub p1: Internal,
    pub p2: Internal,
}

enum InteractionResult {
    TimeOut,
    WrongCharacter,
    NonContiguous,
    Target,
}

pub fn check_players(game: Game, query: Query) -> Option<Characters> {
    match game.metadata.players {
        Some(players) => {
            if players.len() != 2 {
                return None;
            }
            let p1;
            let p2;
            let char1_map = &players[0].characters.as_ref()?;
            let char2_map = &players[1].characters.as_ref()?;
            // This is ugly, but it saves us from iterating over keys
            if char1_map.contains_key(&query.character) {
                p1 = query.character;
                if char2_map.contains_key(&query.opponent) {
                    p2 = query.opponent;
                } else {
                    return None;
                };
            } else if char1_map.contains_key(&query.opponent) {
                p1 = query.opponent;
                if char2_map.contains_key(&query.character) {
                    p2 = query.character;
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
        File::open(infile).map_err(|e| format!("couldn't open `{}`: {}", infile.display(), e))?,
    );
    buf.set_min_read_size(8192);
    let game = peppi::game(&mut buf, None, None)?;
    Ok(game)
}

pub fn parse_game(game: Game, query: Query, players: Characters) -> Result<QueryResult, Box<dyn Error>> {
    let result: QueryResult;
    match game.frames {
        Frames::P2(frames) => {
            result = parse_frames(frames, query.interactions, players).unwrap();
        }
        _ => panic!("Only 2 player games are supported at this moment."),
    }
    Ok(result)
}

pub fn parse_frames(
    frames: Vec<Frame<2>>,
    interactions: Vec<Interaction>,
    players: Characters
) -> Result<QueryResult, Box<dyn Error>> {
//!TODO continue to replace character with players from this point on
    let mut target_indices: Vec<usize> = Vec::new();
    let mut result = Vec::new();

    let mut contiguous = &false;

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
        for port in &frame.ports {
            match check_interaction(port, target_interaction, &mut remaining) {
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
                    target_interaction = reset_interaction;
                }
                InteractionResult::NonContiguous => contiguous = &false,
                InteractionResult::Target => {
                    if !contiguous {
                        //TODO Excessively moving memory around in this block
                        target_interaction = match interaction_iter.next() {
                            Some(interaction) => {
                                target_indices.push(index);
                                contiguous = &true;
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
                                contiguous = &true;
                                remaining = reset_interaction.within;
                                reset_interaction
                            }
                        }
                    }
                }
            };
        }
    }
    Ok(QueryResult { result })
}

fn check_interaction(
    port: &PortData,
    target: &Interaction,
    remaining: &mut Option<u32>,
) -> InteractionResult {
    if let Some(amount) = remaining {
        if amount == &0 {
            return InteractionResult::TimeOut;
        }
        *remaining = Some(*amount - 1);
    }

    let post_frame = port.leader.post;
    if post_frame.character != target.from_player {
        return InteractionResult::WrongCharacter;
    }
    if post_frame.state != target.action {
        return InteractionResult::NonContiguous;
    } else {
        return InteractionResult::Target;
    }
}

//TODO If it's possible, try using serde

#[cfg(test)]
mod tests {
    use peppi::model::enums::action_state::{Common, Fox};

    use super::*;

    #[test]
    fn special_moves() {
        let path = Path::new("test.slp");
        let game = read_game(path).unwrap();
        let query = Query {
            player_name: None,
            character: Internal::FOX,
            opponent: Internal::PIKACHU,
            interactions: vec![Interaction {
                action: State::Fox(Fox::BLASTER_AIR_LOOP),
                from_player: Internal::FOX,
                within: None,
            }],
        };
        let parsed = parse_game(game, query).unwrap();
        println!("{:#?}", parsed.result);
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
        let query = Query {
            player_name: None,
            character: Internal::FOX,
            opponent: Internal::PIKACHU,
            interactions: vec![Interaction {
                action: State::Common(Common::ATTACK_AIR_LW),
                from_player: Internal::FOX,
                within: None,
            }],
        };
        let parsed = parse_game(game, query).unwrap();
        println!("{:#?}", parsed.result);
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
        let query = Query {
            player_name: None,
            character: Internal::FOX,
            opponent: Internal::PIKACHU,
            interactions: vec![
                Interaction {
                    action: State::Common(Common::ATTACK_AIR_LW),
                    from_player: Internal::FOX,
                    within: None,
                },
                Interaction {
                    action: State::Common(Common::DAMAGE_FLY_TOP),
                    from_player: Internal::PIKACHU,
                    within: Some(60),
                },
            ],
        };
        let parsed = parse_game(game, query).unwrap();
        println!("{:#?}", parsed.result);
        assert_eq!(parsed.result, [[1230]])
    }
}
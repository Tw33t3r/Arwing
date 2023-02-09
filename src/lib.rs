use std::{error::Error, fs::File, path::Path};

use peekread::BufPeekReader;

use peppi::model::{
    enums::{action_state::State, character::Internal},
    frame::Frame,
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
    pub frame_indices: Vec<usize>,
}

pub fn read_game(infile: &Path) -> Result<Game, Box<dyn Error>> {
    let mut buf = BufPeekReader::new(
        File::open(infile).map_err(|e| format!("couldn't open `{}`: {}", infile.display(), e))?,
    );
    buf.set_min_read_size(8192);
    let game = peppi::game(&mut buf, None, None)?;
    Ok(game)
}

pub fn parse_game(game: Game, query: Query) -> Result<QueryResult, Box<dyn Error>> {
    let result: QueryResult;
    match game.frames {
        Frames::P2(frames) => {
            result = parse_frames(frames, query.interactions).unwrap();
        }
        _ => panic!("Only 2 player games are supported at this moment."),
    }
    Ok(result)
}

pub fn parse_frames(
    frames: Vec<Frame<2>>,
    interactions: Vec<Interaction>,
) -> Result<QueryResult, Box<dyn Error>> {
    let mut frame_indices = Vec::new();

    let mut contiguous = &false;
    let mut first_frame = 0;
    let mut interaction_iter = interactions.iter();
    let mut target_interaction = match interaction_iter.next() {
        Some(interaction) => interaction,
        None => panic!("There were no interactions listed when parsing frames"),
    };

    let iter = frames.iter();
    // We need to:
    // Step through each interaction piece by piece, when we reach the end then we push, the first
    // frame of the occurence into the vec of frame_indices
    // During each step of the interaction we need to check that it is:
    // The first frame of the correct state
    // Done by the correct character
    // Within the amount of frames specified from the previous move.

    // TODO Change contiguous, add within, investigate why we aren't getting output from test
    for (index, frame) in iter.enumerate() {
        for port in &frame.ports {
            let post_frame = port.leader.post;
            if post_frame.character != target_interaction.from_player {
                break;
            }
            if post_frame.state != target_interaction.action {
            contiguous = &false;
            }
            if !contiguous {
                frame_indices.push(index);
                contiguous = &true;
            }
        }
    }
    Ok(QueryResult { frame_indices })
}
//TODO parse game using serde instead of manually

#[cfg(test)]
mod tests {
    use peppi::model::enums::action_state::Fox;

    use super::*;

    #[test]
    fn amount_of_lasers() {
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
        println!("{:#?}", parsed.frame_indices);
        assert_eq!(
            parsed.frame_indices,
            vec![
                1161, 1181, 2489, 2702, 2895, 4874, 5633, 7174, 11075, 11095, 11124, 14038, 14059,
                14395
            ]
        );
    }
}

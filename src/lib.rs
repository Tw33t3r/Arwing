use std::{error::Error, fs::File, path::Path};

use peekread::BufPeekReader;

use peppi::model::{
    enums::{action_state::{State, self}, character::External},
    frame::Frame,
    game::{Frames, Game},
    primitives::Port,
};

pub struct Interaction {
    pub action: State,
    pub from_player: Port,
    pub within: Option<u32>,
}

pub struct Query {
    pub player_name: Option<String>,
    pub character: External,
    pub opponent: External,
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
        
        let character_result = External::try_from(&args[1][..]);
        let character = match character_result{
            Ok(External(character)) => character,
            Err(error) => panic!("Couldn't match the player's character {:?}", error),
        };
        let opponent_result = External::try_from(&args[2][..]);
        let opponent = match opponent_result{
            Ok(External(opponent)) => opponent,
            Err(error) => panic!("Couldn't match the opponent's character {:?}", error),
        };
        
        args = &args[3..];
        println!("{:?}", args);
        let mut interactions = Vec::new();
        let iter = args.chunks(3);
        //TODO I think I can collect the chunks
        //for (action, from_player, within) in iter{
            
        //}
        Ok(Query {
            //TODO I think cloned is slow here
            player_name: name.cloned(),
            character: External(character),
            opponent: External(opponent),
            interactions: interactions,
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
            result = parse_frames(frames, query).unwrap();
        }
        _ => panic!("Only 2 player games are supported at this moment."),
    }
    Ok(result)
}

pub fn parse_frames(frames: Vec<Frame<2>>, query: Query) -> Result<QueryResult, Box<dyn Error>> {
    let mut frame_indices = Vec::new();
    let mut target_state = &query.interactions[0];
    let mut contiguous = &false;
    let iter = frames.iter();
    //TODO Using windows here would be much better
    for (index, frame) in iter.enumerate() {
        //TODO This line is very ugly
        if frame.ports[1].leader.post.state == target_state.action {
            if !contiguous {
                frame_indices.push(index);
                contiguous = &true;
            }
        } else {
            //TODO Definitely change this
            contiguous = &false;
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
            character: External::FOX,
            opponent: External::PIKACHU,
            interactions: vec![Interaction {
                action: State::Fox(Fox::BLASTER_AIR_LOOP),
                from_player: peppi::model::primitives::Port::P2,
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

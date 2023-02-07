use std::{error::Error, fs::File, path::Path};

use peekread::BufPeekReader;

use peppi::model::{
    enums::{
        action_state::{State},
        character::External,
    },
    frame::Frame,
    game::{Frames, Game},
    primitives::Port,
};

struct Interaction {
    action: State,
    from_player: Port,
    within: Option<u32>,
}

struct Query {
    player_name: Option<String>,
    character: External,
    interactions: Vec<Interaction>,
}

struct QueryResult {
    frame_indices: Vec<usize>,
}

fn read_game(infile: &Path) -> Result<Game, Box<dyn Error>> {
    let mut buf = BufPeekReader::new(
        File::open(infile).map_err(|e| format!("couldn't open `{}`: {}", infile.display(), e))?,
    );
    buf.set_min_read_size(8192);
    let game = peppi::game(&mut buf, None, None)?;
    Ok(game)
}

fn parse_game(game: Game, query: Query) -> Result<QueryResult, Box<dyn Error>> {
    let result: QueryResult;
    match game.frames {
        Frames::P2(frames) => {
            result = parse_frames(frames, query).unwrap();
        }
        _ => panic!("Only 2 player games are supported at this moment."),
    }
    Ok(result)
}

fn parse_frames(frames: Vec<Frame<2>>, query: Query) -> Result<QueryResult, Box<dyn Error>> {
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
        let now = std::time::Instant::now();
        let path = Path::new("test.slp");
        let game = read_game(path).unwrap();
        let query = Query {
            player_name: None,
            character: External::FOX,
            interactions: vec![Interaction {
                action: State::Fox(Fox::BLASTER_AIR_LOOP),
                from_player: peppi::model::primitives::Port::P2,
                within: None,
            }],
        };
        let parsed = parse_game(game, query).unwrap();
        println!("Parsed replay in {} μs", now.elapsed().as_micros());
        println!("{:#?}", parsed.frame_indices);
        assert_eq!(parsed.frame_indices, vec![1161, 1181, 2489, 2702, 2895, 4874, 5633, 7174, 11075, 11095, 11124, 14038, 14059, 14395]);
    }
}

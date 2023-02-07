use std::{error::Error, fs::File, path::Path};

use peekread::BufPeekReader;

use peppi::model::{
    enums::action_state::{Fox, State},
    frame::Frame,
    game::{Frames, Game},
};

struct Query {
    //player_name: String,
    //character: External,
    action_state: State,
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
    let iter = frames.iter();
    for (index, frame) in iter.enumerate() {
        if frame.ports[0].leader.post.state == query.action_state {
            frame_indices.push(index);
        }
    }
    Ok(QueryResult { frame_indices })
}

fn main() {
    let now = std::time::Instant::now();
    let path = Path::new("slippi/Game_20230201T182227.slp");
    let game = read_game(path).unwrap();
    let query = Query {
        //player_name: "Me".to_string(),
        //character: External::FOX,
        action_state: State::Fox(Fox::BLASTER_AIR_LOOP),
    };
    let parsed = parse_game(game, query).unwrap();
    println!("Parsed replay in {} μs", now.elapsed().as_micros());
    println!("{:#?}", parsed.frame_indices);
}

use std::{error::Error, fs::File, io, path::Path};

use peekread::BufPeekReader;

use peppi::{
    model::{
        enums::{
            action_state::{Fox, State},
            character::External,
            item,
            stage::Stage,
        },
        frame,
        game::{Frames, Game},
    },
    serde::de::{FrameEvent, Handlers, PortId},
};

struct Query {
    player_name: String,
    character: External,
    action_state: State,
}

struct QueryResult {
    frame_indices: Vec<u16>,
}

fn read_game(infile: &Path) -> Result<Game, Box<dyn Error>> {
    let now = std::time::Instant::now();
    let mut buf = BufPeekReader::new(
        File::open(infile).map_err(|e| format!("couldn't open `{}`: {}", infile.display(), e))?,
    );
    buf.set_min_read_size(8192);
    let game = peppi::game(&mut buf, None, None)?;
    println!("Parsed replay in {} μs", now.elapsed().as_micros());
    Ok(game)
}

fn parse_game(game: Game, query: Query) -> Result<QueryResult, Box<dyn Error>> {
    //TODO: Add the port option, and 1 2 3 4 =>
    match game.frames {
        Frames::P1(frames) => {
            println!("{:#?}", frames.len());
        }
        Frames::P2(frames) => {
            println!("{:#?}", frames);
        }
        Frames::P3(frames) => {
            println!("{:#?}", frames.len());
        }
        Frames::P4(frames) => {
            println!("{:#?}", frames.len());
        }
    }
    Ok(QueryResult {
        frame_indices: vec![1],
    })
}

fn parse_frames(frames: Frames, query: Query) -> Result<QueryResult, Box<dyn Error>> {
    for frame in frames{
     
    }
    Ok(QueryResult {
        frame_indices: vec![1],
    })
}

fn main() {
    let path = Path::new("slippi/Game_20230201T182227.slp");
    let game = read_game(path).unwrap();
    let query = Query {
        player_name: "Me".to_string(),
        character: External::FOX,
        action_state: State::Fox(Fox::BLASTER_AIR_LOOP),
    };
    let parsed = parse_game(game, query).unwrap();
    println!("{:?}", parsed.frame_indices);
}

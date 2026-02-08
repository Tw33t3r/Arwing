use ssbm_data::character::External;
use std::collections::HashMap;

pub fn character_from_str(s: &str) -> Option<External> {
    let map: HashMap<&str, External> = HashMap::from([
        ("mario", External::Mario),
        ("fox", External::Fox),
        ("captainFalcon", External::CaptainFalcon),
        ("donkeyKong", External::DonkeyKong),
        ("kirby", External::Kirby),
        ("bowser", External::Bowser),
        ("link", External::Link),
        ("sheik", External::Sheik),
        ("ness", External::Ness),
        ("peach", External::Peach),
        ("popo", External::Popo),
        ("pikachu", External::Pikachu),
        ("samus", External::Samus),
        ("yoshi", External::Yoshi),
        ("jigglypuff", External::Jigglypuff),
        ("mewtwo", External::Mewtwo),
        ("luigi", External::Luigi),
        ("marth", External::Marth),
        ("zelda", External::Zelda),
        ("younglink", External::YoungLink),
        ("drmario", External::DrMario),
        ("falco", External::Falco),
        ("pichu", External::Pichu),
        ("gameandwatch", External::GameAndWatch),
        ("ganondorf", External::Ganondorf),
        ("roy", External::Roy),
        ("masterhand", External::MasterHand),
        ("crazyhand", External::CrazyHand),
        ("wireframemale", External::WireFrameMale),
        ("wireframefemale", External::WireFrameFemale),
        ("gigabowser", External::GigaBowser),
        ("sandbag", External::Sandbag),
    ]);

    let input_lower = s.to_ascii_lowercase();
    return map.get(input_lower.as_str()).copied();
}

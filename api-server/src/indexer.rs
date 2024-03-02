use wot_replays::{{read_raw_from_file, models::Replay}};
use std::{fs, io};
use std::convert::TryFrom;

pub fn index(path: String){
    log::info!("Start indexing {}", &path);
    for entry in fs::read_dir(&path).unwrap() {
        let entry = entry.unwrap();
        let maybe_file = entry.path();
        if let Some(extension) = maybe_file.extension() {
            if extension == "wotreplay" {
                let raw_replay = read_raw_from_file(maybe_file.to_str().unwrap(), false).unwrap();
                // let replay = Replay::try_from(&raw_replay).unwrap();
                match Replay::try_from(&raw_replay) {
                    Ok(replay) => {
                        println!("{} {} {}", replay.battle_info.player_name, replay.battle_info.player_vehicle, replay.battle_info.map_display_name);
                    },
                    Err(e) => {
                        log::error!("Failed to parse replay: {:?}. Reason: {}", maybe_file, e);
                    }
                };
            }
        }
    }
}
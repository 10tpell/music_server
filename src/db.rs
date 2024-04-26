use crate::track::Track;

use std::fs::File;
use std::io::{BufReader, Read, BufWriter, Write};

const DB_TRACK_FILENAME : &str = "tracks.txt";

pub fn write_tracks(track_list: Vec<Track>) {
    let tracks_json_str = serde_json::to_string(&track_list).unwrap();
    let track_file = File::create(DB_TRACK_FILENAME).unwrap();
    let mut writer = BufWriter::new(track_file);
    match writer.write_all(tracks_json_str.as_bytes()) {
        Err(error) => {
            println!("ERROR: {}", error);
            return;
        },
        _ => ()
    };
    match writer.flush() {
        Err(error) => println!("ERROR: {}", error),
        _ => ()
    }
}

pub fn read_tracks() -> Vec<Track> {
    let track_file = File::open(DB_TRACK_FILENAME).unwrap();
    let mut reader = BufReader::new(track_file);
    let mut buf = Vec::<u8>::new();
    match reader.read_to_end(&mut buf) {
        Err(error) => println!("ERROR: {}", error),
        _ => ()
    };
    
    serde_json::from_str(std::str::from_utf8(&buf).unwrap()).unwrap()
}
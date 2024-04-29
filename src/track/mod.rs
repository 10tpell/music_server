use std::sync::RwLock;
use serde::{Deserialize, Serialize};
use rocket::tokio::sync::mpsc;
use rocket::tokio::sync::mpsc::{Sender, Receiver};
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

mod wav;
mod mp3;
mod flac;
mod aiff;
mod ogg;

#[derive(PartialEq)]
struct FileExtensionMapping<'a> {
    pub extension : &'a str,
    pub extension_handler : fn(&PathBuf) -> Option<Track>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Track {
    pub title: Box<str>,
    pub path: PathBuf,
    pub duration: u64,
    pub artist: Box<str>
}

pub struct TracksStatus {
    pub refresh_done : bool,
    pub notify : Sender<()>,
    pub wait : Receiver<()>
}

impl TracksStatus {
    pub fn set_refresh_status(&mut self, status: bool) {
        self.refresh_done = status;
        self.notify.blocking_send(()).expect("Refresh status not set");
    }
}

const TRACK_PATH_LIST: [&str; 3] = ["C:/Users/travis/Music", "Dunnyaga", "C:/Users/travis/Downloads"];
const TRACK_FILE_EXT_LIST: [FileExtensionMapping; 5] = [
    FileExtensionMapping{ extension: "mp3", extension_handler: mp3::mp3_to_track },
    FileExtensionMapping{extension: "wav", extension_handler: wav::wav_to_track }, 
    FileExtensionMapping{extension: "flac", extension_handler: flac::flac_to_track}, 
    FileExtensionMapping{extension: "aiff", extension_handler: aiff::aiff_to_track },
    FileExtensionMapping{extension: "ogg", extension_handler: ogg::ogg_to_track }
];

pub fn init() -> Arc<RwLock<TracksStatus>> {
    let (sender, receiver) = mpsc::channel(3);
    Arc::new(TracksStatus {  refresh_done: false, notify: sender, wait: receiver }.into())
}

pub fn get_tracks(track_status: Arc<RwLock<TracksStatus>>) -> Vec<Track> {
    let mut track_list : Vec<Track> = Vec::<Track>::new();
    {
        {
            let mut status = track_status.write().unwrap();
            status.refresh_done = false;
        }
        for dir_path in TRACK_PATH_LIST {
            let path_buf = PathBuf::from(dir_path);
            track_list.append(&mut iter_over_dir(path_buf));
        }
        {
            let mut status = track_status.write().unwrap();
            status.refresh_done = true;
        }
    }
    println!("TRACK: Got tracks");
    track_list
}

fn iter_over_dir(path: PathBuf) -> Vec<Track> {
    let mut ret : Vec<Track> = Vec::<Track>::new();
    let paths = match fs::read_dir(path) {
        Ok(res) => res,
        Err(error) => {
            println!("Error {}", error);
            return ret;
        }
    };

    for path in paths {
        let tmp_path = path.unwrap().path();
        if tmp_path.is_dir() {
            ret.append(&mut iter_over_dir(tmp_path));
        } else {
            let file_ext = match tmp_path.extension() {
                Some(file) => file,
                None => continue
            };

            for mapping in TRACK_FILE_EXT_LIST {
                if mapping.extension == file_ext {
                    let tmp_track = match (mapping.extension_handler)(&tmp_path) {
                        Some(file) => file,
                        _ => continue
                    };
                    ret.push(tmp_track);
                }
            }
        }
    }
    ret
}

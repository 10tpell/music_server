extern crate mp3_metadata;

use std::{panic, path::PathBuf};
use crate::track::Track;

pub fn mp3_to_track(filepath: &PathBuf) -> Option<Track> {
    let meta = match panic::catch_unwind(|| {
        match mp3_metadata::read_from_file(filepath) {
            Ok(meta) => Ok(meta),
            Err(error) => Err(error)
        }
    }) {
        Ok(meta) => {
            match meta {
                Ok(meta) => meta,
                Err(_) => {
                    println!("MP3: Error reading file");
                    return None;
                }
            }
        }
        Err(error) => { 
            println!("MP3: ERROR reading file: {:?}", error);
            return None;
        }
    };

    match meta.tag {
        Some(tag) => {
            println!("MP3: found tag: {} {} {}", tag.title.trim(), tag.artist.trim(), meta.duration.as_secs());
            Some(Track {
                title: tag.title.trim().into(),
                path: filepath.to_path_buf(),
                duration: meta.duration.as_secs(),
                artist: tag.artist.trim().into()
            })
        },
        // handle the case where no tag was found
        None => {
            println!("MP3: didn't find tag, using filename");
            Some(Track {title: filepath.file_name().unwrap().to_str().unwrap().into(), path: filepath.to_path_buf(), duration: meta.duration.as_secs(), artist: "Unkown Artist".into() })
        }
    }
}
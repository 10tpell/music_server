use std::path::PathBuf;


use crate::track::Track;

pub fn aiff_to_track(filepath: &PathBuf) -> Option<Track> {
    Some(Track {title: filepath.file_name().unwrap().to_str().unwrap().into(), path: filepath.to_path_buf(), duration: 300})
}
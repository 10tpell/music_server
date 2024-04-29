use wavers::Wav;
use std::path::PathBuf;
use std::panic;


use crate::track::Track;

pub fn wav_to_track(filepath: &PathBuf) -> Option<Track> {
    // Wav::<i16>::from_path() panics on certain file formats, this isn't an issue for us so we catch it return None
    // the function comes from wavers, perhaps this should be changed to return None rather than panic
    let wav_option = panic::catch_unwind(|| { 
        match Wav::<i16>::from_path(filepath) {
            Err(error) => {
                println!("WAV: ERROR reading file: {}", error);
                return None;
            }
            Ok(file) => Some(file)
        }
    });
    let wav = match wav_option {
        Err(_) => {
            println!("WAV: ERROR: Panic");
            return None;
        },
        Ok(file) => match file {
            Some(file) => file,
            None => return None
        }
    };

    println!("WAV header: {}", wav.header().to_string());
    Some(Track {title: filepath.file_name().unwrap().to_str().unwrap().into(), path: filepath.to_path_buf(), duration: wav.duration().into(), artist: "Unkown Artist".into()})
}
use std::path::PathBuf;
use rocket::response::status;
use rocket::State;
use crate::track::TracksStatus;
use std::sync::{Arc, RwLock};

pub mod music_api;

#[get("/api/<api_index>/<api_path..>")]
pub fn handle_api_call(api_index: &str, api_path: PathBuf) -> String {
    match api_index {
        "music" => music_api::get_music(api_path),
        _ => "not_found".to_string()
    }
}

#[post("/api/<api_index>/<api_path..>")]
pub fn handle_api_post(api_index: &str, api_path: PathBuf, status: &State<[Arc<RwLock<TracksStatus>>; 1]>) -> Result<status::Accepted<String>, status::NotFound<String>> {
   let status = status;
   match api_index {
        "music" =>  {
            match music_api::post_music(api_path, status[0].clone()) {
                Ok(result) => Ok(status::Accepted(result)),
                Err(result) => Err(status::NotFound(result))
            }
        },
        _ => Err(status::NotFound("API not found".to_string()))
    }
}
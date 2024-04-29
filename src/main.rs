#[macro_use] extern crate rocket;
use rocket::fs::NamedFile;
use rocket_dyn_templates::{context, Template};

mod api_handler;
pub mod db;
pub mod track;

#[get("/")]
fn index() -> Template {
    Template::render("index", context! { tracks: db::read_tracks() })
}

#[get("/play/<track_name>")]
async fn play_track(track_name: &str) -> Result<NamedFile, std::io::Error> {
    let track_list = db::read_tracks();
    for track in track_list {
        if track.title.to_string() == track_name {
            return NamedFile::open(track.path).await;
        }
    }
    Err(std::io::Error::new(std::io::ErrorKind::NotFound, "File not found"))
}

#[launch]
fn rocket() -> _ {
    println!("Checking tracks...");
    let status_list = [track::init()];
    let status_list_clone = std::sync::Arc::clone(&status_list[0]);
    std::thread::spawn(|| {
        db::write_tracks(track::get_tracks(status_list_clone))
    });
    rocket::build().mount("/", routes![index])
                   .mount("/", routes![api_handler::handle_api_call])
                   .mount("/", routes![api_handler::handle_api_post])
                   .mount("/", routes![api_handler::music_api::track_status])
                   .mount("/", routes![play_track])
                   .manage(status_list.clone())
                   .attach(Template::fairing())
}

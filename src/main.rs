#[macro_use] extern crate rocket;
use rocket::State;
use rocket::fs::NamedFile;
use rocket_dyn_templates::{context, Template};
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

mod api_handler;
pub mod db;
pub mod track;

#[derive(Deserialize)]
struct Config {
    database_url: String
}

#[get("/")]
async fn index(pool: &State<PgPool>) -> Template {
    Template::render("index", context! { tracks: db::read_tracks(pool).await })
}

#[get("/play/<track_name>")]
async fn play_track(track_name: &str, pool_state: &State<PgPool>) -> Result<NamedFile, std::io::Error> {
    let track_list = db::read_tracks(pool_state).await;
    for track in track_list {
        if track.title.to_string() == track_name {
            return NamedFile::open(track.path).await;
        }
    }
    Err(std::io::Error::new(std::io::ErrorKind::NotFound, "File not found"))
}

#[launch]
async fn rocket() -> _ {
    println!("Checking tracks...");
    let status_list = [track::init()];
    let status_list_clone = std::sync::Arc::clone(&status_list[0]);
    let rock = rocket::build();
    let config: Config = rock.figment().extract().expect("Invalid configuration");
    let pool = PgPoolOptions::new()
                   .max_connections(5)
                   .connect(&config.database_url)
                   .await.expect("Failed to connect to db");

    let pool_clone = pool.clone();
    rocket::tokio::spawn(async move {
        db::write_tracks(&pool_clone, track::get_tracks(status_list_clone)).await;
    });

    rock.mount("/", routes![index])
                   .mount("/", routes![api_handler::handle_api_call])
                   .mount("/", routes![api_handler::handle_api_post])
                   .mount("/", routes![api_handler::music_api::track_status])
                   .mount("/", routes![play_track])
                   .manage(status_list.clone())
                   .manage(pool)
                   .attach(Template::fairing())
}

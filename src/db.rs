use crate::track::Track;

use std::path::PathBuf;
use sqlx::PgPool;

pub async fn write_tracks(pool: &PgPool, track_list: Vec<Track>) {
    for track in track_list {
        write_track(pool, &track).await;
    }
}

pub async fn write_track(pool: &PgPool, track: &Track) {
    let _result = sqlx::query!(
        "INSERT INTO tracks (title, artist, duration, path) VALUES ($1, $2, $3, $4)",
        track.title.to_string().replace(|c: char| !c.is_ascii() || (c as u8) == 0, ""),
        track.artist.to_string().replace(|c: char| !c.is_ascii() || (c as u8) == 0, ""),
        i32::try_from(track.duration).expect("Couldn't downcast int'"),
        track.path.display().to_string()
    ).execute(pool).await; // TODO: do something with this return value
}

pub async fn read_tracks(pool: &PgPool) -> Vec<Track> {
    let tracks = sqlx::query!(
        "SELECT * FROM tracks ORDER BY artist ASC"
    ).fetch_all(pool).await;
    let mut track_vec: Vec<Track> = Vec::<Track>::new();
    match tracks {
        Ok(track_list) => {
            for row in track_list {
                track_vec.push(Track {title: row.title.into(), artist: row.artist.into(), duration: u64::try_from(row.duration).expect("Couldn't cast to u64'"), path: PathBuf::from(row.path)});
            }
        },
        Err(_) => {
            println!("Nothing found during query");
        }
    }
    track_vec
}
use core::time;
use std::path::PathBuf;
use std::sync::Arc;
use crate::db;
use crate::track::TracksStatus;
use crate as root;
use std::sync::RwLock;
use rocket::State;
use rocket::futures::{SinkExt, StreamExt};
use rocket::tokio;
use rocket::tokio::time::interval;
use sqlx::PgPool;


pub async fn get_music(api_path: PathBuf, pool: &PgPool) -> String {
    match api_path.to_str().unwrap() {
        "tracks" => music_tracks(pool).await,
        _ => "music".to_string()
    }
}

pub async fn post_music(api_path: PathBuf, track_status: Arc<RwLock<TracksStatus>>, db_pool: &PgPool) -> Result<String, String> {
    match api_path.to_str().unwrap() {
        "tracks/refresh" => {
            Ok(refresh_tracks(track_status, db_pool).await)
        },
        _ => {
            println!("API Not found: {}", api_path.display());
            Err("API Not found".to_string())
        }
    }
}

#[get("/api/music/tracks/status")]
pub fn track_status(ws: ws::WebSocket, track_status: &State<[Arc<RwLock<TracksStatus>>; 1]>) -> ws::Channel<'static> {
    let status = track_status[0].clone();
    ws.channel(move |mut stream: ws::stream::DuplexStream| Box::pin(async move {
        let mut prev_refresh = false;
        let mut inter = interval(time::Duration::from_secs(1));
        
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = inter.tick() => {
                        let refresh: bool;
                        {
                            let tmp_status = status.read().unwrap();
                            refresh = tmp_status.refresh_done;
                            if refresh == prev_refresh {
                                continue;
                            }
                        }
                        println!("Refreshing");
                        let message = format!("TrackStatus: refresh - {}", refresh);
                        prev_refresh = refresh;
                        let _ = stream.send(message.into()).await;
                    },
                    Some(Ok(message)) = stream.next() => {
                        match message {
                            ws::Message::Close(_) => {
                                println!("Closed!!");
                                let close_frame = ws::frame::CloseFrame {
                                    code: ws::frame::CloseCode::Normal,
                                    reason: "Client disconected".to_string().into(),
                                };
                                println!("Closing connection");
                                let _ = stream.close(Some(close_frame)).await;
                                break;
                            },
                            ws::Message::Text(message) => {
                                println!("Received {}", message);
                            },
                            _ => {}
                        }
                    },
                    else => {
                        println!("Connection closed");
                        let close_frame = ws::frame::CloseFrame {
                            code: ws::frame::CloseCode::Normal,
                            reason: "Client disconected".to_string().into(),
                        };
                        let _ = stream.close(Some(close_frame)).await;
                        // The connection is closed by the client
                        break;
                    }
                }
            }
        });
        tokio::signal::ctrl_c().await.unwrap();
        // # TODO: this isn't being reached
        println!("Stream closed");
        Ok(())
    }))
}

async fn music_tracks(pool: &PgPool) -> String {
    let tracks_list = db::read_tracks(pool);
    serde_json::to_string(&tracks_list.await).unwrap()
}

async fn refresh_tracks(track_status: Arc<RwLock<TracksStatus>>, pool: &PgPool) -> String {
    db::write_tracks(&pool, root::track::get_tracks(track_status)).await;
    "Success".to_string()
}
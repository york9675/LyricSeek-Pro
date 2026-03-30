#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod db;
pub mod fs_track;
pub mod library;
pub mod lrclib;
pub mod lyrics;
pub mod persistent_entities;
pub mod player;
pub mod providers;
pub mod state;
pub mod utils;

use persistent_entities::{PersistentAlbum, PersistentArtist, PersistentConfig, PersistentTrack};
use player::Player;
use regex::Regex;
use rusqlite::Connection;
use serde::Serialize;
use state::{AppState, ServiceAccess, Notify, NotifyType};
use tauri::{AppHandle, Manager, State, Emitter};
#[cfg(target_os = "macos")]
use tauri::menu::{Menu, MenuItem, PredefinedMenuItem, Submenu};

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct PublishLyricsProgress {
    request_challenge: String,
    solve_challenge: String,
    publish_lyrics: String,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct FlagLyricsProgress {
    request_challenge: String,
    solve_challenge: String,
    flag_lyrics: String,
}

#[tauri::command]
async fn get_directories(app_state: State<'_, AppState>) -> Result<Vec<String>, String> {
    let conn_guard = app_state.db.lock().unwrap();
    let conn = conn_guard.as_ref().unwrap();
    let directories = db::get_directories(conn);
    match directories {
        Ok(directories) => Ok(directories),
        Err(error) => Err(format!(
            "Cannot get existing directories from database. Error: {}",
            error
        )),
    }
}

#[tauri::command]
async fn set_directories(
    directories: Vec<String>,
    app_state: State<'_, AppState>,
) -> Result<(), String> {
    let conn_guard = app_state.db.lock().unwrap();
    let conn = conn_guard.as_ref().unwrap();
    db::set_directories(directories, conn).map_err(|err| err.to_string())?;

    Ok(())
}

#[tauri::command]
async fn get_init(app_state: State<'_, AppState>) -> Result<bool, String> {
    let conn_guard = app_state.db.lock().unwrap();
    let conn = conn_guard.as_ref().unwrap();
    let init = library::get_init(conn).map_err(|err| err.to_string())?;

    Ok(init)
}

#[tauri::command]
async fn get_config(app_state: State<'_, AppState>) -> Result<PersistentConfig, String> {
    let conn_guard = app_state.db.lock().unwrap();
    let conn = conn_guard.as_ref().unwrap();
    let config = db::get_config(conn).map_err(|err| err.to_string())?;

    Ok(config)
}

#[tauri::command]
async fn set_config(
    skip_tracks_with_synced_lyrics: bool,
    skip_tracks_with_plain_lyrics: bool,
    show_line_count: bool,
    show_cover_art_in_track_list: bool,
    try_embed_lyrics: bool,
    embed_lyrics_only: bool,
    prefer_synced_lyrics: bool,
    theme_mode: &str,
    lrclib_instance: &str,
    providers_order: Vec<String>,
    enabled_providers: Vec<String>,
    lastfm_links_enabled: bool,
    app_state: State<'_, AppState>,
) -> Result<(), String> {
    let conn_guard = app_state.db.lock().unwrap();
    let conn = conn_guard.as_ref().unwrap();
    db::set_config(
        skip_tracks_with_synced_lyrics,
        skip_tracks_with_plain_lyrics,
        show_line_count,
        show_cover_art_in_track_list,
        try_embed_lyrics,
        embed_lyrics_only,
        prefer_synced_lyrics,
        theme_mode,
        lrclib_instance,
        &providers_order,
        &enabled_providers,
        lastfm_links_enabled,
        conn,
    )
    .map_err(|err| err.to_string())?;

    Ok(())
}

#[tauri::command]
async fn initialize_library(
    app_state: State<'_, AppState>,
    app_handle: AppHandle,
) -> Result<(), String> {
    let mut conn_guard = app_state.db.lock().unwrap();
    let conn = conn_guard.as_mut().unwrap();
    library::initialize_library(conn, app_handle).map_err(|err| err.to_string())?;

    Ok(())
}

#[tauri::command]
async fn uninitialize_library(app_state: State<'_, AppState>) -> Result<(), String> {
    let conn_guard = app_state.db.lock().unwrap();
    let conn = conn_guard.as_ref().unwrap();

    library::uninitialize_library(conn).map_err(|err| err.to_string())?;

    Ok(())
}

#[tauri::command]
async fn refresh_library(
    app_state: State<'_, AppState>,
    app_handle: AppHandle,
) -> Result<(), String> {
    let mut conn_guard = app_state.db.lock().unwrap();
    let conn = conn_guard.as_mut().unwrap();

    library::uninitialize_library(conn).map_err(|err| err.to_string())?;
    library::initialize_library(conn, app_handle).map_err(|err| err.to_string())?;

    Ok(())
}

#[tauri::command]
async fn get_tracks(app_state: State<'_, AppState>) -> Result<Vec<PersistentTrack>, String> {
    let conn_guard = app_state.db.lock().unwrap();
    let conn = conn_guard.as_ref().unwrap();
    let tracks = library::get_tracks(conn).map_err(|err| err.to_string())?;

    Ok(tracks)
}

#[tauri::command]
async fn get_track_ids(
    search_query: Option<String>,
    synced_lyrics_tracks: Option<bool>,
    plain_lyrics_tracks: Option<bool>,
    instrumental_tracks: Option<bool>,
    no_lyrics_tracks: Option<bool>,
    app_state: State<'_, AppState>,
) -> Result<Vec<i64>, String> {
    let conn_guard = app_state.db.lock().unwrap();
    let conn = conn_guard.as_ref().unwrap();
    let search_query = search_query.filter(|s| !s.is_empty());
    let track_ids = library::get_track_ids(
        search_query,
        synced_lyrics_tracks.unwrap_or(true),
        plain_lyrics_tracks.unwrap_or(true),
        instrumental_tracks.unwrap_or(true),
        no_lyrics_tracks.unwrap_or(true),
        conn,
    )
    .map_err(|err| err.to_string())?;

    Ok(track_ids)
}

#[tauri::command]
async fn get_track(
    track_id: i64,
    app_state: State<'_, AppState>,
) -> Result<PersistentTrack, String> {
    let conn_guard = app_state.db.lock().unwrap();
    let conn = conn_guard.as_ref().unwrap();
    let track = library::get_track(track_id, conn).map_err(|err| err.to_string())?;

    Ok(track)
}

#[tauri::command]
async fn get_albums(app_state: State<'_, AppState>) -> Result<Vec<PersistentAlbum>, String> {
    let conn_guard = app_state.db.lock().unwrap();
    let conn = conn_guard.as_ref().unwrap();
    let albums = library::get_albums(conn).map_err(|err| err.to_string())?;

    Ok(albums)
}

#[tauri::command]
async fn get_album_ids(app_state: State<'_, AppState>) -> Result<Vec<i64>, String> {
    let conn_guard = app_state.db.lock().unwrap();
    let conn = conn_guard.as_ref().unwrap();
    let album_ids = library::get_album_ids(conn).map_err(|err| err.to_string())?;

    Ok(album_ids)
}

#[tauri::command]
async fn get_album(
    album_id: i64,
    app_state: State<'_, AppState>,
) -> Result<PersistentAlbum, String> {
    let conn_guard = app_state.db.lock().unwrap();
    let conn = conn_guard.as_ref().unwrap();
    let album = library::get_album(album_id, conn).map_err(|err| err.to_string())?;

    Ok(album)
}

#[tauri::command]
async fn get_artists(app_state: State<'_, AppState>) -> Result<Vec<PersistentArtist>, String> {
    let conn_guard = app_state.db.lock().unwrap();
    let conn = conn_guard.as_ref().unwrap();
    let artists = library::get_artists(conn).map_err(|err| err.to_string())?;

    Ok(artists)
}

#[tauri::command]
async fn get_artist_ids(app_state: State<'_, AppState>) -> Result<Vec<i64>, String> {
    let conn_guard = app_state.db.lock().unwrap();
    let conn = conn_guard.as_ref().unwrap();
    let artist_ids = library::get_artist_ids(conn).map_err(|err| err.to_string())?;

    Ok(artist_ids)
}

#[tauri::command]
async fn get_artist(
    artist_id: i64,
    app_state: State<'_, AppState>,
) -> Result<PersistentArtist, String> {
    let conn_guard = app_state.db.lock().unwrap();
    let conn = conn_guard.as_ref().unwrap();
    let artist = library::get_artist(artist_id, conn).map_err(|err| err.to_string())?;

    Ok(artist)
}

#[tauri::command]
async fn get_album_tracks(
    album_id: i64,
    app_state: State<'_, AppState>,
) -> Result<Vec<PersistentTrack>, String> {
    let conn_guard = app_state.db.lock().unwrap();
    let conn = conn_guard.as_ref().unwrap();
    let tracks = library::get_album_tracks(album_id, conn).map_err(|err| err.to_string())?;

    Ok(tracks)
}

#[tauri::command]
async fn get_artist_tracks(
    artist_id: i64,
    app_state: State<'_, AppState>,
) -> Result<Vec<PersistentTrack>, String> {
    let conn_guard = app_state.db.lock().unwrap();
    let conn = conn_guard.as_ref().unwrap();
    let tracks = library::get_artist_tracks(artist_id, conn).map_err(|err| err.to_string())?;

    Ok(tracks)
}

#[tauri::command]
async fn get_album_track_ids(
    album_id: i64,
    without_plain_lyrics: Option<bool>,
    without_synced_lyrics: Option<bool>,
    app_state: State<'_, AppState>,
) -> Result<Vec<i64>, String> {
    let conn_guard = app_state.db.lock().unwrap();
    let conn = conn_guard.as_ref().unwrap();
    let track_ids = library::get_album_track_ids(album_id, without_plain_lyrics.unwrap_or(false), without_synced_lyrics.unwrap_or(false), conn).map_err(|err| err.to_string())?;

    Ok(track_ids)
}

#[tauri::command]
async fn get_artist_track_ids(
    artist_id: i64,
    without_plain_lyrics: Option<bool>,
    without_synced_lyrics: Option<bool>,
    app_state: State<'_, AppState>,
) -> Result<Vec<i64>, String> {
    let conn_guard = app_state.db.lock().unwrap();
    let conn = conn_guard.as_ref().unwrap();
    let track_ids =
        library::get_artist_track_ids(artist_id, without_plain_lyrics.unwrap_or(false), without_synced_lyrics.unwrap_or(false), conn).map_err(|err| err.to_string())?;

    Ok(track_ids)
}

#[tauri::command]
async fn download_lyrics(track_id: i64, app_handle: AppHandle) -> Result<String, String> {
    let track = app_handle
        .db(|db| db::get_track_by_id(track_id, db))
        .map_err(|err| err.to_string())?;
    let config = app_handle
        .db(|db| db::get_config(db))
        .map_err(|err| err.to_string())?;
    let download_result = lyrics::download_lyrics_for_track(
        track,
        config.try_embed_lyrics,
        config.embed_lyrics_only,
        &config.lrclib_instance,
        &config.providers_order,
        &config.enabled_providers,
        config.prefer_synced_lyrics,
    )
    .await
    .map_err(|err| err.to_string())?;

    let provider_name = providers::provider_display_name(&download_result.provider);

    match download_result.lyrics {
        lrclib::get::Response::SyncedLyrics(synced_lyrics, plain_lyrics) => {
            app_handle
                .db(|db: &Connection| {
                    db::update_track_synced_lyrics(track_id, &synced_lyrics, &plain_lyrics, db)
                })
                .map_err(|err| err.to_string())?;
            app_handle.emit("reload-track-id", track_id).unwrap();
            Ok(format!("Synced lyrics downloaded from {}", provider_name))
        }
        lrclib::get::Response::UnsyncedLyrics(plain_lyrics) => {
            app_handle
                .db(|db: &Connection| db::update_track_plain_lyrics(track_id, &plain_lyrics, db))
                .map_err(|err| err.to_string())?;
            app_handle.emit("reload-track-id", track_id).unwrap();
            Ok(format!("Plain lyrics downloaded from {}", provider_name))
        }
        lrclib::get::Response::IsInstrumental => {
            app_handle
                .db(|db: &Connection| db::update_track_instrumental(track_id, db))
                .map_err(|err| err.to_string())?;
            Ok(format!("Marked track as instrumental from {}", provider_name))
        }
        lrclib::get::Response::None => Err(lyrics::GetLyricsError::NotFound.to_string()),
    }
}

#[tauri::command]
async fn apply_lyrics(
    track_id: i64,
    lrclib_response: lrclib::get::RawResponse,
    app_handle: AppHandle,
) -> Result<String, String> {
    let track = app_handle
        .db(|db| db::get_track_by_id(track_id, db))
        .map_err(|err| err.to_string())?;
    let config = app_handle
        .db(|db| db::get_config(db))
        .map_err(|err| err.to_string())?;

    let lyrics = lrclib::get::Response::from_raw_response(lrclib_response);
    let lyrics = lyrics::apply_lyrics_for_track(
        track,
        lyrics,
        config.try_embed_lyrics,
        config.embed_lyrics_only,
    )
        .await
        .map_err(|err| err.to_string())?;

    match lyrics {
        lrclib::get::Response::SyncedLyrics(synced_lyrics, plain_lyrics) => {
            app_handle
                .db(|db: &Connection| {
                    db::update_track_synced_lyrics(track_id, &synced_lyrics, &plain_lyrics, db)
                })
                .map_err(|err| err.to_string())?;
            std::thread::spawn(move || {
                app_handle.emit("reload-track-id", track_id).unwrap();
            });
            Ok("Synced lyrics downloaded".to_owned())
        }
        lrclib::get::Response::UnsyncedLyrics(plain_lyrics) => {
            app_handle
                .db(|db: &Connection| db::update_track_plain_lyrics(track_id, &plain_lyrics, db))
                .map_err(|err| err.to_string())?;
            std::thread::spawn(move || {
                app_handle.emit("reload-track-id", track_id).unwrap();
            });
            Ok("Plain lyrics downloaded".to_owned())
        }
        lrclib::get::Response::IsInstrumental => {
            app_handle
                .db(|db: &Connection| db::update_track_instrumental(track_id, db))
                .map_err(|err| err.to_string())?;
            Ok("Marked track as instrumental".to_owned())
        }
        lrclib::get::Response::None => Err(lyrics::GetLyricsError::NotFound.to_string()),
    }
}

#[tauri::command]
async fn retrieve_lyrics(
    title: String,
    album_name: String,
    artist_name: String,
    duration: f64,
    app_handle: AppHandle,
) -> Result<lrclib::get::RawResponse, String> {
    let config = app_handle
        .db(|db: &Connection| db::get_config(db))
        .map_err(|err| err.to_string())?;

    let response = lrclib::get::request_raw(
        &title,
        &album_name,
        &artist_name,
        duration,
        &config.lrclib_instance,
    )
    .await
    .map_err(|err| err.to_string())?;

    Ok(response)
}

#[tauri::command]
async fn retrieve_lyrics_by_id(
    id: i64,
    app_handle: AppHandle,
) -> Result<lrclib::get_by_id::RawResponse, String> {
    let config = app_handle
        .db(|db: &Connection| db::get_config(db))
        .map_err(|err| err.to_string())?;

    let response = lrclib::get_by_id::request_raw(id, &config.lrclib_instance)
        .await
        .map_err(|err| err.to_string())?;

    Ok(response)
}

#[tauri::command]
async fn search_lyrics(
    title: String,
    album_name: String,
    artist_name: String,
    q: String,
    provider: Option<String>,
    lrclib_tag: Option<String>,
    app_handle: AppHandle,
) -> Result<Vec<providers::ProviderSearchItem>, String> {
    let config = app_handle
        .db(|db: &Connection| db::get_config(db))
        .map_err(|err| err.to_string())?;

    let provider = provider.unwrap_or("auto".to_owned());
    let lrclib_tag = lrclib_tag.unwrap_or_default();

    let response = providers::search_lyrics(
        &provider,
        &title,
        &album_name,
        &artist_name,
        &q,
        &lrclib_tag,
        &config.lrclib_instance,
        &config.providers_order,
        &config.enabled_providers,
    )
    .await
    .map_err(|err| err.to_string())?;

    Ok(response)
}

#[tauri::command]
async fn retrieve_search_result_lyrics(
    search_result_item: providers::ProviderSearchItem,
    app_handle: AppHandle,
) -> Result<providers::ProviderSearchItem, String> {
    let config = app_handle
        .db(|db: &Connection| db::get_config(db))
        .map_err(|err| err.to_string())?;

    providers::retrieve_search_item_lyrics(&search_result_item, &config.lrclib_instance)
        .await
        .map_err(|err| err.to_string())
}

#[tauri::command]
async fn apply_search_result_lyrics(
    track_id: i64,
    search_result_item: providers::ProviderSearchItem,
    app_handle: AppHandle,
) -> Result<String, String> {
    let track = app_handle
        .db(|db| db::get_track_by_id(track_id, db))
        .map_err(|err| err.to_string())?;
    let config = app_handle
        .db(|db| db::get_config(db))
        .map_err(|err| err.to_string())?;

    let resolved_item =
        providers::retrieve_search_item_lyrics(&search_result_item, &config.lrclib_instance)
            .await
            .map_err(|err| err.to_string())?;
    let provider_name = providers::provider_display_name(&resolved_item.provider);

    let lyrics_response = providers::response_from_search_item(&resolved_item);
    let lyrics = lyrics::apply_lyrics_for_track(
        track,
        lyrics_response,
        config.try_embed_lyrics,
        config.embed_lyrics_only,
    )
        .await
        .map_err(|err| err.to_string())?;

    match lyrics {
        lrclib::get::Response::SyncedLyrics(synced_lyrics, plain_lyrics) => {
            app_handle
                .db(|db: &Connection| {
                    db::update_track_synced_lyrics(track_id, &synced_lyrics, &plain_lyrics, db)
                })
                .map_err(|err| err.to_string())?;
            app_handle.emit("reload-track-id", track_id).unwrap();
            Ok(format!("Synced lyrics applied from {}", provider_name))
        }
        lrclib::get::Response::UnsyncedLyrics(plain_lyrics) => {
            app_handle
                .db(|db: &Connection| db::update_track_plain_lyrics(track_id, &plain_lyrics, db))
                .map_err(|err| err.to_string())?;
            app_handle.emit("reload-track-id", track_id).unwrap();
            Ok(format!("Plain lyrics applied from {}", provider_name))
        }
        lrclib::get::Response::IsInstrumental => {
            app_handle
                .db(|db: &Connection| db::update_track_instrumental(track_id, db))
                .map_err(|err| err.to_string())?;
            Ok(format!("Marked track as instrumental from {}", provider_name))
        }
        lrclib::get::Response::None => Err(lyrics::GetLyricsError::NotFound.to_string()),
    }
}

#[tauri::command]
async fn save_lyrics(
    track_id: i64,
    plain_lyrics: String,
    synced_lyrics: String,
    app_handle: AppHandle,
) -> Result<String, String> {
    let track = app_handle
        .db(|db| db::get_track_by_id(track_id, db))
        .map_err(|err| err.to_string())?;
    let config = app_handle
        .db(|db| db::get_config(db))
        .map_err(|err| err.to_string())?;

    // Create a regex to match "[au: instrumental]" or "[au:instrumental]"
    let re = Regex::new(r"\[au:\s*instrumental\]").expect("Invalid regex");
    let is_instrumental = re.is_match(&synced_lyrics);

    lyrics::apply_string_lyrics_for_track(
        &track,
        &plain_lyrics,
        &synced_lyrics,
        config.try_embed_lyrics,
        config.embed_lyrics_only,
    )
    .await
    .map_err(|err| err.to_string())?;

    if is_instrumental {
        app_handle
            .db(|db: &Connection| db::update_track_instrumental(track.id, db))
            .map_err(|err| err.to_string())?;
    } else if !synced_lyrics.is_empty() {
        app_handle
            .db(|db: &Connection| {
                db::update_track_synced_lyrics(track.id, &synced_lyrics, &plain_lyrics, db)
            })
            .map_err(|err| err.to_string())?;
    } else if !plain_lyrics.is_empty() {
        app_handle
            .db(|db: &Connection| db::update_track_plain_lyrics(track.id, &plain_lyrics, db))
            .map_err(|err| err.to_string())?;
    } else {
        app_handle
            .db(|db: &Connection| db::update_track_null_lyrics(track.id, db))
            .map_err(|err| err.to_string())?;
    }

    app_handle.emit("reload-track-id", track_id).unwrap();

    Ok("Lyrics saved successfully".to_owned())
}

#[tauri::command]
async fn publish_lyrics(
    title: String,
    album_name: String,
    artist_name: String,
    duration: f64,
    plain_lyrics: String,
    synced_lyrics: String,
    app_handle: AppHandle,
) -> Result<(), String> {
    let config = app_handle
        .db(|db: &Connection| db::get_config(db))
        .map_err(|err| err.to_string())?;

    let mut progress = PublishLyricsProgress {
        request_challenge: "Pending".to_owned(),
        solve_challenge: "Pending".to_owned(),
        publish_lyrics: "Pending".to_owned(),
    };
    progress.request_challenge = "In Progress".to_owned();
    app_handle
        .emit("publish-lyrics-progress", &progress)
        .unwrap();
    let challenge_response = lrclib::request_challenge::request(&config.lrclib_instance)
        .await
        .map_err(|err| err.to_string())?;
    progress.request_challenge = "Done".to_owned();
    progress.solve_challenge = "In Progress".to_owned();
    app_handle
        .emit("publish-lyrics-progress", &progress)
        .unwrap();
    let nonce = lrclib::challenge_solver::solve_challenge(
        &challenge_response.prefix,
        &challenge_response.target,
    );
    progress.solve_challenge = "Done".to_owned();
    progress.publish_lyrics = "In Progress".to_owned();
    app_handle
        .emit("publish-lyrics-progress", &progress)
        .unwrap();
    let publish_token = format!("{}:{}", challenge_response.prefix, nonce);
    lrclib::publish::request(
        &title,
        &album_name,
        &artist_name,
        duration,
        &plain_lyrics,
        &synced_lyrics,
        &publish_token,
        &config.lrclib_instance,
    )
    .await
    .map_err(|err| err.to_string())?;
    progress.publish_lyrics = "Done".to_owned();
    app_handle
        .emit("publish-lyrics-progress", &progress)
        .unwrap();
    Ok(())
}

#[tauri::command]
async fn flag_lyrics(
    track_id: i64,
    flag_reason: String,
    app_handle: AppHandle,
) -> Result<(), String> {
    let config = app_handle
        .db(|db: &Connection| db::get_config(db))
        .map_err(|err| err.to_string())?;

    let mut progress = FlagLyricsProgress {
        request_challenge: "Pending".to_owned(),
        solve_challenge: "Pending".to_owned(),
        flag_lyrics: "Pending".to_owned(),
    };
    progress.request_challenge = "In Progress".to_owned();
    app_handle
        .emit("flag-lyrics-progress", &progress)
        .unwrap();
    let challenge_response = lrclib::request_challenge::request(&config.lrclib_instance)
        .await
        .map_err(|err| err.to_string())?;
    progress.request_challenge = "Done".to_owned();
    progress.solve_challenge = "In Progress".to_owned();
    app_handle
        .emit("flag-lyrics-progress", &progress)
        .unwrap();
    let nonce = lrclib::challenge_solver::solve_challenge(
        &challenge_response.prefix,
        &challenge_response.target,
    );
    progress.solve_challenge = "Done".to_owned();
    progress.flag_lyrics = "In Progress".to_owned();
    app_handle
        .emit("flag-lyrics-progress", &progress)
        .unwrap();
    let publish_token = format!("{}:{}", challenge_response.prefix, nonce);
    lrclib::flag::request(
        track_id,
        &flag_reason,
        &publish_token,
        &config.lrclib_instance,
    )
    .await
    .map_err(|err| err.to_string())?;
    progress.flag_lyrics = "Done".to_owned();
    app_handle
        .emit("flag-lyrics-progress", &progress)
        .unwrap();
    Ok(())
}

#[tauri::command]
fn play_track(
    track_id: i64,
    app_state: tauri::State<AppState>,
    app_handle: AppHandle,
) -> Result<(), String> {
    let track = app_handle
        .db(|db| db::get_track_by_id(track_id, db))
        .map_err(|err| err.to_string())?;

    let mut player_guard = app_state.player.lock().unwrap();

    if let Some(ref mut player) = *player_guard {
        player.play(track).map_err(|err| err.to_string())?;
    }

    Ok(())
}

#[tauri::command]
fn pause_track(app_state: tauri::State<AppState>) -> Result<(), String> {
    let mut player_guard = app_state.player.lock().map_err(|e| e.to_string())?;

    if let Some(ref mut player) = *player_guard {
        player.pause();
    }

    Ok(())
}

#[tauri::command]
fn resume_track(app_state: tauri::State<AppState>) -> Result<(), String> {
    let mut player_guard = app_state.player.lock().map_err(|e| e.to_string())?;

    if let Some(ref mut player) = *player_guard {
        player.resume();
    }

    Ok(())
}

#[tauri::command]
fn seek_track(position: f64, app_state: tauri::State<AppState>) -> Result<(), String> {
    let mut player_guard = app_state.player.lock().map_err(|e| e.to_string())?;

    if let Some(ref mut player) = *player_guard {
        player.seek(position);
    }

    Ok(())
}

#[tauri::command]
fn stop_track(app_state: tauri::State<AppState>) -> Result<(), String> {
    let mut player_guard = app_state.player.lock().map_err(|e| e.to_string())?;

    if let Some(ref mut player) = *player_guard {
        player.stop();
    }

    Ok(())
}

#[tauri::command]
fn set_volume(volume: f64, app_state: tauri::State<AppState>) -> Result<(), String> {
    let mut player_guard = app_state.player.lock().map_err(|e| e.to_string())?;

    if let Some(ref mut player) = *player_guard {
        player.set_volume(volume);
    }

    Ok(())
}

#[tauri::command]
fn get_cover_image(image_path: &str) -> Result<Vec<u8>, String> {
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open(image_path)
        .map_err(|e| format!("Cannot open image: {}", e))?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .map_err(|e| format!("Cannot read image: {}", e))?;
    Ok(buffer)
}

#[tauri::command]
fn open_devtools(app_handle: AppHandle) {
    app_handle.get_webview_window("main").unwrap().open_devtools();
}

#[tauri::command]
fn drain_notifications(app_state: tauri::State<AppState>) -> Vec<Notify> {
    let mut queued_notifications = app_state.queued_notifications.lock().unwrap();
    let notifications = queued_notifications.drain(..).collect();
    notifications
}

#[cfg(target_os = "macos")]
fn build_macos_menu(app: &AppHandle) -> tauri::Result<Menu<tauri::Wry>> {
    let package_info = app.package_info();

    let about_item = MenuItem::with_id(
        app,
        "open_about",
        format!("About {}", package_info.name),
        true,
        None::<&str>,
    )?;
    let settings_item = MenuItem::with_id(
        app,
        "open_settings",
        "Settings...",
        true,
        Some("CmdOrCtrl+,"),
    )?;
    let devtools_item = MenuItem::with_id(app, "open_devtools", "Open Developer Tools", true, None::<&str>)?;

    let app_menu = Submenu::with_items(
        app,
        package_info.name.clone(),
        true,
        &[
            &about_item,
            &PredefinedMenuItem::separator(app)?,
            &settings_item,
            &PredefinedMenuItem::separator(app)?,
            &PredefinedMenuItem::services(app, None)?,
            &PredefinedMenuItem::separator(app)?,
            &PredefinedMenuItem::hide(app, None)?,
            &PredefinedMenuItem::hide_others(app, None)?,
            &PredefinedMenuItem::show_all(app, None)?,
            &PredefinedMenuItem::separator(app)?,
            &PredefinedMenuItem::quit(app, None)?,
        ],
    )?;

    let file_menu = Submenu::with_items(
        app,
        "File",
        true,
        &[&PredefinedMenuItem::close_window(app, None)?],
    )?;

    let edit_menu = Submenu::with_items(
        app,
        "Edit",
        true,
        &[
            &PredefinedMenuItem::undo(app, None)?,
            &PredefinedMenuItem::redo(app, None)?,
            &PredefinedMenuItem::separator(app)?,
            &PredefinedMenuItem::cut(app, None)?,
            &PredefinedMenuItem::copy(app, None)?,
            &PredefinedMenuItem::paste(app, None)?,
            &PredefinedMenuItem::select_all(app, None)?,
        ],
    )?;

    let view_menu = Submenu::with_items(
        app,
        "View",
        true,
        &[
            &devtools_item,
            &PredefinedMenuItem::separator(app)?,
            &PredefinedMenuItem::fullscreen(app, None)?,
        ],
    )?;

    let window_menu = Submenu::with_items(
        app,
        "Window",
        true,
        &[
            &PredefinedMenuItem::minimize(app, None)?,
            &PredefinedMenuItem::maximize(app, None)?,
            &PredefinedMenuItem::separator(app)?,
            &PredefinedMenuItem::close_window(app, None)?,
        ],
    )?;

    Menu::with_items(
        app,
        &[&app_menu, &file_menu, &edit_menu, &view_menu, &window_menu],
    )
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_os::init())
        .manage(AppState {
            db: Default::default(),
            player: Default::default(),
            queued_notifications: std::sync::Mutex::new(Vec::new()),
        })
        .setup(|app| {
            let handle = app.handle();

            #[cfg(target_os = "macos")]
            {
                let menu = build_macos_menu(&handle)?;
                handle.set_menu(menu)?;

                app.on_menu_event(|app, event| match event.id().as_ref() {
                    "open_about" => {
                        if let Err(e) = app.emit("menu-open-about", ()) {
                            eprintln!("Failed to emit about menu event: {}", e);
                        }
                    }
                    "open_settings" => {
                        if let Err(e) = app.emit("menu-open-settings", ()) {
                            eprintln!("Failed to emit settings menu event: {}", e);
                        }
                    }
                    "open_devtools" => {
                        if let Some(window) = app.get_webview_window("main") {
                            window.open_devtools();
                        }
                    }
                    _ => {}
                });
            }

            let app_state: State<AppState> = handle.state();
            let db = db::initialize_database(&handle).expect("Database initialize should succeed");
            *app_state.db.lock().unwrap() = Some(db);

            let maybe_player = Player::new();
            match maybe_player {
                Ok(player) => {
                    *app_state.player.lock().unwrap() = Some(player);
                }
                Err(e) => {
                    eprintln!("Failed to initialize audio player: {}", e);
                    let mut buf = app_state.queued_notifications.lock().unwrap();
                    buf.push(Notify {
                        message: format!("Failed to initialize audio player: {}", e),
                        notify_type: NotifyType::Error,
                    });
                }
            }

            let handle_clone = handle.clone();

            tokio::spawn(async move {
                let mut interval = tokio::time::interval(std::time::Duration::from_millis(40));
                loop {
                    interval.tick().await;
                    {
                        let app_state: State<AppState> = handle_clone.state();
                        let player_guard = app_state.player.lock();

                        match player_guard {
                            Ok(mut player_guard) => {
                                if let Some(ref mut player) = *player_guard {
                                    player.renew_state();

                                    let emit_player_state =
                                        handle_clone.emit("player-state", &player);

                                    if let Err(e) = emit_player_state {
                                        eprintln!("Failed to emit player state: {}", e);
                                    }
                                }
                            }
                            Err(e) => eprintln!("Failed to lock player: {}", e),
                        }
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_directories,
            set_directories,
            get_init,
            get_config,
            set_config,
            initialize_library,
            uninitialize_library,
            refresh_library,
            get_tracks,
            get_track_ids,
            get_track,
            get_albums,
            get_album_ids,
            get_album,
            get_artists,
            get_artist_ids,
            get_artist,
            get_album_tracks,
            get_artist_tracks,
            get_album_track_ids,
            get_artist_track_ids,
            download_lyrics,
            apply_lyrics,
            retrieve_lyrics,
            retrieve_lyrics_by_id,
            search_lyrics,
            retrieve_search_result_lyrics,
            apply_search_result_lyrics,
            save_lyrics,
            publish_lyrics,
            flag_lyrics,
            play_track,
            pause_track,
            resume_track,
            seek_track,
            stop_track,
            set_volume,
            get_cover_image,
            open_devtools,
            drain_notifications,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

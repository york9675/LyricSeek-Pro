use crate::lrclib;
use crate::persistent_entities::PersistentTrack;
use crate::utils::strip_timestamp;
use anyhow::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::Duration;

pub const PROVIDER_LRCLIB: &str = "lrclib";
pub const PROVIDER_NETEASE: &str = "netease";
pub const PROVIDER_SIMPMUSIC: &str = "simpmusic";
pub const PROVIDER_GENIUS: &str = "genius";

const DEFAULT_PROVIDERS_ORDER: [&str; 4] = [
    PROVIDER_LRCLIB,
    PROVIDER_NETEASE,
    PROVIDER_SIMPMUSIC,
    PROVIDER_GENIUS,
];

const NETEASE_SEARCH_URL: &str = "https://music.163.com/api/search/get";
const NETEASE_LYRICS_URL: &str = "https://music.163.com/api/song/lyric";
const SIMPMUSIC_API_URL: &str = "https://api-lyrics.simpmusic.org/v1";
const GENIUS_SEARCH_URL: &str = "https://genius.com/api/search/song";

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderSearchItem {
    pub id: String,
    pub provider: String,
    pub provider_name: String,
    pub lrclib_id: Option<i64>,
    pub name: String,
    pub artist_name: String,
    pub album_name: String,
    pub duration: Option<f64>,
    pub instrumental: bool,
    pub plain_lyrics: Option<String>,
    pub synced_lyrics: Option<String>,
}

impl ProviderSearchItem {
    pub fn has_lyrics(&self) -> bool {
        self.instrumental
            || self
                .plain_lyrics
                .as_ref()
                .is_some_and(|lyrics| !lyrics.trim().is_empty())
            || self
                .synced_lyrics
                .as_ref()
                .is_some_and(|lyrics| !lyrics.trim().is_empty())
    }
}

pub fn default_providers_order() -> Vec<String> {
    DEFAULT_PROVIDERS_ORDER
        .iter()
        .map(|provider| provider.to_string())
        .collect()
}

pub fn default_enabled_providers() -> Vec<String> {
    default_providers_order()
}

pub fn parse_providers_order(order_raw: &str) -> Vec<String> {
    let parsed = order_raw
        .split(',')
        .map(|provider| provider.trim().to_lowercase())
        .collect::<Vec<_>>();

    normalize_providers_order(&parsed)
}

pub fn parse_enabled_providers(enabled_raw: &str) -> Vec<String> {
    let parsed = enabled_raw
        .split(',')
        .map(|provider| provider.trim().to_lowercase())
        .collect::<Vec<_>>();

    normalize_enabled_providers(&parsed)
}

pub fn stringify_providers_order(order: &[String]) -> String {
    normalize_providers_order(order).join(",")
}

pub fn stringify_enabled_providers(enabled_providers: &[String]) -> String {
    normalize_enabled_providers(enabled_providers).join(",")
}

pub fn normalize_providers_order(order: &[String]) -> Vec<String> {
    let mut normalized = Vec::new();

    for provider in order {
        let provider = provider.trim().to_lowercase();
        if is_supported_provider(&provider) && !normalized.contains(&provider) {
            normalized.push(provider);
        }
    }

    for provider in DEFAULT_PROVIDERS_ORDER {
        if !normalized.iter().any(|item| item == provider) {
            normalized.push(provider.to_owned());
        }
    }

    normalized
}

pub fn normalize_enabled_providers(enabled_providers: &[String]) -> Vec<String> {
    let mut normalized = Vec::new();

    for provider in enabled_providers {
        let provider = provider.trim().to_lowercase();
        if is_supported_provider(&provider) && !normalized.contains(&provider) {
            normalized.push(provider);
        }
    }

    if normalized.is_empty() {
        return default_enabled_providers();
    }

    normalized
}

pub fn ordered_active_providers(order: &[String], enabled_providers: &[String]) -> Vec<String> {
    let normalized_order = normalize_providers_order(order);
    let normalized_enabled = normalize_enabled_providers(enabled_providers);

    normalized_order
        .into_iter()
        .filter(|provider| normalized_enabled.contains(provider))
        .collect()
}

pub fn provider_display_name(provider: &str) -> &'static str {
    match provider.trim().to_lowercase().as_str() {
        PROVIDER_LRCLIB => "LRCLIB",
        PROVIDER_NETEASE => "NetEase",
        PROVIDER_SIMPMUSIC => "SimpMusic",
        PROVIDER_GENIUS => "Genius",
        _ => "Unknown Provider",
    }
}

pub async fn request_by_provider(
    provider: &str,
    track: &PersistentTrack,
    lrclib_instance: &str,
) -> Result<lrclib::get::Response> {
    let client = build_client(8)?;

    match provider.trim().to_lowercase().as_str() {
        PROVIDER_LRCLIB => {
            lrclib::get::request(
                &track.title,
                &track.album_name,
                &track.artist_name,
                track.duration,
                lrclib_instance,
            )
            .await
        }
        PROVIDER_NETEASE => request_netease(track, &client).await,
        PROVIDER_SIMPMUSIC => request_simpmusic(track, &client).await,
        PROVIDER_GENIUS => request_genius(track, &client).await,
        _ => Ok(lrclib::get::Response::None),
    }
}

pub async fn search_lyrics(
    provider: &str,
    title: &str,
    album_name: &str,
    artist_name: &str,
    q: &str,
    lrclib_tag: &str,
    lrclib_instance: &str,
    providers_order: &[String],
    enabled_providers: &[String],
) -> Result<Vec<ProviderSearchItem>> {
    let client = build_client(10)?;
    let search_providers = resolve_search_providers(provider, providers_order, enabled_providers);

    let mut merged_results = Vec::new();

    for provider in search_providers {
        let mut provider_results = search_provider(
            &provider,
            title,
            album_name,
            artist_name,
            q,
            lrclib_tag,
            lrclib_instance,
            &client,
        )
        .await?;
        merged_results.append(&mut provider_results);
    }

    Ok(merged_results)
}

pub async fn retrieve_search_item_lyrics(
    item: &ProviderSearchItem,
    lrclib_instance: &str,
) -> Result<ProviderSearchItem> {
    let mut resolved_item = item.clone();
    if resolved_item.has_lyrics() {
        return Ok(resolved_item);
    }

    let provider = normalized_provider(&resolved_item.provider);
    let client = build_client(10)?;

    match provider.as_str() {
        PROVIDER_LRCLIB => {
            resolved_item = retrieve_lrclib_item(resolved_item, lrclib_instance).await?;
        }
        PROVIDER_NETEASE => {
            resolved_item = retrieve_netease_item(resolved_item, &client).await?;
        }
        PROVIDER_SIMPMUSIC => {
            resolved_item = retrieve_simpmusic_item(resolved_item, &client).await?;
        }
        PROVIDER_GENIUS => {
            resolved_item = retrieve_genius_item(resolved_item, &client).await?;
        }
        _ => {}
    }

    Ok(resolved_item)
}

pub fn response_from_search_item(item: &ProviderSearchItem) -> lrclib::get::Response {
    if item.instrumental {
        return lrclib::get::Response::IsInstrumental;
    }

    if let Some(synced_lyrics) = item.synced_lyrics.as_ref() {
        let synced_lyrics = synced_lyrics.trim();
        if !synced_lyrics.is_empty() {
            let plain_lyrics = item
                .plain_lyrics
                .as_ref()
                .map(|lyrics| lyrics.trim().to_owned())
                .filter(|lyrics| !lyrics.is_empty())
                .unwrap_or_else(|| strip_timestamp(synced_lyrics));

            return lrclib::get::Response::SyncedLyrics(synced_lyrics.to_owned(), plain_lyrics);
        }
    }

    if let Some(plain_lyrics) = item.plain_lyrics.as_ref() {
        let plain_lyrics = plain_lyrics.trim();
        if !plain_lyrics.is_empty() {
            return lrclib::get::Response::UnsyncedLyrics(plain_lyrics.to_owned());
        }
    }

    lrclib::get::Response::None
}

fn resolve_search_providers(
    provider: &str,
    providers_order: &[String],
    enabled_providers: &[String],
) -> Vec<String> {
    let provider = normalized_provider(provider);

    if provider.is_empty() || provider == "auto" {
        return ordered_active_providers(providers_order, enabled_providers);
    }

    let normalized_enabled = normalize_enabled_providers(enabled_providers);
    if !is_supported_provider(&provider) || !normalized_enabled.contains(&provider) {
        return Vec::new();
    }

    vec![provider]
}

fn normalized_provider(provider: &str) -> String {
    provider.trim().to_lowercase()
}

fn is_supported_provider(provider: &str) -> bool {
    matches!(
        provider,
        PROVIDER_LRCLIB | PROVIDER_NETEASE | PROVIDER_SIMPMUSIC | PROVIDER_GENIUS
    )
}

fn build_client(timeout_seconds: u64) -> Result<reqwest::Client> {
    reqwest::Client::builder()
        .timeout(Duration::from_secs(timeout_seconds))
        .user_agent(format!(
            "LyricSeek Pro v{}",
            env!("CARGO_PKG_VERSION")
        ))
        .build()
        .map_err(Into::into)
}

async fn search_provider(
    provider: &str,
    title: &str,
    album_name: &str,
    artist_name: &str,
    q: &str,
    lrclib_tag: &str,
    lrclib_instance: &str,
    client: &reqwest::Client,
) -> Result<Vec<ProviderSearchItem>> {
    match provider {
        PROVIDER_LRCLIB => {
            search_lrclib_results(title, album_name, artist_name, q, lrclib_tag, lrclib_instance)
                .await
        }
        PROVIDER_NETEASE => search_netease_results(title, artist_name, q, client).await,
        PROVIDER_SIMPMUSIC => search_simpmusic_results(title, artist_name, q, client).await,
        PROVIDER_GENIUS => search_genius_results(title, artist_name, q, client).await,
        _ => Ok(Vec::new()),
    }
}

async fn search_lrclib_results(
    title: &str,
    album_name: &str,
    artist_name: &str,
    q: &str,
    lrclib_tag: &str,
    lrclib_instance: &str,
) -> Result<Vec<ProviderSearchItem>> {
    let query_filter = if lrclib_tag.trim().is_empty() {
        q.trim().to_owned()
    } else {
        lrclib_tag.trim().to_owned()
    };

    let response =
        lrclib::search::request(title, album_name, artist_name, &query_filter, lrclib_instance)
            .await?;
    let response_value = serde_json::to_value(&response)?;
    let rows = response_value
        .as_array()
        .cloned()
        .unwrap_or_default();

    let mut tracks = Vec::new();

    for row in rows {
        let lrclib_id = row.get("id").and_then(Value::as_i64);
        let id = match lrclib_id {
            Some(id) => id.to_string(),
            None => continue,
        };

        tracks.push(ProviderSearchItem {
            id,
            provider: PROVIDER_LRCLIB.to_owned(),
            provider_name: provider_display_name(PROVIDER_LRCLIB).to_owned(),
            lrclib_id,
            name: string_from_value(row.get("name")),
            artist_name: string_from_value(row.get("artistName")),
            album_name: string_from_value(row.get("albumName")),
            duration: row.get("duration").and_then(value_to_f64),
            instrumental: row
                .get("instrumental")
                .and_then(Value::as_bool)
                .unwrap_or(false),
            plain_lyrics: row
                .get("plainLyrics")
                .and_then(Value::as_str)
                .map(ToOwned::to_owned),
            synced_lyrics: row
                .get("syncedLyrics")
                .and_then(Value::as_str)
                .map(ToOwned::to_owned),
        });
    }

    Ok(tracks)
}

async fn search_netease_results(
    title: &str,
    artist_name: &str,
    q: &str,
    client: &reqwest::Client,
) -> Result<Vec<ProviderSearchItem>> {
    let search_query = build_external_search_query(title, artist_name, q);
    if search_query.is_empty() {
        return Ok(Vec::new());
    }

    let search_params = vec![
        ("limit".to_owned(), "10".to_owned()),
        ("offset".to_owned(), "0".to_owned()),
        ("type".to_owned(), "1".to_owned()),
        ("s".to_owned(), search_query),
    ];

    let response = client
        .get(NETEASE_SEARCH_URL)
        .query(&search_params)
        .send()
        .await?;
    if !response.status().is_success() {
        return Ok(Vec::new());
    }

    let payload: Value = response.json().await?;
    let songs = payload
        .get("result")
        .and_then(|item| item.get("songs"))
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();

    let mut tracks = Vec::new();

    for song in songs {
        let song_id = song.get("id").and_then(value_to_string);
        let song_id = match song_id {
            Some(song_id) => song_id,
            None => continue,
        };

        let artist_names = song
            .get("artists")
            .and_then(Value::as_array)
            .map(|artists| {
                artists
                    .iter()
                    .filter_map(|artist| artist.get("name").and_then(Value::as_str))
                    .collect::<Vec<_>>()
                    .join(", ")
            })
            .unwrap_or_default();

        let duration_seconds = song
            .get("duration")
            .and_then(value_to_f64)
            .map(|duration| duration / 1000.0);

        tracks.push(ProviderSearchItem {
            id: song_id,
            provider: PROVIDER_NETEASE.to_owned(),
            provider_name: provider_display_name(PROVIDER_NETEASE).to_owned(),
            lrclib_id: None,
            name: string_from_value(song.get("name")),
            artist_name: artist_names,
            album_name: song
                .get("album")
                .and_then(|album| album.get("name"))
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_owned(),
            duration: duration_seconds,
            instrumental: false,
            plain_lyrics: None,
            synced_lyrics: None,
        });
    }

    Ok(tracks)
}

async fn search_simpmusic_results(
    title: &str,
    artist_name: &str,
    q: &str,
    client: &reqwest::Client,
) -> Result<Vec<ProviderSearchItem>> {
    let search_query = build_simpmusic_search_query(title, artist_name, q);
    if search_query.is_empty() {
        return Ok(Vec::new());
    }

    let search_url = format!("{}/search", SIMPMUSIC_API_URL);
    let response = client
        .get(&search_url)
        .query(&[("q", search_query.as_str())])
        .send()
        .await?;

    if !response.status().is_success() {
        return Ok(Vec::new());
    }

    let payload: Value = response.json().await?;
    let songs = payload
        .get("data")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();

    let mut tracks = Vec::new();

    for song in songs {
        let id = song
            .get("videoId")
            .and_then(value_to_string)
            .or_else(|| song.get("id").and_then(value_to_string));
        let id = match id {
            Some(id) => id,
            None => continue,
        };

        let synced_lyrics = song
            .get("syncedLyrics")
            .or_else(|| song.get("richSyncLyrics"))
            .and_then(Value::as_str)
            .map(ToOwned::to_owned);
        let plain_lyrics = song
            .get("plainLyric")
            .or_else(|| song.get("plainLyrics"))
            .and_then(Value::as_str)
            .map(ToOwned::to_owned);

        tracks.push(ProviderSearchItem {
            id,
            provider: PROVIDER_SIMPMUSIC.to_owned(),
            provider_name: provider_display_name(PROVIDER_SIMPMUSIC).to_owned(),
            lrclib_id: None,
            name: string_from_value(song.get("songTitle")),
            artist_name: string_from_value(song.get("artistName")),
            album_name: string_from_value(song.get("albumName")),
            duration: song.get("durationSeconds").and_then(value_to_f64),
            instrumental: false,
            plain_lyrics,
            synced_lyrics,
        });
    }

    Ok(tracks)
}

async fn search_genius_results(
    title: &str,
    artist_name: &str,
    q: &str,
    client: &reqwest::Client,
) -> Result<Vec<ProviderSearchItem>> {
    let search_query = build_external_search_query(title, artist_name, q);
    if search_query.is_empty() {
        return Ok(Vec::new());
    }

    let search_params = vec![
        ("per_page".to_owned(), "10".to_owned()),
        ("q".to_owned(), search_query),
    ];

    let response = client
        .get(GENIUS_SEARCH_URL)
        .query(&search_params)
        .send()
        .await?;
    if !response.status().is_success() {
        return Ok(Vec::new());
    }

    let payload: Value = response.json().await?;
    let hits = payload
        .pointer("/response/sections/0/hits")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();

    let mut tracks = Vec::new();

    for hit in hits {
        let result = match hit.get("result") {
            Some(result) => result,
            None => continue,
        };
        let url = match result.get("url").and_then(Value::as_str) {
            Some(url) => url.to_owned(),
            None => continue,
        };

        tracks.push(ProviderSearchItem {
            id: url,
            provider: PROVIDER_GENIUS.to_owned(),
            provider_name: provider_display_name(PROVIDER_GENIUS).to_owned(),
            lrclib_id: None,
            name: result
                .get("full_title")
                .and_then(Value::as_str)
                .or_else(|| result.get("title").and_then(Value::as_str))
                .unwrap_or_default()
                .to_owned(),
            artist_name: result
                .get("artist_names")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_owned(),
            album_name: String::new(),
            duration: None,
            instrumental: false,
            plain_lyrics: None,
            synced_lyrics: None,
        });
    }

    Ok(tracks)
}

async fn retrieve_lrclib_item(
    item: ProviderSearchItem,
    lrclib_instance: &str,
) -> Result<ProviderSearchItem> {
    let item_id = item.lrclib_id.or_else(|| item.id.parse::<i64>().ok());
    let item_id = match item_id {
        Some(item_id) => item_id,
        None => return Ok(item),
    };

    let response = lrclib::get_by_id::request(item_id, lrclib_instance).await?;
    let mapped = match response {
        lrclib::get_by_id::Response::SyncedLyrics(synced, plain) => {
            lrclib::get::Response::SyncedLyrics(synced, plain)
        }
        lrclib::get_by_id::Response::UnsyncedLyrics(plain) => {
            lrclib::get::Response::UnsyncedLyrics(plain)
        }
        lrclib::get_by_id::Response::IsInstrumental => lrclib::get::Response::IsInstrumental,
        lrclib::get_by_id::Response::None => lrclib::get::Response::None,
    };

    Ok(apply_response_to_item(item, mapped))
}

async fn retrieve_netease_item(
    item: ProviderSearchItem,
    client: &reqwest::Client,
) -> Result<ProviderSearchItem> {
    if item.id.trim().is_empty() {
        return Ok(item);
    }

    let lyric_params = vec![
        ("id".to_owned(), item.id.clone()),
        ("kv".to_owned(), "-1".to_owned()),
        ("lv".to_owned(), "-1".to_owned()),
        ("tv".to_owned(), "-1".to_owned()),
    ];

    let response = client
        .get(NETEASE_LYRICS_URL)
        .query(&lyric_params)
        .send()
        .await?;
    if !response.status().is_success() {
        return Ok(item);
    }

    let payload: Value = response.json().await?;
    let lyrics = payload
        .get("lrc")
        .and_then(|item| item.get("lyric"))
        .and_then(Value::as_str)
        .unwrap_or("");

    Ok(apply_response_to_item(item, response_from_lyrics_text(lyrics)))
}

async fn retrieve_simpmusic_item(
    item: ProviderSearchItem,
    client: &reqwest::Client,
) -> Result<ProviderSearchItem> {
    if item.id.trim().is_empty() {
        return Ok(item);
    }

    let detail_url = format!("{}/{}", SIMPMUSIC_API_URL, item.id.trim());
    let response = client.get(&detail_url).send().await?;
    if !response.status().is_success() {
        return Ok(item);
    }

    let payload: Value = response.json().await?;
    let lyrics = extract_simpmusic_lyrics_from_payload(&payload);

    let mapped = lyrics
        .as_ref()
        .map(|lyrics| response_from_lyrics_text(lyrics))
        .unwrap_or(lrclib::get::Response::None);

    Ok(apply_response_to_item(item, mapped))
}

async fn retrieve_genius_item(
    item: ProviderSearchItem,
    client: &reqwest::Client,
) -> Result<ProviderSearchItem> {
    if item.id.trim().is_empty() {
        return Ok(item);
    }

    let response = client.get(item.id.trim()).send().await?;
    if !response.status().is_success() {
        return Ok(item);
    }

    let html = response.text().await?;
    let lyrics = extract_genius_lyrics(&html);

    let mapped = lyrics
        .as_ref()
        .map(|lyrics| response_from_lyrics_text(lyrics))
        .unwrap_or(lrclib::get::Response::None);

    Ok(apply_response_to_item(item, mapped))
}

fn apply_response_to_item(
    mut item: ProviderSearchItem,
    response: lrclib::get::Response,
) -> ProviderSearchItem {
    match response {
        lrclib::get::Response::SyncedLyrics(synced_lyrics, plain_lyrics) => {
            item.synced_lyrics = Some(synced_lyrics);
            item.plain_lyrics = Some(plain_lyrics);
            item.instrumental = false;
        }
        lrclib::get::Response::UnsyncedLyrics(plain_lyrics) => {
            item.plain_lyrics = Some(plain_lyrics);
            item.synced_lyrics = None;
            item.instrumental = false;
        }
        lrclib::get::Response::IsInstrumental => {
            item.instrumental = true;
            item.plain_lyrics = None;
            item.synced_lyrics = None;
        }
        lrclib::get::Response::None => {}
    }

    item
}

async fn request_netease(track: &PersistentTrack, client: &reqwest::Client) -> Result<lrclib::get::Response> {
    let search_query = build_track_search_query(track);
    if search_query.is_empty() {
        return Ok(lrclib::get::Response::None);
    }

    let search_params = vec![
        ("limit".to_owned(), "5".to_owned()),
        ("offset".to_owned(), "0".to_owned()),
        ("type".to_owned(), "1".to_owned()),
        ("s".to_owned(), search_query),
    ];

    let search_response = client
        .get(NETEASE_SEARCH_URL)
        .query(&search_params)
        .send()
        .await?;
    if !search_response.status().is_success() {
        return Ok(lrclib::get::Response::None);
    }

    let search_payload: Value = search_response.json().await?;
    let song_id = search_payload
        .get("result")
        .and_then(|item| item.get("songs"))
        .and_then(Value::as_array)
        .and_then(|songs| songs.first())
        .and_then(|song| song.get("id"))
        .and_then(value_to_string);

    let song_id = match song_id {
        Some(song_id) => song_id,
        None => return Ok(lrclib::get::Response::None),
    };

    let lyric_params = vec![
        ("id".to_owned(), song_id),
        ("kv".to_owned(), "-1".to_owned()),
        ("lv".to_owned(), "-1".to_owned()),
        ("tv".to_owned(), "-1".to_owned()),
    ];

    let lyrics_response = client
        .get(NETEASE_LYRICS_URL)
        .query(&lyric_params)
        .send()
        .await?;
    if !lyrics_response.status().is_success() {
        return Ok(lrclib::get::Response::None);
    }

    let lyrics_payload: Value = lyrics_response.json().await?;
    let lyrics = lyrics_payload
        .get("lrc")
        .and_then(|item| item.get("lyric"))
        .and_then(Value::as_str)
        .unwrap_or("");

    Ok(response_from_lyrics_text(lyrics))
}

async fn request_simpmusic(
    track: &PersistentTrack,
    client: &reqwest::Client,
) -> Result<lrclib::get::Response> {
    let search_query = build_track_search_query(track);
    if search_query.is_empty() {
        return Ok(lrclib::get::Response::None);
    }

    let search_url = format!("{}/search", SIMPMUSIC_API_URL);
    let search_response = client
        .get(&search_url)
        .query(&[("q", search_query.as_str())])
        .send()
        .await?;

    if !search_response.status().is_success() {
        return Ok(lrclib::get::Response::None);
    }

    let search_payload: Value = search_response.json().await?;
    let first_result = search_payload
        .get("data")
        .and_then(Value::as_array)
        .and_then(|items| items.first());

    let first_result = match first_result {
        Some(result) => result,
        None => return Ok(lrclib::get::Response::None),
    };

    if let Some(lyrics) = extract_simpmusic_lyrics(first_result) {
        return Ok(response_from_lyrics_text(&lyrics));
    }

    let maybe_video_id = first_result
        .get("videoId")
        .and_then(value_to_string)
        .or_else(|| first_result.get("id").and_then(value_to_string));

    let video_id = match maybe_video_id {
        Some(video_id) => video_id,
        None => return Ok(lrclib::get::Response::None),
    };

    let detail_url = format!("{}/{}", SIMPMUSIC_API_URL, video_id);
    let detail_response = client.get(&detail_url).send().await?;
    if !detail_response.status().is_success() {
        return Ok(lrclib::get::Response::None);
    }

    let detail_payload: Value = detail_response.json().await?;

    if let Some(lyrics) = extract_simpmusic_lyrics_from_payload(&detail_payload) {
        return Ok(response_from_lyrics_text(&lyrics));
    }

    Ok(lrclib::get::Response::None)
}

async fn request_genius(track: &PersistentTrack, client: &reqwest::Client) -> Result<lrclib::get::Response> {
    let search_query = build_track_search_query(track);
    if search_query.is_empty() {
        return Ok(lrclib::get::Response::None);
    }

    let search_params = vec![
        ("per_page".to_owned(), "1".to_owned()),
        ("q".to_owned(), search_query),
    ];

    let search_response = client
        .get(GENIUS_SEARCH_URL)
        .query(&search_params)
        .send()
        .await?;
    if !search_response.status().is_success() {
        return Ok(lrclib::get::Response::None);
    }

    let search_payload: Value = search_response.json().await?;
    let song_url = search_payload
        .pointer("/response/sections/0/hits/0/result/url")
        .and_then(Value::as_str);

    let song_url = match song_url {
        Some(song_url) => song_url,
        None => return Ok(lrclib::get::Response::None),
    };

    let page_response = client.get(song_url).send().await?;
    if !page_response.status().is_success() {
        return Ok(lrclib::get::Response::None);
    }

    let html = page_response.text().await?;
    let lyrics = match extract_genius_lyrics(&html) {
        Some(lyrics) => lyrics,
        None => return Ok(lrclib::get::Response::None),
    };

    Ok(response_from_lyrics_text(&lyrics))
}

fn build_track_search_query(track: &PersistentTrack) -> String {
    build_external_search_query(&track.title, &track.artist_name, "")
}

fn build_external_search_query(title: &str, artist_name: &str, q: &str) -> String {
    if !q.trim().is_empty() {
        return q.trim().to_owned();
    }

    format!("{} {}", artist_name.trim(), title.trim())
        .trim()
        .to_owned()
}

fn build_simpmusic_search_query(title: &str, artist_name: &str, q: &str) -> String {
    if !q.trim().is_empty() {
        return q.trim().to_owned();
    }

    if !title.trim().is_empty() {
        return title.trim().to_owned();
    }

    format!("{} {}", artist_name.trim(), title.trim())
        .trim()
        .to_owned()
}

fn value_to_string(value: &Value) -> Option<String> {
    if let Some(value) = value.as_str() {
        return Some(value.to_owned());
    }

    if let Some(value) = value.as_i64() {
        return Some(value.to_string());
    }

    None
}

fn value_to_f64(value: &Value) -> Option<f64> {
    value.as_f64().or_else(|| value.as_i64().map(|value| value as f64))
}

fn string_from_value(value: Option<&Value>) -> String {
    value
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_owned()
}

fn extract_simpmusic_lyrics_from_payload(payload: &Value) -> Option<String> {
    if let Some(lyrics) = extract_simpmusic_lyrics(payload) {
        return Some(lyrics);
    }

    if let Some(data) = payload.get("data") {
        if let Some(lyrics) = extract_simpmusic_lyrics(data) {
            return Some(lyrics);
        }

        if let Some(first_item) = data.as_array().and_then(|items| items.first()) {
            return extract_simpmusic_lyrics(first_item);
        }
    }

    None
}

fn extract_simpmusic_lyrics(payload: &Value) -> Option<String> {
    let synced = payload
        .get("syncedLyrics")
        .or_else(|| payload.get("richSyncLyrics"))
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|lyrics| !lyrics.is_empty());

    if let Some(synced) = synced {
        return Some(synced.to_owned());
    }

    payload
        .get("plainLyric")
        .or_else(|| payload.get("plainLyrics"))
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|lyrics| !lyrics.is_empty())
        .map(str::to_owned)
}

fn extract_genius_lyrics(html: &str) -> Option<String> {
    let html = html
        .replace("<br />", "\n")
        .replace("<br/>", "\n")
        .replace("<br>", "\n");

    let old_container_regex = Regex::new(r#"(?s)<div class=\"lyrics\">(.*?)</div>"#).ok()?;
    if let Some(captures) = old_container_regex.captures(&html) {
        if let Some(content) = captures.get(1) {
            let stripped = strip_html_tags(content.as_str());
            let cleaned = clean_lyrics_text(&stripped);
            if !cleaned.is_empty() {
                return Some(cleaned);
            }
        }
    }

    let container_regex =
        Regex::new(r#"(?s)<div data-lyrics-container=\"true\"[^>]*>(.*?)</div>"#).ok()?;
    let mut sections = Vec::new();

    for captures in container_regex.captures_iter(&html) {
        if let Some(content) = captures.get(1) {
            let stripped = strip_html_tags(content.as_str());
            let cleaned = clean_lyrics_text(&stripped);

            if !cleaned.is_empty() {
                sections.push(cleaned);
            }
        }
    }

    if sections.is_empty() {
        None
    } else {
        Some(sections.join("\n"))
    }
}

fn strip_html_tags(html: &str) -> String {
    let tag_regex = Regex::new(r"(?s)<[^>]+>").expect("HTML tag regex should compile");
    tag_regex.replace_all(html, "").to_string()
}

fn decode_html_entities(input: &str) -> String {
    input
        .replace("&nbsp;", " ")
        .replace("&#39;", "'")
        .replace("&#x27;", "'")
        .replace("&quot;", "\"")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
}

fn clean_lyrics_text(raw_lyrics: &str) -> String {
    let trailing_embed_regex = Regex::new(r"\d*Embed\s*$").expect("Embed regex should compile");

    let mut output = decode_html_entities(raw_lyrics).replace('\r', "");
    output = trailing_embed_regex.replace(&output, "").to_string();

    let mut lines = Vec::new();

    for line in output.lines() {
        let trimmed = line.trim();
        if trimmed.eq_ignore_ascii_case("you might also like") {
            continue;
        }

        if trimmed.is_empty() {
            if lines.last().is_some_and(|item: &String| item.is_empty()) {
                continue;
            }
            lines.push(String::new());
        } else {
            lines.push(trimmed.to_owned());
        }
    }

    lines.join("\n").trim().to_owned()
}

fn response_from_lyrics_text(raw_lyrics: &str) -> lrclib::get::Response {
    let lyrics = raw_lyrics.trim();
    if lyrics.is_empty() {
        return lrclib::get::Response::None;
    }

    if lyrics.eq_ignore_ascii_case("[au: instrumental]")
        || lyrics.eq_ignore_ascii_case("[au:instrumental]")
    {
        return lrclib::get::Response::IsInstrumental;
    }

    if looks_like_synced_lyrics(lyrics) {
        let plain_lyrics = synced_to_plain_lyrics(lyrics);
        return lrclib::get::Response::SyncedLyrics(lyrics.to_owned(), plain_lyrics);
    }

    lrclib::get::Response::UnsyncedLyrics(lyrics.to_owned())
}

fn looks_like_synced_lyrics(lyrics: &str) -> bool {
    let synced_line_regex =
        Regex::new(r"^\[\d{2}:\d{2}(?:\.\d{1,3})?\]").expect("Synced regex should compile");

    lyrics
        .lines()
        .any(|line| synced_line_regex.is_match(line.trim()))
}

fn synced_to_plain_lyrics(synced_lyrics: &str) -> String {
    let timestamp_regex = Regex::new(r"\[[0-9]{2}:[0-9]{2}(?:\.[0-9]{1,3})?\]")
        .expect("Timestamp regex should compile");

    synced_lyrics
        .lines()
        .map(|line| timestamp_regex.replace_all(line, "").trim().to_owned())
        .collect::<Vec<_>>()
        .join("\n")
}

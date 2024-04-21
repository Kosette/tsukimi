use once_cell::sync::Lazy;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;
use std::env;
use std::fs;
use std::io::Write;
use std::sync::OnceLock;
use tokio::runtime;

use crate::config::proxy::ReqClient;
use crate::config::{self, get_cache_dir, get_device_name, save_cfg, Account, APP_VERSION};
use crate::ui::models::SETTINGS;

pub static RUNTIME: Lazy<tokio::runtime::Runtime> = Lazy::new(|| {
    const STACK_SIZE: usize = 6 * 1024 * 1024;
    runtime::Builder::new_multi_thread()
        .worker_threads(SETTINGS.threads() as usize)
        .thread_stack_size(STACK_SIZE)
        .enable_all()
        .build()
        .expect("Failed to create runtime")
});

fn client() -> &'static Client {
    static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();
    CLIENT.get_or_init(ReqClient::build)
}

pub async fn loginv2(
    servername: String,
    server: String,
    username: String,
    password: String,
    port: String,
) -> Result<(), Error> {
    let mut headers = HeaderMap::new();
    headers.insert("X-Emby-Client", HeaderValue::from_static("Tsukimi"));
    headers.insert(
        "X-Emby-Device-Name",
        HeaderValue::from_str(&get_device_name()).unwrap(),
    );
    headers.insert(
        "X-Emby-Device-Id",
        HeaderValue::from_str(&env::var("UUID").unwrap()).unwrap(),
    );
    headers.insert(
        "X-Emby-Client-Version",
        HeaderValue::from_static(APP_VERSION),
    );
    headers.insert("X-Emby-Language", HeaderValue::from_static("zh-cn"));

    let body = json!({
        "Username": username,
        "Pw": password
    });

    let res = client()
        .post(&format!(
            "{}:{}/emby/Users/authenticatebyname",
            server, port
        ))
        .headers(headers)
        .json(&body)
        .send()
        .await?;

    let v: Value = res.json().await?;

    // 获取 "User" 对象中的 "Id" 字段
    let user_id = v["User"]["Id"].as_str().unwrap();
    println!("UserId: {}", user_id);

    // 获取 "AccessToken" 字段
    let access_token = v["AccessToken"].as_str().unwrap();
    println!("AccessToken: {}", access_token);

    let config = Account {
        servername,
        server,
        username,
        password,
        port,
        user_id: user_id.to_string(),
        access_token: access_token.to_string(),
    };
    save_cfg(config).await.unwrap();
    Ok(())
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SearchResult {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Type")]
    pub result_type: String,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "UserData")]
    pub user_data: Option<UserData>,
    #[serde(rename = "ProductionYear")]
    pub production_year: Option<i16>,
}

struct SearchModel {
    search_results: Vec<SearchResult>,
}

pub(crate) async fn search(searchinfo: String) -> Result<Vec<SearchResult>, Error> {
    let mut model = SearchModel {
        search_results: Vec::new(),
    };
    let server_info = config::set_config();

    let url = format!(
        "{}:{}/emby/Users/{}/Items",
        server_info.domain, server_info.port, server_info.user_id
    );
    let items: Vec<SearchResult> = {
        let device_name = &get_device_name();
        let device_id = &env::var("UUID").unwrap();

        let params = Box::new([
            (
                "Fields",
                "BasicSyncInfo,CanDelete,PrimaryImageAspectRatio,ProductionYear,Status,EndDate",
            ),
            ("IncludeItemTypes", "Movie,Series"),
            ("StartIndex", "0"),
            ("SortBy", "SortName"),
            ("SortOrder", "Ascending"),
            ("EnableImageTypes", "Primary,Backdrop,Thumb"),
            ("ImageTypeLimit", "1"),
            ("Recursive", "True"),
            ("SearchTerm", &searchinfo),
            ("GroupProgramsBySeries", "True"),
            ("Limit", "50"),
            ("X-Emby-Client", "Tsukimi"),
            ("X-Emby-Device-Name", device_name),
            ("X-Emby-Device-Id", device_id),
            ("X-Emby-Client-Version", APP_VERSION),
            ("X-Emby-Token", &server_info.access_token),
            ("X-Emby-Language", "zh-cn"),
        ]);

        let response = client().get(&url).query(&params).send().await?;
        let mut json: serde_json::Value = response.json().await?;
        serde_json::from_value(json["Items"].take()).unwrap()
    };
    model.search_results = items;
    Ok(model.search_results)
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SeriesInfo {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Overview")]
    pub overview: Option<String>,
    #[serde(rename = "IndexNumber")]
    pub index_number: u32,
    #[serde(rename = "ParentIndexNumber")]
    pub parent_index_number: u32,
    #[serde(rename = "UserData")]
    pub user_data: Option<UserData>,
}

pub async fn get_series_info(id: String) -> Result<Vec<SeriesInfo>, Error> {
    let server_info = config::set_config();
    let url = format!(
        "{}:{}/emby/Shows/{}/Episodes",
        server_info.domain, server_info.port, id
    );
    let mut json: serde_json::Value = {
        let device_name = &get_device_name();
        let device_id = &env::var("UUID").unwrap();

        let params = Box::new([
            ("Fields", "Overview"),
            ("EnableTotalRecordCount", "True"),
            ("EnableImages", "True"),
            ("UserId", &server_info.user_id),
            ("X-Emby-Client", "Tsukimi"),
            ("X-Emby-Device-Name", device_name),
            ("X-Emby-Device-Id", device_id),
            ("X-Emby-Client-Version", APP_VERSION),
            ("X-Emby-Token", &server_info.access_token),
            ("X-Emby-Language", "zh-cn"),
        ]);
        let response = client().get(&url).query(&params).send().await?;
        response.json().await?
    };
    let seriesinfo: Vec<SeriesInfo> = serde_json::from_value(json["Items"].take()).unwrap();
    Ok(seriesinfo)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MediaStream {
    #[serde(rename = "DisplayTitle")]
    pub display_title: Option<String>,
    #[serde(rename = "Type")]
    pub stream_type: String,
    #[serde(rename = "DeliveryUrl")]
    pub delivery_url: Option<String>,
    #[serde(rename = "IsExternal")]
    pub is_external: bool,
    #[serde(rename = "Title")]
    pub title: Option<String>,
    #[serde(rename = "DisplayLanguage")]
    pub display_language: Option<String>,
    #[serde(rename = "Codec")]
    pub codec: Option<String>,
    #[serde(rename = "BitRate")]
    pub bit_rate: Option<u64>,
    #[serde(rename = "BitDepth")]
    pub bit_depth: Option<u64>,
    #[serde(rename = "AverageFrameRate")]
    pub average_frame_rate: Option<f64>,
    #[serde(rename = "Height")]
    pub height: Option<u64>,
    #[serde(rename = "Width")]
    pub width: Option<u64>,
    #[serde(rename = "PixelFormat")]
    pub pixel_format: Option<String>,
    #[serde(rename = "ColorSpace")]
    pub color_space: Option<String>,
    #[serde(rename = "SampleRate")]
    pub sample_rate: Option<u64>,
    #[serde(rename = "Channels")]
    pub channels: Option<u64>,
    #[serde(rename = "ChannelLayout")]
    pub channel_layout: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MediaSource {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Size")]
    pub size: u64,
    #[serde(rename = "Container")]
    pub container: String,
    #[serde(rename = "DirectStreamUrl")]
    pub direct_stream_url: Option<String>,
    #[serde(rename = "MediaStreams")]
    pub media_streams: Vec<MediaStream>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Media {
    #[serde(rename = "MediaSources")]
    pub media_sources: Vec<MediaSource>,
    #[serde(rename = "PlaySessionId")]
    pub play_session_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Item {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "SeriesId")]
    pub series_id: Option<String>,
    #[serde(rename = "SeriesName")]
    pub series_name: Option<String>,
    #[serde(rename = "ParentIndexNumber")]
    pub parent_index_number: Option<u32>,
    #[serde(rename = "IndexNumber")]
    pub index_number: Option<u32>,
    #[serde(rename = "ProductionYear")]
    pub production_year: Option<u16>,
    #[serde(rename = "ExternalUrls")]
    pub external_urls: Option<Vec<Urls>>,
    #[serde(rename = "Overview")]
    pub overview: Option<String>,
    #[serde(rename = "People")]
    pub people: Option<Vec<People>>,
    #[serde(rename = "Studios")]
    pub studios: Option<Vec<SGTitem>>,
    #[serde(rename = "GenreItems")]
    pub genres: Option<Vec<SGTitem>>,
    #[serde(rename = "TagItems")]
    pub tags: Option<Vec<SGTitem>>,
    #[serde(rename = "UserData")]
    pub user_data: Option<UserData>,
    #[serde(rename = "CommunityRating")]
    pub community_rating: Option<f64>,
    #[serde(rename = "OfficialRating")]
    pub official_rating: Option<String>,
    #[serde(rename = "RunTimeTicks")]
    pub run_time_ticks: Option<u64>,
    #[serde(rename = "Taglines")]
    pub taglines: Option<Vec<String>>,
    #[serde(rename = "BackdropImageTags")]
    pub backdrop_image_tags: Option<Vec<String>>,
    #[serde(rename = "AlbumArtist")]
    pub album_artist: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct People {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Role")]
    pub role: Option<String>,
    #[serde(rename = "Type")]
    pub people_type: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SGTitem {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Id")]
    pub id: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Urls {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Url")]
    pub url: String,
}

pub async fn get_item_overview(id: String) -> Result<Item, Error> {
    let server_info = config::set_config();
    let url = format!(
        "{}:{}/emby/Users/{}/Items/{}",
        server_info.domain, server_info.port, server_info.user_id, id
    );
    let mut json: serde_json::Value = {
        let device_name = &get_device_name();
        let device_id = &env::var("UUID").unwrap();

        let params = Box::new([
            ("Fields", "ShareLevel"),
            ("X-Emby-Client", "Tsukimi"),
            ("X-Emby-Device-Name", device_name),
            ("X-Emby-Device-Id", device_id),
            ("X-Emby-Client-Version", APP_VERSION),
            ("X-Emby-Token", &server_info.access_token),
            ("X-Emby-Language", "zh-cn"),
        ]);
        let response = client().get(&url).query(&params).send().await?;
        response.json().await?
    };
    let item: Item = serde_json::from_value(json.take()).unwrap();
    Ok(item)
}

pub async fn _markwatched(id: String, sourceid: String) -> Result<String, Error> {
    let server_info = config::set_config();
    let client = client();
    let url = format!(
        "{}:{}/emby/Users/{}/PlayingItems/{}",
        server_info.domain, server_info.port, server_info.user_id, id
    );
    println!("{}", url);
    let params = [
        ("X-Emby-Client", "Tsukimi"),
        ("X-Emby-Device-Name", &get_device_name()),
        ("X-Emby-Device-Id", &env::var("UUID").unwrap()),
        ("X-Emby-Client-Version", APP_VERSION),
        ("X-Emby-Token", &server_info.access_token),
        ("X-Emby-Language", "zh-cn"),
        ("reqformat", "json"),
    ];
    let inplay = json!({
        "UserId": &server_info.user_id,
        "Id": &id,
        "MediaSourceId": &sourceid,
    });
    let response = client
        .post(&url)
        .query(&params)
        .json(&inplay)
        .send()
        .await?;
    let text = response.text().await?;
    Ok(text)
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Resume {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Type")]
    pub resume_type: String,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "SeriesId")]
    pub series_id: Option<String>,
    #[serde(rename = "IndexNumber")]
    pub index_number: Option<u32>,
    #[serde(rename = "ParentIndexNumber")]
    pub parent_index_number: Option<u32>,
    #[serde(rename = "ParentThumbItemId")]
    pub parent_thumb_item_id: Option<String>,
    #[serde(rename = "SeriesName")]
    pub series_name: Option<String>,
    #[serde(rename = "UserData")]
    pub user_data: Option<UserData>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserData {
    #[serde(rename = "PlayedPercentage")]
    pub played_percentage: Option<f64>,
    #[serde(rename = "PlaybackPositionTicks")]
    pub playback_position_ticks: Option<u64>,
    #[serde(rename = "Played")]
    pub played: bool,
    #[serde(rename = "UnplayedItemCount")]
    pub unplayed_item_count: Option<u32>,
    #[serde(rename = "IsFavorite")]
    pub is_favorite: Option<bool>,
}
struct ResumeModel {
    resume: Vec<Resume>,
}

pub(crate) async fn resume() -> Result<Vec<Resume>, Error> {
    let mut model = ResumeModel { resume: Vec::new() };
    let server_info = config::set_config();

    let url = format!(
        "{}:{}/emby/Users/{}/Items/Resume",
        server_info.domain, server_info.port, server_info.user_id
    );
    let mut json: serde_json::Value = {
        let device_name = &get_device_name();
        let device_id = &env::var("UUID").unwrap();
        let params = Box::new([
            ("Recursive", "True"),
            (
                "Fields",
                "BasicSyncInfo,CanDelete,PrimaryImageAspectRatio,ProductionYear",
            ),
            ("EnableImageTypes", "Primary,Backdrop,Thumb"),
            ("ImageTypeLimit", "1"),
            ("MediaTypes", "Video"),
            ("Limit", "24"),
            ("X-Emby-Client", "Tsukimi"),
            ("X-Emby-Device-Name", device_name),
            ("X-Emby-Device-Id", device_id),
            ("X-Emby-Client-Version", APP_VERSION),
            ("X-Emby-Token", &server_info.access_token),
            ("X-Emby-Language", "zh-cn"),
        ]);

        let response = client().get(&url).query(&params).send().await?;
        response.json().await?
    };
    let items: Vec<Resume> = serde_json::from_value(json["Items"].take()).unwrap();
    model.resume = items;
    Ok(model.resume)
}

pub async fn get_image(id: String) -> Result<String, Error> {
    let server_info = config::set_config();

    let result = client()
        .get(&format!(
            "{}:{}/emby/Items/{}/Images/Primary?maxHeight=400",
            server_info.domain, server_info.port, id
        ))
        .send()
        .await;

    match result {
        Ok(response) => {
            let bytes_result = response.bytes().await;
            match bytes_result {
                Ok(bytes) => {
                    if bytes.len() < 10240 {
                        return Ok(id);
                    }

                    let pathbuf = get_cache_dir(env::var("EMBY_NAME").unwrap());
                    if pathbuf.exists() {
                        fs::write(pathbuf.join(format!("{}.png", id)), &bytes).unwrap();
                    } else {
                        fs::create_dir_all(&pathbuf).unwrap();

                        fs::write(pathbuf.join(format!("{}.png", id)), &bytes).unwrap();
                    }
                    Ok(id)
                }
                Err(e) => {
                    eprintln!("loading error");
                    Err(e)
                }
            }
        }
        Err(e) => {
            eprintln!("loading error");
            Err(e)
        }
    }
}

pub async fn get_thumbimage(id: String) -> Result<String, Error> {
    let server_info = config::set_config();

    let result = client()
        .get(&format!(
            "{}:{}/emby/Items/{}/Images/Thumb?maxHeight=400",
            server_info.domain, server_info.port, id
        ))
        .send()
        .await;

    match result {
        Ok(response) => {
            let bytes_result = response.bytes().await;
            match bytes_result {
                Ok(bytes) => {
                    if bytes.len() < 10240 {
                        return Ok(id);
                    }

                    let pathbuf = get_cache_dir(env::var("EMBY_NAME").unwrap());
                    if pathbuf.exists() {
                        fs::write(pathbuf.join(format!("t{}.png", id)), &bytes).unwrap();
                    } else {
                        fs::create_dir_all(&pathbuf).unwrap();

                        fs::write(pathbuf.join(format!("t{}.png", id)), &bytes).unwrap();
                    }
                    Ok(id)
                }
                Err(e) => {
                    eprintln!("loading error");
                    Err(e)
                }
            }
        }
        Err(e) => {
            eprintln!("loading error");
            Err(e)
        }
    }
}

pub async fn get_backdropimage(id: String, tag: u8) -> Result<String, Error> {
    let server_info = config::set_config();

    let result = client()
        .get(&format!(
            "{}:{}/emby/Items/{}/Images/Backdrop/{}?maxHeight=1200",
            server_info.domain, server_info.port, id, tag
        ))
        .send()
        .await;

    match result {
        Ok(response) => {
            let bytes_result = response.bytes().await;
            match bytes_result {
                Ok(bytes) => {
                    if bytes.len() < 10240 {
                        return Ok(id);
                    }

                    let pathbuf = get_cache_dir(env::var("EMBY_NAME").unwrap());
                    if pathbuf.exists() {
                        fs::write(pathbuf.join(format!("b{}_{}.png", id, tag)), &bytes).unwrap();
                    } else {
                        fs::create_dir_all(&pathbuf).unwrap();

                        fs::write(pathbuf.join(format!("b{}_{}.png", id, tag)), &bytes).unwrap();
                    }
                    Ok(id)
                }
                Err(e) => {
                    eprintln!("loading error");
                    Err(e)
                }
            }
        }
        Err(e) => {
            eprintln!("loading error");
            Err(e)
        }
    }
}

pub async fn get_logoimage(id: String) -> Result<String, Error> {
    let server_info = config::set_config();

    let result = client()
        .get(&format!(
            "{}:{}/emby/Items/{}/Images/Logo?maxHeight=400",
            server_info.domain, server_info.port, id
        ))
        .send()
        .await;

    match result {
        Ok(response) => {
            let bytes_result = response.bytes().await;
            match bytes_result {
                Ok(bytes) => {
                    if bytes.len() < 10240 {
                        return Ok(id);
                    }

                    let pathbuf = get_cache_dir(env::var("EMBY_NAME").unwrap());
                    if pathbuf.exists() {
                        fs::write(pathbuf.join(format!("l{}.png", id)), &bytes).unwrap();
                    } else {
                        fs::create_dir_all(&pathbuf).unwrap();

                        fs::write(pathbuf.join(format!("l{}.png", id)), &bytes).unwrap();
                    }
                    Ok(id)
                }
                Err(e) => {
                    eprintln!("loading error");
                    Err(e)
                }
            }
        }
        Err(e) => {
            eprintln!("loading error");
            Err(e)
        }
    }
}

pub async fn get_mediainfo(id: String) -> Result<Media, Error> {
    let server_info = config::set_config();
    let url = format!(
        "{}:{}/emby/Users/{}/Items/{}",
        server_info.domain, server_info.port, server_info.user_id, id
    );
    let mut json: serde_json::Value = {
        let device_name = &get_device_name();
        let device_id = &env::var("UUID").unwrap();
        let params = Box::new([
            ("Fields", "ShareLevel"),
            ("X-Emby-Client", "Tsukimi"),
            ("X-Emby-Device-Name", device_name),
            ("X-Emby-Device-Id", device_id),
            ("X-Emby-Client-Version", APP_VERSION),
            ("X-Emby-Token", &server_info.access_token),
            ("X-Emby-Language", "zh-cn"),
        ]);
        let response = client().get(&url).query(&params).send().await?;
        response.json().await?
    };
    let mediainfo: Media = serde_json::from_value(json.take()).unwrap();
    Ok(mediainfo)
}

pub async fn get_playbackinfo(id: String) -> Result<Media, Error> {
    let server_info = config::set_config();
    let url = format!(
        "{}:{}/emby/Items/{}/PlaybackInfo",
        server_info.domain, server_info.port, id
    );

    let json: serde_json::Value = {
        let device_name = &get_device_name();
        let device_id = &env::var("UUID").unwrap();
        let params = Box::new([
            ("StartTimeTicks", "0"),
            ("UserId", &server_info.user_id),
            ("AutoOpenLiveStream", "False"),
            ("IsPlayback", "False"),
            ("AudioStreamIndex", "1"),
            ("SubtitleStreamIndex", "1"),
            ("MaxStreamingBitrate", "1000000000"),
            ("X-Emby-Client", "Tsukimi"),
            ("X-Emby-Device-Name", device_name),
            ("X-Emby-Device-Id", device_id),
            ("X-Emby-Client-Version", APP_VERSION),
            ("X-Emby-Token", &server_info.access_token),
            ("X-Emby-Language", "zh-cn"),
            ("reqformat", "json"),
        ]);
        let profile = serde_json::json!(

            {"DeviceProfile":{"Name":"Direct play all","MaxStaticBitrate":1000000000,"MaxStreamingBitrate":1000000000,"MusicStreamingTranscodingBitrate":1500000,"DirectPlayProfiles":[{"Container":"mkv","Type":"Video","VideoCodec":"hevc,h264,av1,vp8,vp9,mp4","AudioCodec":"aac,ac3,alac,eac3,dts,flac,mp3,opus,truehd,vorbis"},{"Container":"mp4,m4v","Type":"Video","VideoCodec":"hevc,h264,av1,vp8,vp9","AudioCodec":"aac,alac,opus,mp3,flac,vorbis"},{"Container":"flv","Type":"Video","VideoCodec":"h264","AudioCodec":"aac,mp3"},{"Container":"mov","Type":"Video","VideoCodec":"h264","AudioCodec":"aac,opus,flac,vorbis"},{"Container":"opus","Type":"Audio"},{"Container":"mp3","Type":"Audio","AudioCodec":"mp3"},{"Container":"mp2,mp3","Type":"Audio","AudioCodec":"mp2"},{"Container":"m4a","AudioCodec":"aac","Type":"Audio"},{"Container":"mp4","AudioCodec":"aac","Type":"Audio"},{"Container":"flac","Type":"Audio"},{"Container":"webma,webm","Type":"Audio"},{"Container":"wav","Type":"Audio","AudioCodec":"PCM_S16LE,PCM_S24LE"},{"Container":"ogg","Type":"Audio"},{"Container":"webm","Type":"Video","AudioCodec":"vorbis,opus","VideoCodec":"av1,VP8,VP9"}],"TranscodingProfiles":[],"ContainerProfiles":[],"CodecProfiles":[],"SubtitleProfiles":[{"Format":"vtt","Method":"Hls"},{"Format":"eia_608","Method":"VideoSideData","Protocol":"hls"},{"Format":"eia_708","Method":"VideoSideData","Protocol":"hls"},{"Format":"vtt","Method":"External"},{"Format":"ass","Method":"External"},{"Format":"ssa","Method":"External"}],"ResponseProfiles":[]}}

        );
        let response = client()
            .post(&url)
            .query(&params)
            .json(&profile)
            .send()
            .await?;
        response.json().await?
    };
    let mediainfo: Media = serde_json::from_value(json).unwrap();
    Ok(mediainfo)
}

pub async fn get_sub(id: String, sourceid: String) -> Result<Media, Error> {
    let server_info = config::set_config();
    let url = format!(
        "{}:{}/emby/Items/{}/PlaybackInfo",
        server_info.domain, server_info.port, id
    );

    let json: serde_json::Value = {
        let device_name = get_device_name();
        let device_id = env::var("UUID").unwrap();

        let params = Box::new([
            ("StartTimeTicks", "0"),
            ("UserId", &server_info.user_id),
            ("AutoOpenLiveStream", "False"),
            ("IsPlayback", "False"),
            ("AudioStreamIndex", "1"),
            ("SubtitleStreamIndex", "1"),
            ("MediaSourceId", &sourceid),
            ("MaxStreamingBitrate", "1000000000"),
            ("X-Emby-Client", "Tsukimi"),
            ("X-Emby-Device-Name", &device_name),
            ("X-Emby-Device-Id", &device_id),
            ("X-Emby-Client-Version", APP_VERSION),
            ("X-Emby-Token", &server_info.access_token),
            ("X-Emby-Language", "zh-cn"),
            ("reqformat", "json"),
        ]);
        let profile = serde_json::json!(

            {"DeviceProfile":{"Name":"Direct play all","MaxStaticBitrate":1000000000,"MaxStreamingBitrate":1000000000,"MusicStreamingTranscodingBitrate":1500000,"DirectPlayProfiles":[{"Container":"mkv","Type":"Video","VideoCodec":"hevc,h264,av1,vp8,vp9,mp4","AudioCodec":"aac,ac3,alac,eac3,dts,flac,mp3,opus,truehd,vorbis"},{"Container":"mp4,m4v","Type":"Video","VideoCodec":"hevc,h264,av1,vp8,vp9","AudioCodec":"aac,alac,opus,mp3,flac,vorbis"},{"Container":"flv","Type":"Video","VideoCodec":"h264","AudioCodec":"aac,mp3"},{"Container":"mov","Type":"Video","VideoCodec":"h264","AudioCodec":"aac,opus,flac,vorbis"},{"Container":"opus","Type":"Audio"},{"Container":"mp3","Type":"Audio","AudioCodec":"mp3"},{"Container":"mp2,mp3","Type":"Audio","AudioCodec":"mp2"},{"Container":"m4a","AudioCodec":"aac","Type":"Audio"},{"Container":"mp4","AudioCodec":"aac","Type":"Audio"},{"Container":"flac","Type":"Audio"},{"Container":"webma,webm","Type":"Audio"},{"Container":"wav","Type":"Audio","AudioCodec":"PCM_S16LE,PCM_S24LE"},{"Container":"ogg","Type":"Audio"},{"Container":"webm","Type":"Video","AudioCodec":"vorbis,opus","VideoCodec":"av1,VP8,VP9"}],"TranscodingProfiles":[],"ContainerProfiles":[],"CodecProfiles":[],"SubtitleProfiles":[{"Format":"vtt","Method":"Hls"},{"Format":"eia_608","Method":"VideoSideData","Protocol":"hls"},{"Format":"eia_708","Method":"VideoSideData","Protocol":"hls"},{"Format":"vtt","Method":"External"},{"Format":"ass","Method":"External"},{"Format":"ssa","Method":"External"}],"ResponseProfiles":[]}}

        );
        let response = client()
            .post(&url)
            .query(&params)
            .json(&profile)
            .send()
            .await?;
        response.json().await?
    };
    let mediainfo: Media = serde_json::from_value(json).unwrap();
    Ok(mediainfo)
}

#[derive(Serialize, Deserialize, Clone)]
pub struct View {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "CollectionType")]
    pub collection_type: Option<String>,
}

pub async fn get_library() -> Result<Vec<View>, Error> {
    let server_info = config::set_config();
    let url = format!(
        "{}:{}/emby/Users/{}/Views",
        server_info.domain, server_info.port, server_info.user_id
    );

    let mut json: serde_json::Value = {
        let device_name = &get_device_name();
        let device_id = &env::var("UUID").unwrap();
        let params = Box::new([
            ("X-Emby-Client", "Tsukimi"),
            ("X-Emby-Device-Name", device_name),
            ("X-Emby-Device-Id", device_id),
            ("X-Emby-Client-Version", APP_VERSION),
            ("X-Emby-Token", &server_info.access_token),
            ("X-Emby-Language", "zh-cn"),
        ]);
        let response = client().get(&url).query(&params).send().await?;
        response.json().await?
    };
    let views: Vec<View> = serde_json::from_value(json["Items"].take()).unwrap();
    let views_json = serde_json::to_string(&views).unwrap();
    let mut pathbuf = get_cache_dir(env::var("EMBY_NAME").unwrap());
    std::fs::DirBuilder::new()
        .recursive(true)
        .create(&pathbuf)
        .unwrap();
    pathbuf.push("views.json");
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&pathbuf)
        .unwrap();
    writeln!(file, "{}", views_json).unwrap();
    Ok(views)
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Latest {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Type")]
    pub latest_type: String,
    #[serde(rename = "UserData")]
    pub user_data: Option<UserData>,
    #[serde(rename = "ProductionYear")]
    pub production_year: Option<u32>,
}

pub async fn get_latest(id: String) -> Result<Vec<Latest>, Error> {
    let server_info = config::set_config();
    let url = format!(
        "{}:{}/emby/Users/{}/Items/Latest",
        server_info.domain, server_info.port, server_info.user_id
    );

    let json: serde_json::Value = {
        let device_name = &get_device_name();
        let device_id = &env::var("UUID").unwrap();
        let params = Box::new([
            ("Limit", "18"),
            (
                "Fields",
                "BasicSyncInfo,CanDelete,PrimaryImageAspectRatio,ProductionYear",
            ),
            ("ParentId", &id),
            ("ImageTypeLimit", "1"),
            ("EnableImageTypes", "Primary,Backdrop,Thumb"),
            ("X-Emby-Client", "Tsukimi"),
            ("X-Emby-Device-Name", device_name),
            ("X-Emby-Device-Id", device_id),
            ("X-Emby-Client-Version", APP_VERSION),
            ("X-Emby-Token", &server_info.access_token),
            ("X-Emby-Language", "zh-cn"),
        ]);
        let response = client().get(&url).query(&params).send().await?;
        response.json().await?
    };
    let latests: Vec<Latest> = serde_json::from_value(json).unwrap();
    let latests_json = serde_json::to_string(&latests).unwrap();
    let mut pathbuf = get_cache_dir(env::var("EMBY_NAME").unwrap());
    std::fs::DirBuilder::new()
        .recursive(true)
        .create(&pathbuf)
        .unwrap();
    pathbuf.push(format!("latest_{}.json", id));
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&pathbuf)
        .unwrap();
    writeln!(file, "{}", latests_json).unwrap();

    Ok(latests)
}

pub async fn get_list(
    id: String,
    start: String,
    mutex: std::sync::Arc<tokio::sync::Mutex<()>>,
) -> Result<List, Error> {
    let _ = mutex.lock().await;
    let server_info = config::set_config();
    let url = format!(
        "{}:{}/emby/Users/{}/Items",
        server_info.domain, server_info.port, server_info.user_id
    );

    let json: serde_json::Value = {
        let device_name = &get_device_name();
        let device_id = &env::var("UUID").unwrap();
        let params = Box::new([
            ("Limit", "30"),
            (
                "Fields",
                "BasicSyncInfo,CanDelete,PrimaryImageAspectRatio,ProductionYear,Status,EndDate",
            ),
            ("ParentId", &id),
            ("ImageTypeLimit", "1"),
            ("StartIndex", &start),
            ("IncludeItemTypes", "Movie,Series,MusicAlbum"),
            ("SortBy", "DateCreated,SortName"),
            ("SortOrder", "Descending"),
            ("Recursive", "True"),
            ("EnableImageTypes", "Primary,Backdrop,Thumb"),
            ("X-Emby-Client", "Tsukimi"),
            ("X-Emby-Device-Name", device_name),
            ("X-Emby-Device-Id", device_id),
            ("X-Emby-Client-Version", APP_VERSION),
            ("X-Emby-Token", &server_info.access_token),
            ("X-Emby-Language", "zh-cn"),
        ]);
        let response = client().get(&url).query(&params).send().await?;
        response.json().await?
    };
    let latests: List = serde_json::from_value(json).unwrap();
    Ok(latests)
}

#[derive(Deserialize, Clone, Default)]
pub struct List {
    #[serde(rename = "TotalRecordCount")]
    pub total_record_count: u32,
    #[serde(rename = "Items")]
    pub items: Vec<Latest>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Back {
    pub id: String,
    pub playsessionid: Option<String>,
    pub mediasourceid: String,
    pub tick: u64,
}

pub async fn positionback(back: Back) {
    let server_info = config::set_config();
    let url = format!(
        "{}:{}/emby/Sessions/Playing/Progress",
        server_info.domain, server_info.port
    );

    let device_name = &get_device_name();
    let device_id = &env::var("UUID").unwrap();
    let params = Box::new([
        ("X-Emby-Client-Version", APP_VERSION),
        ("X-Emby-Device-Name", device_name),
        ("X-Emby-Device-Id", device_id),
        ("X-Emby-Token", &server_info.access_token),
        ("X-Emby-Language", "zh-cn"),
        ("reqformat", "json"),
    ]);
    let profile = serde_json::json!({"VolumeLevel":100,"IsMuted":false,"IsPaused":false,"RepeatMode":"RepeatNone","SubtitleOffset":0,"PlaybackRate":1,"MaxStreamingBitrate":4000000,"PositionTicks":back.tick,"PlaybackStartTimeTicks":0,"SubtitleStreamIndex":1,"AudioStreamIndex":1,"BufferedRanges":[],"PlayMethod":"DirectStream","PlaySessionId":back.playsessionid,"MediaSourceId":back.mediasourceid,"CanSeek":true,"ItemId":back.id,"PlaylistIndex":0,"PlaylistLength":23,"NextMediaType":"Video"});
    client()
        .post(&url)
        .query(&params)
        .json(&profile)
        .send()
        .await
        .unwrap();
}

pub async fn positionstop(back: Back) {
    let server_info = config::set_config();
    let url = format!(
        "{}:{}/emby/Sessions/Playing/Stopped",
        server_info.domain, server_info.port
    );

    let device_name = &get_device_name();
    let device_id = &env::var("UUID").unwrap();
    let params = Box::new([
        ("X-Emby-Client-Version", APP_VERSION),
        ("X-Emby-Device-Name", device_name),
        ("X-Emby-Device-Id", device_id),
        ("X-Emby-Token", &server_info.access_token),
        ("X-Emby-Language", "zh-cn"),
        ("reqformat", "json"),
    ]);
    let profile = serde_json::json!({"VolumeLevel":100,"IsMuted":false,"IsPaused":false,"RepeatMode":"RepeatNone","SubtitleOffset":0,"PlaybackRate":1,"MaxStreamingBitrate":4000000,"PositionTicks":back.tick,"PlaybackStartTimeTicks":0,"SubtitleStreamIndex":1,"AudioStreamIndex":1,"BufferedRanges":[],"PlayMethod":"DirectStream","PlaySessionId":back.playsessionid,"MediaSourceId":back.mediasourceid,"CanSeek":true,"ItemId":back.id,"PlaylistIndex":0,"PlaylistLength":23,"NextMediaType":"Video"});
    client()
        .post(&url)
        .query(&params)
        .json(&profile)
        .send()
        .await
        .unwrap();
}

pub async fn playstart(back: Back) {
    let server_info = config::set_config();
    let url = format!(
        "{}:{}/emby/Sessions/Playing",
        server_info.domain, server_info.port
    );

    let device_name = &get_device_name();
    let device_id = &env::var("UUID").unwrap();
    let params = Box::new([
        ("X-Emby-Client-Version", APP_VERSION),
        ("X-Emby-Device-Name", device_name),
        ("X-Emby-Device-Id", device_id),
        ("X-Emby-Token", &server_info.access_token),
        ("X-Emby-Language", "zh-cn"),
        ("reqformat", "json"),
    ]);
    let profile = serde_json::json!({"VolumeLevel":100,"IsMuted":false,"IsPaused":false,"RepeatMode":"RepeatNone","SubtitleOffset":0,"PlaybackRate":1,"MaxStreamingBitrate":4000000,"PositionTicks":back.tick,"PlaybackStartTimeTicks":0,"SubtitleStreamIndex":1,"AudioStreamIndex":1,"BufferedRanges":[],"PlayMethod":"DirectStream","PlaySessionId":back.playsessionid,"MediaSourceId":back.mediasourceid,"CanSeek":true,"ItemId":back.id,"PlaylistIndex":0,"PlaylistLength":23,"NextMediaType":"Video"});
    client()
        .post(&url)
        .query(&params)
        .json(&profile)
        .send()
        .await
        .unwrap();
}

pub(crate) async fn similar(id: &str) -> Result<Vec<SearchResult>, Error> {
    let mut model = SearchModel {
        search_results: Vec::new(),
    };
    let server_info = config::set_config();

    let url = format!(
        "{}:{}/emby/Items/{}/Similar",
        server_info.domain, server_info.port, id
    );
    let mut json: serde_json::Value = {
        let device_name = &get_device_name();
        let device_id = &env::var("UUID").unwrap();
        let params = [
            (
                "Fields",
                "BasicSyncInfo,CanDelete,PrimaryImageAspectRatio,ProductionYear,Status,EndDate",
            ),
            ("UserId", &server_info.user_id),
            ("ImageTypeLimit", "1"),
            ("Limit", "12"),
            ("X-Emby-Client", "Tsukimi"),
            ("X-Emby-Device-Name", device_name),
            ("X-Emby-Device-Id", device_id),
            ("X-Emby-Client-Version", APP_VERSION),
            ("X-Emby-Token", &server_info.access_token),
            ("X-Emby-Language", "zh-cn"),
        ];

        let response = client().get(&url).query(&params).send().await?;
        response.json().await?
    };
    let items: Vec<SearchResult> = serde_json::from_value(json["Items"].take()).unwrap();
    model.search_results = items;
    Ok(model.search_results)
}

pub(crate) async fn person_item(id: &str, types: &str) -> Result<Vec<Item>, Error> {
    let server_info = config::set_config();

    let url = format!(
        "{}:{}/emby/Users/{}/Items",
        server_info.domain, server_info.port, server_info.user_id
    );
    let mut json: serde_json::Value = {
        let device_name = &get_device_name();
        let device_id = &env::var("UUID").unwrap();
        let params = [
            ("Fields", "PrimaryImageAspectRatio,ProductionYear"),
            ("PersonIds", id),
            ("Recursive", "True"),
            ("CollapseBoxSetItems", "False"),
            ("SortBy", "SortName"),
            ("SortOrder", "Ascending"),
            ("IncludeItemTypes", types),
            ("ImageTypeLimit", "1"),
            ("Limit", "12"),
            ("X-Emby-Client", "Tsukimi"),
            ("X-Emby-Device-Name", device_name),
            ("X-Emby-Device-Id", device_id),
            ("X-Emby-Client-Version", APP_VERSION),
            ("X-Emby-Token", &server_info.access_token),
            ("X-Emby-Language", "zh-cn"),
        ];

        let response = client().get(&url).query(&params).send().await?;
        response.json().await?
    };
    let items: Vec<Item> = serde_json::from_value(json["Items"].take()).unwrap();
    Ok(items)
}

pub async fn get_search_recommend() -> Result<List, Error> {
    let server_info = config::set_config();
    let url = format!(
        "{}:{}/emby/Users/{}/Items",
        server_info.domain, server_info.port, server_info.user_id
    );
    let json: serde_json::Value = {
        let device_name = get_device_name();
        let device_id = env::var("UUID").unwrap();
        let params = Box::new([
            ("Limit", "15"),
            ("EnableTotalRecordCount", "False"),
            ("ImageTypeLimit", "0"),
            ("Recursive", "True"),
            ("IncludeItemTypes", "Movie,Series"),
            ("SortBy", "IsFavoriteOrLiked,Random"),
            ("X-Emby-Client", "Tsukimi"),
            ("X-Emby-Device-Name", &device_name),
            ("X-Emby-Device-Id", &device_id),
            ("X-Emby-Client-Version", APP_VERSION),
            ("X-Emby-Token", &server_info.access_token),
            ("X-Emby-Language", "zh-cn"),
        ]);
        let response = client().get(&url).query(&params).send().await?;
        response.json().await?
    };
    let latests: List = serde_json::from_value(json).unwrap();
    Ok(latests)
}

pub(crate) async fn like_item(types: &str) -> Result<Vec<Item>, Error> {
    let server_info = config::set_config();

    let url = if types == "People" {
        format!("{}:{}/emby/Persons", server_info.domain, server_info.port)
    } else {
        format!(
            "{}:{}/emby/Users/{}/Items",
            server_info.domain, server_info.port, server_info.user_id
        )
    };
    let params = [
        (
            "Fields",
            "BasicSyncInfo,CanDelete,PrimaryImageAspectRatio,ProductionYear",
        ),
        ("Filters", "IsFavorite"),
        ("Recursive", "True"),
        ("CollapseBoxSetItems", "False"),
        ("SortBy", "SortName"),
        ("SortOrder", "Ascending"),
        ("IncludeItemTypes", types),
        ("Limit", "12"),
        if types == "People" {
            ("UserId", &server_info.user_id)
        } else {
            ("", "")
        },
        ("X-Emby-Client", "Tsukimi"),
        ("X-Emby-Device-Name", &get_device_name()),
        ("X-Emby-Device-Id", &env::var("UUID").unwrap()),
        ("X-Emby-Client-Version", APP_VERSION),
        ("X-Emby-Token", &server_info.access_token),
        ("X-Emby-Language", "zh-cn"),
    ];

    let response = client().get(&url).query(&params).send().await?;
    let mut json: serde_json::Value = response.json().await?;
    let items: Vec<Item> = serde_json::from_value(json["Items"].take()).unwrap();
    Ok(items)
}

pub async fn like(id: &str) -> Result<(), Error> {
    let server_info = config::set_config();
    let url = format!(
        "{}:{}/emby/Users/{}/FavoriteItems/{}",
        server_info.domain, server_info.port, server_info.user_id, id
    );

    let params = [
        ("X-Emby-Client", "Tsukimi"),
        ("X-Emby-Device-Name", &get_device_name()),
        ("X-Emby-Device-Id", &env::var("UUID").unwrap()),
        ("X-Emby-Client-Version", APP_VERSION),
        ("X-Emby-Token", &server_info.access_token),
        ("X-Emby-Language", "zh-cn"),
    ];
    client().post(&url).query(&params).send().await?;
    Ok(())
}

pub async fn unlike(id: &str) -> Result<(), Error> {
    let server_info = config::set_config();
    let url = format!(
        "{}:{}/emby/Users/{}/FavoriteItems/{}/Delete",
        server_info.domain, server_info.port, server_info.user_id, id
    );

    let params = [
        ("X-Emby-Client", "Tsukimi"),
        ("X-Emby-Device-Name", &get_device_name()),
        ("X-Emby-Device-Id", &env::var("UUID").unwrap()),
        ("X-Emby-Client-Version", APP_VERSION),
        ("X-Emby-Token", &server_info.access_token),
        ("X-Emby-Language", "zh-cn"),
    ];
    client().post(&url).query(&params).send().await?;
    Ok(())
}

pub async fn played(id: &str) -> Result<(), Error> {
    let server_info = config::set_config();
    let url = format!(
        "{}:{}/emby/Users/{}/PlayedItems/{}",
        server_info.domain, server_info.port, server_info.user_id, id
    );

    let params = [
        ("X-Emby-Client", "Tsukimi"),
        ("X-Emby-Device-Name", &get_device_name()),
        ("X-Emby-Device-Id", &env::var("UUID").unwrap()),
        ("X-Emby-Client-Version", APP_VERSION),
        ("X-Emby-Token", &server_info.access_token),
        ("X-Emby-Language", "zh-cn"),
    ];
    client().post(&url).query(&params).send().await?;
    Ok(())
}

pub async fn unplayed(id: &str) -> Result<(), Error> {
    let server_info = config::set_config();
    let url = format!(
        "{}:{}/emby/Users/{}/PlayedItems/{}/Delete",
        server_info.domain, server_info.port, server_info.user_id, id
    );

    let params = [
        ("X-Emby-Client", "Tsukimi"),
        ("X-Emby-Device-Name", &get_device_name()),
        ("X-Emby-Device-Id", &env::var("UUID").unwrap()),
        ("X-Emby-Client-Version", APP_VERSION),
        ("X-Emby-Token", &server_info.access_token),
        ("X-Emby-Language", "zh-cn"),
    ];
    client().post(&url).query(&params).send().await?;
    Ok(())
}

use dotenv::dotenv;
use egui::Image;
use egui_extras::RetainedImage;
use error_chain::error_chain;
use futures::executor;
use image::DynamicImage;
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use std::env;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest_wasm::Error);
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Pagination {
    pub cursor: String,
}

#[derive(Deserialize, Serialize)]
pub struct ResponseStreamsData {
    pub id: String,
    pub user_id: String,
    pub user_login: String,
    pub user_name: String,
    pub game_id: String,
    pub game_name: String,
    pub title: String,
    pub viewer_count: u32,
    pub started_at: String,
    pub language: String,
    pub thumbnail_url: String,
    pub tag_ids: Vec<String>,
    pub is_mature: bool,
    #[serde(rename(deserialize = "type"))]
    pub stream_type: String,
    #[serde(skip)]
    pub image: Option<RetainedImage>,
}

#[derive(Deserialize, Serialize)]
pub struct ResponseStreams {
    pub data: Vec<ResponseStreamsData>,
    pub pagination: Pagination,
}

pub fn run() -> Result<ResponseStreamsData> {
    dotenv().ok();
    let popular_streams = executor::block_on(get_streams())?;
    println!("Choosing from {} streams", popular_streams.len());
    let selected_stream = executor::block_on(choose_stream(popular_streams))?;
    println!(
        "Chosen: {} streaming '{}' to {} viewers",
        selected_stream.user_name, selected_stream.game_name, selected_stream.viewer_count
    );
    Ok(selected_stream)
}

pub async fn choose_stream(streams: Vec<ResponseStreamsData>) -> Result<ResponseStreamsData> {
    let mut rng = thread_rng();
    let stream = streams.choose(&mut rng).unwrap().clone();
    let url = stream
        .thumbnail_url
        .replace("{width}", &(1920 / 3).to_string())
        .replace("{height}", &(1080 / 3).to_string());
    let image =
        RetainedImage::from_image_bytes("image", &reqwest_wasm::get(&url).await?.bytes().await?)
            .ok();
    Ok(ResponseStreamsData {
        image,
        id: stream.id.clone(),
        user_id: stream.user_id.clone(),
        user_login: stream.user_login.clone(),
        game_id: stream.game_id.clone(),
        game_name: stream.game_name.clone(),
        user_name: stream.user_name.clone(),
        title: stream.title.clone(),
        viewer_count: stream.viewer_count,
        started_at: stream.started_at.clone(),
        language: stream.language.clone(),
        thumbnail_url: stream.thumbnail_url.clone(),
        tag_ids: stream.tag_ids.clone(),
        is_mature: stream.is_mature,
        stream_type: stream.stream_type.clone(),
    })
}

pub async fn get_token() -> Result<()> {
    let client = reqwest_wasm::Client::new();
    let client_secret = env::var("CLIENT_SECRET").unwrap();
    let client_id = env::var("CLIENT_ID").unwrap();
    let res = client
        .post("https://id.twitch.tv/oauth2/token")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(format!(
            "client_id={client_id}
        &client_secret={client_secret}
        &grant_type=client_credentials"
        ))
        .send()
        .await?
        .text()
        .await?;
    println!("{res}");
    Ok(())
}

pub async fn get_streams() -> Result<Vec<ResponseStreamsData>> {
    let client = reqwest_wasm::Client::new();
    let access_token = env::var("ACCESS_TOKEN").unwrap();
    let client_id = env::var("CLIENT_ID").unwrap();
    let res = client
        .get("https://api.twitch.tv/helix/streams")
        .query(&[("first", "100")])
        .bearer_auth(access_token)
        .header("Client-Id", client_id)
        .send()
        .await?
        .json::<ResponseStreams>()
        .await?;
    Ok(res.data)
}

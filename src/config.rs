use serde::Deserialize;
use std::sync::LazyLock;

pub static CONFIG: LazyLock<Config> =
    LazyLock::new(|| serde_json::from_slice(include_bytes!("../config.json")).unwrap());

#[derive(Deserialize)]
pub struct Config {
    pub api: Api,
    pub app_settings: AppSettings,
}

#[derive(Deserialize)]
pub struct Api {
    pub web: String,
    pub grpc: String,
}

#[derive(Deserialize)]
pub struct AppSettings {
    pub id: String,
    pub home_dir: String,
    pub window_settings: WindowSettings,
}

#[derive(Deserialize)]
pub struct WindowSettings {
    pub width: f32,
    pub height: f32,
    pub fullscreen: bool,
    pub resizable: bool,
}

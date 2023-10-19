use dotenvy::dotenv;
use log::info;
use std::env;

pub struct EnvConfig {
    pub api_key: String,
    pub app_id: String,
}

impl EnvConfig {
    pub fn new() -> EnvConfig {
        dotenv().ok();
        info!("[+] Reading ENV configuration.");

        EnvConfig {
            api_key: env::var("APIDECK_API_KEY").expect("APIDECK_API_KEY must be set"),
            app_id: env::var("APIDECK_APP_ID").expect("APIDECK_APP_ID must be set"),
        }
    }
}

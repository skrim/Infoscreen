use std::{sync::Arc, time::{self, Duration}};

use async_trait::async_trait;

use crate::{api::ApiQuery, config::InfoscreenConfig};

pub struct OpenWeatherMap;

#[async_trait]
impl ApiQuery for OpenWeatherMap {
    fn get_name(&self) -> String {
        "weather".to_string()
    }

    async fn get_response(&self, config: Arc<InfoscreenConfig>) -> (String, std::time::Instant) {
        let url = format!("https://api.openweathermap.org/data/3.0/onecall?lat=60.1689&lon=24.92187&appid={}&exclude=minutely", config.openweathermap_key);
        println!("Calling api at {}", url);

        let response = reqwest::get(&url).await.expect("failed to send request");
        let response_text = response.text().await.expect("failed to read response text");
        let valid_until = time::Instant::now() + Duration::from_secs(2 * 60 * 60);
        (response_text, valid_until)
    }
}
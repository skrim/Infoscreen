extern crate libc;
extern crate confy;
extern crate serde_derive;

use chrono::Timelike;
use tokio::fs;

use crate::config::InfoscreenConfig;
use crate::web_server_host::WebServerHost;

mod api;
mod api_digitransit;
mod api_openweathermap;
mod api_wodconnect;
mod browser_control;
mod config;
mod data_manager;
mod eink_screen;
mod web_server;
mod web_server_host;

fn load_config() -> InfoscreenConfig {
    confy::load("infoscreen", None).expect("failed to load config")
}

async fn screen_update() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Starting update task");
    eink_screen::clear();

    let config = load_config();
    let image_path = config.work_directory + "/screenshot.bmp";
    let browser_control = browser_control::BrowserControl::new(load_config());

    loop {
        let timestamp = chrono::Utc::now().timestamp();
        let url = format!("http://{}/index.html?{}", config.listen_address, timestamp);
        println!("Loading from {}", url);

        let image_data = browser_control.take_screenshot(url);
        println!("Received {} bytes", image_data.len());
        fs::write(image_path.clone(), image_data).await?;

        eink_screen::display_bmp(&image_path, 0, 0, 1200, 825);

        let remaining_seconds = 60 - chrono::Utc::now().second();
        println!("Sleeping for {} seconds", remaining_seconds);

        // headless_chrome gets stuck with async sleep!
        std::thread::sleep(std::time::Duration::from_secs(remaining_seconds.into()));
    }
}

#[tokio::main]
async fn main() {
    let screen_update_task = tokio::spawn(async { screen_update().await.expect("Screen update failed") });
    let web_server_task = tokio::spawn(async { WebServerHost::run(load_config()).await.expect("Web server failed") });

    tokio::try_join!(screen_update_task, web_server_task).expect("Task join failed");
}

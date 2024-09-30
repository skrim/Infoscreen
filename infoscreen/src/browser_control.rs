use std::ffi::OsStr;
use std::io::Cursor;
use std::path::PathBuf;
use std::sync::Arc;

use headless_chrome::{Browser, LaunchOptions, Tab};
use headless_chrome::protocol::cdp::Page;
use image::ImageFormat;

use crate::config::InfoscreenConfig;

pub struct BrowserControl {
    // if this is not stored, headless_chrome crashes
    _browser: Browser,
    tab: Arc<Tab>
}

impl BrowserControl {
    pub fn new(config: InfoscreenConfig) -> BrowserControl {
        let launch_options = LaunchOptions::default_builder()
            .window_size(Some((1200, 825)))
            .headless(true)
            .enable_logging(true)
            .idle_browser_timeout(std::time::Duration::from_secs(300))
            .args([
                OsStr::new("--hide-scrollbars")
            ].to_vec())
            .user_data_dir(Some(PathBuf::from(format!("{}/infoscreen-chrome-profile", config.work_directory))))
            .build()
            .expect("Couldn't find Chrome binary.");

        let browser = Browser::new(launch_options).expect("Failed to launch browser");
        let tab = browser.new_tab().expect("New tab failed");

        BrowserControl { _browser: browser, tab }
    }

    fn convert_png_to_bmp(&self, input: Vec<u8>) -> Vec<u8> {
        let img = image::load_from_memory(&input).expect("failed to load image");
        let mut output = vec![];
        img.write_to(&mut Cursor::new(&mut output), ImageFormat::Bmp).expect("failed to write image");
        output
    }

    pub fn take_screenshot(&self, url: String) -> Vec<u8> {
        self.tab.navigate_to(&url).expect("Navigate failed");
        self.tab.wait_until_navigated().expect("Wait until navigated failed");
        let image_data = self.tab.capture_screenshot(Page::CaptureScreenshotFormatOption::Png, None, None, true).expect("Screenshot failed");

        self.convert_png_to_bmp(image_data)
    }
}
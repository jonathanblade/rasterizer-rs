use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use fantoccini::{Client, ClientBuilder};
use serde_json::json;
use serde_json::map::Map as JsonMap;
use url::Url;

use crate::error::RasterizerError;

pub struct RasterizerWebDriver {
    pub client: Client,
}

impl RasterizerWebDriver {
    pub async fn new(wd_url: Url) -> Self {
        let mut caps = JsonMap::new();
        let opts = json!({ "args": ["--headless"] });
        caps.insert("moz:firefoxOptions".to_string(), opts);
        let client = ClientBuilder::native()
            .capabilities(caps)
            .connect(wd_url.as_str())
            .await
            .expect("Failed to create webdriver client");
        Self { client }
    }

    pub async fn get_screenshot(
        &self,
        url: Url,
        width: u32,
        height: u32,
    ) -> Result<String, RasterizerError> {
        self.client.goto(url.as_str()).await?;
        self.client.set_window_size(width, height).await?;
        let screenshot = self.client.screenshot().await?;
        let screenshot = BASE64.encode(&screenshot);
        Ok(screenshot)
    }
}

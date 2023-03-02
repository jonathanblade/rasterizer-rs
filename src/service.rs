use url::Url;

use crate::context::RasterizerContext;
use crate::error::RasterizerError;

pub struct RasterizerService;

impl RasterizerService {
    pub async fn get_screenshot(
        ctx: &RasterizerContext,
        url: Url,
        width: u32,
        height: u32,
    ) -> Result<String, RasterizerError> {
        let screenshot = ctx.wd.get_screenshot(url, width, height).await?;
        Ok(screenshot)
    }
}

use poem::web::Data;
use poem::Result;
use poem_openapi::param::Query;
use poem_openapi::OpenApi;
use url::Url;

use crate::context::RasterizerContext;
use crate::schemas::GetScreenshotResponse;
use crate::service::RasterizerService;

pub struct RasterizerApi;

#[OpenApi]
impl RasterizerApi {
    /// Get screenshot
    #[oai(path = "/screenshot", method = "get")]
    async fn get_screenshot(
        &self,
        ctx: Data<&RasterizerContext>,
        url: Query<Url>,
        width: Query<u32>,
        height: Query<u32>,
    ) -> Result<GetScreenshotResponse> {
        let data = RasterizerService::get_screenshot(ctx.0, url.0, width.0, height.0).await?;
        Ok(data.into())
    }
}

use poem::error::ResponseError;
use poem::http::{header, StatusCode};
use poem::{Body, Endpoint, IntoResponse, Middleware, Request, Response, Result};

use crate::schemas::ErrorSchema;

#[derive(Debug, thiserror::Error)]
pub enum RasterizerError {
    #[error(transparent)]
    Database(#[from] sqlx::Error),
    #[error(transparent)]
    WebDriver(#[from] fantoccini::error::CmdError),
}

impl ResponseError for RasterizerError {
    fn status(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }

    fn as_response(&self) -> Response {
        build_response_error(self.status(), self.to_string())
    }
}

impl IntoResponse for RasterizerError {
    fn into_response(self) -> Response {
        build_response_error(self.status(), self.to_string())
    }
}

pub struct CatchError;

pub struct CatchErrorEndpoint<E> {
    ep: E,
}

impl<E: Endpoint> Middleware<E> for CatchError {
    type Output = CatchErrorEndpoint<E>;

    fn transform(&self, ep: E) -> Self::Output {
        CatchErrorEndpoint { ep }
    }
}

#[poem::async_trait]
impl<E: Endpoint> Endpoint for CatchErrorEndpoint<E> {
    type Output = Response;

    async fn call(&self, req: Request) -> Result<Self::Output> {
        let resp = self.ep.call(req).await;
        match resp {
            Ok(resp) => Ok(resp.into_response()),
            Err(e) => Ok(build_response_error(e.status(), e.to_string())),
        }
    }
}

fn build_response_error(status: StatusCode, s: String) -> Response {
    let schema = ErrorSchema::new(s);
    let body = Body::from_json(schema).unwrap();
    Response::builder()
        .header(header::CONTENT_TYPE, "application/json; charset=utf-8")
        .status(status)
        .body(body)
}

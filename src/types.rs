use poem::middleware::{AddDataEndpoint, CorsEndpoint};
use poem::Route;

use crate::context::RasterizerContext;
use crate::error::CatchErrorEndpoint;

pub type RasterizerEnpoint =
    AddDataEndpoint<CorsEndpoint<CatchErrorEndpoint<Route>>, RasterizerContext>;
pub type DbPool = sqlx::PgPool;

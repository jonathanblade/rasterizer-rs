use std::sync::Arc;

use crate::database::RasterizerDatabase;
use crate::webdriver::RasterizerWebDriver;

#[derive(Clone)]
pub struct RasterizerContext {
    pub db: Arc<RasterizerDatabase>,
    pub wd: Arc<RasterizerWebDriver>,
}

impl RasterizerContext {
    pub fn new(db: RasterizerDatabase, wd: RasterizerWebDriver) -> Self {
        Self {
            db: Arc::new(db),
            wd: Arc::new(wd),
        }
    }
}

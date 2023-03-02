use std::net::SocketAddr;

use dotenv::dotenv;
use url::Url;

use crate::utils::read_env_url;
use crate::utils::read_env_var;

#[derive(Clone)]
pub struct RasterizerConfig {
    pub db_url: Url,
    pub wd_url: Url,
    pub server_addr: SocketAddr,
}

impl RasterizerConfig {
    pub fn new() -> Self {
        dotenv().ok();
        let db_url = read_env_url("DATABASE_URL");
        let wd_url = read_env_url("WEBDRIVER_URL");
        let server_addr = read_env_var::<SocketAddr>("SERVER_ADDRESS");
        Self {
            db_url,
            wd_url,
            server_addr,
        }
    }
}

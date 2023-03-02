use std::{env, fmt, str::FromStr};

use url::Url;

pub fn capitalize_string(s: String) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn read_env_var<T: FromStr>(key: &str) -> T
where
    T: FromStr,
    <T as FromStr>::Err: fmt::Debug,
{
    env::var(key)
        .expect(format!("Missing environment variable {}", key).as_str())
        .parse::<T>()
        .expect(format!("Failed to parse environment variable {}", key).as_str())
}

pub fn read_env_url(key: &str) -> Url {
    let var = read_env_var::<String>(key);
    Url::parse(var.as_str()).expect(&format!("Failed to parse environment variable {}", key))
}

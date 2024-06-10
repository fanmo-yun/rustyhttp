use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    server: ServerConfig,
    public: PublicFileConfig,
    custom: Vec<CustomPathConfig>,
    cache: CacheConfig,
}

#[derive(Debug, Deserialize)]
struct ServerConfig {
    address: Option<String>,
    port: Option<u16>,
    cap_size: Option<u16>,
}

#[derive(Debug, Deserialize)]
struct PublicFileConfig {
    public_path: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CustomPathConfig {
    custom_path: Option<String>,
    index: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CacheConfig {
    enabled: Option<bool>,
    max_size: Option<u64>,
    expiration_time: Option<u64>,
}

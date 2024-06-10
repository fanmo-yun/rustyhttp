use std::path::PathBuf;

use tokio::{
    fs::File,
    io::{AsyncReadExt, BufReader},
};

use super::conf::Config;

pub async fn read_config() -> Config {
    let mut config_path = PathBuf::new();
    config_path.push("conf");
    config_path.push("config.toml");

    let mut buf = String::new();
    let fp = File::open(config_path.as_path()).await.unwrap();
    let mut reader = BufReader::new(fp);
    reader.read_to_string(&mut buf).await.unwrap();

    let config: Config = toml::from_str(&buf).unwrap();
    return config;
}

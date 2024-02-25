

use clap::{Arg, Command};
use serde::Deserialize;
use std::{fs, path::Path};

#[derive(Deserialize)]
struct Config {
    settings: Settings,
}

#[derive(Deserialize)]
struct Settings {
    log_level: String,
    // max_connections: u32,
    // enable_caching: bool,
    // cache_duration: u64,
}

fn main() {
    let matches = Command::new("My CLI App")
        .version("1.0")
        .author("Your Name <your_email@example.com>")
        .about("Does awesome things")
        .arg(Arg::new("config")
             .short('c')
             .long("config")
             .value_name("FILE")
             .help("Sets a custom config file"))
        .get_matches();

    let default_config = "default.conf".to_string();
    let config_path = matches.get_one::<String>("config").unwrap_or(&default_config);
    let config = read_config(config_path).expect("Failed to read config");

    println!("Settings: {}", config.settings.log_level);
}

fn read_config<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn std::error::Error>> {
    let config_str = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&config_str)?;
    Ok(config)
}


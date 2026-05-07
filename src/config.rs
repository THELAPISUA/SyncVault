use serde::Deserialize;
use serde::Serialize;
use std::fs;
use std::io::BufWriter;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub name: String,
    pub port: u16,
    pub ip: String,
    pub version: String,
    pub keys: String,
    pub data: String,
}

impl Config {
    pub fn get_basic_config() -> Self {
        Config {
            name: String::from("SYNCVAULT"),
            port: 9090,
            ip: String::from("0.0.0.0"),
            version: String::from("1.0"),
            keys: String::from("keys.txt"),
            data: String::from("data.txt"),
        }
    }
}

pub fn load_config(path: &str) -> Config {
    let file = fs::read_to_string(path).unwrap_or("".to_string());

    let config = yaml_serde::from_str(&file);

    match config {
        Ok(con) => con,
        Err(_) => {
            let basic_config = Config::get_basic_config();
            let _ = fs::File::create_new("keys.txt");
            let _ = fs::File::create_new("data.txt");
            let con_file = fs::File::create_new("config.yaml").unwrap();
            let writer = BufWriter::new(con_file);
            yaml_serde::to_writer(writer, &basic_config).unwrap();

            basic_config
        }
    }
}



use serde::{Serialize, Deserialize};
use serde_json;
use std::{env, fs};
use std::fs::File;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
struct Config
{
    host: String,
    port: u16,
}

fn read_from_file()
{
    let cfg: Config = Config { host: "127.0.0.1".to_string(), port: 8080 };
    let configFile: PathBuf = env::current_dir().unwrap().join("resources/config.json");

    // Save to file
    let json_str: String = serde_json::to_string_pretty(&cfg).unwrap();
    fs::write(&configFile, json_str).unwrap();

    // Load from file
    let data: String  = fs::read_to_string(&configFile).unwrap();
    let decoded: Config = serde_json::from_str(&data).unwrap();

    println!("Config loaded: {:?}", decoded);
}

pub fn test_all()
{
    read_from_file();
}
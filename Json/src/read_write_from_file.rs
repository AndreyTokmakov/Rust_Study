

use serde::{Serialize, Deserialize};
use serde_json;
use std::{env, fs};
use std::fmt::format;
use std::fs::File;
use std::path::PathBuf;
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
struct Config
{
    host: String,
    port: u16,
}

 static JSON_DIR: &str = "resources/json_files";

fn read_from_file()
{
    let cfg: Config = Config { host: "127.0.0.1".to_string(), port: 8080 };
    let configFile: PathBuf = env::current_dir().unwrap().join(format!("{}/config.json", JSON_DIR));

    // Save to file
    let json_str: String = serde_json::to_string_pretty(&cfg).unwrap();
    fs::write(&configFile, json_str).unwrap();

    // Load from file
    let data: String  = fs::read_to_string(&configFile).unwrap();
    let decoded: Config = serde_json::from_str(&data).unwrap();

    println!("Config loaded: {:?}", decoded);
}

fn parse_values_from_file()
{
    let file_path: PathBuf = env::current_dir().unwrap().join(format!("{}/simple_value.json", JSON_DIR));

    let data: String = fs::read_to_string(file_path).unwrap();
    let v: Value = serde_json::from_str(&data).unwrap();

    println!("Status: {}", v["status"]);
    println!("Count: {}", v["count"]);
    println!("First item: {}", v["items"][0]);
}

pub fn test_all()
{
    // read_from_file();
    parse_values_from_file();
}
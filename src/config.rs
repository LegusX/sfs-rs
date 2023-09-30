use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::str;
use fancy_regex::Regex;
use std::env;
use std::path::Path;
use simple_home_dir::home_dir;

#[derive(Debug)]
#[derive(Hash)]
pub struct Device {
    pub id: String,
    timeused: String,
    pub description: String,
    tokenid: String,
}

#[derive(Debug)]
pub struct Config {
    path: String,
    pub vdf: Vec<Device>,
    raw: String,
}

pub enum Error {
    NoAuthorizedDevice,
    ConfigNotFound,
}
impl Config {
    //Writes the new AuthorizedDevice value to the config file
    pub fn write(&self) -> () {
        let replace_regex = Regex::new(r#""AuthorizedDevice"(.|\n)*(?=}\n\s})}"#);
        let mut replace_string: String = String::from("\"AuthorizedDevice\"\n        {");
        for device in &self.vdf {
            let device_string = format!(
                r#"
                "{}"
                {{
                    "timeused"		"{}"
                    "description"		"{}"
                    "tokenid"		"{}"
                }}"#,
                device.id,
                device.timeused,
                device.description,
                device.tokenid
            );
            replace_string.push_str(&device_string);
        }
        let new_text = replace_regex.unwrap().replace(&self.raw, replace_string);

        fs::write(&self.path, new_text.as_ref()).expect("Failed to write to config file!");
    }
    pub fn init(&mut self, config_path: String) -> Result<(), Error> {
        let raw_vdf = read_steam_config(&config_path);
        create_backup(&raw_vdf, &config_path);
        let vdf = parse_raw_vdf(&raw_vdf)?;
        self.path = config_path;
        self.vdf = vdf;
        self.raw = raw_vdf.to_string();
        Ok(())
    }
}

pub fn new() -> Config {
    let config = Config {
        path: Default::default(),
        vdf: Default::default(),
        raw: Default::default(),
    };
    return config;
}

fn read_steam_config(path: &str) -> String {
    let mut file = File::open(path).expect("Failed to open config file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read config file to string");

    return contents;
}

fn create_backup(raw: &str, config_path: &str) -> () {
    let backup_path = config_path.replace("config.vdf", "config.vdf.backup");
    fs::write(&backup_path, raw).expect("Failed to write config backup file!");
}

//Creates a struct to represent the config.vdf
fn parse_raw_vdf(file: &str) -> Result<Vec<Device>, Error> {
    let mut data: Vec<Device> = Default::default();
    let regex: Regex = Regex::new(
        r#"(?P<id>\d+)"\s*\{\s*"timeused"\s*"(?P<timeused>\d+)"\s*"description"\s*"(?P<description>[^"]+)"\s*"tokenid"\s*"(?P<tokenid>-?\d+)"\s*}"#
    ).unwrap();
    let captures = regex.captures_iter(file);

    for capture in captures {
        let unwrapped = match capture {
            Ok(c) => c,
            Err(_e) => {
                return Err(Error::NoAuthorizedDevice);
            }
        };
        let device = Device {
            id: unwrapped.get(1).unwrap().as_str().to_string(),
            timeused: unwrapped.get(2).unwrap().as_str().to_string(),
            description: unwrapped.get(3).unwrap().as_str().to_string(),
            tokenid: unwrapped.get(4).unwrap().as_str().to_string(),
        };
        data.push(device);
    }
    return Ok(data);
}

pub fn get_config_path() -> Result<String, Error> {
    let vdf_path: String;
    if cfg!(debug_assertions) && !cfg!(target_os = "windows") {
        vdf_path = String::from("./config.vdf");
    } else if cfg!(target_os = "linux") {
        let linux_path = format!(
            "{}/.local/share/Steam/config/config.vdf",
            home_dir().unwrap().to_string_lossy()
        );
        if file_exists(linux_path.as_ref()) {
            vdf_path = linux_path;
        } else {
            return Err(Error::ConfigNotFound);
        }
    } else if cfg!(target_os = "windows") {
        let windows_path = r#"C:\Program Files (x86)\Steam\config\config.vdf"#;
        if file_exists(windows_path) {
            vdf_path = windows_path.to_string();
        } else {
            return Err(Error::ConfigNotFound);
        }
    } else {
        panic!("OS {} not supported", env::consts::OS);
    }
    return Ok(vdf_path);
}

fn file_exists(path: &str) -> bool {
    return Path::new(path).is_file();
}

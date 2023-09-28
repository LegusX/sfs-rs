use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::path::Path;
use simple_home_dir::home_dir;
use std::process::Command;
use std::str;
use fancy_regex::Regex;

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
}

pub fn new() -> Config {
    let config_path = get_config_path();
    let raw_vdf = read_steam_config(&config_path);
    create_backup(&raw_vdf, &config_path);
    let vdf = parse_raw_vdf(&raw_vdf);

    let config = Config {
        path: config_path,
        vdf,
        raw: raw_vdf.to_string(),
    };
    return config;
}

fn get_config_path() -> String {
    let vdf_path: String;
    if cfg!(debug_assertions) {
        vdf_path = String::from("./config.vdf");
    } else if cfg!(target_os = "linux") {
        let linux_path = format!(
            "{}/.local/share/Steam/config/config.vdf",
            home_dir().unwrap().to_string_lossy()
        );
        if file_exists(linux_path.as_ref()) {
            vdf_path = linux_path;
        } else {
            panic!("Can't find config path");
        }
    } else if cfg!(target_os = "windows") {
        let output = Command::new("where")
            .arg("steam")
            .output()
            .expect("Failed to execute process");
        let steam_install_str = str
            ::from_utf8(output.stdout.as_slice())
            .expect("Failed to retrieve steam install path");
        let steam_install_path = Path::new(steam_install_str);
        let steam_root: &str = steam_install_path.parent().unwrap().to_str().unwrap();
        let windows_path = format!("{}/config/config.vdf", steam_root);

        if file_exists(&windows_path) {
            vdf_path = format!("{}/config/config.vdf", steam_root);
        } else {
            panic!("Can't find config path");
        }
    } else {
        panic!("OS {} not supported", env::consts::OS);
    }
    return vdf_path;
}

fn file_exists(path: &str) -> bool {
    return Path::new(path).is_file();
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
fn parse_raw_vdf(file: &str) -> Vec<Device> {
    let mut data: Vec<Device> = Default::default();
    let regex: Regex = Regex::new(
        r#"(?P<id>\d+)"\s*\{\s*"timeused"\s*"(?P<timeused>\d+)"\s*"description"\s*"(?P<description>[^"]+)"\s*"tokenid"\s*"(?P<tokenid>-?\d+)"\s*}"#
    ).unwrap();
    let captures = regex.captures_iter(file);

    for capture in captures {
        let unwrapped = capture.unwrap();
        let device = Device {
            id: unwrapped.get(1).unwrap().as_str().to_string(),
            timeused: unwrapped.get(2).unwrap().as_str().to_string(),
            description: unwrapped.get(3).unwrap().as_str().to_string(),
            tokenid: unwrapped.get(4).unwrap().as_str().to_string(),
        };
        data.push(device);
    }
    return data;
}

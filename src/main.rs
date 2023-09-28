use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::path::Path;
use simple_home_dir::home_dir;
use std::process::Command;
use std::str;
use fancy_regex::Regex;

fn main() {
    let config_path = get_config_path();
    //TODO: Make backup of config file
    let raw_vdf = read_steam_config(&config_path);
    let vdf = parse_raw_vdf(&raw_vdf);
    write_config(&vdf, &raw_vdf, &config_path);
}

fn file_exists(path: &str) -> bool {
    return Path::new(path).is_file();
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
        let output = Command::new("which")
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

fn read_steam_config(path: &str) -> String {
    let mut file = File::open(path).expect("Failed to open config file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read config file to string");

    return contents;
}

#[derive(Debug)]
struct Device<'a> {
    id: &'a str,
    timeused: &'a str,
    description: &'a str,
    tokenid: &'a str,
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
            id: unwrapped.get(1).unwrap().as_str(),
            timeused: unwrapped.get(2).unwrap().as_str(),
            description: unwrapped.get(3).unwrap().as_str(),
            tokenid: unwrapped.get(4).unwrap().as_str(),
        };
        data.push(device);
    }
    return data;
}

//Writes the new AuthorizedDevice value to the config file
fn write_config(vdf: &Vec<Device>, raw: &str, config_path: &str) -> () {
    let replace_regex = Regex::new(r#""AuthorizedDevice"(.|\n)*(?=}\n\s})}"#);
    let mut replace_string: String = String::from("\"AuthorizedDevice\"\n        {");
    for device in vdf {
        let device_string = format!(
            r#"
                "{}"
                {{
                    "timeused"		"{}"
                    "description"		"{}"
                    "tokenid"		"{}"
                }}"#,
            device.tokenid,
            device.timeused,
            device.description,
            device.id
        );
        replace_string.push_str(&device_string);
    }
    let new_text = replace_regex.unwrap().replace(raw, replace_string);

    fs::write(config_path, new_text.as_ref()).expect("Failed to write to config file!");
}

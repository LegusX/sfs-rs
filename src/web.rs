use std::env;
use reqwest::blocking::get;
use serde::Deserialize;

pub enum Error {
    InvalidID,
    RequestFailed,
}

//Most of these fields won't be read, but we need them to deserialize the api response
#[derive(Hash)]
#[derive(Deserialize)]
#[allow(dead_code)]
pub struct User {
    steamid: String,
    communityvisibilitystate: isize,
    profilestate: isize,
    pub personaname: String,
    profileurl: String,
    pub avatar: String,
    pub avatarmedium: String,
    pub avatarfull: String,
    avatarhash: String,
    personastate: isize,
    realname: String,
    primaryclanid: String,
    timecreated: isize,
    personastateflags: isize,
    loccountrycode: String,
    locstatecode: String,
    loccityid: isize,
}

pub fn get_users(devices: &Vec<crate::config::Device>) -> Result<Vec<User>, Error> {
    println!("Hello world");
    let mut ids: Vec<String> = Default::default();

    for device in devices {
        let id: usize = match device.id.parse::<usize>() {
            //Convert from AccountID to CommunityID
            Ok(i) => i * 2 + 76561197960265728,
            Err(_) => {
                return Err(Error::InvalidID);
            }
        };
        ids.push(id.to_string());
    }
    println!("{:?}", ids);
    let result = get(
        format!(
            "https://api.steampowered.com/ISteamUser/GetPlayerSummaries/v0002/?key={}&steamids={}",
            env::var("STEAM_API").expect("STEAM_API not set"),
            ids.join(",")
        )
    );
    match result {
        Ok(response) => {
            match response.json::<Vec<User>>() {
                Ok(data) => Ok(data),
                Err(_) => Err(Error::RequestFailed),
            }
        }
        Err(_) => {
            return Err(Error::RequestFailed);
        }
    }
}

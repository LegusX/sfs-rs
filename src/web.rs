use std::env;

#[derive(Debug)]
pub enum Error {
    InvalidID,
    RequestFailed,
}

#[derive(Debug, Hash)]
pub struct User {
    pub personaname: String,
    pub uri: String,
}

pub fn get_users(devices: &Vec<crate::config::Device>) -> Result<Vec<User>, Error> {
    let mut ids: Vec<String> = Default::default();

    for device in devices {
        let id: u64 = match device.id.parse::<u64>() {
            //Convert from AccountID to CommunityID
            Ok(i) => i + 76561197960265728,
            Err(_) => {
                return Err(Error::InvalidID);
            }
        };
        ids.push(id.to_string());
    }
    let users = steam_api::get_player_summaries(
        &ids.join(","),
        &env::var("STEAM_API").expect("STEAM_API not set")
    );
    match users {
        Ok(users) => {
            Ok(
                users
                    .into_iter()
                    .map(|steam_user| {
                        User {
                            personaname: steam_user.personaname,
                            uri: steam_user.avatarfull,
                        }
                    })
                    .collect()
            )
        }
        Err(_) => {
            return Err(Error::RequestFailed);
        }
    }
}

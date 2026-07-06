use serde::{Deserialize, Serialize};
use crate::database::User;

#[derive(Serialize, Deserialize, Debug)]
pub struct Body {
    pub message: String,
    // TODO : Implement token for each use
    // pub token: String,
}

pub fn get_from_json(json: String) -> Option<Body> {
    match serde_json::from_str(&json) {
        Ok(t) => return Some(t),
        Err(e) => {
            eprintln!("JSON ERROR : {e}");
            return None;
        }
    };
}

// Function for deconstructing User struct at database::User
pub fn user_from_json(json: String) -> Option<User> {
    match serde_json::from_str(&json) {
        Ok(t) => return Some(t),
        Err(e) => {
            eprintln!("JSON ERROR : {e}");
            return None;
        }
    };
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Body {
    pub message: String,
    // TODO : Implement token for each use
    // pub token: String,
}

pub fn get_from_json(json: String) -> Option<Body> {
    match serde_json::from_str(&json){
        Ok(t) => return Some(t),
        Err(_e) => {
            // eprintln!("ERROR : {e}");
            return None
        },
    };
}

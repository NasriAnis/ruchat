use sled::Config;
use serde::{Deserialize, Serialize};

use crate::json::json_from_slice;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct User {
    pub username: String,
    pub password: String,
}

pub fn init(path: &str) -> Result<sled::Db, sled::Error> {
    Config::new()
        .path(path)
        .open()
}

// todo: Not quite happy of this function may need ameliorations about error handling
pub fn register_user(db: &sled::Db, user: User) -> Result<(), sled::Error> {
    let key = format!("user:{}", user.username);
    let value = match serde_json::to_string(&user){
        Ok(s) => s,
        Err(e) => {
            eprintln!("JSON ERROR IN REGISTRATION API: {e}");
            return Ok(());
        }
    };

    db.insert(&key, value.as_bytes())
        .map(|_| ())
}

// todo: Not quite happy of this function may need ameliorations about error handling
pub fn check_login(db: &sled::Db, username: &str, password: &str) -> Result<bool, sled::Error> {
    let key = format!("user:{}", username);
    let stored_value = db.get(&key)?;
    let value = match stored_value{
        Some(v) => v,
        None => return Ok(false),
    };
    let user: User = Default::default();
    let stored_user = match json_from_slice(user, &value){
        Ok(t) => t,
        Err(e) =>{
            eprintln!("JSON ERROR IN CHECK LOGIN: {e}");
            return Ok(false)
        },
    };

    Ok(stored_user.password == password)
}

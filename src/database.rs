use sled::Config;
use serde::{Deserialize, Serialize};

use crate::json::json_from_slice;

#[derive(Clone)]
pub struct Databases {
    pub users: sled::Db,
    pub cookies: sled::Db,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct User {
    pub username: String,
    pub password: String,
}

pub fn init() -> Databases {
    Databases {
        users: match fire("db_users"){
            Ok(t) => t,
            Err(e) => {
                eprintln!("DATABASE: Failed to open database: {e}");
                panic!();
            }
        },
        cookies: match fire("db_cookies"){
            Ok(t) => t,
            Err(e) => {
                eprintln!("DATABASE: Failed to open database: {e}");
                panic!();
            }
        },
    }
}

fn fire(path: &str) -> Result<sled::Db, sled::Error> {
    Config::new()
        .path(path)
        .open()
}

// todo: Not quite happy of this function may need ameliorations about error handling
pub fn register_user(db: &sled::Db, user: &User) -> Result<(), sled::Error> {
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

pub fn save_cookie(db: &sled::Db, username: &str, cookie: &str) -> Result<(), sled::Error> {
    if let Some(old_cookie) = get_cookie_from_username(db, username)? {
        db.remove(format!("cookie:{}", old_cookie))?;
    }

    let cookie_key = format!("cookie:{}", cookie);
    let user_key = format!("user_cookie:{}", username);

    db.insert(&cookie_key, username.as_bytes())?;
    db.insert(&user_key, cookie.as_bytes())?;

    Ok(())
}

pub fn get_username_from_cookie(db: &sled::Db, cookie: &str) -> Result<Option<String>, sled::Error> {
    let key = format!("cookie:{}", cookie);
    let stored_value = db.get(&key)?;
    Ok(stored_value.map(|ivec| String::from_utf8_lossy(&ivec).to_string()))
}

pub fn get_cookie_from_username(db: &sled::Db, username: &str) -> Result<Option<String>, sled::Error> {
    let key = format!("user_cookie:{}", username);
    let stored_value = db.get(&key)?;
    Ok(stored_value.map(|ivec| String::from_utf8_lossy(&ivec).to_string()))
}

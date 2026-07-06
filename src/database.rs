use sled::Config;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct User {
    username: String,
    password: String,
}

pub fn init(path: &str) -> Result<sled::Db, sled::Error> {
    Config::new()
        .path(path)
        .open()
}

pub fn register_user(db: &sled::Db, user: User) -> Result<(), sled::Error> {
    // debugging
    println!("DATABASE: will save: {user:?}");
    let key = format!("user:{}", user.username);
    let value = serde_json::to_string(&user).unwrap_or_default(); // todo: need handling

    db.insert(&key, value.as_bytes())
        .map(|_| ())
}

// pub fn check_login(username: &str, password: &str) -> Result<bool, sled::Error> {
//     let db = open_db("./DB");
//     let key = format!("user:{}", username);
//     let stored_value = db.get(&key)?;
//     if let Some(value) = stored_value {
//         let stored_user: User = serde_json::from_slice(&value).unwrap_or_default(); // todo: need handling
//         Ok(stored_user.password == password)
//     } else {
//         Ok(false)
//     }
// }

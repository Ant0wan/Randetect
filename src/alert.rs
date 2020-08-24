//use std::fs::File;
//use std::io;
use crate::query::Log;
use crate::query::QType;
use std::collections::HashMap;

#[derive(Debug)]
pub enum ActivityType {
    Suspicious(i32),     // Containing nb of files manipulated.
    Misbehaving(String), // Contaning name of directory been moved.
    Normal,              // Normal user activity.
}

#[derive(Debug)]
pub struct User {
    pub username: String,
    ip: Vec<String>,
    kind: ActivityType,
  //  kind: Vec<ActivityType>,
}

/* Maximum suspicious action limit */
pub const BAN_LIMIT: u16 = 50;

pub fn log_user(entry: Vec<Log>, mut users: HashMap<String, User>, query: QType) -> HashMap<String, User> {
    for relation in entry {
        users.insert(
            relation.get_username(),
            User {
                username: relation.get_username(),
                ip: {
                    let ip = users.get(&relation.get_username());
                    println!("ip {:?}", ip);
                 //   match ip {
                 //       None => { let mut ip = Vec::new(); ip.push(relation.get_ip()); ip},
                 //       Some(ip) => { ip.push(relation.get_ip()); ip },
                  //  }
                        Vec::new()
                },
                kind: { match query {
                    QType::Delete => ActivityType::Normal,
                    _ => ActivityType::Normal,
                } 
                   // if !map.get(&relation.kind) let mut k = Vec::new();
                   // v.push()
                 },
            },
        );
    }
    users
}

pub mod sms {
    pub fn send() {}
}

pub mod email {

    pub fn send() -> Result<std::fs::File, std::io::Error> {
        let f = std::fs::File::open("email.txt")?;
        Ok(f)
    }
}

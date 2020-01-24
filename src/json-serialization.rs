use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: i32,
    first_name: String,
    last_name: String,
    active: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let user = User {
        id: 2,
        first_name: String::from("Vagmi"),
        last_name: String::from("Mudumbai"),
        active: true,
    };
    let json = serde_json::to_string_pretty(&user)?;
    let de_user: User = serde_json::from_str(&json)?;
    println!("{}", json);
    println!("{:?}", de_user);
    return Ok(());
}

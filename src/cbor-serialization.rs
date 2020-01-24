use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: i32,
    first_name: String,
    last_name: String,
    active: bool,
}

fn write_cbor_file(user: &User) -> Result<(), Box<dyn Error>> {
    let file = File::create("temp-user.cbor")?;
    serde_cbor::to_writer(file, user)?;
    Ok(())
}

fn read_cbor_file() -> Result<User, Box<dyn Error>> {
    let file = File::open("temp-user.cbor")?;
    let user: User = serde_cbor::from_reader(file)?;
    Ok(user)
}

fn write_cbor_bytes(user: &User) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut bytes: Vec<u8> = Vec::new();
    serde_cbor::to_writer(&mut bytes, user)?;
    Ok(bytes)
}

fn read_cbor_bytes(bytes: &[u8]) -> Result<User, Box<dyn Error>> {
    let user: User = serde_cbor::from_reader(bytes)?;
    Ok(user)
}

fn main() -> Result<(), Box<dyn Error>> {
    let user = User {
        id: 2,
        first_name: String::from("Vagmi"),
        last_name: String::from("Mudumbai"),
        active: true,
    };
    println!("Reading and writing CBOR from files");
    write_cbor_file(&user)?;
    let read_user = read_cbor_file()?;
    println!("Read user {:?}", read_user);

    println!("Write and reading CBOR from bytes");
    let bytes = write_cbor_bytes(&user)?;
    // vec<u8> implements the Write trait.
    // The cool part about it is that vec<u8>
    // derefs to &[u8] which implements to Read trait
    let bytes_user = read_cbor_bytes(&bytes)?;

    println!("Read user from {} bytes - {:?}", &bytes.len(), bytes_user);
    Ok(())
}

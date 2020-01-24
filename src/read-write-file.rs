use std::fs::File;
use std::io::{BufRead, BufReader, Error, Write};

fn read_file() -> Result<(), Error> {
    let path = "src/main.rs";
    let input = File::open(path)?;
    let reader = BufReader::new(input);
    for line in reader.lines() {
        println!("{}", line?);
    }
    Ok(())
}

fn write_file() -> Result<(), Error> {
    let path = "temp-file.txt";
    let mut file = File::create(path)?;
    write!(file, "The file\n has been\n written to.")?;
    Ok(())
}

fn main() -> Result<(), Error> {
    read_file()?;
    write_file()?;
    Ok(())
}

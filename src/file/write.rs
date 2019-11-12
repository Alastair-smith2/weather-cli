use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use crate::result::Result;

pub fn write_file<T: std::fmt::Display>(to_write: T) -> Result<()> {
    let mut file = File::create("weather.txt")?;
    file.write_all(to_write.to_string().as_bytes())?;
    Ok(())
}

pub fn read_file() -> Result<()> {
    let file = File::open("weather.txt")?;
    let metadata = &file.metadata()?;
    println!("The content {:#?}", metadata);
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    println!("The content {:?}", contents);
    Ok(())
}
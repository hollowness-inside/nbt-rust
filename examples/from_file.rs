use nbt_rust::{de::from_bytes, error::Result};

fn main() -> Result<()> {
    // Well I'm a bit cheating here
    // I ungzipped the file beforehand
    let file = include_bytes!("file.nbt");

    let (name, value) = from_bytes(file)?;
    print!("Name: {name}");
    println!("Value: {value}");
    Ok(())
}

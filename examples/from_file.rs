use nbt_rust::{de::from_bytes, error::Result};

fn main() -> Result<()> {
    let file = include_bytes!("file.nbt");
    let (name, value) = from_bytes(file)?;
    print!("Name: {name}");
    println!("Value: {value}");
    Ok(())
}

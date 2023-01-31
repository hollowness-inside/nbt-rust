use nbt_rust::{de::from_bytes, error::Result};

fn main() -> Result<()> {
    let file = include_bytes!("file.nbt");
    let res = from_bytes(file)?;
    println!("{res:?}");
    Ok(())
}

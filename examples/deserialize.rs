use std::io::Cursor;

use nbt_rust::{de::from_bytes, error::Result, ser::Serializer};

fn main() -> Result<()> {
    // Some binary NBT data to deserialize
    let data = data()?;

    // Deserializes the data
    let deserialized = from_bytes(&data)?;

    // Prints the parsed NBT data
    println!("{deserialized:#?}");

    Ok(())
}

// Copied from examples/serialize.rs
// Generates some test NBT data, NBT compound tag with 3 fields.
fn data() -> Result<Vec<u8>> {
    let writer = Cursor::new(Vec::new());

    // Creates a new serializer with the given writer.
    // Serializer is capable of writing NBT Tags to the writer.
    let ser = Serializer::new(writer);

    // Starts a new compound tag.
    let mut comp = ser.start_compound("my_compound")?;

    // Writes fields to the compound tag.
    comp.write_field("item_1", 123u8)?;
    comp.write_field("item_2", "Hello World".to_string())?;
    comp.write_field("item_3", vec![1i32, 2, 3, 4, 5])?;

    // Ends the compound tag and returns the serializer.
    let ser = comp.end()?;

    // Prints the value of the owned writer.
    // into_inner is called twice because the Vec is wrapped in a Cursor.
    Ok(ser.into_inner().into_inner())
}

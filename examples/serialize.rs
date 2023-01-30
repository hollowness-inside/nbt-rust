use std::io::Cursor;

use rnbt::{error::Result, Serializer};

fn main() -> Result<()> {
    let writer = Cursor::new(Vec::new());

    // Creates a new serializer with the given writer.
    // Serializer is capable of writing NBT Tags to the writer.
    let ser = Serializer::new(writer);

    // Starts a new compound tag.
    let mut comp = ser.start_compound();

    // Writes fields to the compound tag.
    comp.write_field("item_1", 123u8)?;
    comp.write_field("item_2", "Hello World".to_string())?;
    comp.write_field("item_3", vec![1i32, 2, 3, 4, 5])?;

    // Ends the compound tag and returns the serializer.
    let ser = comp.end()?;

    // Prints the value of the owned writer.
    // into_inner is called twice because the Vec is wrapped in a Cursor.
    println!("{:?}", ser.into_inner().into_inner());
    Ok(())
}

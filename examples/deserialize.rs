use std::io::Cursor;

use rnbt::{de::from_bytes, Serializer};

fn main() {
    // Some binary NBT data to deserialize
    let data = data();

    // Deserializes the data
    let deserialized = from_bytes(&data).unwrap();

    // Prints the parsed NBT data
    println!("{deserialized:#?}");
}

// Copied from examples/serialize.rs
// Generates some test NBT data, NBT compound tag with 3 fields.
fn data() -> Vec<u8> {
    let writer = Cursor::new(Vec::new());

    // Creates a new serializer with the given writer.
    // Serializer is capable of writing NBT Tags to the writer.
    let ser = Serializer::new(writer);

    // Starts a new compound tag.
    let mut comp = ser.start_compound();

    // Writes fields to the compound tag.
    comp.write_field("item_1", 123u8).unwrap();
    comp.write_field("item_2", "Hello World".to_string())
        .unwrap();
    comp.write_field("item_3", vec![1i32, 2, 3, 4, 5]).unwrap();

    // Ends the compound tag and returns the serializer.
    let ser = comp.end().unwrap();

    // Prints the value of the owned writer.
    // into_inner is called twice because the Vec is wrapped in a Cursor.
    ser.into_inner().into_inner()
}

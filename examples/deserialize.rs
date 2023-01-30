use std::io::Cursor;

use rnbt::Serializer;

fn main() {
    let writer = Cursor::new(Vec::new());
    let ser = Serializer::new(writer);

    let mut comp = ser.start_compound();
    comp.write_field("item_1", 123u8).unwrap();
    comp.write_field("item_2", "Hello World".to_string()).unwrap();
    comp.write_field("item_3", vec![1i32, 2, 3, 4, 5]).unwrap();
    let ser = comp.end().unwrap();

    let compound = ser.into_inner().into_inner();
    println!("{compound:?}");
}

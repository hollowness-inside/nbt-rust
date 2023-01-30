use std::io::Cursor;

use rnbt::Serializer;

fn main() {
    let mut ser = Serializer::new(Cursor::new(Vec::new()));
    ser.serialize_byte_array(&[1,2,3,4,5]).unwrap();
    println!("{:?}", ser.into_inner().into_inner());
}

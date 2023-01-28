use std::io::Cursor;

use rnbt::Serializer;

fn main() {
    let v = vec![0;128];
    let mut c = Cursor::new(v);

    let mut ser = Serializer::new(&mut c);
    ser.serialize_int_array(&vec![1,2,3,4,5]).unwrap();

    c.set_position(0);

    let de = rnbt::de::from_reader(&mut c).unwrap();
    println!("{de:?}")
}

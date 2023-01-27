use std::io;

use rnbt::Serializer;


fn main() {
    let mut writer = io::stdout();
    let mut ser = Serializer::new(&mut writer);
    // ser.serialize_string(" World").unwrap();
    // ser.serialize_float(5.23).unwrap();
    ser.serialize("Hello World").unwrap();
}

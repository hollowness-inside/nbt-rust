use std::io;

use rnbt::Serializer;

fn main() {
    let mut writer = io::stdout();
    let mut ser = Serializer::new(&mut writer);
    ser.serialize_f32(23.3);
}

use std::io;

use rnbt::Serializer;
use serde::ser::SerializeSeq;
use serde::Serializer as _;

fn main() {
    let mut writer = io::stdout();
    let mut ser = Serializer::new(&mut writer);

    let seq = ser.serialize_seq(Some(3)).unwrap();
    let seq = seq.set_type(ByteArray);
    seq.serialize_element(123);
    seq.end().unwrap();
}

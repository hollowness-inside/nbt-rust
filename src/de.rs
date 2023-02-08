use std::io;

pub struct Deserializer<R: io::Read> {
    reader: R,
}

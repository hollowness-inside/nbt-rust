use std::io;

pub struct Deserializer<R: io::Read> {
    reader: R,
}

impl<R: io::Read> Deserializer<R> {
    pub fn from_reader(reader: R) -> Deserializer<R> {
        Deserializer { reader }
    }
}
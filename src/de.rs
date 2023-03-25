use std::io::Read;

pub struct Deserializer<R>(R);

impl<R: Read> Deserializer<R> {
    pub fn new(reader: R) -> Self {
        Deserializer(reader)
    }

    pub fn into_inner(self) -> R {
        self.0
    }
}
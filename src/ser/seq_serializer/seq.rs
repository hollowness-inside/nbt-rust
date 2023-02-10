use std::io;

use crate::{error::Error, ser::Serializer};
use serde::ser::{SerializeSeq, SerializeTuple};

use super::ByteArraySerializer;

pub struct SeqSerializer<'a, W> {
    pub(crate) ser: &'a mut Serializer<W>,
    pub(crate) len: usize,
}

impl<'a, W: io::Write> SeqSerializer<'a, W> {
    pub fn as_byte_array(self) -> ByteArraySerializer<'a, W> {
        ByteArraySerializer {
            ser: self.ser,
            len: self.len,
        }
    }
}

impl<'a, W: io::Write> SerializeSeq for SeqSerializer<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, _: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        unimplemented!("Type of sequence must be specified")
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!("Type of sequence must be specified")
    }
}

impl<'a, W: io::Write> SerializeTuple for SeqSerializer<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, _: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        unimplemented!("Type of sequence must be specified")
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!("Type of sequence must be specified")
    }
}

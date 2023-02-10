use crate::error::Error;
use serde::ser::{SerializeSeq, SerializeTuple};

use super::ByteArraySerializer;

pub struct SeqSerializer {
    len: usize,
}

impl SeqSerializer {
    pub fn as_byte_array(self) -> ByteArraySerializer {
        ByteArraySerializer {}
    }
}

impl SerializeSeq for SeqSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        unimplemented!("Type of sequence must be specified")
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!("Type of sequence must be specified")
    }
}

impl SerializeTuple for SeqSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize {
        unimplemented!("Type of sequence must be specified")
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!("Type of sequence must be specified")
    }
}
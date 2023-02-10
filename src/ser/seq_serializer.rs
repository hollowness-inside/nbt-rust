use serde::ser::SerializeSeq;

use crate::error::Error;

pub struct SeqSerializer {
    len: usize,
}

impl SeqSerializer {
    pub fn as_byte_array(self) -> ByteArraySerializer {
        ByteArraySerializer(self)
    }
}

impl SerializeSeq for SeqSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

pub struct ByteArraySerializer(SeqSerializer);

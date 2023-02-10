use serde::ser::SerializeSeq;

use crate::error::Error;

pub struct ByteArraySerializer;

impl SerializeSeq for ByteArraySerializer {
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
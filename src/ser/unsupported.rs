use crate::error::{Error, Result};

pub struct Unsupported;

impl serde::ser::SerializeSeq for Unsupported {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        Err(Error::UnsupportedMethod)
    }

    fn end(self) -> Result<()> {
        Err(Error::UnsupportedMethod)
    }
}

impl serde::ser::SerializeTuple for Unsupported {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        Err(Error::UnsupportedMethod)
    }

    fn end(self) -> Result<()> {
        Err(Error::UnsupportedMethod)
    }
}

impl serde::ser::SerializeTupleStruct for Unsupported {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        Err(Error::UnsupportedMethod)
    }

    fn end(self) -> Result<()> {
        Err(Error::UnsupportedMethod)
    }
}

impl serde::ser::SerializeTupleVariant for Unsupported {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        Err(Error::UnsupportedMethod)
    }

    fn end(self) -> Result<()> {
        Err(Error::UnsupportedMethod)
    }
}

impl serde::ser::SerializeMap for Unsupported {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, _key: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        Err(Error::UnsupportedMethod)
    }

    fn serialize_value<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        Err(Error::UnsupportedMethod)
    }

    fn end(self) -> Result<()> {
        Err(Error::UnsupportedMethod)
    }
}

impl serde::ser::SerializeStruct for Unsupported {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, _value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        Err(Error::UnsupportedMethod)
    }

    fn end(self) -> Result<()> {
        Err(Error::UnsupportedMethod)
    }
}

impl serde::ser::SerializeStructVariant for Unsupported {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, _value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        Err(Error::UnsupportedMethod)
    }

    fn end(self) -> Result<()> {
        Err(Error::UnsupportedMethod)
    }
}

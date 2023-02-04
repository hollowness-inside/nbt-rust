use crate::error::{Error, Result};


pub struct Unsupported;

impl serde::ser::SerializeSeq for Unsupported {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: serde::Serialize {
        Err(Error::Unsupported)
    }

    fn end(self) -> Result<()> {
        Err(Error::Unsupported)
    }
}

impl serde::ser::SerializeTuple for Unsupported {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: serde::Serialize {
        Err(Error::Unsupported)
    }

    fn end(self) -> Result<()> {
        Err(Error::Unsupported)
    }
}

impl serde::ser::SerializeTupleStruct for Unsupported {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: serde::Serialize {
        Err(Error::Unsupported)
    }

    fn end(self) -> Result<()> {
        Err(Error::Unsupported)
    }
}

impl serde::ser::SerializeTupleVariant for Unsupported {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: serde::Serialize {
        Err(Error::Unsupported)
    }

    fn end(self) -> Result<()> {
        Err(Error::Unsupported)
    }
}

impl serde::ser::SerializeMap for Unsupported {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<()>
    where
        T: serde::Serialize {
        Err(Error::Unsupported)
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: serde::Serialize {
        Err(Error::Unsupported)
    }

    fn end(self) -> Result<()> {
        Err(Error::Unsupported)
    }
}

impl serde::ser::SerializeStruct for Unsupported {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: serde::Serialize {
        Err(Error::Unsupported)
    }

    fn end(self) -> Result<()> {
        Err(Error::Unsupported)
    }
}

impl serde::ser::SerializeStructVariant for Unsupported {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: serde::Serialize {
        Err(Error::Unsupported)
    }

    fn end(self) -> Result<()> {
        Err(Error::Unsupported)
    }
}
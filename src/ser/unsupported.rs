use crate::error::{Error, Result};

macro_rules! unsupported {
    ($name:ident) => {
        fn $name(self) -> Result<()> {
            Err(Error::UnsupportedMethod(format!("{}", stringify!($name))))
        }
    };

    ($name:ident, $($types:ty),*) => {
        fn $name(self, $(_: $types),*) -> Result<()> {
            Err(Error::UnsupportedMethod(format!("{}", stringify!($name))))
        }
    };

    ($name:ident -> $out:ident, $($types:ty),*) => {
        fn $name(self, $(_: $types),*) -> Result<Self::$out> {
            Err(Error::UnsupportedMethod(format!("{}", stringify!($name))))
        }
    };
}

pub(crate) use unsupported;

pub struct Unsupported;

impl serde::ser::SerializeSeq for Unsupported {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        Err(Error::UnsupportedMethod("Unsupported".to_string()))
    }

    fn end(self) -> Result<()> {
        Err(Error::UnsupportedMethod("Unsupported".to_string()))
    }
}

impl serde::ser::SerializeTuple for Unsupported {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        Err(Error::UnsupportedMethod("Unsupported".to_string()))
    }

    fn end(self) -> Result<()> {
        Err(Error::UnsupportedMethod("Unsupported".to_string()))
    }
}

impl serde::ser::SerializeTupleStruct for Unsupported {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        Err(Error::UnsupportedMethod("Unsupported".to_string()))
    }

    fn end(self) -> Result<()> {
        Err(Error::UnsupportedMethod("Unsupported".to_string()))
    }
}

impl serde::ser::SerializeTupleVariant for Unsupported {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        Err(Error::UnsupportedMethod("Unsupported".to_string()))
    }

    fn end(self) -> Result<()> {
        Err(Error::UnsupportedMethod("Unsupported".to_string()))
    }
}

impl serde::ser::SerializeMap for Unsupported {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, _key: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        Err(Error::UnsupportedMethod("Unsupported".to_string()))
    }

    fn serialize_value<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        Err(Error::UnsupportedMethod("Unsupported".to_string()))
    }

    fn end(self) -> Result<()> {
        Err(Error::UnsupportedMethod("Unsupported".to_string()))
    }
}

impl serde::ser::SerializeStruct for Unsupported {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, _value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        Err(Error::UnsupportedMethod("Unsupported".to_string()))
    }

    fn end(self) -> Result<()> {
        Err(Error::UnsupportedMethod("Unsupported".to_string()))
    }
}

impl serde::ser::SerializeStructVariant for Unsupported {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, _value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        Err(Error::UnsupportedMethod("Unsupported".to_string()))
    }

    fn end(self) -> Result<()> {
        Err(Error::UnsupportedMethod("Unsupported".to_string()))
    }
}

use crate::error::{Error, Result};

/// Macro to generate unsupported methods for the serde::Serializer trait.
macro_rules! unsupported {
    ($name:ident) => {
        fn $name(self) -> Result<()> {
            unimplemented!("{}", stringify!($name))
        }
    };

    ($name:ident, $($types:ty),*) => {
        fn $name(self, $(_: $types),*) -> Result<()> {
            unimplemented!("{}", stringify!($name))
        }
    };

    ($name:ident -> $out:ident, $($types:ty),*) => {
        fn $name(self, $(_: $types),*) -> Result<Self::$out> {
            unimplemented!("{}", stringify!($name))
        }
    };
}

pub(crate) use unsupported;

/// A struct that implements all the required type fields for the
/// `serde::Serializer` trait, but returns `Error::UnsupportedMethod`
/// for all methods.
pub struct Unsupported;

impl serde::ser::SerializeSeq for Unsupported {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, _: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl serde::ser::SerializeTuple for Unsupported {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, _: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl serde::ser::SerializeTupleStruct for Unsupported {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl serde::ser::SerializeTupleVariant for Unsupported {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl serde::ser::SerializeMap for Unsupported {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, _key: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        unimplemented!()
    }

    fn serialize_value<T: ?Sized>(&mut self, _: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl serde::ser::SerializeStruct for Unsupported {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, _: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl serde::ser::SerializeStructVariant for Unsupported {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, _: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

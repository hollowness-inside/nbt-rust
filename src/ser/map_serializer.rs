use std::io;

use serde::ser;

use crate::error::{Error, Result};

use super::{
    key_serializer::KeySerializer, serializer::Serializer, value_serializer::ValueSerializer,
};

/// A serializer for the map data structure
pub struct MapSerializer<'a, W> {
    /// The underlying serializer
    pub(super) ser: &'a mut Serializer<W>,

    /// The key of the current map entry
    pub(super) key: Option<Vec<u8>>,
}

impl<'a, W: io::Write> ser::SerializeMap for MapSerializer<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        let mut name = Vec::new();
        key.serialize(&mut KeySerializer { ser: &mut name })?;
        self.key = Some(name);

        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        let Some(key) = self.key.clone() else {
            return Err(Error::MissingKey);
        };

        value.serialize(&mut ValueSerializer {
            ser: self.ser,
            name: key,
        })
    }

    fn end(self) -> Result<()> {
        self.ser.0.write_all(&[0])?;
        Ok(())
    }
}

impl<'a, W: io::Write> ser::SerializeStruct for MapSerializer<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        <Self as ser::SerializeMap>::serialize_key(self, key)?;
        <Self as ser::SerializeMap>::serialize_value(self, value)?;
        Ok(())
    }

    fn end(self) -> Result<()> {
        <Self as ser::SerializeMap>::end(self)
    }
}

impl<'a, W: io::Write> ser::SerializeStructVariant for MapSerializer<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        <Self as ser::SerializeStruct>::serialize_field(self, key, value)?;
        Ok(())
    }

    fn end(self) -> Result<()> {
        <Self as ser::SerializeStruct>::end(self)
    }
}

use std::io;

use serde::ser;

use crate::error::{Error, Result};

use super::{
    name_serializer::NameSerializer, serializer::Serializer, value_serializer::ValueSerializer,
};

pub struct MapSerializer<'a, W> {
    pub(super) ser: &'a mut Serializer<W>,
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
        key.serialize(&mut NameSerializer { ser: &mut name })?;
        self.key = Some(name);

        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        let Some(key) = self.key else {
            return Err(Error::MissingKey);
        };

        value.serialize(&mut ValueSerializer {
            ser: self.ser,
            name: self.key,
        })
    }

    fn end(self) -> Result<()> {
        todo!()
    }
}

impl<'a, W: io::Write> ser::SerializeStruct for MapSerializer<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, _value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<()> {
        todo!()
    }
}

impl<'a, W: io::Write> ser::SerializeStructVariant for MapSerializer<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, _value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<()> {
        todo!()
    }
}

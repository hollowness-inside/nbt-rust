use std::io::Write;

use serde::Serialize;

use crate::{error::Error, nbt_tag::TagType};

use super::{key_ser::KeySerializer, seq_ser::SeqSerializer, unsupported::unsupported, Serializer};

pub struct MapSerializer<'a, W> {
    pub(super) ser: &'a mut Serializer<W>,
    pub(super) key: Option<Vec<u8>>,
}

impl<'a, W: Write> serde::ser::SerializeMap for MapSerializer<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> std::result::Result<(), Self::Error>
    where
        T: Serialize,
    {
        let mut out = Vec::new();
        key.serialize(&mut KeySerializer(&mut out))?;

        self.key = Some(out);
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> std::result::Result<(), Self::Error>
    where
        T: Serialize,
    {
        if self.key.is_none() {
            return Err(Error::MissingKey)
        }

        value.serialize(self)?;
        Ok(())
    }

    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        self.ser.0.write_all(&[0x00])?;
        Ok(())
    }
}

impl<'a, W: Write> serde::ser::SerializeStruct for MapSerializer<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> std::result::Result<(), Self::Error>
    where
        T: Serialize,
    {
        <Self as serde::ser::SerializeMap>::serialize_key(self, key)?;
        <Self as serde::ser::SerializeMap>::serialize_value(self, value)?;

        Ok(())
    }

    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        <Self as serde::ser::SerializeMap>::end(self)
    }
}

impl<'a, W: Write> serde::ser::SerializeStructVariant for MapSerializer<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> std::result::Result<(), Self::Error>
    where
        T: Serialize,
    {
        <Self as serde::ser::SerializeMap>::serialize_key(self, key)?;
        <Self as serde::ser::SerializeMap>::serialize_value(self, value)?;

        Ok(())
    }

    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        <Self as serde::ser::SerializeMap>::end(self)
    }
}

#[inline]
pub(crate) fn make_header(tag_type: TagType, name: &[u8]) -> Vec<u8> {
    let mut res = vec![tag_type as u8];
    res.extend((name.len() as u16).to_be_bytes());
    res.extend(name);
    res
}

impl<'a, 'b, W: Write> serde::Serializer for &'a mut MapSerializer<'b, W> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = serde::ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = serde::ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = serde::ser::Impossible<Self::Ok, Self::Error>;

    type SerializeTupleStruct = SeqSerializer<'a, W>;
    type SerializeMap = MapSerializer<'a, W>;
    type SerializeStruct = MapSerializer<'a, W>;
    type SerializeStructVariant = MapSerializer<'a, W>;

    fn serialize_bool(self, v: bool) -> std::result::Result<Self::Ok, Self::Error> {
        self.serialize_i8(if v { 1 } else { 0 })
    }

    fn serialize_i8(self, v: i8) -> std::result::Result<Self::Ok, Self::Error> {
        let mut data = make_header(TagType::Byte, self.key.as_ref().unwrap());
        data.push(v as u8);
        self.ser.0.write_all(&data)?;
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> std::result::Result<Self::Ok, Self::Error> {
        let mut data = make_header(TagType::Short, self.key.as_ref().unwrap());
        data.extend(v.to_be_bytes());
        self.ser.0.write_all(&data)?;
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> std::result::Result<Self::Ok, Self::Error> {
        let mut data = make_header(TagType::Int, self.key.as_ref().unwrap());
        data.extend(v.to_be_bytes());
        self.ser.0.write_all(&data)?;
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> std::result::Result<Self::Ok, Self::Error> {
        let mut data = make_header(TagType::Long, self.key.as_ref().unwrap());
        data.extend(v.to_be_bytes());
        self.ser.0.write_all(&data)?;
        Ok(())
    }

    #[inline]
    fn serialize_u8(self, v: u8) -> std::result::Result<Self::Ok, Self::Error> {
        self.serialize_i8(v as i8)
    }

    #[inline]
    fn serialize_u16(self, v: u16) -> std::result::Result<Self::Ok, Self::Error> {
        self.serialize_i16(v as i16)
    }

    #[inline]
    fn serialize_u32(self, v: u32) -> std::result::Result<Self::Ok, Self::Error> {
        self.serialize_i32(v as i32)
    }

    #[inline]
    fn serialize_u64(self, v: u64) -> std::result::Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_f32(self, v: f32) -> std::result::Result<Self::Ok, Self::Error> {
        let mut data = make_header(TagType::Float, self.key.as_ref().unwrap());
        data.extend(v.to_be_bytes());
        self.ser.0.write_all(&data)?;
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> std::result::Result<Self::Ok, Self::Error> {
        let mut data = make_header(TagType::Double, self.key.as_ref().unwrap());
        data.extend(v.to_be_bytes());
        self.ser.0.write_all(&data)?;
        Ok(())
    }

    #[inline]
    fn serialize_char(self, v: char) -> std::result::Result<Self::Ok, Self::Error> {
        self.serialize_i8(v as i8)
    }

    fn serialize_str(self, v: &str) -> std::result::Result<Self::Ok, Self::Error> {
        let mut data = make_header(TagType::String, self.key.as_ref().unwrap());
        data.extend(v.as_bytes());
        self.ser.0.write_all(&data)?;
        Ok(())
    }

    fn serialize_bytes(self, _v: &[u8]) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }

    #[inline]
    fn serialize_some<T: ?Sized>(self, value: &T) -> std::result::Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> std::result::Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> std::result::Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> std::result::Result<Self::SerializeTupleStruct, Self::Error> {
        let ttype = match name {
            "ByteArray" => TagType::ByteArray,
            "IntArray" => TagType::IntArray,
            "LongArray" => TagType::LongArray,
            "List" => TagType::List,
            _ => return Err(Error::WrongType),
        };
        let header = make_header(ttype, self.key.as_ref().unwrap());
        self.ser.0.write_all(&header)?;

        Ok(Self::SerializeTupleStruct::new(self.ser, len, ttype))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> std::result::Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }

    #[inline]
    fn serialize_map(
        self,
        len: Option<usize>,
    ) -> std::result::Result<Self::SerializeMap, Self::Error> {
        self.ser.serialize_map(len)
    }

    #[inline]
    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> std::result::Result<Self::SerializeStruct, Self::Error> {
        self.ser.serialize_struct(name, len)
    }

    #[inline]
    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> std::result::Result<Self::SerializeStructVariant, Self::Error> {
        self.ser
            .serialize_struct_variant(name, variant_index, variant, len)
    }

    unsupported!(serialize_none);
    unsupported!(serialize_unit);
    unsupported!(serialize_unit_struct, &'static str);
    unsupported!(serialize_unit_variant, &'static str, u32, &'static str);

    unsupported!(serialize_seq -> SerializeSeq, Option<usize>);
    unsupported!(serialize_tuple -> SerializeTuple, usize);
}

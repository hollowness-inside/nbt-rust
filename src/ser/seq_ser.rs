use std::io::Write;

use serde::Serialize;

use crate::{error::Error, nbt_tag::TagType};

use super::{unsupported::unsupported, Serializer};

pub struct SeqSerializer<'a, W> {
    pub(super) ser: &'a mut Serializer<W>,
    pub(super) typ: TagType,
    pub(super) list_type: Option<TagType>,
    pub(super) len: i32,
    pub(super) first: bool,
}

impl<'a, W> SeqSerializer<'a, W>
where
    W: Write,
{
    pub fn new(ser: &'a mut Serializer<W>, len: usize, typ: TagType) -> SeqSerializer<'a, W> {
        SeqSerializer {
            ser,
            typ,
            list_type: None,
            len: len as i32,
            first: true,
        }
    }
}

impl<'a, W: Write> serde::ser::SerializeTupleStruct for SeqSerializer<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        if self.first {
            match self.typ {
                TagType::ByteArray | TagType::IntArray | TagType::LongArray => {
                    self.ser.0.write_all(&self.len.to_be_bytes())?;
                    self.first = false;
                }

                TagType::List => {}

                _ => return Err(Error::WrongType),
            }
        }

        value.serialize(self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, 'b, W: Write> serde::Serializer for &'a mut SeqSerializer<'b, W> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = serde::ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = serde::ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = serde::ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = serde::ser::Impossible<Self::Ok, Self::Error>;
    type SerializeMap = serde::ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = serde::ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = serde::ser::Impossible<Self::Ok, Self::Error>;

    #[inline]
    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_u8(v as u8)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        if self.first && self.typ == TagType::List {
            self.list_type = Some(TagType::Int);

            let mut buf = vec![TagType::Int as u8];
            buf.extend(self.len.to_be_bytes());
            self.ser.0.write_all(&buf)?;
        }

        #[cfg(not(feature="no_check"))]
        if self.typ != TagType::IntArray && self.list_type != Some(TagType::Int) {
            return Err(Error::WrongType);
        }

        self.ser.0.write_all(&v.to_be_bytes())?;

        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        if self.first && self.typ == TagType::List {
            self.list_type = Some(TagType::Long);

            let mut buf = vec![TagType::Long as u8];
            buf.extend(self.len.to_be_bytes());
            self.ser.0.write_all(&buf)?;
        }

        #[cfg(not(feature="no_check"))]
        if self.typ != TagType::LongArray && self.list_type != Some(TagType::Long) {
            return Err(Error::WrongType);
        }

        self.ser.0.write_all(&v.to_be_bytes())?;

        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        if self.first && self.typ == TagType::List {
            self.list_type = Some(TagType::Byte);

            let mut buf = vec![TagType::Byte as u8];
            buf.extend(self.len.to_be_bytes());
            self.ser.0.write_all(&buf)?;
        }

        #[cfg(not(feature="no_check"))]
        if self.typ != TagType::ByteArray && self.list_type != Some(TagType::Byte) {
            return Err(Error::WrongType);
        }

        self.ser.0.write_all(&[v])?;

        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_i32(v as i32)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    unsupported!(serialize_bool, bool);
    unsupported!(serialize_i16, i16);
    unsupported!(serialize_u16, u16);
    unsupported!(serialize_f32, f32);
    unsupported!(serialize_f64, f64);
    unsupported!(serialize_char, char);
    unsupported!(serialize_str, &str);
    unsupported!(serialize_bytes, &[u8]);
    unsupported!(serialize_none);
    unsupported!(serialize_unit);
    unsupported!(serialize_unit_struct, &'static str);
    unsupported!(serialize_unit_variant, &'static str, u32, &'static str);

    unsupported!(serialize_seq -> SerializeSeq, Option<usize>);
    unsupported!(serialize_tuple -> SerializeTuple, usize);
    unsupported!(serialize_tuple_struct -> SerializeTupleStruct, &'static str, usize);
    unsupported!(serialize_tuple_variant -> SerializeTupleVariant, &'static str, u32, &'static str, usize);
    unsupported!(serialize_map -> SerializeMap, Option<usize>);
    unsupported!(serialize_struct -> SerializeMap, &'static str, usize);
    unsupported!(serialize_struct_variant -> SerializeMap, &'static str, u32, &'static str, usize);

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> std::result::Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(Error::Unsupported("serialize_newtype_struct".to_string()))
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
        Err(Error::Unsupported("serialize_newtype_variant".to_string()))
    }
}

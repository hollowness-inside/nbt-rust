use std::io;

use crate::{
    error::{Error, Result},
    nbt_tag::TagType,
};

use super::{
    map_serializer::MapSerializer,
    serializer::Serializer,
    unsupported::{unsupported, Unsupported},
};

/// Creates a byte vector that represents the header of an NBT tag
/// with the given name and type and returns it.
#[inline]
fn make_header(tag_type: TagType, name: &[u8]) -> Vec<u8> {
    let mut res = vec![tag_type as u8];
    res.extend((name.len() as i16).to_be_bytes());
    res.extend(name);
    res
}

/// A serializer that writes a single NBT Tag i.e. its name and value.
pub struct ValueSerializer<'a, W> {
    /// The underlying writer.
    pub(crate) ser: &'a mut Serializer<W>,

    /// The name of the tag that is being serialized.
    pub(crate) name: Vec<u8>,
}

impl<'a, W: io::Write> serde::Serializer for &'a mut ValueSerializer<'a, W> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Unsupported;
    type SerializeTuple = Unsupported;
    type SerializeTupleStruct = Unsupported;
    type SerializeTupleVariant = Unsupported;

    type SerializeMap = MapSerializer<'a, W>;
    type SerializeStruct = MapSerializer<'a, W>;
    type SerializeStructVariant = MapSerializer<'a, W>;

    unsupported!(serialize_none);
    unsupported!(serialize_unit);
    unsupported!(serialize_unit_struct, &'static str);
    unsupported!(serialize_unit_variant, &'static str, u32, &'static str);
    unsupported!(serialize_seq -> SerializeSeq, Option<usize>);
    unsupported!(serialize_tuple -> SerializeTuple, usize);
    unsupported!(serialize_tuple_struct -> SerializeTupleStruct, &'static str, usize);
    unsupported!(serialize_tuple_variant -> SerializeTupleVariant, &'static str, u32, &'static str, usize);
    unsupported!(serialize_struct_variant -> SerializeStructVariant, &'static str, u32, &'static str, usize);

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<()>
    where
        T: serde::Serialize,
    {
        Err(Error::UnsupportedMethod(
            "ValueSerializer::serialize_newtype_variant".to_string(),
        ))
    }

    fn serialize_bool(self, v: bool) -> Result<()> {
        self.serialize_i8(if v { 1 } else { 0 })
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        let mut data = make_header(TagType::Byte, &self.name.clone());
        data.push(v as u8);
        self.ser.0.write_all(&data)?;
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        let mut data = make_header(TagType::Short, &self.name.clone());
        data.extend(v.to_be_bytes());
        self.ser.0.write_all(&data)?;
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        let mut data = make_header(TagType::Int, &self.name.clone());
        data.extend(v.to_be_bytes());
        self.ser.0.write_all(&data)?;
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        let mut data = make_header(TagType::Long, &self.name.clone());
        data.extend(v.to_be_bytes());
        self.ser.0.write_all(&data)?;
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.serialize_i8(v as i8)
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.serialize_i16(v as i16)
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.serialize_i32(v as i32)
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.serialize_i64(v as i64)
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        let mut data = make_header(TagType::Float, &self.name.clone());
        data.extend(v.to_be_bytes());
        self.ser.0.write_all(&data)?;
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        let mut data = make_header(TagType::Double, &self.name.clone());
        data.extend(v.to_be_bytes());
        self.ser.0.write_all(&data)?;
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<()> {
        self.serialize_i8(v as i8)
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        let mut data = make_header(TagType::String, &self.name.clone());
        data.extend((v.len() as u16).to_be_bytes());
        data.extend(v.as_bytes());
        self.ser.0.write_all(&data)?;
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        let mut data = make_header(TagType::ByteArray, &self.name.clone());
        data.extend((v.len() as i32).to_be_bytes());
        data.extend(v);
        self.ser.0.write_all(&data)?;
        Ok(())
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        let header = make_header(TagType::Compound, &self.name.clone());
        self.ser.0.write_all(&header)?;

        Ok(MapSerializer {
            ser: self.ser,
            key: None,
        })
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        self.serialize_map(None)
    }
}

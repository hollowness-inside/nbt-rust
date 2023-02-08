use std::io;

use serde::de;

use crate::{
    error::{Error, Result},
    nbt_tag::TagType,
    NbtTag,
};

pub struct Deserializer<R: io::Read> {
    reader: R,
}

impl<R: io::Read> Deserializer<R> {
    pub fn from_reader(reader: R) -> Deserializer<R> {
        Deserializer { reader }
    }

    fn get_next_value(&mut self) -> Result<(String, NbtTag)> {
        let tt: TagType = self.read_u8()?.try_into()?;
        if tt == TagType::End {
            return Ok(("".to_string(), NbtTag::End));
        }

        let mut name = vec![0; self.read_i16()? as usize];
        self.reader.read_exact(&mut name)?;
        let name = String::from_utf8(name)?;

        let tag: NbtTag = match tt {
            TagType::Byte => self.read_u8()?.into(),
            TagType::Short => self.read_i16()?.into(),
            TagType::Int => self.read_i32()?.into(),
            TagType::Long => self.read_i64()?.into(),
            TagType::Float => self.read_f32()?.into(),
            TagType::Double => self.read_f64()?.into(),
            TagType::ByteArray => self.read_byte_array()?,
            TagType::String => self.read_string()?,
            TagType::List => self.read_list()?,
            TagType::Compound => self.read_compound()?,
            TagType::IntArray => self.read_int_array()?,
            TagType::LongArray => self.read_long_array()?,
            TagType::End => unreachable!(),
        };

        Ok((name, tag))
    }

    fn read_u8(&mut self) -> Result<u8> {
        let mut buf = [0; 1];
        self.reader.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    fn read_i16(&mut self) -> Result<i16> {
        let mut buf = [0; 2];
        self.reader.read_exact(&mut buf)?;
        Ok(i16::from_be_bytes(buf))
    }

    fn read_i32(&mut self) -> Result<i32> {
        let mut buf = [0; 4];
        self.reader.read_exact(&mut buf)?;
        Ok(i32::from_be_bytes(buf))
    }

    fn read_i64(&mut self) -> Result<i64> {
        let mut buf = [0; 8];
        self.reader.read_exact(&mut buf)?;
        Ok(i64::from_be_bytes(buf))
    }

    fn read_f32(&mut self) -> Result<f32> {
        let mut buf = [0; 4];
        self.reader.read_exact(&mut buf)?;
        Ok(f32::from_be_bytes(buf))
    }

    fn read_f64(&mut self) -> Result<f64> {
        let mut buf = [0; 8];
        self.reader.read_exact(&mut buf)?;
        Ok(f64::from_be_bytes(buf))
    }

    fn read_byte_array(&mut self) -> Result<NbtTag> {
        let len = self.read_i32()?;
        let mut buf = vec![0; len as usize];
        self.reader.read_exact(&mut buf)?;
        Ok(NbtTag::ByteArray(buf))
    }

    fn read_string(&mut self) -> Result<NbtTag> {
        let len = self.read_i16()?;
        let mut buf = vec![0; len as usize];
        self.reader.read_exact(&mut buf)?;
        Ok(NbtTag::String(String::from_utf8(buf).unwrap()))
    }

    fn read_list(&mut self) -> Result<NbtTag> {
        let tt: TagType = self.read_u8()?.try_into()?;
        let len = self.read_i32()?;
        let mut list = Vec::with_capacity(len as usize);
        for _ in 0..len {
            list.push(self.get_next_value()?.1);
        }
        Ok(NbtTag::List(list))
    }

    fn read_compound(&mut self) -> Result<NbtTag> {
        let mut compound = Vec::new();
        loop {
            let (name, tag) = self.get_next_value()?;
            if tag.tag_type() == TagType::End {
                break;
            }
            compound.push((name, tag));
        }
        Ok(NbtTag::Compound(compound.into_iter().collect()))
    }

    fn read_int_array(&mut self) -> Result<NbtTag> {
        let len = self.read_i32()? as usize;
        let mut buf = vec![0; len];
        for i in 0..len {
            buf[i] = self.read_i32()?;
        }
        Ok(NbtTag::IntArray(buf))
    }

    fn read_long_array(&mut self) -> Result<NbtTag> {
        let len = self.read_i32()? as usize;
        let mut buf = vec![0; len];
        for i in 0..len {
            buf[i] = self.read_i64()?;
        }
        Ok(NbtTag::LongArray(buf))
    }
}

impl<'de, 'a, R: io::Read> de::Deserializer<'de> for &'a mut Deserializer<R> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }
}

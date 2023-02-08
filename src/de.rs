use std::io;

use crate::{error::Result, nbt_tag::TagType, NbtTag};

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

        let tag: TagType = match tt {
        let tag = match tt {
            TagType::End => NbtTag::End,
            TagType::Byte => self.read_u8()?,
            TagType::Short => self.read_i16()?,
            TagType::Int => self.read_i32()?,
            TagType::Long => self.read_i64()?,
            TagType::Float => self.read_f32()?,
            TagType::Double => self.read_f64()?,
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
}

use std::io;

use crate::{
    error::{Error, Result},
    nbt_tag::prefixes,
    NbtTag,
};

pub struct Serializer<W>(W);

impl<W: io::Write> Serializer<W> {
    pub fn serialize<T: Into<NbtTag>>(&mut self, v: T) -> Result<()> {
        self.serialize_tag(&v.into())
    }

    pub fn serialize_tag(&mut self, v: &NbtTag) -> Result<()> {
        match v {
            NbtTag::End => self.serialize_end(),
            NbtTag::Byte(v) => self.serialize_u8(*v),
            NbtTag::Short(v) => self.serialize_i16(*v),
            NbtTag::Int(v) => self.serialize_i32(*v),
            NbtTag::Long(v) => self.serialize_i64(*v),
            NbtTag::Float(v) => self.serialize_f32(*v),
            NbtTag::Double(v) => self.serialize_f64(*v),
            NbtTag::ByteArray(v) => self.serialize_byte_array(v),
            NbtTag::String(v) => self.serialize_str(v),
            NbtTag::List(v) => self.serialize_list(v),
            NbtTag::Compound(v) => self.serialize_compound(v),
            NbtTag::IntArray(v) => self.serialize_int_array(v),
            NbtTag::LongArray(v) => self.serialize_long_array(v),
        }
    }

    pub fn serialize_end(&mut self) -> Result<()> {
        self.0.write_all(&[prefixes::END]);
        Ok(())
    }

    pub fn serialize_bool(&mut self, v: bool) -> Result<()> {
        self.serialize_u8(if v { 1 } else { 0 });
        Ok(())
    }

    pub fn serialize_u8(&mut self, v: u8) -> Result<()> {
        self.0.write_all(&[prefixes::BYTE, v]);
        Ok(())
    }

    pub fn serialize_i8(&mut self, v: i8) -> Result<()> {
        self.serialize_u8(v as u8);
        Ok(())
    }

    pub fn serialize_u16(&mut self, v: u16) -> Result<()> {
        let mut res = vec![prefixes::SHORT];
        res.extend(v.to_be_bytes());
        self.0.write_all(&res);
        Ok(())
    }

    pub fn serialize_i16(&mut self, v: i16) -> Result<()> {
        self.serialize_u16(v as u16)
    }

    pub fn serialize_u32(&mut self, v: u32) -> Result<()> {
        let mut res = vec![prefixes::INT];
        res.extend(v.to_be_bytes());
        self.0.write_all(&res);

        Ok(())
    }

    pub fn serialize_i32(&mut self, v: i32) -> Result<()> {
        self.serialize_u32(v as u32)
    }

    pub fn serialize_u64(&mut self, v: u64) -> Result<()> {
        let mut res = vec![prefixes::LONG];
        res.extend(v.to_be_bytes());
        self.0.write_all(&res);

        Ok(())
    }

    pub fn serialize_i64(&mut self, v: i64) -> Result<()> {
        self.serialize_u64(v as u64)
    }

    pub fn serialize_f32(&mut self, v: f32) -> Result<()> {
        let mut res = vec![prefixes::FLOAT];
        res.extend(v.to_be_bytes());
        self.0.write_all(&res);

        Ok(())
    }

    pub fn serialize_f64(&mut self, v: f64) -> Result<()> {
        let mut res = vec![prefixes::DOUBLE];
        res.extend(v.to_be_bytes());
        self.0.write_all(&res);

        Ok(())
    }

    pub fn serialize_char(&mut self, v: char) -> Result<()> {
        self.serialize_u8(v as u8)
    }

    pub fn serialize_str(&mut self, v: &str) -> Result<()> {
        let mut res = vec![prefixes::STRING];
        res.extend((v.len() as i16).to_be_bytes());
        res.extend(v.as_bytes());
        self.0.write_all(&res);

        Ok(())
    }

    pub fn serialize_compound<S: ToString>(&mut self, v: &[(S, NbtTag)]) -> Result<()> {
        self.0.write_all(&[prefixes::COMPOUND])?;

        for (name, tag) in v {
            self.serialize_str(&name.to_string())?;
            self.serialize_tag(tag)?;
        }
        self.0.write_all(&[prefixes::END])?;

        Ok(())
    }

    pub fn serialize_byte_array(&mut self, v: &[u8]) -> Result<()> {
        let mut res = vec![prefixes::BYTE_ARRAY];
        res.extend((v.len() as u16).to_be_bytes());
        res.extend(v);
        self.0.write_all(&res);

        Ok(())
    }

    pub fn serialize_list(&mut self, value: &[NbtTag]) -> Result<()> {
        let Some(tag_type) = value.first().map(|t| t.tag_type()) else {
            return Err(Error::Generic("Cannot serialize an empty list".to_string()));
        };

        if !value.iter().all(|x| x.tag_type() == tag_type) {
            return Err(Error::Generic(
                "All elements in a list must have the same type".to_string(),
            ));
        }

        let mut res = vec![prefixes::LIST];
        res.push(tag_type);
        res.extend((value.len() as i32).to_be_bytes());
        self.0.write_all(&res);

        match value.first() {
            Some(NbtTag::Byte(v)) => {
                for _i in value {
                    self.serialize_u8(*v)?;
                }
            }
            Some(NbtTag::Short(v)) => {
                for _i in value {
                    self.serialize_i16(*v)?;
                }
            }
            Some(NbtTag::Int(v)) => {
                for _i in value {
                    self.serialize_i32(*v)?;
                }
            }
            Some(NbtTag::Long(v)) => {
                for _i in value {
                    self.serialize_i64(*v)?;
                }
            }
            Some(NbtTag::Float(v)) => {
                for _i in value {
                    self.serialize_f32(*v)?;
                }
            }
            Some(NbtTag::Double(v)) => {
                for _i in value {
                    self.serialize_f64(*v)?;
                }
            }
            Some(NbtTag::ByteArray(v)) => {
                for _i in value {
                    self.serialize_byte_array(v)?;
                }
            }
            Some(NbtTag::String(v)) => {
                for _i in value {
                    self.serialize_str(v)?;
                }
            }
            Some(NbtTag::List(v)) => {
                for _i in value {
                    self.serialize_list(v)?;
                }
            }
            Some(NbtTag::Compound(v)) => {
                for _i in value {
                    self.serialize_compound(v)?;
                }
            }
            Some(NbtTag::IntArray(v)) => {
                for _i in value {
                    self.serialize_int_array(v)?;
                }
            }
            Some(NbtTag::LongArray(v)) => {
                for _i in value {
                    self.serialize_long_array(v)?;
                }
            }
            _ => {}
        }

        Ok(())
    }

    pub fn serialize_int_array(&mut self, v: &[i32]) -> Result<()> {
        let mut res = vec![prefixes::INT_ARRAY];
        res.extend((v.len() as i32).to_be_bytes());
        for i in v {
            res.extend(i.to_be_bytes());
        }
        self.0.write_all(&res);

        Ok(())
    }

    pub fn serialize_long_array(&mut self, v: &[i64]) -> Result<()> {
        let mut res = vec![prefixes::LONG_ARRAY];
        res.extend((v.len() as i32).to_be_bytes());
        for i in v {
            res.extend(i.to_be_bytes());
        }
        self.0.write_all(&res);

        Ok(())
    }

    pub fn start_compound(mut self) -> CompoundSerializer<W> {
        CompoundSerializer {
            ser: self,
            is_first: true,
        }
    }
}

pub struct CompoundSerializer<W> {
    ser: Serializer<W>,
    is_first: bool,
}

impl<W: io::Write> CompoundSerializer<W> {
    pub fn write_field<T: Into<NbtTag>>(&mut self, key: &str, value: T) -> Result<()> {
        if self.is_first {
            self.ser.0.write_all(&[prefixes::COMPOUND])?;
            self.is_first = false;
        }

        self.ser.serialize_str(key)?;
        self.ser.serialize(value)?;

        Ok(())
    }

    pub fn end(mut self) -> Result<()> {
        self.ser.serialize_end()?;
        Ok(())
    }
}

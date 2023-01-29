use std::io::Read;

use crate::{
    error::{Error, Result},
    nbt_tag::prefixes,
    NbtTag,
};

pub fn from_reader<R: Read>(reader: &mut R) -> Result<NbtTag> {
    let mut bytes = reader.bytes();
    if let Some(byte) = bytes.next() {
        match byte? {
            prefixes::BYTE => {
                let mut buf = [0; 1];
                reader.read_exact(&mut buf)?;
                Ok(NbtTag::Byte(buf[0]))
            }
            prefixes::SHORT => {
                let mut buf = [0; 2];
                reader.read_exact(&mut buf)?;
                Ok(NbtTag::Short(i16::from_be_bytes(buf)))
            }
            prefixes::INT => {
                let mut buf = [0; 4];
                reader.read_exact(&mut buf)?;
                Ok(NbtTag::Int(i32::from_be_bytes(buf)))
            }
            prefixes::LONG => {
                let mut buf = [0; 8];
                reader.read_exact(&mut buf)?;
                Ok(NbtTag::Long(i64::from_be_bytes(buf)))
            }
            prefixes::FLOAT => {
                let mut buf = [0; 4];
                reader.read_exact(&mut buf)?;
                Ok(NbtTag::Float(f32::from_be_bytes(buf)))
            }
            prefixes::DOUBLE => {
                let mut buf = [0; 8];
                reader.read_exact(&mut buf)?;
                Ok(NbtTag::Double(f64::from_be_bytes(buf)))
            }
            prefixes::BYTE_ARRAY => {
                let mut buf = [0; 4];
                reader.read_exact(&mut buf)?;
                let len = i32::from_be_bytes(buf) as usize;
                let mut bytes = vec![0; len];
                reader.read_exact(&mut bytes)?;
                Ok(NbtTag::ByteArray(bytes))
            }
            prefixes::STRING => {
                let mut buf = [0; 2];
                reader.read_exact(&mut buf)?;
                let len = i16::from_be_bytes(buf) as usize;

                let mut bytes = vec![0; len];
                reader.read_exact(&mut bytes)?;
                Ok(NbtTag::String(String::from_utf8(bytes)?))
            }
            prefixes::LIST => {
                let mut buf = [0; 1];
                reader.read_exact(&mut buf)?;

                let mut buf = [0; 4];
                reader.read_exact(&mut buf)?;
                let len = i32::from_be_bytes(buf) as usize;

                let mut tags = Vec::with_capacity(len);
                for _ in 0..len {
                    tags.push(from_reader(reader)?);
                }
                Ok(NbtTag::List(tags))
            }
            prefixes::COMPOUND => {
                let mut tags = Vec::new();

                loop {
                    // println!("{:?}", from_reader(reader)?);
                    let key = match from_reader(reader)? {
                        NbtTag::String(name) => name,
                        NbtTag::End => break,
                        _ => return Err(Error::Generic("Expected string".to_string())),
                    };

                    let value = from_reader(reader)?;
                    tags.push((key, value));
                }
                Ok(NbtTag::Compound(tags))
            }
            prefixes::INT_ARRAY => {
                let mut buf = [0; 4];
                reader.read_exact(&mut buf)?;
                let len = i32::from_be_bytes(buf) as usize;

                let mut bytes = vec![0; len * 4];
                reader.read_exact(&mut bytes)?;
                let mut ints = Vec::with_capacity(len);

                for i in 0..len {
                    ints.push(i32::from_be_bytes([
                        bytes[i * 4],
                        bytes[i * 4 + 1],
                        bytes[i * 4 + 2],
                        bytes[i * 4 + 3],
                    ]));
                }
                Ok(NbtTag::IntArray(ints))
            }
            prefixes::LONG_ARRAY => {
                let mut buf = [0; 4];
                reader.read_exact(&mut buf)?;
                let len = i32::from_be_bytes(buf) as usize;

                let mut bytes = vec![0; len * 8];
                reader.read_exact(&mut bytes)?;
                let mut longs = Vec::with_capacity(len);

                for i in 0..len {
                    longs.push(i64::from_be_bytes([
                        bytes[i * 8],
                        bytes[i * 8 + 1],
                        bytes[i * 8 + 2],
                        bytes[i * 8 + 3],
                        bytes[i * 8 + 4],
                        bytes[i * 8 + 5],
                        bytes[i * 8 + 6],
                        bytes[i * 8 + 7],
                    ]));
                }
                Ok(NbtTag::LongArray(longs))
            }
            prefixes::END => Ok(NbtTag::End),
            _ => Err(Error::Generic("Unknown tag type".to_string())),
        }
    } else {
        Err(crate::error::Error::Generic("Empty stream".to_string()))
    }
}

pub fn from_bytes(bytes: &[u8]) -> Result<NbtTag> {
    let mut reader = std::io::Cursor::new(bytes);
    from_reader(&mut reader)
}

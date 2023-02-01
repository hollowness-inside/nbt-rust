use std::{
    collections::HashMap,
    io::{Cursor, Read},
};

use crate::{error::Result, nbt_tag::TagType, NbtTag};

/// Reads a single NBT tag from a reader
pub fn from_reader<R: Read>(reader: &mut R) -> Result<(String, NbtTag)> {
    let (prefix, name) = read_tag_header(reader)?;
    let value = match prefix {
        TagType::Byte => read_headless_byte(reader)?,
        TagType::Short => read_headless_short(reader)?,
        TagType::Int => read_headless_int(reader)?,
        TagType::Long => read_headless_long(reader)?,
        TagType::Float => read_headless_float(reader)?,
        TagType::Double => read_headless_double(reader)?,
        TagType::ByteArray => read_headless_byte_array(reader)?,
        TagType::String => read_headless_string(reader)?,
        TagType::List => read_headless_list(reader)?,
        TagType::Compound => read_headless_compound(reader)?,
        TagType::IntArray => read_headless_int_array(reader)?,
        TagType::LongArray => read_headless_long_array(reader)?,
        TagType::End => NbtTag::End,
    };

    Ok((name, value))
}

/// Reads a single NBT tag from a byte slice
pub fn from_bytes(bytes: &[u8]) -> Result<(String, NbtTag)> {
    let mut reader = Cursor::new(bytes);
    from_reader(&mut reader)
}

fn read_tag_header<R: Read>(reader: &mut R) -> Result<(TagType, String)> {
    let mut prefix = [0; 1];
    reader.read_exact(&mut prefix)?;
    let prefix: TagType = prefix[0].try_into()?;

    if prefix == TagType::End {
        return Ok((prefix, String::new()));
    }

    let mut name_len = [0; 2];
    reader.read_exact(&mut name_len)?;
    let name_len = u16::from_be_bytes(name_len);

    let mut name = vec![0; name_len as usize];
    reader.read_exact(&mut name)?;
    let name = String::from_utf8(name)?;

    Ok((prefix, name))
}

#[inline]
fn read_headless_byte<R: Read>(reader: &mut R) -> Result<NbtTag> {
    let mut byte = [0; 1];
    reader.read_exact(&mut byte)?;

    Ok(NbtTag::Byte(byte[0]))
}

#[inline]
fn read_headless_short<R: Read>(reader: &mut R) -> Result<NbtTag> {
    let mut short = [0; 2];
    reader.read_exact(&mut short)?;

    Ok(NbtTag::Short(i16::from_be_bytes(short)))
}

#[inline]
fn read_headless_int<R: Read>(reader: &mut R) -> Result<NbtTag> {
    let mut int = [0; 4];
    reader.read_exact(&mut int)?;

    Ok(NbtTag::Int(i32::from_be_bytes(int)))
}

#[inline]
fn read_headless_long<R: Read>(reader: &mut R) -> Result<NbtTag> {
    let mut long = [0; 8];
    reader.read_exact(&mut long)?;

    Ok(NbtTag::Long(i64::from_be_bytes(long)))
}

#[inline]
fn read_headless_float<R: Read>(reader: &mut R) -> Result<NbtTag> {
    let mut float = [0; 4];
    reader.read_exact(&mut float)?;

    Ok(NbtTag::Float(f32::from_be_bytes(float)))
}

#[inline]
fn read_headless_double<R: Read>(reader: &mut R) -> Result<NbtTag> {
    let mut double = [0; 8];
    reader.read_exact(&mut double)?;

    Ok(NbtTag::Double(f64::from_be_bytes(double)))
}

#[inline]
fn read_headless_byte_array<R: Read>(reader: &mut R) -> Result<NbtTag> {
    let mut len = [0; 4];
    reader.read_exact(&mut len)?;
    let len = i32::from_be_bytes(len);

    let mut bytes = vec![0; len as usize];
    reader.read_exact(&mut bytes)?;

    Ok(NbtTag::ByteArray(bytes))
}

fn read_headless_string<R: Read>(reader: &mut R) -> Result<NbtTag> {
    let mut len = [0; 2];
    reader.read_exact(&mut len)?;
    let len = u16::from_be_bytes(len);

    let mut bytes = vec![0; len as usize];
    reader.read_exact(&mut bytes)?;
    let string = String::from_utf8(bytes)?;

    Ok(NbtTag::String(string))
}

fn read_headless_list<R: Read>(reader: &mut R) -> Result<NbtTag> {
    let mut prefix = [0; 1];
    reader.read_exact(&mut prefix)?;
    let prefix: TagType = prefix[0].try_into()?;

    let mut len = [0; 4];
    reader.read_exact(&mut len)?;
    let len = i32::from_be_bytes(len);

    let mut tags = Vec::new();
    for _ in 0..len {
        tags.push(read_headless_tag(reader, prefix)?);
    }

    Ok(NbtTag::List(tags))
}

fn read_headless_compound<R: Read>(reader: &mut R) -> Result<NbtTag> {
    let mut tags: Vec<(String, NbtTag)> = Vec::new();

    loop {
        let (prefix, name) = read_tag_header(reader)?;
        if prefix == TagType::End {
            break;
        }

        let value = read_headless_tag(reader, prefix)?;

        tags.push((name, value));
    }

    let tags: HashMap<String, NbtTag> = tags.into_iter().collect();
    Ok(NbtTag::Compound(tags))
}

fn read_headless_int_array<R: Read>(reader: &mut R) -> Result<NbtTag> {
    let mut len = [0; 4];
    reader.read_exact(&mut len)?;
    let len = i32::from_be_bytes(len);

    let mut ints = Vec::new();
    for _ in 0..len {
        let mut int = [0; 4];
        reader.read_exact(&mut int)?;
        ints.push(i32::from_be_bytes(int));
    }

    Ok(NbtTag::IntArray(ints))
}

fn read_headless_long_array<R: Read>(reader: &mut R) -> Result<NbtTag> {
    let mut len = [0; 4];
    reader.read_exact(&mut len)?;
    let len = i32::from_be_bytes(len);

    let mut longs = Vec::new();
    for _ in 0..len {
        let mut long = [0; 8];
        reader.read_exact(&mut long)?;
        longs.push(i64::from_be_bytes(long));
    }

    Ok(NbtTag::LongArray(longs))
}

fn read_headless_tag<R: Read>(reader: &mut R, prefix: TagType) -> Result<NbtTag> {
    match prefix {
        TagType::Byte => read_headless_byte(reader),
        TagType::Short => read_headless_short(reader),
        TagType::Int => read_headless_int(reader),
        TagType::Long => read_headless_long(reader),
        TagType::Float => read_headless_float(reader),
        TagType::Double => read_headless_double(reader),
        TagType::ByteArray => read_headless_byte_array(reader),
        TagType::String => read_headless_string(reader),
        TagType::List => read_headless_list(reader),
        TagType::Compound => read_headless_compound(reader),
        TagType::IntArray => read_headless_int_array(reader),
        TagType::LongArray => read_headless_long_array(reader),
        TagType::End => Ok(NbtTag::End),
    }
}

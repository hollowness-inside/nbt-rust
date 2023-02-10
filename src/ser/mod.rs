mod key_serializer;
mod map_serializer;
mod seq_serializer;
mod value_serializer;

mod serializer;
mod unsupported;

pub use self::map_serializer::MapSerializer;
pub use self::seq_serializer::SeqSerializer;
pub use self::serializer::Serializer;
pub use self::unsupported::Unsupported;
pub use self::value_serializer::ValueSerializer;

use crate::error::Result;
use crate::nbt_tag::TagType;

use serde::Serialize;
use std::io;

/// Serialize a value into a byte stream and write it to the given writer.
///
/// Since in the NBT format the root value is always a compound, this function
/// will return an error if the given value is not of a compound type i.e. a
/// struct or a map.
pub fn to_writer<W, S>(writer: &mut W, value: &S) -> Result<()>
where
    W: io::Write,
    S: Serialize,
{
    let mut serializer = Serializer::new(writer);
    value.serialize(&mut serializer)?;
    Ok(())
}

/// Creates a byte vector that represents the header of an NBT tag
/// with the given name and type and returns it.
#[inline]
pub(crate) fn make_header(tag_type: TagType, name: &[u8]) -> Vec<u8> {
    let mut res = vec![tag_type as u8];
    res.extend((name.len() as u16).to_be_bytes());
    res.extend(name);
    res
}

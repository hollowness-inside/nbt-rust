pub mod key_serializer;
pub mod map_serializer;
pub mod serializer;
pub mod unsupported;
pub mod value_serializer;

use serde::Serialize;
use std::io;

use self::serializer::Serializer;
use crate::error::Result;

/// Serialize a value into a byte stream and write it to the given writer.
/// 
/// Since in the NBT format, the root value is always a compound, this function
/// will return an error if the given value is not of a compound type i.e. a
/// struct or a map.
pub fn to_writer(writer: &mut impl io::Write, value: &impl Serialize) -> Result<()> {
    let mut serializer = Serializer::new(writer);
    value.serialize(&mut serializer)?;
    Ok(())
}

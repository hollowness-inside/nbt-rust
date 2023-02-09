mod key_serializer;
mod map_serializer;
mod serializer;
mod unsupported;
mod value_serializer;

pub use self::map_serializer::MapSerializer;
pub use self::serializer::Serializer;
pub use self::unsupported::Unsupported;
pub use self::value_serializer::ValueSerializer;

use crate::error::Result;

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

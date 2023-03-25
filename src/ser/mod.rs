mod key_ser;
mod map_ser;
mod seq_ser;
mod ser;
mod unsupported;

pub use self::ser::Serializer;
use crate::error::Result;

use serde::Serialize;
use std::io::Write;

pub fn to_writer<W, S>(writer: &mut W, value: &S) -> Result<()>
where
    W: Write,
    S: Serialize,
{
    let mut serializer = Serializer::new(writer);
    value.serialize(&mut serializer)?;
    Ok(())
}

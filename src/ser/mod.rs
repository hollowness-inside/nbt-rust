pub mod serializer;
pub mod name_serializer;

use serde::Serialize;
use std::io;

use self::serializer::Serializer;
use crate::error::Result;

pub fn to_writer(writer: &mut impl io::Write, value: &impl Serialize) -> Result<()> {
    let mut serializer = Serializer::new(writer);
    value.serialize(&mut serializer)?;
    Ok(())
}

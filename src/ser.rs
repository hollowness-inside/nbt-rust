use std::io::Write;

use serde::Serialize;

use crate::error::Result;

pub fn to_writer<W, S>(writer: &mut W, value: &S) -> Result<()>
where
    W: Write,
    S: Serialize,
{
    let mut serializer = Serializer::new(writer);
    value.serialize(serializer);
    Ok(())
}

pub struct Serializer<W>(pub(crate) W);

impl<W: Write> Serializer<W> {
    pub fn new(writer: W) -> Self {
        Serializer(writer)
    }

    pub fn into_inner(self) -> W {
        self.0
    }
}
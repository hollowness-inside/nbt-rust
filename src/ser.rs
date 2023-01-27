use std::io;

use serde::ser;

use crate::{error::{Result, Error}, NbtTag};

pub struct Serializer<W> {
    writer: W,
}


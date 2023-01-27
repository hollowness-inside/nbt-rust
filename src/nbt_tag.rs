pub enum NbtTag {
    End,
    Byte(u8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<u8>),
    String(String),
    List(Vec<NbtTag>),
    Compound(Vec<(String, NbtTag)>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

impl NbtTag {
    pub const fn tag_type(&self) -> u8 {
        match self {
            NbtTag::End => 0x00,
            NbtTag::Byte(_) => 0x01,
            NbtTag::Short(_) => 0x02,
            NbtTag::Int(_) => 0x03,
            NbtTag::Long(_) => 0x04,
            NbtTag::Float(_) => 0x05,
            NbtTag::Double(_) => 0x06,
            NbtTag::ByteArray(_) => 0x07,
            NbtTag::String(_) => 0x08,
            NbtTag::List(_) => 0x09,
            NbtTag::Compound(_) => 0x0a,
            NbtTag::IntArray(_) => 0x0b,
            NbtTag::LongArray(_) => 0x0c,
        }
    }
}

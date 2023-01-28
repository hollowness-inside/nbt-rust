pub(crate) mod prefixes {
    pub const END: u8 = 0x00;
    pub const BYTE: u8 = 0x01;
    pub const SHORT: u8 = 0x02;
    pub const INT: u8 = 0x03;
    pub const LONG: u8 = 0x04;
    pub const FLOAT: u8 = 0x05;
    pub const DOUBLE: u8 = 0x06;
    pub const BYTE_ARRAY: u8 = 0x07;
    pub const STRING: u8 = 0x08;
    pub const LIST: u8 = 0x09;
    pub const COMPOUND: u8 = 0x0a;
    pub const INT_ARRAY: u8 = 0x0b;
    pub const LONG_ARRAY: u8 = 0x0c;
}

#[derive(Clone)]
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

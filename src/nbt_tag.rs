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

#[derive(Debug, Clone)]
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
            NbtTag::End => prefixes::END,
            NbtTag::Byte(_) => prefixes::BYTE,
            NbtTag::Short(_) => prefixes::SHORT,
            NbtTag::Int(_) => prefixes::INT,
            NbtTag::Long(_) => prefixes::LONG,
            NbtTag::Float(_) => prefixes::FLOAT,
            NbtTag::Double(_) => prefixes::DOUBLE,
            NbtTag::ByteArray(_) => prefixes::BYTE_ARRAY,
            NbtTag::String(_) => prefixes::STRING,
            NbtTag::List(_) => prefixes::LIST,
            NbtTag::Compound(_) => prefixes::COMPOUND,
            NbtTag::IntArray(_) => prefixes::INT_ARRAY,
            NbtTag::LongArray(_) => prefixes::LONG_ARRAY,
        }
    }
}

impl From<u8> for NbtTag {
    fn from(v: u8) -> Self {
        NbtTag::Byte(v)
    }
}

impl From<i16> for NbtTag {
    fn from(v: i16) -> Self {
        NbtTag::Short(v)
    }
}

impl From<i32> for NbtTag {
    fn from(v: i32) -> Self {
        NbtTag::Int(v)
    }
}

impl From<i64> for NbtTag {
    fn from(v: i64) -> Self {
        NbtTag::Long(v)
    }
}

impl From<f32> for NbtTag {
    fn from(v: f32) -> Self {
        NbtTag::Float(v)
    }
}

impl From<f64> for NbtTag {
    fn from(v: f64) -> Self {
        NbtTag::Double(v)
    }
}

impl From<Vec<u8>> for NbtTag {
    fn from(v: Vec<u8>) -> Self {
        NbtTag::ByteArray(v)
    }
}

impl From<String> for NbtTag {
    fn from(v: String) -> Self {
        NbtTag::String(v)
    }
}

impl From<Vec<NbtTag>> for NbtTag {
    fn from(v: Vec<NbtTag>) -> Self {
        NbtTag::List(v)
    }
}

impl From<Vec<(String, NbtTag)>> for NbtTag {
    fn from(v: Vec<(String, NbtTag)>) -> Self {
        NbtTag::Compound(v)
    }
}

impl From<Vec<i32>> for NbtTag {
    fn from(v: Vec<i32>) -> Self {
        NbtTag::IntArray(v)
    }
}

impl From<Vec<i64>> for NbtTag {
    fn from(v: Vec<i64>) -> Self {
        NbtTag::LongArray(v)
    }
}

impl From<()> for NbtTag {
    fn from(_: ()) -> Self {
        NbtTag::End
    }
}
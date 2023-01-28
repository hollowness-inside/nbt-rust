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

impl From<u32> for NbtTag {
    fn from(value: u32) -> Self {
        NbtTag::Int(value as i32)
    }
}

impl From<i32> for NbtTag {
    fn from(value: i32) -> Self {
        NbtTag::Int(value)
    }
}

impl From<u64> for NbtTag {
    fn from(value: u64) -> Self {
        NbtTag::Long(value as i64)
    }
}

impl From<i64> for NbtTag {
    fn from(value: i64) -> Self {
        NbtTag::Long(value)
    }
}

impl From<f32> for NbtTag {
    fn from(value: f32) -> Self {
        NbtTag::Float(value)
    }
}

impl From<f64> for NbtTag {
    fn from(value: f64) -> Self {
        NbtTag::Double(value)
    }
}

impl From<&str> for NbtTag {
    fn from(value: &str) -> Self {
        NbtTag::String(value.to_string())
    }
}

impl From<String> for NbtTag {
    fn from(value: String) -> Self {
        NbtTag::String(value)
    }
}

impl From<Vec<u8>> for NbtTag {
    fn from(value: Vec<u8>) -> Self {
        NbtTag::ByteArray(value)
    }
}

impl From<&[u8]> for NbtTag {
    fn from(value: &[u8]) -> Self {
        NbtTag::ByteArray(value.to_vec())
    }
}

impl From<Vec<i32>> for NbtTag {
    fn from(value: Vec<i32>) -> Self {
        NbtTag::IntArray(value)
    }
}

impl From<&[i32]> for NbtTag {
    fn from(value: &[i32]) -> Self {
        NbtTag::IntArray(value.to_vec())
    }
}

impl From<Vec<i64>> for NbtTag {
    fn from(value: Vec<i64>) -> Self {
        NbtTag::LongArray(value)
    }
}

impl From<&[i64]> for NbtTag {
    fn from(value: &[i64]) -> Self {
        NbtTag::LongArray(value.to_vec())
    }
}

impl From<Vec<NbtTag>> for NbtTag {
    fn from(value: Vec<NbtTag>) -> Self {
        NbtTag::List(value)
    }
}

impl From<&[NbtTag]> for NbtTag {
    fn from(value: &[NbtTag]) -> Self {
        NbtTag::List(value.to_vec())
    }
}

impl From<Vec<(String, NbtTag)>> for NbtTag {
    fn from(value: Vec<(String, NbtTag)>) -> Self {
        NbtTag::Compound(value)
    }
}

impl From<&[(String, NbtTag)]> for NbtTag {
    fn from(value: &[(String, NbtTag)]) -> Self {
        NbtTag::Compound(value.to_vec())
    }
}
use super::serializer::Serializer;

pub struct ValueSerializer<'a, W> {
    pub(crate) ser: &'a mut Serializer<W>,
    pub(crate) name: Vec<u8>,
}

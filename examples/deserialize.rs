use rnbt::Serializer;

fn main() {
    let c = Serializer::serialize_int_array(&vec![1,2,3,4,5]).unwrap();
    let de = rnbt::de::from_bytes(&c).unwrap();
    println!("{de:?}")
}

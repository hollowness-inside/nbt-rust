use rnbt::Serializer;
use rnbt::NbtTag;
use rnbt::de;

fn main() {
    let c = Serializer::serialize_int_array(&vec![1,2,3,4,5]).unwrap();
    let c = Serializer::serialize_compound(
        &vec![
            ("a".to_string(), NbtTag::Int(1)),
            ("b".to_string(), NbtTag::Long(1)),
            ("c".to_string(), NbtTag::String("Hello World".to_string())),
            ]
        ).unwrap();
    let de = de::from_bytes(&c).unwrap();
    println!("{de:?}");

    let mut c = Serializer::start_compound();
    c.write_field("cat", &123).unwrap();
    c.write_field("trophy", &"strong".to_string()).unwrap();
    let c = c.end();
    println!("{:?}", c);
}

use rnbt::Serializer;
use rnbt::NbtTag;
use rnbt::de;

fn main() {
    let bytes = Serializer::serialize_compound(
        &vec![
            ("item_1".to_string(), NbtTag::Int(1)),
            ("item_2".to_string(), NbtTag::Long(1)),
            ("item_3".to_string(), NbtTag::String("Hello World".to_string())),
            ]
        ).unwrap();

    let compound = de::from_bytes(&bytes).unwrap();
    println!("{compound:?}");
}

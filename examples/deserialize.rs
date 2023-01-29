use rnbt::de;
use rnbt::NbtTag;
use rnbt::Serializer;

fn main() {
    let bytes = Serializer::serialize_compound(&[
        ("item_1", NbtTag::Int(1)),
        ("item_2", NbtTag::Long(1)),
        ("item_3", NbtTag::String("Hello World".to_string())),
    ])
    .unwrap();

    let compound = de::from_bytes(&bytes).unwrap();
    println!("{compound:?}");
}

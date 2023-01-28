use rnbt::Serializer;

fn main() {
    let a = Serializer::serialize_bool(false).unwrap();
    println!("{:?}", a);
}

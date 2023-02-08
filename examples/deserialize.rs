use nbt_rust::{de::from_reader, NbtTag};

fn main() {
    let j = b"\x03\x00\x02hi\x00\x00\x00\x12";
    let reader = std::io::Cursor::new(j);
    let d: (String, NbtTag) = from_reader(reader).unwrap();
    println!("{:} {:}", d.0, d.1);
}

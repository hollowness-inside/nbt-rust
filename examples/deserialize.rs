use nbt_rust::{de::from_bytes, NbtTag};

fn main() {
    let j = b"\x03\x00\x02hi\x00\x00\x00\x12";
    let d: (String, NbtTag) = from_bytes(j).unwrap();
    println!("{:} {:}", d.0, d.1);
}

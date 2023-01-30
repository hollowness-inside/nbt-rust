# nbt_rust
A Rust library for serializing and deserializing the NBT (Named Binary Tag) file format used by [Minecraft](https://www.minecraft.net/en-us).

[NBT format documentation](https://minecraft.fandom.com/wiki/NBT_format)

# Serialization
Check out the [examples](https://github.com/hollowness-inside/nbt-rust/tree/main/examples) or simply run
- `cargo run --example serialize`
- `cargo run --example deserialize`


This library allows you to write a single `u8`, or `String`, or any other supported type.


```rust
use nbt_rust::ser::Serializer;

let writer = /* impl std::io::Write */;
let ser = Serializer::new(writer);
ser.serialize(123u8)?; // or ser.serialize_u8(123)
ser.serialize(123i16)?; // or ser.serialize_i8(123)
ser.serialize("Das ist cool")?; // or ser.serialize_str("Das ist cool")
```

Note: In this example, different data types are serialized consecutively but not within a List or an NBT array, so the Deserializer will only read the first value.

Minecraft typically uses Compound tags in its files, so consider using `start_compound` immediately.

```rust
use nbt_rust::ser::Serializer;

let writer = /* impl std::io::Write */;
let compound_ser = Serializer::new(writer).start_compound();

// Write fields to the Compound tag
compound_ser.write_field("item_1", 213u32)?;
compound_ser.write_field("item_1", &[1u8,2,3,4])?;

// Finish the compound tag and get your writer back (if needed)
compound_ser.end()?; // This will return the owned `Serializer` object
```

# Deserialization

There was no real need to wrap deserialization methods into a struct.

There are two ways to deserialize: `from_bytes` and `from_reader`

- `from_bytes(&[u8])`

```rust
use nbt_rust::de::from_bytes;
use nbt_rust::NbtTag;

let bytes = /* Some NBT formatted binary data */
let result: NbtTag = from_bytes(bytes);
/* Do something with the `result` */
```

- `from_reader(impl std::io::Read)`
```rust
use nbt_rust::de::from_reader;
use nbt_rust::NbtTag;

let data = /* impl std::io::Read */
let result: NbtTag = from_reader(bytes);
/* Do something with the `result` */
```

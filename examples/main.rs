use nbt_rust::to_writer;
use serde::{Serialize, Deserialize};
use std::io::Cursor;

#[derive(Debug, Serialize, Deserialize)]
struct GameData {
    name: String,
    version: u64,
    player: Player,
}

#[derive(Debug, Serialize, Deserialize)]
struct Player {
    name: String,
    health: u32,
}

fn main() {
    let buffer: Vec<u8> = Vec::new();
    let mut writer = Cursor::new(buffer);

    let game_data = GameData {
        name: "My Game".to_string(),
        version: 1,
        player: Player {
            name: "Player 1".to_string(),
            health: 100,
        },
    };

    to_writer(&mut writer, &game_data).unwrap();
    let buffer = writer.into_inner();
    println!("{:?}", String::from_utf8(buffer.clone()).unwrap());
}

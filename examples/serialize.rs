use std::io::Cursor;
use nbt_rust::to_writer;

#[derive(serde::Serialize)]
struct GameData {
    name: String,
    version: u32,
    player: Player
}

#[derive(serde::Serialize)]
struct Player {
    name: String,
    health: u32,
    // position: (f32, f32, f32)
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
            // position: (0.0, 0.0, 0.0)
        }
    };

    to_writer(&mut writer, &game_data).unwrap();

    println!("{:?}", String::from_utf8(writer.into_inner()).unwrap());
}
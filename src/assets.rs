use image::{load_from_memory, DynamicImage};
use rust_embed::RustEmbed;
use std::io::Cursor;

#[derive(RustEmbed)]
#[folder = "assets"]
struct Assets;

pub fn get_pokemon_json_string() -> String {
    let json_data = Assets::get("pokemon.json").expect("Pokemon Data not found.");
    let json_str =
        std::str::from_utf8(json_data.data.as_ref()).expect("Failed to convert bytes to string");
    json_str.to_string()
}

pub fn get_pokemon_image(file_path: &str) -> DynamicImage {
    let file = Assets::get(file_path).expect("Pokemon Image not found.");
    let cursor = Cursor::new(file.data);
    load_from_memory(cursor.get_ref()).expect("Failed to load Pokemon image from memory")
}

use serde::Deserialize;

use ron::de::from_reader;

use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

use super::FileLoader;
use super::TEXTURES_PATH;

#[derive(Debug, Deserialize)]
pub struct TextureData {
    pub file: String,
    pub default_uv: String,
    pub uv: HashMap<String, Vec<(u32, u32)>>,
}

impl TextureData {
    pub fn load(file_loader: &mut FileLoader, file: &Path) -> Option<Self> {
        let ron_path = TEXTURES_PATH.join(file);
        println!("Opening {:?}", ron_path);
        let f = file_loader
            .load(ron_path)
            .expect("Error opening RON Texture file. Path might be wrong?");

        match from_reader(f) {
            Ok(x) => Some(x),
            Err(e) => {
                eprintln!("Can't load texture data: {}", e);
                None
            }
        }
    }
}

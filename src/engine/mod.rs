// use luminance::context::GraphicsContext;

use lazy_static::lazy_static;

use std::path::{Path, PathBuf};

// RENDERING MODULES
pub mod hud;
pub mod spatial;

// Public mods
pub mod text;
pub mod texture;
pub mod sound;

// Mods to re-export
mod entity_registry;
mod key_registry;
mod utils;

pub use entity_registry::EntityRegistry;
pub use key_registry::KeyRegistry;
pub use utils::RgbaColor;

lazy_static! {
	pub static ref ASSETS_PATH: &'static Path = Path::new("assets");
	pub static ref TEXTURES_PATH: PathBuf = ASSETS_PATH.join("textures");
	pub static ref MODELS_PATH: PathBuf = ASSETS_PATH.join("models");
	pub static ref FONTS_PATH: PathBuf = ASSETS_PATH.join("fonts");
}

// pub trait Game {
// 	fn init<C: GraphicsContext>(surface: C);
// 	fn update<C: GraphicsContext>(surface: C);
// 	fn render<C: GraphicsContext>(surface: C, tess_gate: luminance::pipeline::TessGate<'_, C>);
// }

// pub struct GameEngine<C: GraphicsContext, G: Game> {
// 	surface: C,
// 	game: G,
// }

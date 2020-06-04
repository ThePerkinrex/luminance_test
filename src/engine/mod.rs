// use luminance::context::GraphicsContext;

use lazy_static::lazy_static;

use std::path::Path;

// RENDERING MODULES
pub mod hud;
pub mod spacial;

// Public mods
pub mod text;
pub mod texture;

// Private mods
mod utils;

// Mods to re-export
mod entity_registry;

pub use entity_registry::EntityRegistry;

lazy_static! {
	pub static ref ASSETS_PATH: &'static Path = Path::new("assets");
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

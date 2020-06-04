// use luminance::context::GraphicsContext;

use lazy_static::lazy_static;

use std::path::Path;

mod utils;
pub mod text;
pub mod hud;
mod entity_registry;

pub use entity_registry::EntityRegistry;

pub mod texture;

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

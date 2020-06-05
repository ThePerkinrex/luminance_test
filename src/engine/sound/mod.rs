use lazy_static::lazy_static;

use std::path::PathBuf;

mod loader;
pub use loader::load_wav;

mod sound_entities;
use super::ASSETS_PATH;

lazy_static! {
	pub static ref SOUNDS_PATH: PathBuf = ASSETS_PATH.join("sounds");
}
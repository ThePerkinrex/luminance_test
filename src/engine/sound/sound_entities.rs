use ambisonic::rodio::Source;
use ambisonic::{Ambisonic, SoundController};

use std::time::Duration;

type Vec3 = [f32; 3];
type Position = Vec3;
type Velocity = Vec3;

#[derive(Debug, Clone)]
pub enum SoundLength {
	Infinite,
	Seconds(f32),
}

impl SoundLength {
	fn update_left(&mut self, d: &Duration) {
		match self {
			Self::Seconds(s) => {
				*s -= d.as_secs_f32();
			}
			_ => (),
		};
	}

	fn is_done(&self) -> bool {
		match self {
			Self::Infinite => false,
			Self::Seconds(s) => *s <= 0.0,
		}
	}
}

pub struct PositionedSound<S: Source<Item = f32> + Send + Clone + 'static> {
	source: (S, SoundLength),
	position: Position,
	velocity: Velocity,
	sound: Option<(SoundController, SoundLength)>,
}

// TODO make the position use a world reeference, not a user reference
impl<S: Source<Item = f32> + Send + Clone + 'static> PositionedSound<S> {
	pub fn new(source: S, length: SoundLength, position: Position) -> Self {
		Self {
			source: (source, length),
			position,
			velocity: [0.0; 3],
			sound: None,
		}
	}

	pub fn set_velocity(&mut self, v: Velocity) {
		self.velocity = v;
		if let Some((sound, _)) = self.sound.as_mut() {
			sound.set_velocity(v);
		}
	}

	pub fn update(&mut self, d: Duration) {
		for i in 0..self.position.len() {
			self.position[i] += self.velocity[i] * d.as_secs_f32();
		}
		if let Some((sound, length)) = self.sound.as_mut() {
			length.update_left(&d);
			if length.is_done() {
				self.sound = None
			} else {
				sound.adjust_position(self.position);
			}
		}
	}

	pub fn play(&mut self, scene: &Ambisonic) {
		let s = scene.play_at(self.source.0.clone(), self.position);
		self.sound = Some((s, self.source.1.clone()))
	}

	pub fn stop(&self) {
		if let Some((s, _)) = self.sound.as_ref() {
			s.stop()
		}
	}

	pub fn is_done(&self) -> bool {
		let mut done = true;
		if let Some((_, l)) = self.sound.as_ref() {
			done = l.is_done()
		}
		done
	}
}

pub struct UnpositionedSound<S: Source<Item = f32> + Send + Clone + 'static> {
	source: (S, SoundLength),
	sound: Option<(SoundController, SoundLength)>,
}

impl<S: Source<Item = f32> + Send + Clone + 'static> UnpositionedSound<S> {
	pub fn new(source: S, length: SoundLength) -> Self {
		Self {
			source: (source, length),
			sound: None,
		}
	}

	pub fn update(&mut self, d: Duration) {
		if let Some((_, length)) = self.sound.as_mut() {
			length.update_left(&d);
			if length.is_done() {
				self.sound = None
			}
		}
	}

	pub fn play(&mut self, scene: &Ambisonic) {
		let s = scene.play_at(self.source.0.clone(), [0.0, 0.0, 0.0]);
		self.sound = Some((s, self.source.1.clone()))
	}

	pub fn stop(&mut self) {
		if let Some((s, _)) = self.sound.as_ref() {
			s.stop()
		}
		self.sound = None
	}

	pub fn is_done(&self) -> bool {
		let mut done = true;
		if let Some((_, l)) = self.sound.as_ref() {
			done = l.is_done()
		}
		done
	}
}

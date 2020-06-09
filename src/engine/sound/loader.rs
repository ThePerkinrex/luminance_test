use ambisonic::rodio::buffer::SamplesBuffer;

use hound::WavReader;

use std::path::Path;

use super::super::FileLoader;
use super::SOUNDS_PATH;

pub fn load_wav<P: AsRef<Path>>(file_loader: &mut FileLoader, filename: P) -> SamplesBuffer<f32> {
	let mut reader =
		WavReader::new(file_loader.load(filename).expect("Error loading wav file")).unwrap();
	let samples = reader
		.samples::<i16>() // Load samples as i16 iter
		.map(|x| if let Ok(v) = x { Some(v as f32) } else { None }) // Map them as an Option<f32> iteer
		.filter(|x| x.is_some()) // Filter None samples
		.map(|x| x.unwrap()) // Unwrap from option the rest of the samples
		.collect::<Vec<f32>>(); // Collect as a Vec<f32>
	SamplesBuffer::new(reader.spec().channels, reader.spec().sample_rate, samples)
}

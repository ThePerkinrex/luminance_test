use zip::ZipWriter;

use std::fs::{read_dir, File};
use std::io::{Read, Result as IOResult, Write};
use std::path::Path;

fn main() {
	if cfg!(feature = "pack") {
		zip_assets();
	}
}

fn zip_assets() {
	let mut zip_writer = ZipWriter::new(File::create("assets.zip").unwrap());
	zip_dir(&mut zip_writer, "assets/").expect("Error creating zip for assets");
}

fn zip_dir<P: AsRef<Path>>(zip_writer: &mut ZipWriter<File>, dir_ref: P) -> IOResult<()> {
	let dir: &Path = dir_ref.as_ref();
	if dir.is_dir() {
		zip_writer
			.add_directory_from_path(dir.as_ref(), Default::default())
			.expect("Error adding dir to zip");
		for entry in read_dir(dir)? {
			let entry = entry?;
			let path = entry.path();
			if path.is_dir() {
				zip_dir(zip_writer, &path)?;
			} else {
				zip_writer
					.start_file_from_path(&path, Default::default())
					.expect("Error starting file to zip");
				let mut buf = Vec::new();
				File::open(&path)?.read_to_end(&mut buf)?;
				zip_writer.write_all(&mut buf)?;
			}
		}
	}
	Ok(())
}

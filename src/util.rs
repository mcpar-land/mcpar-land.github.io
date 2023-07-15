use std::{fs, path::Path};

use crate::Result;

/// Copy a directory and all its contents recursively.
pub fn copy_dir<A: AsRef<Path>, B: AsRef<Path>>(from: A, to: B) -> Result<()> {
	fs::create_dir_all(&to)?;
	for entry in fs::read_dir(&from)? {
		let entry = entry?;
		let ty = entry.file_type()?;
		if ty.is_dir() {
			copy_dir(entry.path(), to.as_ref().join(entry.file_name()))?;
		} else {
			fs::copy(entry.path(), to.as_ref().join(entry.file_name()))?;
		}
	}

	Ok(())
}

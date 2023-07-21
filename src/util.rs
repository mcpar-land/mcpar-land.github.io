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

pub struct Siblings<'a, T> {
	src: &'a Vec<T>,
	i: usize,
}

impl<'a, T> Siblings<'a, T> {
	pub fn new(src: &'a Vec<T>) -> Self {
		Self { src, i: 0 }
	}
}

impl<'a, T> Iterator for Siblings<'a, T> {
	type Item = (Option<&'a T>, &'a T, Option<&'a T>);

	fn next(&mut self) -> Option<Self::Item> {
		let len = self.src.len();
		let i = self.i;
		self.i += 1;
		if i >= len {
			return None;
		}
		if len == 1 {
			return Some((None, &self.src[0], None));
		}
		if i == 0 {
			return Some((None, &self.src[0], Some(&self.src[1])));
		}
		if i == len - 1 {
			return Some((Some(&self.src[i - 1]), &self.src[i], None));
		}
		return Some((
			Some(&self.src[i - 1]),
			&self.src[i],
			Some(&self.src[i + 1]),
		));
	}
}

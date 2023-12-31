use std::path::PathBuf;

use crate::parsers::error::ParsingError;

#[derive(Debug)]
pub enum Error {
	Parsing { path: PathBuf, error: ParsingError },
	FileNotFound(PathBuf),
	InvalidPostFile { path: PathBuf, reason: String },
	NoChildrenNoTemplate,
	Fs(std::io::Error),
	Zip(zip::result::ZipError),
}

impl From<std::io::Error> for Error {
	fn from(err: std::io::Error) -> Self {
		Self::Fs(err)
	}
}

impl From<zip::result::ZipError> for Error {
	fn from(err: zip::result::ZipError) -> Self {
		Self::Zip(err)
	}
}

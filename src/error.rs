use std::path::PathBuf;

use crate::parsers::error::ParsingError;

#[derive(Debug)]
pub enum Error {
	Parsing { path: PathBuf, error: ParsingError },
	FileNotFound(PathBuf),
	InvalidPostFile { path: PathBuf, reason: String },
	NoChildrenNoTemplate,
	Fs(std::io::Error),
	Rss(rss::validation::ValidationError),
	Date(chrono::ParseError),
	Zip(zip::result::ZipError),
	SyntaxLoading(syntect::LoadingError),
}

impl From<std::io::Error> for Error {
	fn from(err: std::io::Error) -> Self {
		Self::Fs(err)
	}
}

impl From<rss::validation::ValidationError> for Error {
	fn from(err: rss::validation::ValidationError) -> Self {
		Self::Rss(err)
	}
}

impl From<chrono::ParseError> for Error {
	fn from(err: chrono::ParseError) -> Self {
		Self::Date(err)
	}
}

impl From<zip::result::ZipError> for Error {
	fn from(err: zip::result::ZipError) -> Self {
		Self::Zip(err)
	}
}

impl From<syntect::LoadingError> for Error {
	fn from(err: syntect::LoadingError) -> Self {
		Self::SyntaxLoading(err)
	}
}

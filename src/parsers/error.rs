use super::frontmatter::FrontmatterParsingError;

#[derive(Debug)]
pub enum ParsingError {
	Frontmatter(FrontmatterParsingError),
}

impl From<FrontmatterParsingError> for ParsingError {
	fn from(value: FrontmatterParsingError) -> Self {
		Self::Frontmatter(value)
	}
}

#[derive(Debug)]
pub enum ParsingError {
	InvalidDate(String),
	CannotFindFrontmatter,
	InvalidFrontmatter(String),
}

use super::error::ParsingError;

pub struct Frontmatter {
	pub title: String,
	pub description: String,
	pub tags: Vec<String>,
}

impl Frontmatter {
	pub fn parse<'a>(
		post: &'a str,
	) -> Result<(&'a str, Frontmatter), ParsingError> {
		let (frontmatter, post) = post
			.strip_prefix("---")
			.ok_or(ParsingError::CannotFindFrontmatter)?
			.split_once("---")
			.ok_or(ParsingError::CannotFindFrontmatter)?;

		let err = || ParsingError::InvalidFrontmatter(frontmatter.to_string());

		let mut lines = frontmatter.trim().lines();

		let title = parse_line(lines.next().ok_or_else(err)?, "title")?.to_string();
		let description =
			parse_line(lines.next().ok_or_else(err)?, "description")?.to_string();
		let tags = parse_tags(parse_line(lines.next().ok_or_else(err)?, "tags")?)?;

		Ok((
			post,
			Frontmatter {
				title,
				description,
				tags,
			},
		))
	}
}

fn parse_tags(tags: &str) -> Result<Vec<String>, ParsingError> {
	let err = || ParsingError::InvalidFrontmatter(tags.to_string());
	Ok(
		tags
			.strip_prefix("[")
			.ok_or_else(err)?
			.strip_suffix("]")
			.ok_or_else(err)?
			.split(",")
			.map(|s| s.trim().to_string())
			.collect::<Vec<String>>(),
	)
}

fn parse_line<'a>(
	line: &'a str,
	expected_key: &'static str,
) -> Result<&'a str, ParsingError> {
	let (key, value) = line
		.split_once(":")
		.ok_or_else(|| ParsingError::InvalidFrontmatter(line.to_string()))?;

	if key.trim() != expected_key {
		return Err(ParsingError::InvalidFrontmatter(format!(
			"{} should be {}",
			key.trim(),
			expected_key
		)));
	}

	Ok(value.trim())
}

pub struct Frontmatter {
	pub title: String,
	pub description: String,
	pub tags: Vec<String>,
}

impl Frontmatter {
	pub fn parse<'a>(
		post: &'a str,
	) -> Result<(&'a str, Frontmatter), FrontmatterParsingError> {
		let (frontmatter, post) = post
			.strip_prefix("---")
			.ok_or(FrontmatterParsingError::MissingDivider)?
			.split_once("---")
			.ok_or(FrontmatterParsingError::MissingSecondDivider)?;

		let mut lines = frontmatter.trim().lines();

		let title = parse_line(
			lines.next().ok_or(FrontmatterParsingError::MissingTitle)?,
			"title",
		)?
		.to_string();
		let description = parse_line(
			lines
				.next()
				.ok_or(FrontmatterParsingError::MissingDescription)?,
			"description",
		)?
		.to_string();
		let tags = parse_tags(parse_line(
			lines.next().ok_or(FrontmatterParsingError::MissingTags)?,
			"tags",
		)?)?;

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

fn parse_tags(tags: &str) -> Result<Vec<String>, FrontmatterParsingError> {
	Ok(
		tags
			.strip_prefix("[")
			.ok_or(FrontmatterParsingError::BadTagsFormat)?
			.strip_suffix("]")
			.ok_or(FrontmatterParsingError::BadTagsFormat)?
			.split(",")
			.map(String::from)
			.collect::<Vec<String>>(),
	)
}

fn parse_line<'a>(
	line: &'a str,
	expected_key: &'static str,
) -> Result<&'a str, FrontmatterParsingError> {
	let (key, value) = line
		.split_once(":")
		.ok_or(FrontmatterParsingError::InvalidLineFormat)?;

	if key.trim() != expected_key {
		return Err(FrontmatterParsingError::IncorrectLinePosition(expected_key));
	}

	Ok(value.trim())
}

#[derive(Debug)]
pub enum FrontmatterParsingError {
	MissingDivider,
	MissingSecondDivider,
	InvalidLineFormat,
	IncorrectLinePosition(&'static str),
	MissingTitle,
	MissingDescription,
	MissingTags,
	BadTagsFormat,
}

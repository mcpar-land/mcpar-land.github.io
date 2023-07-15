use std::path::{Path, PathBuf};

use anyhow::anyhow;
use chrono::NaiveDate;
use gray_matter::{engine::YAML, Matter};
use maud::{html, Markup, PreEscaped, Render};
use pulldown_cmark::Options;
use serde::Deserialize;

use crate::Result;

pub fn read_all_posts() -> Result<Vec<Post>> {
	let posts = std::fs::read_dir("./posts")?;

	let mut parsed_posts: Vec<Post> = Vec::new();

	for post in posts {
		let post = parse_post_from_file(post?.path())?;
		parsed_posts.push(post);
	}

	Ok(parsed_posts)
}

pub struct Post {
	pub frontmatter: PostFrontmatter,
	pub filename: String,
	pub href: String,
	pub date: NaiveDate,
	pub content: Markup,
}

impl Render for Post {
	fn render(&self) -> Markup {
		html! {
			h1.post-title { (self.frontmatter.title) }
			p.post-description {
				(self.frontmatter.description)
			}
			p.post-date {
				(self.date.format("%B %d, %Y"))
			}
			hr;
			.markdown {
				(self.content)
			}
		}
	}
}

#[derive(Deserialize)]
pub struct PostFrontmatter {
	pub title: String,
	pub description: String,
}

pub fn parse_post_from_file<P: AsRef<Path>>(path: P) -> Result<Post> {
	let filename_str = path
		.as_ref()
		.file_name()
		.ok_or_else(|| anyhow!("Invalid path for post"))?
		.to_string_lossy()
		.to_string();

	let (date, _) = filename_str
		.split_once("_")
		.ok_or_else(|| anyhow!("Invalid file format {}", filename_str))?;

	let date = chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d")?;
	let filename_no_ext = filename_str
		.strip_suffix(".md")
		.ok_or_else(|| anyhow!("Post file must end in .md"))?;
	let href = format!("/posts/{}.html", &filename_no_ext);

	let raw = std::fs::read_to_string(path)?;

	let matter = Matter::<YAML>::new();

	let front_matter = matter
		.parse_with_struct::<PostFrontmatter>(&raw)
		.ok_or_else(|| {
			anyhow!("Error in {}: Failed to parse frontmatter", &filename_str)
		})?;

	let html_output = parse_markdown(&front_matter.content);

	Ok(Post {
		frontmatter: front_matter.data,
		filename: filename_no_ext.to_string(),
		href,
		date,
		content: html! { (PreEscaped(html_output)) },
	})
}

fn parse_markdown(input: &str) -> String {
	let mut options = Options::empty();
	options.insert(Options::ENABLE_TABLES);
	options.insert(Options::ENABLE_STRIKETHROUGH);
	options.insert(Options::ENABLE_FOOTNOTES);
	options.insert(Options::ENABLE_SMART_PUNCTUATION);

	let parser = pulldown_cmark::Parser::new_ext(&input, options);
	let parser =
		highlight_pulldown::highlight_with_theme(parser, "InspiredGitHub").unwrap();
	let mut html_output = String::new();
	pulldown_cmark::html::push_html(&mut html_output, parser.into_iter());

	html_output
}

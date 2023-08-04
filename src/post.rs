use std::path::Path;

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

	parsed_posts.sort();
	parsed_posts.reverse();

	Ok(parsed_posts)
}

pub struct Post {
	pub frontmatter: PostFrontmatter,
	pub filename: String,
	pub href: String,
	pub date: NaiveDate,
	pub content: Markup,
}

impl Post {
	pub fn as_list_item(&self) -> Markup {
		html! {
			.post-list-post {
				.post-list-header {
					a.post-list-title href=(self.href) { (self.frontmatter.title) }
					.post-list-line {}
					.post-list-date {
						(self.date.format("%B %d, %Y"))
					}
				}
				.post-list-description {
					(self.frontmatter.description)
				}
			}
		}
	}

	pub fn opengraph_head(&self) -> Markup {
		html! {
			meta property="og:title" content=(&self.frontmatter.title);
			meta property="og:description" content=(&self.frontmatter.description);
			meta property="og:article:published_time" content=(self.date);
		}
	}

	pub fn render(&self, prev: Option<&Post>, next: Option<&Post>) -> Markup {
		html! {
			h1.post-title { (self.frontmatter.title) }
			.post-description {
				(self.frontmatter.description)
			}
			time.post-date datetime=(self.date.format("%Y-%m-%d")) {
				(self.date.format("%B %d, %Y"))
			}
			hr;
			article.markdown {
				(self.content)
			}
			hr.pn-rule;
			.post-prev-next {
				@if let Some(prev) = prev {
					.pn-item.pn-prev {
						.pn-title { (&prev.frontmatter.title) }
						.pn-description { (&prev.frontmatter.description) }
						a.pn-link href=(&prev.href) { "← Previous" }
					}
				} @else {
					div {}
				}
				@if let Some(next) = next {
					.pn-item.pn-next {
						.pn-title { (&next.frontmatter.title) }
						.pn-description { (&next.frontmatter.description) }
						a.pn-link href=(&next.href) { "Next →" }
					}
				} @else {
					div {}
				}
			}
		}
	}
}

impl PartialEq for Post {
	fn eq(&self, other: &Self) -> bool {
		self.date.eq(&other.date)
	}
}

impl Eq for Post {
	fn assert_receiver_is_total_eq(&self) {}
}

impl PartialOrd for Post {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		self.date.partial_cmp(&other.date)
	}
}

impl Ord for Post {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.date.cmp(&other.date)
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

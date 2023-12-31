use highlight_pulldown::PulldownHighlighter;
use maud::{html, Markup, PreEscaped};
use pulldown_cmark::Options;
use std::path::{Path, PathBuf};

use crate::{
	error::Error,
	parsers::{date::Date, frontmatter::Frontmatter},
	Result,
};

pub fn read_all_posts() -> Result<Vec<Post>> {
	let posts = std::fs::read_dir("./posts")?;

	let mut parsed_posts: Vec<Post> = Vec::new();

	let hl = PulldownHighlighter::new("InspiredGitHub").unwrap();

	for post in posts {
		let post = parse_post_from_file(post?.path(), &hl)?;
		parsed_posts.push(post);
	}

	parsed_posts.sort();
	parsed_posts.reverse();

	Ok(parsed_posts)
}

pub struct Post {
	pub frontmatter: Frontmatter,
	pub filename: String,
	pub href: String,
	pub date: Date,
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
						(self.date.pretty())
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
			meta property="og:article:published_time" content=(self.date.iso_8601());
		}
	}

	pub fn render(&self, prev: Option<&Post>, next: Option<&Post>) -> Markup {
		html! {
			h1.post-title { (self.frontmatter.title) }
			.post-description {
				(self.frontmatter.description)
			}
			.post-tags {
				@for tag in &self.frontmatter.tags {
					a href=(format!("/tag/{}.html", tag)) { (tag) }
				}
			}
			time.post-date datetime=(self.date.iso_8601()) {
				(self.date.pretty())
			}
			hr;
			article.markdown {
				(self.content)
			}
			p.back-to-top { a href="#" { "↑ Top" } }
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

pub fn parse_post_from_file<P: AsRef<Path>>(
	path: P,
	hl: &PulldownHighlighter,
) -> Result<Post> {
	let path_buf = PathBuf::from(path.as_ref());

	let filename_str = path
		.as_ref()
		.file_name()
		.ok_or_else(|| Error::FileNotFound(path_buf.clone()))?
		.to_string_lossy()
		.to_string();

	let (date, _) =
		filename_str
			.split_once("_")
			.ok_or_else(|| Error::InvalidPostFile {
				path: path_buf.clone(),
				reason: format!("Invalid file format {}", filename_str),
			})?;

	let date = Date::parse(date).map_err(|error| Error::Parsing {
		path: path_buf.clone(),
		error,
	})?;

	let filename_no_ext =
		filename_str
			.strip_suffix(".md")
			.ok_or_else(|| Error::InvalidPostFile {
				path: path_buf.clone(),
				reason: "Post file must end in .md".to_string(),
			})?;
	let href = format!("/posts/{}.html", &filename_no_ext);

	let raw = std::fs::read_to_string(path)?;

	let (post_content, frontmatter) =
		Frontmatter::parse(&raw).map_err(|error| Error::Parsing {
			path: path_buf.clone(),
			error,
		})?;

	let html_output = parse_markdown(&post_content, hl);

	Ok(Post {
		frontmatter,
		filename: filename_no_ext.to_string(),
		href,
		date,
		content: html! { (PreEscaped(html_output)) },
	})
}

fn parse_markdown(input: &str, hl: &PulldownHighlighter) -> String {
	let parser = pulldown_cmark::Parser::new_ext(&input, markdown_options());
	let parser = parse_markdown_custom(parser.into_iter());
	let parser = hl.highlight(parser).unwrap();
	let mut html_output = String::new();
	pulldown_cmark::html::push_html(&mut html_output, parser.into_iter());

	html_output
}

fn parse_markdown_custom<'a, I: Iterator<Item = pulldown_cmark::Event<'a>>>(
	iter: I,
) -> impl Iterator<Item = pulldown_cmark::Event<'a>> {
	use pulldown_cmark::{CowStr, Event, Tag};
	iter.map(|event| match event {
		Event::Start(Tag::Image(_link_type, url, title)) => {
			let v = html! {
				img src=(url) title=(title);
				@if title.as_ref().trim().len() > 0 {
					p class="markdown-image-title" { (title) }
				}
			};
			Event::Html(CowStr::from(v.0))
		}
		v => v,
	})
}

fn markdown_options() -> Options {
	let mut options = Options::empty();
	options.insert(Options::ENABLE_TABLES);
	options.insert(Options::ENABLE_STRIKETHROUGH);
	options.insert(Options::ENABLE_FOOTNOTES);
	options.insert(Options::ENABLE_SMART_PUNCTUATION);
	options
}

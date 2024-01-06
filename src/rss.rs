use crate::post::Post;
use crate::Result;
use maud::{html, Markup, PreEscaped};

pub fn rss_feed(all_posts: &Vec<Post>) -> Result<Markup> {
	let most_recent_date = all_posts.iter().map(|post| &post.date).max();

	Ok(html! {
		rss xmlns:atom="http://www.w3.org/2005/Atom" version="2.0" {
			channel {
				title { "mcpar.land" }
				link { "https://mcpar.land" }
				description { "Post feed for mcpar.land" }
				generator { "https://github.com/mcpar-land/mcpar-land.github.io" }
				@if let Some(most_recent_date) = most_recent_date {
					pubDate { (most_recent_date.rfc2822()) }
				}
				@for post in all_posts {
					(post.rss_markup())
				}
			}
		}
	})
}

impl Post {
	pub fn rss_markup(&self) -> Markup {
		html! {
			item {
				title { (&self.frontmatter.title) }
				link { (format!("https://mcpar.land{}", self.href)) }
				description {
					(PreEscaped("<![CDATA["))
					(&self.content)
					(PreEscaped("]]>"))
				}
				@for tag in &self.frontmatter.tags {
					category { (tag) }
				}
				pubDate { (&self.date.rfc2822()) }
				guid isPermaLink="true" { (&self.filename) }
			}
		}
	}
}

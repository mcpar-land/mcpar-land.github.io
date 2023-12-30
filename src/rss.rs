use crate::post::{read_all_posts, Post};
use crate::Result;
use maud::{html, Markup};

pub fn rss_feed() -> Result<Markup> {
	// let channel = ChannelBuilder::default()
	// 	.title("mcpar.land".to_string())
	// 	.link("https://mcpar.land".to_string())
	// 	.description("Feed for posts from mcpar.land".to_string())
	// 	.items(
	// 		read_all_posts()?
	// 			.into_iter()
	// 			.take(MAX_RSS_FEED_SIZE)
	// 			.map(|post| post.as_rss_item())
	// 			.collect::<Vec<rss::Item>>(),
	// 	)
	// 	.build();

	// channel.validate()?;g

	// Ok(html! { (PreEscaped(channel.to_string())) })

	let all_posts = read_all_posts()?;

	let most_recent_date = all_posts.iter().map(|post| &post.date).max();

	Ok(html! {
		rss xmlns:atom="http://www.w3.org/2005/Atom" version="2.0" {
			channel {
				title { "mcpar.land" }
				link { "https://mcpar.land" }
				description { "Post feed for mcpar.land" }
				@if let Some(most_recent_date) = most_recent_date {
					pubDate { (most_recent_date.rfc2822()) }
				}
				@for post in &all_posts {
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
				description { (&self.frontmatter.description) }
				@for tag in &self.frontmatter.tags {
					category { (tag) }
				}
				pubDate { (&self.date.rfc2822()) }
				guid isPermaLink="true" { (&self.filename) }
			}
		}
	}
}

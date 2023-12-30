use maud::{html, Markup, PreEscaped};
use rss::Category;
use rss::{validation::Validate, ChannelBuilder};

use crate::post::{read_all_posts, Post};
use crate::Result;

const MAX_RSS_FEED_SIZE: usize = 10;

pub fn rss_feed() -> Result<Markup> {
	let channel = ChannelBuilder::default()
		.title("mcpar.land".to_string())
		.link("https://mcpar.land".to_string())
		.description("Feed for posts from mcpar.land".to_string())
		.items(
			read_all_posts()?
				.into_iter()
				.take(MAX_RSS_FEED_SIZE)
				.map(|post| post.as_rss_item())
				.collect::<Vec<rss::Item>>(),
		)
		.build();

	channel.validate()?;

	Ok(html! { (PreEscaped(channel.to_string())) })
}

impl Post {
	pub fn as_rss_item(&self) -> rss::Item {
		rss::Item {
			title: Some(self.frontmatter.title.clone()),
			link: Some(format!("https://mcpar.land{}", self.href)),
			categories: self
				.frontmatter
				.tags
				.iter()
				.map(|tag| Category {
					name: tag.clone(),
					domain: None,
				})
				.collect(),
			pub_date: Some(self.date.rfc2822()),
			description: Some(self.frontmatter.description.clone()),
			content: Some(self.content.0.clone()),
			..Default::default()
		}
	}
}

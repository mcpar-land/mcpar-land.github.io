use crate::{
	post::{read_all_posts, Post},
	Result,
};
use chrono::{Datelike, NaiveDate};
use maud::{html, Markup};

pub fn blog_list() -> Result<Markup> {
	let posts: Vec<Post> = read_all_posts()?;

	let mut items = Vec::<BlogListItem>::new();

	let mut year: i32 = posts[0].date.year();
	let mut month: u32 = posts[0].date.month();

	items.push(BlogListItem::Header { year, month });

	for post in posts {
		if post.date.year() != year || post.date.month() != month {
			year = post.date.year();
			month = post.date.month();
			items.push(BlogListItem::Header { year, month });
		}

		items.push(BlogListItem::Post(post));
	}

	Ok(html! {
		p {
			"Posts by date"
			" - ";
			a href="/tags.html" { "Posts by tag" }
		}
		.post-list.post-list-with-dates {
			@for item in items {
				@match item {
					BlogListItem::Header { year, month } => h2.post-list-date-header {
						(NaiveDate::from_ymd_opt(year, month, 1).unwrap().format("%B %Y"))
					},
					BlogListItem::Post(post) => {
						(post.as_list_item())
					}
				}
			}
		}
	})
}

enum BlogListItem {
	Header { year: i32, month: u32 },
	Post(Post),
}

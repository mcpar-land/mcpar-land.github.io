use crate::{parsers::date::Date, post::Post, Result};
use maud::{html, Markup};

pub fn blog_list(posts: &Vec<Post>) -> Result<Markup> {
	let mut items = Vec::<BlogListItem>::new();

	let mut year: u16 = posts[0].date.year;
	let mut month: u16 = posts[0].date.month;

	items.push(BlogListItem::Header { year, month });

	for post in posts {
		if post.date.year != year || post.date.month != month {
			year = post.date.year;
			month = post.date.month;
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
						(Date::new(year, month, 1).pretty_no_day())
					},
					BlogListItem::Post(post) => {
						(post.as_list_item())
					}
				}
			}
		}
	})
}

enum BlogListItem<'a> {
	Header { year: u16, month: u16 },
	Post(&'a Post),
}

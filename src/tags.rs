use std::collections::HashMap;

use maud::{html, Markup};

use crate::{page_builder::PageBuilder, post::Post, Result};

pub fn gen_tag_pages(builder: &PageBuilder, posts: &Vec<Post>) -> Result<()> {
	std::fs::create_dir_all("./output/tag")?;

	let mut tags: HashMap<&str, Vec<&Post>> = HashMap::new();

	for post in posts.iter() {
		for tag in post.frontmatter.tags.iter() {
			if !tags.contains_key(&tag.as_str()) {
				tags.insert(tag.as_str(), Vec::new());
			}
			let tags = tags.get_mut(tag.as_str()).unwrap();
			tags.push(post);
		}
	}

	builder
		.clone()
		.body(html! {
			p {
				a href="/blog.html" { "Posts by date" }
				" - ";
				"Posts by tag";
			}
			ul {
				@for (name, posts) in &tags {
					li {
						a href=(format!("/tag/{}.html", name)) { (name) }
						" - ";
						(posts.len());
						" post";
						@if posts.len() > 1 { "s" }
					}
				}
			}
		})
		.write("tags.html")?;

	for (name, posts) in tags {
		builder
			.clone()
			.body(gen_tag_page(name, &posts))
			.write(format!("tag/{}.html", name))?;
	}

	Ok(())
}

fn gen_tag_page(name: &str, posts: &Vec<&Post>) -> Markup {
	html! {
		p {
			a href="/blog.html" { "Posts by date" }
			" - ";
			a href="/tags.html" { "Posts by tag" }
		}
		p {
			"All posts tagged ";
			b { (name) }
		}
		.post-list {
			@for post in posts {
				(post.as_list_item())
			}
		}
	}
}

use crate::{
	blog::blog_list, page_builder::PageBuilder, post::read_all_posts,
	rss::rss_feed, tags::gen_tag_pages, util::Siblings,
};
use maud::{html, Markup};
use post::Post;
use std::path::PathBuf;

pub mod blog;
pub mod error;
pub mod page_builder;
pub mod post;
pub mod rss;
pub mod tags;
pub mod util;
pub mod parsers {
	pub mod date;
	pub mod error;
	pub mod frontmatter;
}

fn main() -> Result<()> {
	// println!("🗑️  Deleting output directory");

	// if std::path::Path::new("./output").is_dir() {
	// 	std::fs::remove_dir_all("./output")?;
	// }

	std::fs::create_dir_all("./output/posts")?;

	std::fs::copy("./robots.txt", "./output/robots.txt")?;

	println!("🗃️  Generating webpages.");

	let mut all_posts = read_all_posts()?;

	let builder = PageBuilder::new()
		.title("john mcparland")
		.description("Hello!");

	// Write all pages
	builder
		.clone()
		.body(homepage(&all_posts)?)
		.write("index.html")?;
	builder
		.clone()
		.body(blog_list(&all_posts)?)
		.write("blog.html")?;
	builder
		.clone()
		.body(rss_feed(&all_posts)?)
		.no_template()
		.write("feed.xml")?;
	gen_tag_pages(&builder, &all_posts)?;
	builder.clone().body(page404()?).write("404.html")?;

	all_posts.reverse();

	// Write all posts
	for (prev, post, next) in Siblings::new(&all_posts) {
		PageBuilder::new()
			.title(&format!("{} - john mcparland", &post.frontmatter.title))
			.description(&post.frontmatter.description)
			.body(post.render(prev, next))
			.write(&format!("posts/{}.html", &post.filename))?;
	}

	println!("💾 Copying static assets");
	crate::util::copy_dir("./static", "./output/static")?;

	println!("🗃️ Creating site archive");
	if PathBuf::from("./output/site.zip").is_file() {
		std::fs::remove_file("./output/site.zip")?;
	}
	zip_extensions::zip_create_from_directory(
		&PathBuf::from("./site.zip"),
		&PathBuf::from("./output"),
	)?;
	std::fs::rename("./site.zip", "./output/site.zip")?;

	println!("✨ Finish!");
	Ok(())
}

fn homepage(all_posts: &Vec<Post>) -> Result<Markup> {
	Ok(html! {
		p {
			"Hello! I'm a developer located in Boston, and this is my website. "
			"Take a look around."
		}
		.post-list {
			@for post in all_posts.iter().take(3) {
				(post.as_list_item())
			}
			a href="/blog.html" style="text-align:right;" {
				"See All " (all_posts.len()) " Posts"
			}
		}
	})
}

fn page404() -> Result<Markup> {
	Ok(html! {
		.not-found-frame {
			p.not-found-header {
				"404"
			}
			p {
				"Page not found!"
			}
			p {
				a href="/" { "Back to home" }
			}
		}
	})
}

pub type Result<T> = std::result::Result<T, crate::error::Error>;

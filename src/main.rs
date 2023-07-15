use std::path::PathBuf;

use maud::{html, Markup, Render};
use post::{parse_post_from_file, Post};

use crate::post::read_all_posts;

pub mod post;
pub mod util;

fn main() -> Result<()> {
	println!("🗑️  Deleting output directory");

	if std::path::Path::new("./output").is_dir() {
		std::fs::remove_dir_all("./output")?;
	}
	std::fs::create_dir_all("./output/posts")?;

	println!("🗃️  Generating webpages.");

	// Write all pages
	output("index.html", homepage()?, Some(base_template))?;

	// Write all posts
	for post in read_all_posts()? {
		output(
			format!("posts/{}.html", &post.filename),
			post.render(),
			Some(base_template),
		)?;
	}

	println!("💾 Copying static assets");
	crate::util::copy_dir("./static", "./output/static")?;

	println!("✨ Finish!");
	Ok(())
}

fn homepage() -> Result<Markup> {
	Ok(html! {
		p {
			"Hello, welcome to the website."
		}
		(post_list()?)
	})
}

fn post_list() -> Result<Markup> {
	Ok(html! {
		#post-list {
			@for post in read_all_posts()? {
				.post-list-post {
					.post-list-header {
						a.post-list-title href=(post.href) { (post.frontmatter.title) }
						.post-list-line {}
						.post-list-date {
							(post.date.format("%B %d, %Y"))
						}
					}
					.post-list-description {
						(post.frontmatter.description)
					}
				}
			}
		}
	})
}

fn base_template(children: Markup) -> Result<Markup> {
	let css = std::fs::read_to_string("./styles.css")?;

	Ok(html! {
		html {
			(maud::DOCTYPE)
			head {
				meta name="viewport" content="width=device-width, initial-scale=1.0";
				style {
					(css)
				}
			}
			body {
				#site-wrapper-right {
					#site-wrapper-left {
						#site-header {
							a #site-title href="/" { "john mcparland" }
							#site-links {
								a href="/about" { "about" }
								a href="/twitter" { "twitter" }
								a href="/" { "blog" }
								a href="/rss" { "rss" }
							}
						}
						#children {
							(children)
						}
					}
				}
			}
		}
	})
}

fn output<P: Into<PathBuf>>(
	path: P,
	value: Markup,
	template: Option<TemplateFn>,
) -> Result<()> {
	let path: PathBuf = path.into();

	let output_path = PathBuf::from("./output/").join(&path);

	println!("📄 {}", path.as_os_str().to_string_lossy());

	let value = match template {
		Some(template) => template(value)?,
		None => value,
	};

	std::fs::write(&output_path, value.0)?;

	Ok(())
}

type TemplateFn = fn(children: Markup) -> Result<Markup>;

pub type Result<T> = std::result::Result<T, anyhow::Error>;

use std::{fs::DirEntry, path::PathBuf};

use maud::{html, Markup, Render};
use post::{parse_post_from_file, Post};

use crate::{blog::blog_list, post::read_all_posts, rss::rss_feed};

pub mod blog;
pub mod post;
pub mod rss;
pub mod util;

fn main() -> Result<()> {
	// println!("🗑️  Deleting output directory");

	// if std::path::Path::new("./output").is_dir() {
	// 	std::fs::remove_dir_all("./output")?;
	// }

	println!("📝 Loading syntax sets");
	syntect::parsing::SyntaxSet::load_from_folder("./syntaxes")?;

	std::fs::create_dir_all("./output/posts")?;

	println!("🗃️  Generating webpages.");

	let og = Some(basic_opengraph("john mcparland", "Hello!"));

	// Write all pages
	output("index.html", homepage()?, og.clone(), Some(base_template))?;
	output("blog.html", blog_list()?, og.clone(), Some(base_template))?;
	output("feed.xml", rss_feed()?, None, None)?;

	// Write all posts
	for post in read_all_posts()? {
		output(
			format!("posts/{}.html", &post.filename),
			post.render(),
			Some(post.opengraph_head()),
			Some(base_template),
		)?;
	}

	println!("💾 Copying static assets");
	crate::util::copy_dir("./static", "./output/static")?;

	println!("✨ Finish!");
	Ok(())
}

fn homepage() -> Result<Markup> {
	let all_posts = read_all_posts()?;
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

fn basic_opengraph(title: &str, desc: &str) -> Markup {
	html! {
		meta property="og:title" content=(title);
		meta property="og:description" content=(desc);
	}
}

fn base_template(children: Markup, head: Option<Markup>) -> Result<Markup> {
	let css = std::fs::read_to_string("./styles.css")?;

	Ok(html! {
		html {
			(maud::DOCTYPE)
			head {
				meta name="viewport" content="width=device-width, initial-scale=1.0";
				style {
					(css)
				}
				link rel="alternate" type="application/rss+xml" title="RSS" href="/feed.xml";
				@if let Some(head) = head {
					(head)
				}
			}
			body {
				#site-wrapper-right {
					#site-wrapper-left {
						header #site-header {
							a #site-title href="/" {
								#site-title-text { "john mcparland" }
								// img src="/static/starheart.gif";
							}

							nav #site-links {
								// a href="/about" { "about" }
								a href="/blog.html" { "blog" }
								a href="/feed.xml" { "rss" }
								a href="https://twitter.com/mcpar_land" target="_blank" { "twitter" }
								a href="https://github.com/mcpar-land" target="_blank" { "github" }
							}
						}
						#children {
							(children)
						}
						footer #site-footer {
							#site-copyright {
								("© John McParland ");
								(chrono::offset::Utc::now().format("%Y"))
							}
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
	head: Option<Markup>,
	template: Option<TemplateFn>,
) -> Result<()> {
	let path: PathBuf = path.into();

	let output_path = PathBuf::from("./output/").join(&path);

	println!("📄 {}", path.as_os_str().to_string_lossy());

	let value = match template {
		Some(template) => template(value, head)?,
		None => value,
	};

	std::fs::write(&output_path, value.0)?;

	Ok(())
}

type TemplateFn = fn(children: Markup, head: Option<Markup>) -> Result<Markup>;

pub type Result<T> = std::result::Result<T, anyhow::Error>;

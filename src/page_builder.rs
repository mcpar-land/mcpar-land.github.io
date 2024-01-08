use crate::{error::Error, Result};
use maud::{html, Markup, PreEscaped};
use std::path::PathBuf;

pub type TemplateFn = fn(&PageBuilder) -> Result<Markup>;

#[derive(Clone)]
pub struct PageBuilder {
	title: Option<String>,
	body: Option<Markup>,
	description: Option<String>,
	template: Option<TemplateFn>,
	head: Option<Markup>,
}

impl PageBuilder {
	pub fn new() -> Self {
		Self {
			title: None,
			body: None,
			description: None,
			head: None,
			template: Some(base_template),
		}
	}

	pub fn title(mut self, title: &str) -> Self {
		self.title = Some(title.to_string());
		self
	}

	pub fn body(mut self, children: Markup) -> Self {
		self.body = Some(children);
		self
	}

	pub fn description(mut self, description: &str) -> Self {
		self.description = Some(description.to_string());
		self
	}

	pub fn head(mut self, head: Markup) -> Self {
		self.head = Some(head);
		self
	}

	pub fn no_template(mut self) -> Self {
		self.template = None;
		self
	}

	pub fn write<P: Into<PathBuf>>(&self, path: P) -> Result<()> {
		let path: PathBuf = path.into();

		let output_path = PathBuf::from("./output/").join(&path);
		println!("📄 {}", path.as_os_str().to_string_lossy());

		if let Some(template) = &self.template {
			std::fs::write(&output_path, template(self)?.0)?;
		} else {
			if let Some(children) = &self.body {
				std::fs::write(&output_path, &children.0)?;
			} else {
				return Err(Error::NoChildrenNoTemplate);
			}
		}

		Ok(())
	}
}

pub fn base_template(builder: &PageBuilder) -> Result<Markup> {
	let css = std::fs::read_to_string("./styles.css")?;

	Ok(html! {
		html {
			(maud::DOCTYPE)
			head {
				meta name="viewport" content="width=device-width, initial-scale=1.0";
				style {
					(PreEscaped(css))
				}
				link rel="alternate" type="application/rss+xml" title="RSS" href="/feed.xml";
				@if let Some(title) = &builder.title {
					title { (&title) }
					meta property="og:title" content=(&title);
				}
				@if let Some(description) = &builder.description {
					meta property="og:description" content=(&description);
				}
				@if let Some(head) = &builder.head {
					(head)
				}
			}
			body {
				#site-wrapper {
					header #site-header {
						a #site-title href="/" {
							#site-title-text { "john mcparland" }
							img src="/static/starheart.gif";
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
						@if let Some(children) = &builder.body {
							(children)
						}
					}
					footer #site-footer {
						// div {
						// 	("© John McParland 2024");
						// 	(chrono::offset::Utc::now().format("%Y"))
						// }
						div {
							a href="/site.zip" { "Download Site" }
						}
					}
				}
			}
		}
	})
}

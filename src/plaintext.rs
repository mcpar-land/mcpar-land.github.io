use crate::{
	post::{read_all_posts, Post},
	Result,
};

pub fn main_plaintext() -> Result<()> {
	let posts = read_all_posts()?;

	for post in posts {
		println!("\n\n");

		println!("{}", post_to_plaintext(&post));
	}

	Ok(())
}

fn post_to_plaintext(post: &Post) -> String {
	let header_box = format!(
		"{}\n{}",
		post.frontmatter.title, post.frontmatter.description
	);
	let header_box = wrap_in_box(&center_multiline(&header_box, None));

	center_multiline(&header_box, Some(80))
}

fn center_multiline(lines: &str, width: Option<usize>) -> String {
	let width =
		width.unwrap_or_else(|| lines.lines().fold(0, |acc, v| v.len().max(acc)));
	lines
		.lines()
		.map(|line| format!("{:^1$}", line, width))
		.collect::<Vec<String>>()
		.join("\n")
}

// fn centered_box(lines: &[String]) -> String {
// 	let width: usize = lines.iter().fold(0, |acc, v| acc.max(v.len()));

// 	let content = lines.iter().fold(String::new(), |acc, line| {
// 		format!("{}\n{:^2$}", acc, line, width)
// 	});
// 	return wrap_in_box(&content);
// }

fn wrap_in_box(content: &str) -> String {
	let width = content.lines().fold(0usize, |acc, v| acc.max(v.len()));

	let mut res = String::new();
	res.push_str(box_drawing::heavy::DOWN_RIGHT);
	res.push_str(&line(width + 2));
	res.push_str(box_drawing::heavy::DOWN_LEFT);
	res.push('\n');

	for line in content.lines() {
		res.push_str(box_drawing::heavy::VERTICAL);
		res.push(' ');
		res.push_str(line);
		for _ in 0..(width - line.len()) {
			res.push(' ');
		}
		res.push(' ');
		res.push_str(box_drawing::heavy::VERTICAL);
		res.push('\n');
	}

	res.push_str(box_drawing::heavy::UP_RIGHT);
	res.push_str(&line(width + 2));
	res.push_str(box_drawing::heavy::UP_LEFT);

	res
}

fn line(len: usize) -> String {
	let mut res = String::new();
	for _ in 0..len {
		res.push_str(box_drawing::heavy::HORIZONTAL);
	}
	res
}

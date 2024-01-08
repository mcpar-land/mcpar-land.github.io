pub fn min_times<'a, T, Tf: Fn(&'a str) -> PResult<'a, T>>(
	times: usize,
	f: Tf,
) -> impl Fn(&'a str) -> PResult<'a, Vec<T>> {
	move |input: &str| {
		let mut res = Vec::new();
		let mut i = 0;
		let mut offset = 0;
		while i < times {
			let (v, input_after) = f(&input[offset..])?;
			res.push(v);
			i += 1;
			offset = input.len() - input_after.len();
		}
		Ok((res, &input[offset..]))
	}
}

pub fn many1<'a, T, Tf: Fn(&'a str) -> PResult<'a, T>>(
	f: Tf,
) -> impl Fn(&'a str) -> PResult<'a, Vec<T>> {
	let parser = many0(f);
	move |input: &str| {
		let (res, input) = parser(input)?;
		if res.len() == 0 {
			Err(ParserError::Detailed(format!("many1 got 0 items")))
		} else {
			Ok((res, input))
		}
	}
}

pub fn many0<'a, T, Tf: Fn(&'a str) -> PResult<'a, T>>(
	f: Tf,
) -> impl Fn(&'a str) -> PResult<'a, Vec<T>> {
	move |input: &str| {
		let mut res = Vec::new();
		let mut offset = 0;
		loop {
			if offset >= input.len() {
				break;
			}
			let input = &input[offset..];
			match f(input) {
				Ok((v, input_after)) => {
					if input.len() <= input_after.len() {
						return Err(ParserError::Detailed(format!(
							"many0 didn't progress"
						)));
					}
					res.push(v);
					offset += input.len() - input_after.len();
				}
				Err(_) => {
					break;
				}
			};
		}
		Ok((res, &input[offset..]))
	}
}

pub fn alpha<'a>(input: &'a str) -> PResult<&'a str> {
	in_set(ALPHA)(input)
}

const ALPHA: &'static str =
	"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn not_in_set<'a>(
	set: &'static str,
) -> impl Fn(&'a str) -> PResult<'a, &'a str> {
	move |input: &str| {
		for (i, c) in input.char_indices() {
			if set.contains(c) {
				return Ok(input.split_at(i));
			}
		}
		Err(ParserError::UnexpectedEof)
	}
}

pub fn in_set<'a>(
	set: &'static str,
) -> impl Fn(&'a str) -> PResult<'a, &'a str> {
	move |input: &str| {
		for (i, c) in input.char_indices() {
			if !set.contains(c) {
				return Ok(input.split_at(i));
			}
		}
		Ok((input, ""))
	}
}

pub fn opt<'a, O, F: Fn(&'a str) -> PResult<'a, O>>(
	f: F,
) -> impl Fn(&'a str) -> PResult<'a, Option<O>> {
	move |input: &str| match f(input) {
		Ok((res, input)) => Ok((Some(res), input)),
		Err(_) => Ok((None, input)),
	}
}

pub fn delimited<'a>(
	start: &'static str,
	end: &'static str,
) -> impl Fn(&'a str) -> PResult<'a, &'a str> {
	move |input: &str| {
		if !input.starts_with(start) {
			return Err(ParserError::Detailed(format!(
				"expected delimited value starting with \"{}\"",
				start
			)));
		}
		let input = &input[start.len()..];
		let first_end_char = end.chars().next().unwrap();
		for (i, c) in input.char_indices() {
			if c == first_end_char && input[i..].starts_with(end) {
				return Ok((&input[..i], &input[(i + end.len())..]));
			}
		}
		Err(ParserError::UnexpectedEof)
	}
}

pub fn until<'a>(
	end: &'static str,
) -> impl Fn(&'a str) -> PResult<'a, &'a str> {
	move |input: &str| {
		let first_end_char = end.chars().next().unwrap();
		for (i, c) in input.char_indices() {
			if c == first_end_char && input[i..].starts_with(end) {
				return Ok((&input[..i], &input[(i + end.len())..]));
			}
		}
		Err(ParserError::UnexpectedEof)
	}
}

pub fn between<
	'a,
	A,
	Af: Fn(&'a str) -> PResult<'a, A>,
	O,
	Of: Fn(&'a str) -> PResult<'a, O>,
	B,
	Bf: Fn(&'a str) -> PResult<'a, B>,
>(
	a: Af,
	output: Of,
	b: Bf,
) -> impl Fn(&'a str) -> PResult<O> {
	move |input: &str| {
		let (_, input) = a(input)?;
		let (res, input) = output(input)?;
		let (_, input) = b(input)?;
		Ok((res, input))
	}
}

pub fn pair<
	'a,
	A,
	Af: Fn(&'a str) -> PResult<'a, A>,
	B,
	Bf: Fn(&'a str) -> PResult<'a, B>,
>(
	af: Af,
	bf: Bf,
) -> impl Fn(&'a str) -> PResult<'a, (A, B)> {
	move |input: &str| {
		let (a, input) = af(input)?;
		let (b, input) = bf(input)?;
		Ok(((a, b), input))
	}
}

pub fn tag<'a>(tag: &'static str) -> impl Fn(&'a str) -> PResult<'a, &'a str> {
	move |input: &str| {
		if input.starts_with(tag) {
			Ok(input.split_at(tag.len()))
		} else {
			Err(ParserError::Detailed(format!("missing tag \"{}\"", tag)))
		}
	}
}

pub fn ws<'a>(input: &'a str) -> PResult<'a, &'a str> {
	for (i, c) in input.char_indices() {
		if !" \t\n".contains(c) {
			return Ok(input.split_at(i));
		}
	}
	Ok(("", input))
}

pub fn map<'a, I, If: Fn(&'a str) -> PResult<'a, I>, O, Of: Fn(I) -> O>(
	in_fn: If,
	map_fn: Of,
) -> impl Fn(&'a str) -> PResult<'a, O> {
	move |input: &str| {
		let (res, input) = in_fn(input)?;
		let res = map_fn(res);
		Ok((res, input))
	}
}

pub fn rest<'a>(input: &'a str) -> PResult<'a, &'a str> {
	Ok((input, ""))
}

type PResult<'a, T> = Result<(T, &'a str), ParserError>;

#[derive(Debug)]
pub enum ParserError {
	UnexpectedEof,
	Detailed(String),
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_tag() {
		let hashtag_bingus = "#bingus";
		let (_, input) = tag("#")(hashtag_bingus).unwrap();
		assert_eq!(input, "bingus");
		assert!(tag("#")("bad input no hashtag").is_err());
	}

	#[test]
	fn test_pair() {
		let hashtag_parser = pair(tag("#"), alpha);

		let input = "#bingus";
		let ((hashtag, bingus), input) = hashtag_parser(input).unwrap();
		assert_eq!(hashtag, "#");
		assert_eq!(bingus, "bingus");
		assert_eq!(input, "");

		hashtag_parser("no hashtag").unwrap_err();
	}

	#[test]
	fn test_alpha() {
		let input = "bingus... or something";
		let (bingus, input) = alpha(input).unwrap();
		assert_eq!(bingus, "bingus");
		assert_eq!(input, "... or something");
		let input = "123?!";
		let (nada, input) = alpha(input).unwrap();
		assert_eq!(nada, "");
		assert_eq!(input, "123?!");
	}

	#[test]
	fn test_map() {
		let parse_bool = map(alpha, |s| match s {
			"true" => Some(true),
			"false" => Some(false),
			_ => None,
		});
		let (res, input) = parse_bool("true").unwrap();
		assert_eq!(res, Some(true));
		assert_eq!(input, "");
		let (res, input) = parse_bool("false").unwrap();
		assert_eq!(res, Some(false));
		assert_eq!(input, "");
		let (res, input) = parse_bool("bingus").unwrap();
		assert_eq!(res, None);
		assert_eq!(input, "");
	}

	#[test]
	fn test_delimited() {
		let paren_parser = delimited("(", ")");
		let input = "(a delimited bingus)...";
		let (res, input) = paren_parser(input).unwrap();
		assert_eq!(res, "a delimited bingus");
		assert_eq!(input, "...");

		let ticks_parser = delimited("```", "```");
		let input = "```bingus in ticks```!!!";
		let (res, input) = ticks_parser(input).unwrap();
		assert_eq!(res, "bingus in ticks");
		assert_eq!(input, "!!!");

		ticks_parser("```missing closing ticks").unwrap_err();
		ticks_parser("missing opening ticks```").unwrap_err();
	}

	#[test]
	fn test_until() {
		fn title_parser<'a>(input: &'a str) -> PResult<'a, &'a str> {
			let (_, input) = tag("#")(input)?;
			let (title, input) = until("\n")(input)?;
			Ok((title, input))
		}
	}

	#[test]
	fn test_many0() {
		let hashtags_parser = many0(tag("#"));
		let (res, input) = hashtags_parser("###three!").unwrap();
		assert_eq!(res.len(), 3);
		assert_eq!(input, "three!");

		let woah_parser = many0(between(ws, tag("woah"), ws));
		let (res, input) = woah_parser(
			"woah woah woahwoahwoah   woahwoah    woah woah woah woah back it up",
		)
		.unwrap();
		assert_eq!(res.len(), 11);
		assert_eq!(input, "back it up");

		let (res, input) = woah_parser("none to be found").unwrap();
		assert_eq!(res.len(), 0);
		assert_eq!(input, "none to be found");
	}

	#[test]
	fn test_many1() {
		let hashtags_parser = many1(tag("#"));
		let (res, input) = hashtags_parser("#####five!!").unwrap();
		assert_eq!(res.len(), 5);
		assert_eq!(input, "five!!");

		hashtags_parser("none to be found").unwrap_err();
	}

	#[test]
	fn test_parse_link() {
		let link_parser = pair(delimited("[", "]"), delimited("(", ")"));

		let input = "[some link text](http://cool.com/)";
		let ((title, url), input) = link_parser(input).unwrap();
		assert_eq!(title, "some link text");
		assert_eq!(url, "http://cool.com/");
		assert_eq!(input, "");
	}
}

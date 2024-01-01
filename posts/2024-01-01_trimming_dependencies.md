---
title: Trimming My Site Dependencies
description: The easy part is done, at least
tags: [rust, patch-note]
---

New year, less dependencies! A few months ago, I [remade my site](/posts/2023-07-16_new_personal_website.html) in a home-grown static site generator. Part of the reason was to pull away from vendors and providers. I think before this I was doing my site in Notion, all the way on the other end of vendor usage.

In my last post I mentioned slowly bringing my dependency list down to zero, like a sort of programming bonsai project. And I've done some of it!

Here's my `Cargo.toml` before:

```toml
anyhow = "1.0.71"
chrono = "0.4.26"
gray_matter = "0.2.6"
highlight-pulldown = "0.2.2"
maud = "0.25.0"
pulldown-cmark = "0.9.3"
rss = { version = "2.0.4", features = ["chrono", "validation"] }
serde = "1.0.171"
syntect = "5.0.0"
zip = "0.6.6"
zip-extensions = "0.6.1"
```

And here it is after!

```toml
highlight-pulldown = "0.2.2"
maud = "0.25.0"
pulldown-cmark = "0.9.3"
zip = "0.5.13"
zip-extensions = "0.6.1"
```

That's pretty good! Here's what I got rid of:

- `rss`: RSS spec is actually really simple, and I already have an HTML templater in `maud` that I can make it with. No problem.
- `gray_matter` / `serde`: Not as easy, but replacing this with a mega strict parser for the exact YAML format I use for frontmatter was doable.
- `chrono`: I was only using this for the post dates, which is exclusively year-month-day, no timezones or days of the month, all the unpleasant stuff date-time libraries handle for us. Didn't need much parsing.
- `anyhow`: Just for error handling. Wrote my own errors. Not a problem.

The stuff that's left over will be a lot more involved. A markdown parser, an HTML templating macro, ZIP file implementation, a _syntax highlighter_? No idea if I can do some of these, but there's no rush. I can trim my little bonsai tree at my own pace.

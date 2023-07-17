---
title: New Personal Website
description: 2000 is the new 2024!
---

Hey, I haven't posted anything here for a while! A bunch has happened in my life over the pandemic.

- I got a full-time job. It's a small software company, and I'm having a great time.
- I moved to Boston! I like it here a lot more than Texas.
- I've learned a LOT more and expanded my programming toolbox.

That last one is mostly through my job. Nothing puts you through the crucible like coding for 8 hours. My toolbox now includes Golang and Python, on top of everything else.

But the big event for posting here is the new website. Recent events in the web-service / social media world have left me very, very disillusioned with modern web trends[^1], and I've found myself nostalgic for the pre-FAANG internet. So I rebuilt my personal site with that philosophy as the guide for both aesthetics and implementation.

## The Look

I tried to go for a combination of modern amenities (flexbox, responsive on mobile, etc) with a Web 1.0 look. Times New Roman is too classic 2000s internet to not use. I was trying to find a way to incorporate old Geocities gifs, but ended up settling on a typography-only look (For now??)

## The Stack

Aside from the hosting, here's what I used to make this new site:

- Rust

That's it, that's the stack. I wanted to keep the tech as vanilla and self-rolled as possible. No JS, no styling framework, no CDN links in the head, etc. Assembling a static site generator with nothing but built-in Rust tools in lieu of Hugo[^2] / Jekyll[^3] / Astro[^4] turned out to be way, way easier than I thought it would be. Like, stunningly simple. I got this running over a weekend. The list of dependencies is short, but contains all the toughest parts.

- Markdown / frontmatter parsing
- RSS serialization
- Date / time stuff

I'd love to make it a project to, over time, bring this dependency list down to zero, but some of these packages have a lot of complexity to them. Reimplementing the whole markdown parser for my personal site? It's not impossible, but it'll take a bit.

# Now What?

I'd love to write up a series with a guide to rolling your own static site gen in rust. Turns out SSG is a very functional, as in functional programming, problem; takes input files, creates output files. No IO, no event handling. Rust is great for coding in the FP style, but this would also be a fun project to do in Haskell. Look out for more posts about this in the near future.

I'm also just eager to write more on here in general. Reading the stuff from resources like [Hundred Rabbits](https://100r.co/site/home.html) and [Permacomputing](https://permacomputing.net/) has lit a fire under me for software dev outside of the current commercial model.

Moving off of the FAANG / Microsoft services that are interested in turning my code and/or writing into AI training fodder is never going to be a complete project, but shifting away from them will be an ongoing project for me. Maybe [Neocities](https://neocities.org/) is a good host for the future.

---

[^1]: see Cory Doctorow's piece on [Enshittification](https://pluralistic.net/2023/01/21/potemkin-ai/#hey-guys), a pattern I can no longer help but see everywhere.

<!---->

[^2]: [Static site generator](https://gohugo.io/)

<!---->

[^3]: [Another static site generator](https://jekyllrb.com/), the one I was using for my site previously.

<!---->

[^4]: [Yet another static site generator](https://astro.build/). This one's really new and has great DX.

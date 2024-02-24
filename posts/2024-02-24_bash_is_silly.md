---
title: Bash is Silly
description: It's just silly
tags: [linux]
---

At some point you might find yourself running a command that looks like this:

```
wget -O my_cool_zip https://www.dropbox.com/scl/fi/m4f4wpg4eiouunnpqjpjd/some_giant_zip_on_dropbox.zip?rlkey=2j34hg234867fd8a&dl=0
```

to download a zip file from some website, such as Dropbox. Run this command and instead of the usual behavior where it shows you a nice download bar, you get the cryptic:

```
Redirecting output to 'wget-log'
```

Now hang on, I didn't ask for that, but whatever. The download finishes, you try to unzip it, and you get met with some kind of file corruption error! What's going on!

i spent weeks just not really even thinking this was something I could fix. Wget is just weird and dropbox files sometimes just download krangled. Oh well, I can deal with these two unrelated things.

I "fixed" these two problems at the same time, on accident.

Without thinking about it much, I ran this command instead once:

```
wget -O my_cool_zip "https://www.dropbox.com/scl/fi/m4f4wpg4eiouunnpqjpjd/some_giant_zip_on_dropbox.zip?rlkey=2j34hg234867fd8a&dl=0"
```

Suddenly, wget behaved as expected? Not in the background? And the file wasn't corrupted either!

The culprit was: `&dl=0`. Or more specifically, `&`! The actual command that was run :

```
wget -O my_cool_zip https://www.dropbox.com/scl/fi/m4f4wpg4eiouunnpqjpjd/some_giant_zip_on_dropbox.zip?rlkey=2j34hg234867fd8a & dl=0
```

The & gets interpreted as "run these two commands concurrently", so we download a truncated version of the original url, and assign `"0"` to the variable `dl`. Wrapping it in quotes fixes it.

And because I'd grown to like the background wget pattern, This restores that behavior, too:

```
wget -O my_cool_zip "https://www.dropbox.com/scl/fi/m4f4wpg4eiouunnpqjpjd/some_giant_zip_on_dropbox.zip?rlkey=2j34hg234867fd8a&dl=0"&
```

Bash is silly.

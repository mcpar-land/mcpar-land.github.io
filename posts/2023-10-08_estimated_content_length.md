---
title: Estimated-Content-Length Should Exist
description: An HTML header problem without a solution
tags: [html, js]
---

A recent problem at work cropped up that required creating an HTTP handler where I didn't know the content size (streaming a multi-gigabyte zip file, created on the fly) before I was finished. I quickly found out that not including `Content-Length` in your HTTP respnose makes downloads look like this:

![](/static/embeds/2023-10-08_estimated_content_length/resuming.png)

A non-progress bar or infinite spinny, depending on your browser. This is pretty unpleasant UX, especially when the download can take multiple minutes. However, when creating a streamed response you often don't or can't know how big your own response is going to be.

I took multiple stabs at vaguely estimating what the download size _would_ be- but I also found that if you don't get your `Content-Length` exactly right, your download will hang indefinitely, or your server framework will complain, or any number of other things. `Content-Length` is meant to be exactly right, or not present at all.

Something like an `Estimated-Content-Length` header, exclusively to satisfy these download bars or to just give a vague idea of how big a streamed response will be, would be _perfect_ for this. But to my surprise, there's no such thing - [Even though the idea has been kicked around since 2008!](https://lists.w3.org/Archives/Public/ietf-http-wg/2008OctDec/0065.html)

There's workarounds, but unpleasant ones - like writing your own file download with [File System Access API](https://developer.chrome.com/articles/file-system-access/), streaming a `fetch` call directly into a file. This API isn't totally available yet across all browsers, and you need to make your own progress bar.

An `Estimated-Content-Length` header has some niche, but very real use cases - big downloads where the file size is not known, because the entire 'file' is never in server memory. [But hey, what's one more?](https://en.wikipedia.org/wiki/List_of_HTTP_header_fields)

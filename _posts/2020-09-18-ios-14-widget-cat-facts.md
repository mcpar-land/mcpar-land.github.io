---
layout: post
title: 'iOS 14 Widget - Cat Facts'
date: 2020-09-18
---

![a](/assets/images/embeds/ios-14-widget-cat-facts/cat_facts.jpeg)

iOS 14's new homescreen widgets mean only one thing: _Ricing._

I wrote a tiny script using [Scriptable](https://scriptable.app/) and [one](https://catfact.ninja/) or [two](https://cataas.com/) public APIs to throw together a widget that shows you a cat fact. And a cat.

```jsx
let fact_req = new Request('https://catfact.ninja/fact?max_length=125')
let json = await fact_req.loadJSON()
let img_req = new Request('https://cataas.com/cat')
let img = await img_req.loadImage()
console.log(img)
const widget = new ListWidget()
widget.backgroundImage = img
widget.addText(json.fact)

Script.setWidget(widget)
```

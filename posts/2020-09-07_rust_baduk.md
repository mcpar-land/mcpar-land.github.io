---
title: Teaching Myself Rust, Part 1
description: Baduk & Bevy
---

[mcpar-land/bevy_baduk](https://github.com/mcpar-land/bevy_baduk)

Teaching myself Rust over the past year has been the toughest, and most rewarding programming challenge I've ever worked through. I had to know why this language I'd never heard of was [the most loved language on Stack Overflow for five years running](https://insights.stackoverflow.com/survey/2020#technology-most-loved-dreaded-and-wanted-languages-loved). Like any prospective Rust programmer, I bonked my head against the borrow checker over and over - but a good bonking, the kind that hammers out a new, internalized way of thinking about a problem, bit by bit. By necessity, I learned about the internal structure of computer memory, sizes of data, multi-threading logic, and plenty more that Rust has, by virtue of its philosophy alone, converted into second nature for me (or moved towards it, at least!)

Rust is great because it's standing on nearly a [half century](<https://simple.wikipedia.org/wiki/C_(programming_language)#:~:text=C%20was%20developed%20in%20the,step%2Dby%2Dstep%20instructions.>) worth of giants' shoulders. The community of programmers have more or less figured out the tools that are so common that Rust opted to have them built-in. Unit testing. Linting. Automatic formatting. A package manager.

Coming over from Node, not having to wrangle a huge `node_modules` folder for each and every project is wonderful. But at the same time, having Cargo as a truly first-class citizen (not having to fuss around with `GOPATH`s or what have you) feels so familiar, I knew how to use it right away.

There was never one single _click_ moment where everything made sense all at once, but lots of little clicks.

- "Oh, that's how borrow scopes work!"
- "Oh, that's what `Option` and `Result` are!"
- "Oh, that's how enums are different!"
- "Oh, that's the point of `Rc` and `Arc`!"
- "Oh, that's what macros do!"

Every one a little more illuminating than the last.

I've assembled a tiny [project](https://github.com/mcpar-land/fallbaq) or [two](https://github.com/mcpar-land/rusx) or [three](https://github.com/mcpar-land/ultra-tournament), learning piles of new concepts every time.

# The Rust Game Engine of the Future: Bevy!

My latest and greatest, though, uses my new favorite game engine, [Bevy](https://bevyengine.org/), so fresh that it came out [last month](https://bevyengine.org/news/introducing-bevy/) and oh boy, it looks like it could go far. The design philosophy and ergonomics make it stand right out in a world of Rust engines that, while type-safe, can come with a looot of boilerplate. Seriously, the way you write the Bevy ECS feels like black magic. Huge props to [@cart](https://twitter.com/cart_cart) for putting his heart and soul into this project. Hopefully I'll be able to contribute to it someday.

But in the meantime, I'm familiarizing myself with it by creating helpful projects like a [general-use 3D flying camera](https://crates.io/crates/bevy_fly_camera) that people are already putting into their own learning projects! But my latest (possibly greatest?) one is a _fully-functioning game of Go._

## Step 1 - Writing Go in Rust

I wanted to do this project in the most "correct" way I could. This means _keeping game logic and display logic separate_. I wanted to aim for a React-style display loop, where I keep my game logic somewhere in the back, then just re-render the entire board every time the game changes. So that's what I did! Before I even touched Bevy, I spent a day making a robust system that reflected Go and its rules, along with plenty of unit tests. I wrote a quick-and-pretty console display for a board state.

![https://i.imgur.com/4u7LwPO.png](https://i.imgur.com/4u7LwPO.png)

This REALLY came in handy for looking at the results of unit tests. I had (almost) every rule of Go's capturing system testing and working before I got to the fun part of writing the GUI.

Robust game logic for Go means:

- Being able to return the array of stones in the 'shape' for any arbitrary position.
- Determining if a move would result in a self-capture (illegal)
- Determining if a move would resulting a self-capture, _but_ would capture an enemy group of stones first (legal)
- Determining if a move would cause the board to return to a previous board state (Ko, illegal)

I also used separate `Board` and `Game` structs. The former just stores a static board state and can say whether or not a move is valid. The latter encapsulates a board, and also stores a list of moves. It can construct a board given an array of moves, meaning at some point I could implement a move history slider!

## Step 2 - Displaying Go in Bevy

For the images, I used the board background from the ubiquitous Go wiki [Sensei's Library](https://senseis.xmp.net/?WhatIsGo). They already have a board templating system set up that's pixel perfect, and making my own board pixel by pixel seemed like manual labor. I did have to make my own graphics for the stones, but that was just making a black and white circle in Aseprite.

Then, it was just a matter of using Bevy's terrific ECS. The basic setup is:

- A mutable `Resource` containing a `Game`, which handles all logic and game state.
- A UI `NodeComponents` for displaying the board.
- A grid of 19x19 invisible `ButtonComponents` bundles, with a `(u8, u8)` component tacked on that stores its position on the grid.
- Another `NodeComponents` bundle, used for displaying the semi-transparent "cursor" stone. It moves to the position of any button the mouse hovers over, and changes its material based on the current turn, or if the position it's overing over is a valid move or not.
- A bunch of `NodeComponents` bundles with a dummy `UiPiece` struct as a component, used to display all the stones on the board. Every time a move is successfully played by the `Game`, all of these are destroyed, then re-added based on the game's current state. This way, I don't have to worry about deleting the exact stones that are removed by any given play or capture.

At most, this means a total of $$19\times19=361$$ entities, in a non-real-time game, which Bevy is more than capable of handling.

The end result? A playable Go board that _only_ allows for valid moves, with instant visual feedback as to whether any given square is valid or not.

<video controls>
<source src="/static/embeds/2020-09-07_rust_baduk/go_in_bevy.mp4" type="video/mp4">
</video>

Could be prettier, but the feedback feels fantastic. I might add sounds at some point to really get the feel down.

# Next Steps

My focus is fickle, so maybe I'll never return to this project. Or maybe I will! If I do, I'm thinking of the following improvements:

- Making Ko tracking more robust. I suspect it's not 100% correct, and it's also stored in the `Board` struct, which should be time-agnostic. Move tracking Ko to the `Game` struct, instead.
- Adding some auditory feedback for placing stones.
- Adding a UI for starting with a handicap. I have handicap as a functionality in the game logic already, I just need to make it a selection at the game start.
- Adding a game history slider at the bottom of the screen.
- Possibly serializing/deserializing from the common Go game file formats available, or writing one of my own.

Feel free to check out the source code on my repo! Running the game is as simple as cloning and running `cargo run`. Make sure you update to Rust 1.46.

[mcpar-land/bevy_baduk](https://github.com/mcpar-land/bevy_baduk/tree/master)

You should also _definitely_ check out Bevy. It's a super-duper new, open source game engine. I suspect it'll be a competitor to Godot within the year. I cannot wait until it's complete enough to make a game with, because that's what I'll be doing!

[Bevy - A data-driven game engine built in Rust](https://bevyengine.org/)

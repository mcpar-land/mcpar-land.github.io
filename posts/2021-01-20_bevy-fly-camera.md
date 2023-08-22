---
title: Bevy Fly Camera
description: Pleasant utility for Bevy
tags: [rust, game-dev]
---

Since the 0.1 release of my favorite Rust game engine, [Bevy](https://bevyengine.org/), I've been maintaining the crate [bevy-fly-camera](https://crates.io/crates/bevy_fly_camera), slowly adding to its tiny suite of features and keeping it updated with the engine's rapidly-changing alpha versions. The idea is that having a free-move camera is one of the best features to have during early iteration, so a super bog-simple to use crate that you can stick in your code in as few lines as possible fills a very important niche.

The concept itself isn't actually that difficult to code — but it's not enough to just exist for me, it has to exist for everyone. Most of this project has been learning how to document code well, how to handle releases and versions, and how to maintain a scope.

Thankfully writing code docs is as slick as can be with Rust, thanks to its built-in documentation compiler. Seriously, everything about the Rust ecosystem is best-in-class.

<video controls>
	<source src="/static/embeds/2021-01-20_bevy-fly-camera/RQL3KyXCAB.mp4" type="video/mp4">
</video>

```rust
use bevy::prelude::*;
use bevy_fly_camera::{
	FlyCamera,
	FlyCameraPlugin
};

fn setup(commands: &mut Commands) {
  commands
    .spawn(Camera3dBundle::default())
    .with(FlyCamera::default());
}

fn main() {
  App::build()
    .add_plugins(DefaultPlugins)
    .add_startup_system(
			setup.system()
		)
    .add_plugin(FlyCameraPlugin)
    .run();
}
```

<video controls>
	<source src="/static/embeds/2021-01-20_bevy-fly-camera/3HD4YoOgL5.mp4" type="video/mp4">
</video>

```rust
use bevy::prelude::*;
use bevy_fly_camera::{
	FlyCamera2d,
	FlyCameraPlugin
};

fn setup(commands: &mut Commands) {
  commands
    .spawn(Camera2dBundle::default())
    .with(FlyCamera2d::default());
}

fn main() {
  App::build()
    .add_plugins(DefaultPlugins)
    .add_startup_system(
			setup.system()
		)
    .add_plugin(FlyCameraPlugin)
    .run();
}
```

When bevy 0.5 hits, I'll be releasing the new version right away, along with the new feature in its slim count: a fly camera for 2D!

[mcpar-land/bevy_fly_camera](https://github.com/mcpar-land/bevy_fly_camera)

[crates.io: Rust Package Registry](https://crates.io/crates/bevy_fly_camera)

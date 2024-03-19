<img src="https://github.com/Multirious/bevy_tween/assets/77918086/38ab44e1-67a4-4c2d-b17c-3a35128e6930" width="100%"/>

[![Crates.io Version](https://img.shields.io/crates/v/bevy_tween?style=for-the-badge)](https://crates.io/crates/bevy_tween)
[![Crates.io License](https://img.shields.io/crates/l/bevy_tween?style=for-the-badge)](https://github.com/Multirious/bevy_tween/blob/main/README.md#license)
[![Docs.rs](https://img.shields.io/docsrs/bevy_tween?style=for-the-badge)](https://docs.rs/bevy_tween)

# `bevy_tween`

Flexible tweening plugin for Bevy.
This crate solves for tweening animation with the approach of integrating everything
into Bevy's ECS, allowing you to exploits the already flexible ECS systems to tune
and extends the animation process to your needs.

## Differences
The main motivation for this tweening crate is that the previous
existing tweening crates is not flexible enough and so to goal is to solve it.
The differences will be explained below.

Differences to [`bevy_tweening`](https://github.com/djeedai/bevy_tweening)
or [`bevy_easings`](https://github.com/vleue/bevy_easings):
- Tweening is not tied to a certain entity. You can create an entity specifically
  for tweening any where in the world.
- Complex animation, such as sequential or parallel tweening, are solved using
  child-parent hierarchy:
  - Solved the issue of modifying animation at runtime presents in the previous
    crates.
  - Everything exists in the ECS world with no hidden structure, everything can
    be freely accessed.
  - Makes a very extendable system, thanks Bevy's ECS!
- Users of this crate are free to decide if they want to only use generic,
  only trait object, or even both for tweening! They both came with their pros
  and cons which will be explained in the documentation.
- Flexibility at the cost of verbosity. APIs can be more verbose than the mentioned
  crates without extra helpers.

## Feature gates
- `"span_tween"`, enabled by default.<br/>
  Tween for a range of time. 
- `"bevy_asset"`, enabled by default.<br/>
  enable `"bevy/bevy_asset"`, add tweening systems for asset.
- `"bevy_render"`, enabled by default.<br/>
  enable `"bevy/bevy_render"`, add nothing but required by the `"bevy_sprite"` feature.
- `"bevy_sprite"`, enabled by default.<br/>
  enable `"bevy/bevy_sprite"`, add some built-in interpolator related to sprite.

## Demos
`cargo run --example demo_follow -F bevy/bevy_winit`<br/>
`cargo run --example demo_click -F bevy/bevy_winit`<br/>
`cargo run --example demo_hold -F bevy/bevy_winit`<br/>

## Bevy Version Support

|`bevy`|`bevy_tween`|
|------|------------|
|0.13  |0.2         |

## Credits
- [`bevy_tweening`](https://github.com/djeedai/bevy_tweening)

  The first crate I discovered and tried to do tweening with in Bevy. I like
  the idea of `Lens` of theirs. I've renamed this to `Interpolator` in this crate.

- [`godot`](https://github.com/godotengine/godot)

  Godot's tween make it simple to animate something which is part of the idea
  for this crate. The Godot's node child-parent hierarchy system and that the
  engine utilizes this to define behavior, powerful stuff!

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

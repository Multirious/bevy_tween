<img src="https://github.com/Multirious/bevy_tween/assets/77918086/38ab44e1-67a4-4c2d-b17c-3a35128e6930" width="100%"/>

[![Crates.io Version](https://img.shields.io/crates/v/bevy_tween?style=for-the-badge)](https://crates.io/crates/bevy_tween)
[![Crates.io License](https://img.shields.io/crates/l/bevy_tween?style=for-the-badge)](https://github.com/Multirious/bevy_tween/blob/main/README.md#license)
[![Docs.rs](https://img.shields.io/docsrs/bevy_tween?style=for-the-badge)](https://docs.rs/bevy_tween)

# `bevy_tween`

Flexible tweening plugin for Bevy.

## Differences
The main motivation or goal for this tweening crate is that the previous
existing tweening crates is not flexible enough. The differences will be
explained below.

Differences to [`bevy_tweening`](https://github.com/djeedai/bevy_tweening)
or [`bevy_easings`](https://github.com/vleue/bevy_easings):
- Tweening is not tied to a certain entity. You can create an entity specifically
  for tweening any where in the world.
- Complex animation, such as sequential or parallel animation, are designed to
  work in child-parent hierarchy which solve the issue presents in the previous crates
  of modifying animation at runtime. Because everything exists in the ECS world
  with no hidden structure, everything can be freely accessed.
- User of this crate are free to decide if they want to only use generic,
  or trait object, or both for their tweening! Both came with pros and cons which
  will be explained in the documentation.

## Feature gates
- `"span_tween"`, tweener implementation by defining a tween in range of time. Enabled by default.
- `"bevy_asset"`, enable `"bevy/bevy_asset"`, add tweening systems for asset. Enabled by default.
- `"bevy_render"`, enable `"bevy/bevy_render"`, add nothing just yet but required by the `"bevy_sprite"` feature. Enabled by default.
- `"bevy_sprite"`, enable `"bevy/bevy_sprite"`, add some built-in interpolator related to sprite. Enabled by default.

## Bevy Version Support

|`bevy`|`bevy_tween`|
|------|------------|
|0.13  |0.2         |

## Credits
- [`bevy_tweening`](https://github.com/djeedai/bevy_tweening)

  The first crate I discovered and tried to do tweening with in Bevy. I like
  the idea of `Lens` of theirs. I've renamed this to `Interpolator` in this crate.

- [`godot`](https://github.com/godotengine/godot)

  Godot's tween make it simple to animate something which what I kept thinking
  about trying to do any animation. What's the big part is the Godot's node
  hierarchy system which utilize hierarchy of child-parent node to define
  behavior. It's an important puzzle piece of how this crate works.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

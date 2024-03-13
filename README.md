<img src="https://github.com/Multirious/bevy_tween/assets/77918086/38ab44e1-67a4-4c2d-b17c-3a35128e6930" width="100%"/>

![Crates.io Version](https://img.shields.io/crates/v/bevy_tween?style=for-the-badge)
![Crates.io License](https://img.shields.io/crates/l/bevy_tween?style=for-the-badge)

# `bevy_tween`

Flexible tweening plugin for Bevy.

## Feature gates
- `"span_tween"`, tween player implementation by defining a tween in range of time. Enabled by default.
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
  the idea of `Lens` of theirs. As I've been experimenting with this, I believe
  that "lens" is a bit misleading as it sounds like a subset of something. So,
  I've renamed this to `Interpolator` to reflect its behavior in this crate.

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

<img src="https://github.com/Multirious/bevy_tween/assets/77918086/38ab44e1-67a4-4c2d-b17c-3a35128e6930" width="100%"/>

[![Crates.io Version](https://img.shields.io/crates/v/bevy_tween?style=for-the-badge)](https://crates.io/crates/bevy_tween)
[![Crates.io License](https://img.shields.io/crates/l/bevy_tween?style=for-the-badge)](https://github.com/Multirious/bevy_tween/blob/main/README.md#license)
[![Docs.rs](https://img.shields.io/docsrs/bevy_tween?style=for-the-badge)](https://docs.rs/bevy_tween)

# `bevy_tween`

Flexible tweening plugin for Bevy.
This crate solves for tweening animation with the approach of integrating everything
into Bevy's ECS and uses dependency injection, allowing you to exploits the already flexible ECS systems to tune
and extends the animation process to your needs.

This is a young plugin and APIs are to be fleshed out.
Breaking changes are to be expected!

See changelog [here](CHANGELOG.md).

## Differences
The main motivation for this tweening crate is that the previous
existing tweening crates is not flexible enough and so the main goal is to solve it.

Goals:
- [x] Flexible 🎉
- [ ] Built-in Keyframe animation support via `splines`.
- integration with other crates (?)
  - [ ] `bevy_animation`
- [ ] Editor. While the original goal is to just be a tweening from code crate,
       this crate absolutely has the capability to work on any complex animations.
       The editor will aid in such jobs.
  - Real-time display at any point in time in the animation.
  - Editing path from point A to point B with arbitary curve using `splines`.

Differences to [`bevy_tweening`](https://github.com/djeedai/bevy_tweening)
or [`bevy_easings`](https://github.com/vleue/bevy_easings):
- Tweening is not tied to a certain entity. You can create an entity specifically
  for tweening any where in the world.
- Complex animations, such as sequential or parallel tweening, are solved using
  child-parent hierarchy:
  - Solved the issue of modifying animation at runtime presents in the previous
    crates.
  - Everything exists in the ECS world with no hidden structure, everything can
    be freely accessed.
  - Makes a very extendable system, thanks Bevy's ECS!
  - There's no limitation on what can and can't be tween.
    It's possible to many `Interpolator` (or `Lens` if you came from `bevy_tweening`)
    tweening the same component because of the multi-entities architecture and
    so is not limited by '1 component type per entitiy'.
- Advanced timer. This crate has custom timer implementation.
  - Looping support.
  - 2 ways playback support.
  - Jump to arbitary time.
  - Ticking in arbitary direction.
- Dependency injection. Systems communicate through various specific components,
  allowing you to extends the behavior to your needs by supplying those components
  and reduce duplication.
  - Custom tweens, targets, and interpolators
  - Custom interpolations
  - Custom tweeners
- Users of this crate are free to decide if they want to only use generic,
  only trait object, or even both for tweening! They both came with their pros
  and cons which will be explained in the documentation.
- Flexibility at the cost of verbosity. APIs can be more verbose than the mentioned
  crates without extra shortcut and helpers.

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
`cargo run --example follow -F bevy/bevy_winit`<br/>
`cargo run --example click -F bevy/bevy_winit`<br/>
`cargo run --example hold -F bevy/bevy_winit`<br/>

## Bevy Version Support

|`bevy`|`bevy_tween`|
|------|------------|
|0.13  |0.2         |

## Credits
- [`bevy_tweening`](https://github.com/djeedai/bevy_tweening)

  The first crate I discovered and tried to do tweening with in Bevy.
  Their method of `Lens` is great and so it's present in this crate.
  Now called `Interpolator`. Usages may be similar but is
  implemented differently.

- [`godot`](https://github.com/godotengine/godot)

  Godot's tween make it simple to animate something which is the main
  inspiration for this crate. The Godot's node child-parent hierarchy
  system and that most of the engine APIs utilizes this to define behavior,
  yoinked.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

<img src="https://github.com/Multirious/bevy_tween/blob/main/examples/animation/banner_triangle.gif" width="100%"/>

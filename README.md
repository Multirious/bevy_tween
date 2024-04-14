<img src="https://github.com/Multirious/bevy_tween/assets/77918086/38ab44e1-67a4-4c2d-b17c-3a35128e6930" width="100%"/>

[![Crates.io Version](https://img.shields.io/crates/v/bevy_tween?style=for-the-badge)](https://crates.io/crates/bevy_tween)
[![Crates.io License](https://img.shields.io/crates/l/bevy_tween?style=for-the-badge)](https://github.com/Multirious/bevy_tween/blob/main/README.md#license)
[![Docs.rs](https://img.shields.io/docsrs/bevy_tween?style=for-the-badge)](https://docs.rs/bevy_tween)

# `bevy_tween`

A fully ECS-based [Bevy](https://github.com/bevyengine/bevy) animation library.
Focuses mainly on tweening but being decoupled and flexible and so can do much more.

This is a young plugin and APIs are to be fleshed out.
Breaking changes are to be expected!

See changelog [here](CHANGELOG.md).

## Features
- ECS-based animation data and system with flexible and modular APIs powered by Bevy.
  Use anything you want to use. Remove anything you don't want. Extends anything that's not there.
- Tween anything, from anywhere.
  - Colors, sprite sheet frames, positions, you define it!
  - Components, assets, resources, you implement it!
- Interpolate with anything
  - Robert Penner's easing functions
  - Closure
  - Or implement one your self!
- Animate at any complexity
- Events at arbitary time (with custom data).
- Timer
  - Looping
  - Fastforward or Rewind
  - Skip backward or forward
  - Jumping to anywhen

Goals:
- [x] Flexible 🎉
- [ ] Built-in Keyframe animation support.
- integration with other crates (?)
  - [ ] `bevy_animation`
  - [ ] `bevy_lookup_curve`
- [ ] Editor. While the original goal for this crate is tweening from code,
       this crate absolutely has the capability to work on any complex animations.
       The editor will aid in such jobs.
  - Real-time display at any point in time in the animation.
  - Editing path from point A to point B with arbitary curve using `splines`.

## Differences
The main motivation for this tweening crate is that the previous
existing tweening crates is not flexible enough and so the main goal is to solve it.

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
  - It's possible to have multiple `Interpolator` (or `Lens` if you came from `bevy_tweening`)
    tweening the same component/asset/resource because of the multi-entities architecture and
    so it is not limited by '1 component type per entitiy'.
- Advanced timer. This crate has custom timer implementation.
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
  crates *without* extra shortcut and helpers.

## Feature gates
- `"span_tween"`, enabled by default.<br/>
  Tween for a range of time. 
- `"bevy_asset"`, enabled by default.<br/>
  enable `"bevy/bevy_asset"`, add tweening systems for asset.
- `"bevy_render"`, enabled by default.<br/>
  enable `"bevy/bevy_render"`, add nothing but required by the `"bevy_sprite"` feature.
- `"bevy_sprite"`, enabled by default.<br/>
  enable `"bevy/bevy_sprite"`, add some built-in interpolator related to sprite.



## Bevy Version Support

|`bevy`|`bevy_tween`|
|------|------------|
|0.13  |0.2–0.4     |

## Credits
- [`bevy_tweening`](https://github.com/djeedai/bevy_tweening)

  The first crate I discovered and tried to do tweening with in Bevy.
  Their method of `Lens` is great and so it's present in this crate.
  Now called `Interpolator`. Usages may be similar but is
  implemented differently.

- [`godot`](https://github.com/godotengine/godot)

  Godot's tween make it simple to animate something which is the
  inspiration for this crate. The multi-entity architecture is mainly inspired by
  Godot's node child-parent hierarchy system and that most of the engine APIs
  utilizes this to define behavior.

## Contributions

Contributions are welcome!

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Your contributions
Unless you explicitly state otherwise, any contribution intentionally submitted for
inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual
licensed as above, without any additional terms or conditions.

<img src="https://github.com/Multirious/bevy_tween/assets/77918086/dbebd0c3-f4b0-432b-8778-70413a6dfa50" width="100%"/>

## Demos

A square will follow your circle with configurable animation.<br>
`cargo run --example follow -F bevy/bevy_winit`<br>

https://github.com/Multirious/bevy_tween/assets/77918086/d582c2de-0f54-4b22-be03-e3bff3348deb

---

Click left to spawn a circle. Hold right click to repetitively spawn a circle every frame.<br>
`cargo run --example click -F bevy/bevy_winit`<br>

https://github.com/Multirious/bevy_tween/assets/77918086/369abdec-32d0-482f-8f2d-b9bb8829ceca

---

Hold left click to increase the effect intensitiy.<br>
`cargo run --example hold -F bevy/bevy_winit`<br>

https://github.com/Multirious/bevy_tween/assets/77918086/33a297a6-19f2-4146-a906-1a88ff037ab3

---

Showcasing the tween event feature.<br>
`cargo run --example event -F bevy/bevy_winit`<br>

https://github.com/Multirious/bevy_tween/assets/77918086/593c9b64-6e7f-40bf-b0b7-29671f971e6e

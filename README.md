<img src="https://github.com/Multirious/bevy_tween/assets/77918086/38ab44e1-67a4-4c2d-b17c-3a35128e6930" width="100%"/>

[![Crates.io Version](https://img.shields.io/crates/v/bevy_tween?style=for-the-badge)](https://crates.io/crates/bevy_tween)
[![Crates.io License](https://img.shields.io/crates/l/bevy_tween?style=for-the-badge)](https://github.com/Multirious/bevy_tween/blob/main/README.md#license)
[![Docs.rs](https://img.shields.io/docsrs/bevy_tween?style=for-the-badge)](https://docs.rs/bevy_tween)

# `bevy_tween`

[Bevy](https://github.com/bevyengine/bevy) procedural and keyframe animation library.

This is a young plugin and APIs are to be fleshed out.
Breaking changes are to be expected!

See changelog [here](CHANGELOG.md).

## Features
- **Ergonomic and user-friendly API**: This crate provides functional animation
  framework, inspired by [nom](https://crates.io/crates/nom) and other functional
  languages, which allows for declarative animation.
  <details>
  <summary>Example</summary>

  ```rust
  let sprite_id = commands.spawn(SpriteBundle { ... }).id();
  let sprite = sprite_id.into_target();
  commands.animation()
      .insert(tween(
          Duration::from_secs(1),
          EaseKind::Linear,
          sprite.with(translation(pos0, pos1))
      ));
  ```

  Many functional techniques are available such as decoupling and composition.
  ```rust
  fn my_animation(
      target: TargetComponent,
      duration: Duration
  ) -> impl FnOnce(&mut AnimationCommands, &mut Duration) {
      parallel((
          tween(duration, EaseKind::QuadraticOut, target.with(translation(...))),
          tween(duration, EaseKind::QuadraticOut, target.with(rotation(...))),
      ))
  }

  let sprite_id = commands.spawn(Sprite { ... }).id();
  let sprite = sprite_id.into_target();
  commands.animation().insert(my_animation(sprite, Duration::from_secs(1)))
  ```

  </details>
- **Flexible and Extensible**: This crate is built on top of [`bevy_time_runner`](https://github.com/Multirious/bevy_time_runner)
  which mean we can extend this crate by adding *any* components and systems.
  - Tween anything from anywhere, built-in or custom system.
  - Interpolate with any curve, built-in or custom system.
  - Anything else.

- **Parallelism**: Tweens are typed and can be query over by their typed system
  which increase chances for system parallelism.
- **Rich timer control**:
  - Looping
  - Time scaling
  - Skip backward or forward
  - Jumping to anywhen

See [demos](#Demos)

Goals:
- [x] Flexible 🎉
- integration with other crates (?)
  - [ ] `bevy_animation`
  - [x] `bevy_lookup_curve`
- [ ] Editor. While the original goal for this crate is tweening from code,
       this crate absolutely has the capability to work on any complex animations.
       The editor will aid in such jobs.
  - Real-time display at any point in time in the animation.
  - Editing path from point A to point B with arbitary curve.

## Differences and how it works
This crate chose a different approach to storing tween data/input to solve flexibility issues present in
[`bevy_tweening`](https://github.com/djeedai/bevy_tweening) or [`bevy_easings`](https://github.com/vleue/bevy_easings),
like difficulty tweening in parallel or sequence of multiple types.

This approach offers:
- As mentioned above, capability to tween in parallel, sequence, or overlapping of any types.
- Extendability. And not just for tweening. For example, this crate adds a custom event
  plugin that can emit arbitrary events anytime in the animation by just building off the same system.

How it works:
1. By building on top of [`bevy_time_runner`](https://github.com/Multirious/bevy_time_runner), an animator is a tree of entities.
2. The parent is [`TimeRunner`](https://docs.rs/bevy_time_runner/latest/bevy_time_runner/struct.TimeRunner.html).
  This entity handles all the timing and progress of its children.
3. The children can be any combination of any components, such as tweens by this crate.
4. Systems are then run in a pipeline, communicating through various dependency-injected components.

## Feature gates
- Defaults
  - `bevy_asset`<br/>
     Add tweening systems for asset.
  - `bevy_render`<br/>
    Currently add nothing but required by the `bevy_sprite` feature.
  - `bevy_sprite`<br/>
    Add some built-in interpolators related to sprite.
  - `bevy_ui`<br/>
    Add some built-in interpolators related to ui.
- Optional
  - `bevy_lookup_curve`.<br/>
    Adds interpolation implementation using [`bevy_lookup_curve`](https://github.com/villor/bevy_lookup_curve).

## Bevy Version Support

|`bevy`|`bevy_tween`|
|------|------------|
|0.15  |0.7         |
|0.14  |0.6         |
|0.13  |0.2–0.5     |

## Credits
- [`bevy_tweening`](https://github.com/djeedai/bevy_tweening)

  The first crate that I've learned and improve this crate upon.

- [`godot`](https://github.com/godotengine/godot)

  There are one common pattern in Godot to uses Node as composition interface
  which inspired how [`bevy_time_runner`](https://github.com/Multirious/bevy_time_runner) works.

- [`nom`](https://crates.io/crates/nom)

  `nom` provides function parser combinator framework that inspried this crate's
  animation framework.

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

`cargo run --example follow -F bevy/bevy_winit`<br>
A square will follow your circle with configurable animation.<br>

https://github.com/Multirious/bevy_tween/assets/77918086/d582c2de-0f54-4b22-be03-e3bff3348deb

---

`cargo run --example click -F bevy/bevy_winit`<br>
Click left to spawn a circle. Hold right click to repetitively spawn a circle every frame.<br>

https://github.com/Multirious/bevy_tween/assets/77918086/fd0fe9d3-13a2-4261-880c-cc2609b875ba

---

`cargo run --example hold -F bevy/bevy_winit`<br>
Hold left click to increase the effect intensitiy.<br>

https://github.com/Multirious/bevy_tween/assets/77918086/33a297a6-19f2-4146-a906-1a88ff037ab3

---

`cargo run --example event -F bevy/bevy_winit`<br>
Showcasing the tween event feature.<br>

https://github.com/Multirious/bevy_tween/assets/77918086/9507c467-6428-4aed-bd00-511f05e6e951

---

`cargo run --example sprite_sheet -F bevy/bevy_winit`<br>
Sprite Sheet animation.<br>

https://github.com/Multirious/bevy_tween/assets/77918086/e3997b06-38e6-4add-85f5-a885b69c6687

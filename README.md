<img src="https://github.com/Multirious/bevy_tween/assets/77918086/38ab44e1-67a4-4c2d-b17c-3a35128e6930" width="100%"/>

# `bevy_tween`

WIP Flexible tweening plugin for Bevy.

Credits:
- [`bevy_tweening`](https://github.com/djeedai/bevy_tweening)
  The first crate I discovered and tried to do tweening with in Bevy. I like
  the idea of `TweenLens` of theirs. I decided to take that, expand on them,
  learn their potential and that I did, making lens even more
  capable and flexible. It's now called `Interpolator` in this crate because
  it's just that and it can be implemented to interpolate anything, not just
  a subset of some component.
- Godot
  Godot's tween make it simple to animate something which what I kept thinking
  about trying to do any animation. What's the big part is the Godot's node
  hierarchy system which utilize hierarchy of child-parent node to define
  behavior. It's an important puzzle piece of how this crate works.

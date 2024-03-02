# `bevy_tween`

Improvement over [`bevy_tweening`](https://github.com/djeedai/bevy_tweening).

I've been trying out `bevy_tweening` for a bit and the API is not flexible enough
for my usecase at all. This plugin has been made to solve that problem.

Tl;dr:
- Tighly integrated with Bevy's ECS system.
  The result is a much more flexible APIs.
- `TweenPlayer` component stores tween states.
- `*Tween` components store tween input (start, end, etc).
- `*Tween` components need to be stored in the same entity with `TweenPlayer` or is a child of one.

Documentation in progress.

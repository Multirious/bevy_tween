<img src="https://github.com/Multirious/bevy_tween/assets/77918086/623d2432-a2e4-4fcf-8b80-115bf850a042" width="100%"/>

# `bevy_tween`


WIP Flexible tweening plugin for Bevy.

Comparison to [`bevy_tweening`](https://github.com/djeedai/bevy_tweening):
- Differences:
  - Types much more tightly integrated with Bevy's ECS.
  - Most tweening related types are seperated into its own component.
- Issues solved:
  - Most types implement reflect and registered. Great for inspecting!
  - Advanced tween animations such as chaining and parallel uses child-parent hierarchy
    which means infinitely extendable functionality.
  - Everything is editable and decoupled.

Todo:
- Docs
- Tween builder
- Events and Callbackcs

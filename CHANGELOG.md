# Changelog

## Unreleased

### Changes

- Add interpolators for some UI components when using the `bevy_ui` feature.
    - `BackgroundColor` and `BorderColor`

## v0.5.0 - 2024-06-09

### Breaking changes

- Move span_tweener and tween_timer types to `bevy_time_runner`
- Remove `tween_timer` module and all types in it. Some types can be found in `bevy_time_runner`
- Remove `span_tween` module and all types in it
- Remove `"span_tween"` feature flag
- Remove `TweenSystemSet::TickTween `
- Remove `TweenSystemSet::Tweener`
- Replace `TargetComponent::Tweener*` with `TargetComponent::Marker` and `AnimationTarget`. Update default accordingly
- Update library to use types from `bevy_time_runner`
- Remove all types, method, and function related to tweener. Most is renamed and move to `bevy_time_runner`

All timing types is moved to `bevy_time_runner` including some changes.

- `Repeat`
- `RepeatStyle`
- `SpanTweener` is replaced with `TimeRunner`
- `AnimationDirection` is replaced with `TimeDirection`
- `SpanTweenerEnded` is replaced with `TimeRunnerEnded`
- `TweenTimeSpan` is replaced with `TimeSpan`
- ...And some more

### Fixes

- Fix tween systems error will flood the console

### Changes

- Supports `bevy_eventlistener` #16 by musjj
- Interpolation implementation for bevy_lookup_curve
- Update readme
- Update docs
- Improve lib docs
- Fix getting started code example. You're suppose to use `bevy_tween::prelude::*` not `bevy_tween::*`!
- Add curve text art to EaseFunction
- Implements combinator
- Implements state
- New animation builder and traits
- Add function constructor for interpolators
- Add `IntoTarget` trait
- pub use bevy_time_runner
- `TweenCorePlugin` adds `TimeRunnerPlugin` automatically if not exists
- Remove deprecated systems and types
- Add build.rs file to actually make CHANNEL_NIGHTLY cfg flag works
- Update all examples to account for new changes
- Add rustc_version to build dependencies
- Remove span_tween example
- Turn off format_code_in_doc_comments rust fmt config

## v0.4.0 - 2024-04-08

### Changes

- Add `SpanTweensBuilder::add` trait
- Add `SpanTweenPreset` trait
- Update examples to use the preset APIs.
- Documentations
- Add "Features" section to README.md
- Add "Contributions" section to README.md
- Add "Your contributions" section to README.md

## v0.3.1 - 2024-04-04

- Fix README.md

## v0.3.0 - 2024-04-03

### Breaking Changes

- Remove unnecessary generics from `TargetComponent` and `TargetResource`
- Add `app_resource: TweenAppResource` field to `TweenCorePlugin`
- All plugins and APIs that uses `PostUpdate` schedule is changed to use schedule from
  `TweenAppResource`
- Delegate `span_tweener_system()`'s ticking responsibility to `tick_span_tweener_system()`
- Remove `Eq` and `Hash` derives from `SpanTweener`, `Elasped`, and `TweenTimer`
- Remove `new()` from `Elasped`
- Remove `state: TweenState` field from SpanTweenBundle
- Remove `TweenState`
- Remove `TweenTarget` impl from `TargetComponent`, `TargetResource` and, `TargetAsset`
- Change `component_tween_system_full`, `resource_tween_system_full`, and `asset_tween_system_full`
  function signature to account for `SkipTween` component
- Remove `TickResult`
- Change `Elasped` struct definition
- Combine `repeat` and `repeat_style` in `TweenTimer` to just `repeat` then
  change corresponding methods.
- Change `TweenTimer::tick()` to accepts `f32` instead of `Duration`
- Change `TweenTimer::tick()` behavior to not update `previous` field in `Elasped`.
  `collaspe_elasped` will update the `previous` field instead.
- Change `Repeat` to use `i32` instead of `usize` and update their corresponding methods.

### Changes

- Add `TweenAppResource`
- Add `DefaultTweenEventsPlugin`
- Add `TweenEventData`
- Add `TweenEvent`
- Add `TweenTimer::set_tick`
- Add `TweenTimer::collaspe_elasped`
- Add `Repeat::advance_counter_by`
- Add `apply_component_tween_system`, `apply_resource_tween_system`,
  and `apply_asset_tween_system`
- Add `SkipTween`
- Add `SkipTweener`
- Add `TweenProgress` to replace `TweenState`
- Add `SpanTweensBuilderExt`
- Add `SpanTweensBuilder`
- Add `tick_span_tweener_system()`
- Add `SpanTweenerBundle::tween_here()`
- Add `SpanTweenHereBundle`
- Add `DefaultTweenEventsPlugin` to `DefaultTweenPlugins`
- Add `BoxedInterpolator` alias for `Box<dyn Interpolator>`
- Add impl `Interpolator` for `Arc<I>` and `dyn Fn`
- Register `EaseFunction` in `EaseFunctionPlugin`
- Register `TweenProgress` in `TweenCorePlugin`
- Add unit tests for `TweenTimer`
- Lots of documentations and code examples
- Remove `TweenTarget` and `Interpolator` trait requirement from `Tween<T, I>`
- Remove many `TweenTarget` requirement from `Tween<T, I>` implementations
- Improves `TweenTimer::tick()` code to account to new `Elasped`
- Improves `span_tweener_system` code to account to new `TweenTimer::tick()` behavior

### Fixes

- Fixed missing `AngleZ` tween system in `DefaultInterpolatorsPlugin`

### Deprecates

- Deprecate `QuickSpanTweenBundle`
- Deprecate `span_tween::span_tween()`
- Deprecate `ChildSpanTweenBuilder`
- Deprecate `ChildSpanTweenBuilderExt`
- Deprecate `WorldChildSpanTweenBuilder`
- Deprecate `WorldChildSpanTweenBuilderExt`
- Deprecate `TweenTarget`
- Deprecate `component_tween_system_full`
- Deprecate `resource_tween_system_full`
- Deprecate `asset_tween_system_full`
- Deprecate `Repeat::try_advance_counter`

## v0.2.0 - 2024-03-14

_First release!_

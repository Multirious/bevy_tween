# Changelog

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

### Adds
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

### Changes
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

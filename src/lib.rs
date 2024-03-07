//! # `bevy_tween`

// #![warn(missing_docs)]

use bevy::{app::PluginGroupBuilder, prelude::*};

mod utils;

pub mod interpolation;
pub mod lenses;
pub mod tween;
pub mod tween_player;

#[cfg(feature = "span_tween")]
pub mod span_tween;

/// Commonly used items
pub mod prelude {
    pub use std::time::Duration;

    pub use crate::interpolation::EaseFunction;
    pub use crate::lenses::{self, TweenLens};
    pub use crate::tween_player::{Repeat, RepeatStyle};
    pub use crate::DefaultTweenPlugins;

    #[cfg(all(feature = "bevy_asset", feature = "tween_unboxed"))]
    pub use crate::tween::AssetTween;
    #[cfg(feature = "tween_unboxed")]
    pub use crate::tween::ComponentTween;
    #[cfg(feature = "tween_unboxed")]
    pub use crate::tween::ResourceTween;

    #[cfg(all(feature = "tween_boxed", feature = "bevy_asset"))]
    pub use crate::tween::AssetTweenBoxed;
    #[cfg(feature = "tween_boxed")]
    pub use crate::tween::ComponentTweenBoxed;
    #[cfg(feature = "tween_boxed")]
    pub use crate::tween::ResourceTweenBoxed;

    #[cfg(feature = "span_tween")]
    pub use crate::span_tween::{
        BuildSpanTweens, SpanTweenBundle, SpanTweenPlayerBundle,
    };
}

/// Default plugins for using crate.
pub struct DefaultTweenPlugins;
impl PluginGroup for DefaultTweenPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let p = PluginGroupBuilder::start::<DefaultTweenPlugins>()
            .add(TweenCorePlugin)
            .add(lenses::DefaultTweenLensesPlugin)
            .add(interpolation::EaseFunctionPlugin);
        #[cfg(feature = "span_tween")]
        let p = p.add(span_tween::SpanTweenPlugin);
        p
    }
}

/// Core, necessary types, and configurations you need to get started with
/// this plugin
pub struct TweenCorePlugin;
impl Plugin for TweenCorePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                TweenSystemSet::TickTweenPlayer,
                TweenSystemSet::TweenPlayer,
                TweenSystemSet::UpdateTweenEaseValue,
                TweenSystemSet::ApplyTween,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (tween_player::tick_tween_player_state_system,)
                .in_set(TweenSystemSet::TickTweenPlayer),
        )
        // .add_event::<tween_player::TweenPlayerEnded>()
        .register_type::<tween_player::TweenPlayerState>()
        .register_type::<tween_player::AnimationDirection>()
        .register_type::<tween_player::Repeat>()
        .register_type::<tween_player::RepeatStyle>()
        .register_type::<tween::TweenState>()
        .register_type::<tween::TweenInterpolationValue>();
    }
}

/// Enum of SystemSet in this crate
/// After adding the plugin [`TweenCorePlugin`], these set will be configured
/// to run in the [`PreUpdate`] schedule so any modification you've done after
/// this schedule should be correctly applied in the next frame.
///
/// The sets should be configured to run in this order:
///  1. TickTweenPlayer
///  2. TweenPlayer
///  3. UpdateTweenEaseValue
///  4. ApplyTween
#[derive(Debug, SystemSet, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TweenSystemSet {
    /// This set is for systems that responsible for updating [`TweenPlayerState`]'s
    /// elasped.
    ///
    /// [`TweenPlayerState`]: tween_player::TweenPlayerState
    TickTweenPlayer,
    /// This set is for systems that responsible for updating any specific
    /// tween player implementation such as the [`span_tween::span_tween_player_system`]
    /// by this crate
    TweenPlayer,
    /// This set is for systems that responsible for updating any
    /// [`tween::TweenEaseValue`] such as
    /// [`interpolation::sample_interpolator_system`] by this crate.
    UpdateTweenEaseValue,
    /// This set is for systems that responsible for actually executing any
    /// active tween and setting the value to its respective tweening item such
    /// as these systems by this crate:
    /// - [`tween::component_tween_system`]
    /// - [`tween::component_tween_boxed_system`]
    /// - [`tween::resource_tween_system`]
    /// - [`tween::resource_tween_boxed_system`]
    /// - [`tween::asset_tween_system`]
    /// - [`tween::asset_tween_boxed_system`]
    /// - [`tween::asset_tween_boxed_system`]
    ApplyTween,
}

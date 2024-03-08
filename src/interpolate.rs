//! Module containg the [`Interpolator`] trait and some basic built-in interpolator

use bevy::prelude::*;

#[cfg(feature = "bevy_sprite")]
use crate::utils::color_lerp;

/// [`Interpolator`] is used to specify how to interpolate an [`Self::Item`] by the
/// implementor.
pub trait Interpolator {
    /// Type to be interpolated.
    type Item;
    /// Interpolate an item using `value` which is typically between 0 and 1.
    /// The value should be already sampled from the [`Interpolation`]
    ///
    /// [`Interpolation`]: crate::interpolation::Interpolation
    fn interpolate(&self, item: &mut Self::Item, value: f32);
}

impl<I> Interpolator for Box<dyn Fn(&mut I, f32) + Send + Sync + 'static> {
    type Item = I;

    fn interpolate(&self, item: &mut Self::Item, value: f32) {
        self(item, value)
    }
}

impl<I> Interpolator for fn(&mut I, f32) {
    type Item = I;

    fn interpolate(&self, item: &mut Self::Item, value: f32) {
        self(item, value)
    }
}

/// Default interpolators
pub struct DefaultInterpolatorsPlugin;
impl Plugin for DefaultInterpolatorsPlugin {
    #[cfg(any(feature = "tween_static", feature = "tween_dyn",))]
    fn build(&self, app: &mut App) {
        use crate::{tween, TweenSystemSet};

        #[cfg(feature = "tween_static")]
        app.add_systems(
            Update,
            (
                tween::component_tween_system::<Translation>,
                tween::component_tween_system::<Rotation>,
                tween::component_tween_system::<Scale>,
            )
                .in_set(TweenSystemSet::ApplyTween),
        )
        .register_type::<tween::ComponentTween<Translation>>()
        .register_type::<tween::ComponentTween<Rotation>>()
        .register_type::<tween::ComponentTween<Scale>>();

        #[cfg(feature = "tween_dyn")]
        app.add_systems(
            Update,
            tween::component_tween_dyn_system::<Transform>
                .in_set(TweenSystemSet::ApplyTween),
        );

        #[cfg(all(feature = "bevy_sprite", feature = "tween_static"))]
        app.add_systems(
            Update,
            (tween::component_tween_system::<SpriteColor>,)
                .in_set(TweenSystemSet::ApplyTween),
        )
        .register_type::<tween::ComponentTween<SpriteColor>>();

        #[cfg(all(feature = "bevy_sprite", feature = "tween_dyn"))]
        app.add_systems(
            Update,
            tween::component_tween_dyn_system::<Sprite>
                .in_set(TweenSystemSet::ApplyTween),
        );

        #[cfg(all(
            feature = "bevy_sprite",
            feature = "bevy_asset",
            feature = "tween_static"
        ))]
        app.add_systems(
            Update,
            (tween::asset_tween_system::<ColorMaterial>,)
                .in_set(TweenSystemSet::ApplyTween),
        )
        .register_type::<tween::AssetTween<ColorMaterial>>();

        #[cfg(all(
            feature = "bevy_sprite",
            feature = "bevy_asset",
            feature = "tween_dyn"
        ))]
        app.add_systems(
            Update,
            tween::asset_tween_dyn_system::<bevy::sprite::ColorMaterial>
                .in_set(TweenSystemSet::ApplyTween),
        );
    }
    #[cfg(not(any(feature = "tween_static", feature = "tween_dyn",)))]
    fn build(&self, _app: &mut App) {
        panic!("This plugin is empty without any feature flag! Enable either the feature \"tween_static\" or \"tween_dyn\".")
    }
}

/// [`Interpolator`] for [`Transform`]'s translation.
#[derive(Debug, Default, Clone, PartialEq, Reflect)]
pub struct Translation {
    #[allow(missing_docs)]
    pub start: Vec3,
    #[allow(missing_docs)]
    pub end: Vec3,
}
impl Interpolator for Translation {
    type Item = Transform;

    fn interpolate(&self, item: &mut Self::Item, value: f32) {
        item.translation = self.start.lerp(self.end, value);
    }
}

/// [`Interpolator`] for [`Transform`]'s rotation using the [`Quat::slerp`] function.
#[derive(Debug, Default, Clone, PartialEq, Reflect)]
pub struct Rotation {
    #[allow(missing_docs)]
    pub start: Quat,
    #[allow(missing_docs)]
    pub end: Quat,
}
impl Interpolator for Rotation {
    type Item = Transform;

    fn interpolate(&self, item: &mut Self::Item, value: f32) {
        item.rotation = self.start.slerp(self.end, value);
    }
}

/// [`Interpolator`] for [`Transform`]'s scale
#[derive(Debug, Default, Clone, PartialEq, Reflect)]
pub struct Scale {
    #[allow(missing_docs)]
    pub start: Vec3,
    #[allow(missing_docs)]
    pub end: Vec3,
}
impl Interpolator for Scale {
    type Item = Transform;

    fn interpolate(&self, item: &mut Self::Item, value: f32) {
        item.scale = self.start.lerp(self.end, value);
    }
}

/// [`Interpolator`] for [`Sprite`]'s color
#[cfg(feature = "bevy_sprite")]
#[derive(Debug, Default, Clone, PartialEq, Reflect)]
pub struct SpriteColor {
    #[allow(missing_docs)]
    pub start: Color,
    #[allow(missing_docs)]
    pub end: Color,
}

#[cfg(feature = "bevy_sprite")]
impl Interpolator for SpriteColor {
    type Item = Sprite;

    fn interpolate(&self, item: &mut Self::Item, value: f32) {
        item.color = color_lerp(self.start, self.end, value)
    }
}

/// [`Interpolator`] for [`Sprite`]'s [`ColorMaterial`]
#[cfg(feature = "bevy_sprite")]
#[derive(Debug, Default, Clone, PartialEq, Reflect)]
pub struct ColorMaterial {
    #[allow(missing_docs)]
    pub start: Color,
    #[allow(missing_docs)]
    pub end: Color,
}

#[cfg(feature = "bevy_sprite")]
impl Interpolator for ColorMaterial {
    type Item = bevy::sprite::ColorMaterial;

    fn interpolate(&self, item: &mut Self::Item, value: f32) {
        item.color = color_lerp(self.start, self.end, value);
    }
}

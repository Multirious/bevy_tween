//! Module containg the [`TweenLens`] trait and some basic built-in lens
//!
//! # TweenLens
//!
//! [`TweenLens`] in this crate will be used to specify *how* an `item` will be
//! interpolated. Which also could be anything. This crate has built-in supports
//! for tweening component, resource, and asset.

use bevy::prelude::*;

#[cfg(feature = "bevy_sprite")]
use crate::utils::color_lerp;

/// [`TweenLens`] is used to specify how to interpolate an [`Self::Item`] by the
/// implementor.
pub trait TweenLens {
    /// Type to be interpolated.
    type Item;
    /// Interpolate an item using `value` which is typically between 0 and 1.
    /// The value should be already sampled from the [`Interpolation`]
    ///
    /// [`Interpolation`]: crate::interpolation::Interpolation
    fn interpolate(&self, item: &mut Self::Item, value: f32);
}

impl<I> TweenLens for Box<dyn Fn(&mut I, f32) + Send + Sync + 'static> {
    type Item = I;

    fn interpolate(&self, item: &mut Self::Item, value: f32) {
        self(item, value)
    }
}

impl<I> TweenLens for fn(&mut I, f32) {
    type Item = I;

    fn interpolate(&self, item: &mut Self::Item, value: f32) {
        self(item, value)
    }
}

/// Default lenses
pub struct DefaultTweenLensesPlugin;
impl Plugin for DefaultTweenLensesPlugin {
    #[cfg(any(feature = "tween_unboxed", feature = "tween_boxed",))]
    fn build(&self, app: &mut App) {
        use crate::{tween, TweenSystemSet};

        #[cfg(feature = "tween_unboxed")]
        app.add_systems(
            Update,
            (
                tween::component_tween_system::<TransformTranslationLens>,
                tween::component_tween_system::<TransformRotationLens>,
                tween::component_tween_system::<TransformScaleLens>,
            )
                .in_set(TweenSystemSet::ApplyTween),
        )
        .register_type::<tween::ComponentTween<TransformTranslationLens>>()
        .register_type::<tween::ComponentTween<TransformRotationLens>>()
        .register_type::<tween::ComponentTween<TransformScaleLens>>();

        #[cfg(feature = "tween_boxed")]
        app.add_systems(
            Update,
            tween::component_tween_boxed_system::<Transform>
                .in_set(TweenSystemSet::ApplyTween),
        );

        #[cfg(all(feature = "bevy_sprite", feature = "tween_unboxed"))]
        app.add_systems(
            Update,
            (tween::component_tween_system::<SpriteColorLens>,)
                .in_set(TweenSystemSet::ApplyTween),
        )
        .register_type::<tween::ComponentTween<SpriteColorLens>>();

        #[cfg(all(feature = "bevy_sprite", feature = "tween_boxed"))]
        app.add_systems(
            Update,
            tween::component_tween_boxed_system::<Sprite>
                .in_set(TweenSystemSet::ApplyTween),
        );

        #[cfg(all(
            feature = "bevy_sprite",
            feature = "bevy_asset",
            feature = "tween_unboxed"
        ))]
        app.add_systems(
            Update,
            (tween::asset_tween_system::<ColorMaterialLens>,)
                .in_set(TweenSystemSet::ApplyTween),
        )
        .register_type::<tween::AssetTween<ColorMaterialLens>>();

        #[cfg(all(
            feature = "bevy_sprite",
            feature = "bevy_asset",
            feature = "tween_boxed"
        ))]
        app.add_systems(
            Update,
            tween::asset_tween_boxed_system::<ColorMaterial>
                .in_set(TweenSystemSet::ApplyTween),
        );
    }
    #[cfg(not(any(feature = "tween_unboxed", feature = "tween_boxed",)))]
    fn build(&self, _app: &mut App) {
        panic!("This plugin is empty without any feature flag! Enable either the feature \"tween_unboxed\" or \"tween_boxed\".")
    }
}

/// [`TweenLens`] for [`Transform`]'s translation.
#[derive(Debug, Default, Clone, PartialEq, Reflect)]
pub struct TransformTranslationLens {
    #[allow(missing_docs)]
    pub start: Vec3,
    #[allow(missing_docs)]
    pub end: Vec3,
}
impl TweenLens for TransformTranslationLens {
    type Item = Transform;

    fn interpolate(&self, item: &mut Self::Item, value: f32) {
        item.translation = self.start.lerp(self.end, value);
    }
}

/// [`TweenLens`] for [`Transform`]'s rotation using the [`Quat::slerp`] function.
#[derive(Debug, Default, Clone, PartialEq, Reflect)]
pub struct TransformRotationLens {
    #[allow(missing_docs)]
    pub start: Quat,
    #[allow(missing_docs)]
    pub end: Quat,
}
impl TweenLens for TransformRotationLens {
    type Item = Transform;

    fn interpolate(&self, item: &mut Self::Item, value: f32) {
        item.rotation = self.start.slerp(self.end, value);
    }
}

/// [`TweenLens`] for [`Transform`]'s scale
#[derive(Debug, Default, Clone, PartialEq, Reflect)]
pub struct TransformScaleLens {
    #[allow(missing_docs)]
    pub start: Vec3,
    #[allow(missing_docs)]
    pub end: Vec3,
}
impl TweenLens for TransformScaleLens {
    type Item = Transform;

    fn interpolate(&self, item: &mut Self::Item, value: f32) {
        item.scale = self.start.lerp(self.end, value);
    }
}

/// [`TweenLens`] for [`Sprite`]'s color
#[cfg(feature = "bevy_sprite")]
#[derive(Debug, Default, Clone, PartialEq, Reflect)]
pub struct SpriteColorLens {
    #[allow(missing_docs)]
    pub start: Color,
    #[allow(missing_docs)]
    pub end: Color,
}

#[cfg(feature = "bevy_sprite")]
impl TweenLens for SpriteColorLens {
    type Item = Sprite;

    fn interpolate(&self, item: &mut Self::Item, value: f32) {
        item.color = color_lerp(self.start, self.end, value)
    }
}

/// [`TweenLens`] for [`Sprite`]'s [`ColorMaterial`]
#[cfg(feature = "bevy_sprite")]
#[derive(Debug, Default, Clone, PartialEq, Reflect)]
pub struct ColorMaterialLens {
    #[allow(missing_docs)]
    pub start: Color,
    #[allow(missing_docs)]
    pub end: Color,
}

#[cfg(feature = "bevy_sprite")]
impl TweenLens for ColorMaterialLens {
    type Item = ColorMaterial;

    fn interpolate(&self, item: &mut Self::Item, value: f32) {
        item.color = color_lerp(self.start, self.end, value);
    }
}

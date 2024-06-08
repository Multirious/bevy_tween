//! Module containing some basic built-in interpolator
//!
//! **Plugins**:
//! - [`DefaultDynInterpolatorsPlugin`]
//! - [`DefaultInterpolatorsPlugin`]
//!
//! **Built-in interpolators**:
//! - [`Translation`]
//! - [`Rotation`]
//! - [`Scale`]
//! - [`AngleZ`]
//! - [`SpriteColor`]
//! - [`ColorMaterial`]
//!
//! # Your own [`Interpolator`]
//!
//! There are a few amount of built-in interpolator because this crate only
//! implemented the most common ones such as [`Translation`] or
//! [`SpriteColor`] and some more.
//! For others, you must implement your own!
//!
//! Let's say you've created some custom component and you want to interpolate it:
//! ```no_run
//! use bevy::prelude::*;
//!
//! #[derive(Component)]
//! struct Foo(f32);
//! ```
//!
//! You'll need to create a specific interpolator for this component by:
//! ```no_run
//! # use bevy::prelude::*;
//! # #[derive(Component)]
//! # struct Foo(f32);
//! use bevy_tween::prelude::*;
//!
//! // First we define an interpolator type for `Foo`.
//! struct InterpolateFoo {
//!     start: f32,
//!     end: f32,
//! }
//!
//! impl Interpolator for InterpolateFoo {
//!     // We define the asscioate type `Item` as the `Foo` component
//!     type Item = Foo;
//!
//!     // Then we define how we want to interpolate `Foo`
//!     fn interpolate(&self, item: &mut Self::Item, value: f32) {
//!         // Usually if the type already have the `.lerp` function provided
//!         // by the `FloatExt` trait then we can use just that
//!         item.0 = self.start.lerp(self.end, value);
//!     }
//! }
//! ```
//!
//! If you've created a custom interpolator or a custom component/asset/resource,
//! you may want to [register some systems](crate::tween#registering-systems).
//!
//! While it's recommended to use the [`Interpolator`] trait, it's not required
//! to make your interpolators work in this crate. the [`Interpolator`] as of
//! currently is only used for registering built-in simple interpolator systems
//! such as [`component_tween_system`], [`resource_tween_system`], and
//! [`asset_tween_system`]. Its next use is being object-safe for dynamic interpolator.
//!
//! If you need interpolators with more specific or complex system param, you
//! have to define your own system!
//!
//! [`component_tween_system`]: crate::tween::component_tween_system
//! [`resource_tween_system`]: crate::tween::resource_tween_system
//! [`asset_tween_system`]: crate::tween::asset_tween_system

use std::sync::Arc;

use bevy::prelude::*;

#[cfg(feature = "bevy_sprite")]
use crate::utils::color_lerp;
use crate::{tween, BevyTweenRegisterSystems};

/// Alias for an `Interpolator` as a boxed trait object.
pub type BoxedInterpolator<Item> = Box<dyn Interpolator<Item = Item>>;

type InterpolatorClosure<I> = Box<dyn Fn(&mut I, f32) + Send + Sync + 'static>;

/// Create boxed closure in order to be used with dynamic [`Interpolator`]
pub fn closure<I, F>(f: F) -> InterpolatorClosure<I>
where
    I: 'static,
    F: Fn(&mut I, f32) + Send + Sync + 'static,
{
    Box::new(f)
}

/// [`Interpolator`] is used to specify how to interpolate an [`Self::Item`] by the
/// implementor.
///
/// Currently only used for registering systems
/// and being object-safe for dyanmic interpolator.
///
/// See [module-level documentation](self) for more info.
pub trait Interpolator: Send + Sync + 'static {
    /// Type to be interpolated.
    type Item;
    /// Interpolate an item using `value` which is typically between 0–1.
    /// The value should be already sampled from an [`Interpolation`]
    ///
    /// [`Interpolation`]: crate::interpolation::Interpolation
    fn interpolate(&self, item: &mut Self::Item, value: f32);
}

// /// Reflect [`Interpolator`] trait
// #[allow(clippy::type_complexity)]
// pub struct ReflectInterpolator<Item> {
//     get_func: fn(&dyn Reflect) -> Option<&dyn Interpolator<Item = Item>>,
//     get_mut_func:
//         fn(&mut dyn Reflect) -> Option<&mut dyn Interpolator<Item = Item>>,
//     get_boxed_func:
//         fn(
//             Box<dyn Reflect>,
//         )
//             -> Result<Box<dyn Interpolator<Item = Item>>, Box<dyn Reflect>>,
// }

// impl<Item> Clone for ReflectInterpolator<Item> {
//     #[inline]
//     fn clone(&self) -> ReflectInterpolator<Item> {
//         ReflectInterpolator {
//             get_func: Clone::clone(&self.get_func),
//             get_mut_func: Clone::clone(&self.get_mut_func),
//             get_boxed_func: Clone::clone(&self.get_boxed_func),
//         }
//     }
// }
// impl<Item> ReflectInterpolator<Item> {
//     /** Downcast a `&dyn Reflect` type to `&dyn Interpolator`.

//     If the type cannot be downcast, `None` is returned.*/
//     pub fn get<'a>(
//         &self,
//         reflect_value: &'a dyn Reflect,
//     ) -> Option<&'a dyn Interpolator<Item = Item>> {
//         (self.get_func)(reflect_value)
//     }

//     // /** Downcast a `&mut dyn Reflect` type to `&mut dyn Interpolator`.

//     // If the type cannot be downcast, `None` is returned.*/
//     // pub fn get_mut<'a>(
//     //     &self,
//     //     reflect_value: &'a mut dyn Reflect,
//     // ) -> Option<&'a mut dyn Interpolator<Item = Item>> {
//     //     (self.get_mut_func)(reflect_value)
//     // }

//     /** Downcast a `Box<dyn Reflect>` type to `Box<dyn Interpolator>`.

//     If the type cannot be downcast, this will return `Err(Box<dyn Reflect>)`.*/
//     pub fn get_boxed(
//         &self,
//         reflect_value: Box<dyn Reflect>,
//     ) -> Result<Box<dyn Interpolator<Item = Item>>, Box<dyn Reflect>> {
//         (self.get_boxed_func)(reflect_value)
//     }
// }

// impl<Item, T> bevy::reflect::FromType<T> for ReflectInterpolator<Item>
// where
//     T: Interpolator<Item = Item> + Reflect,
// {
//     fn from_type() -> Self {
//         Self {
//             get_func: |reflect_value| {
//                 <dyn Reflect>::downcast_ref::<T>(reflect_value)
//                     .map(|value| value as &dyn Interpolator<Item = Item>)
//             },
//             get_mut_func: |reflect_value| {
//                 <dyn Reflect>::downcast_mut::<T>(reflect_value)
//                     .map(|value| value as &mut dyn Interpolator<Item = Item>)
//             },
//             get_boxed_func: |reflect_value| {
//                 <dyn Reflect>::downcast::<T>(reflect_value)
//                     .map(|value| value as Box<dyn Interpolator<Item = Item>>)
//             },
//         }
//     }
// }

impl<I> Interpolator for Box<I>
where
    I: Interpolator + ?Sized,
{
    type Item = I::Item;

    fn interpolate(&self, item: &mut Self::Item, value: f32) {
        (**self).interpolate(item, value)
    }
}

impl<I> Interpolator for &'static I
where
    I: Interpolator + ?Sized,
{
    type Item = I::Item;

    fn interpolate(&self, item: &mut Self::Item, value: f32) {
        (**self).interpolate(item, value)
    }
}

impl<I> Interpolator for Arc<I>
where
    I: Interpolator + ?Sized,
{
    type Item = I::Item;

    fn interpolate(&self, item: &mut Self::Item, value: f32) {
        (**self).interpolate(item, value)
    }
}

impl<I: 'static> Interpolator for dyn Fn(&mut I, f32) + Send + Sync + 'static {
    type Item = I;

    fn interpolate(&self, item: &mut Self::Item, value: f32) {
        self(item, value)
    }
}

/// Default interpolators
///
/// Register type and systems for the following interpolators:
/// - [`Translation`]
/// - [`Rotation`]
/// - [`Scale`]
/// - [`AngleZ`]
/// - [`SpriteColor`] if `"bevy_sprite"` feature is enabled.
/// - [`ColorMaterial`] if `"bevy_sprite"` feature is enabled.
pub struct DefaultInterpolatorsPlugin;
impl Plugin for DefaultInterpolatorsPlugin {
    /// # Panics
    ///
    /// Panics if [`TweenAppResource`] does not exist in world.
    ///
    /// [`TweenAppResource`]: crate::TweenAppResource
    fn build(&self, app: &mut App) {
        app.add_tween_systems((
            tween::component_tween_system::<Translation>(),
            tween::component_tween_system::<Rotation>(),
            tween::component_tween_system::<Scale>(),
            tween::component_tween_system::<AngleZ>(),
        ))
        .register_type::<tween::ComponentTween<Translation>>()
        .register_type::<tween::ComponentTween<Rotation>>()
        .register_type::<tween::ComponentTween<Scale>>()
        .register_type::<tween::ComponentTween<AngleZ>>();

        #[cfg(feature = "bevy_sprite")]
        app.add_tween_systems(tween::component_tween_system::<SpriteColor>())
            .register_type::<tween::ComponentTween<SpriteColor>>();

        #[cfg(all(feature = "bevy_sprite", feature = "bevy_asset",))]
        app.add_tween_systems(tween::asset_tween_system::<ColorMaterial>())
            .register_type::<tween::AssetTween<ColorMaterial>>();
    }
}

/// Default dynamic interpolators
///
/// Register systems for the following:
/// - [`Transform`] component.
/// - [`Sprite`] component if `"bevy_sprite"` feature is enabled.
/// - [`ColorMaterial`] asset if `"bevy_sprite"` feature is enabled.
///
/// [`ColorMaterial`]: bevy::sprite::ColorMaterial
pub struct DefaultDynInterpolatorsPlugin;
impl Plugin for DefaultDynInterpolatorsPlugin {
    /// # Panics
    ///
    /// Panics if [`TweenAppResource`] does not exist in world.
    ///
    /// [`TweenAppResource`]: crate::TweenAppResource
    fn build(&self, app: &mut App) {
        app.add_tween_systems(tween::component_tween_system::<
            BoxedInterpolator<Transform>,
        >());

        #[cfg(feature = "bevy_sprite")]
        app.add_tween_systems(tween::component_tween_system::<
            BoxedInterpolator<Sprite>,
        >());

        #[cfg(all(feature = "bevy_sprite", feature = "bevy_asset",))]
        app.add_tween_systems(tween::asset_tween_system::<
            BoxedInterpolator<bevy::sprite::ColorMaterial>,
        >());
    }
}

// type ReflectInterpolatorTransform = ReflectInterpolator<Transform>;

/// [`Interpolator`] for [`Transform`]'s translation.
#[derive(Debug, Default, Clone, PartialEq, Reflect)]
// #[reflect(InterpolatorTransform)]
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

/// Constructor for [`Translation`]
pub fn translation(start: Vec3, end: Vec3) -> Translation {
    Translation { start, end }
}

/// Constructor for [`Translation`] that's relative to previous value using currying.
pub fn translation_to(to: Vec3) -> impl Fn(&mut Vec3) -> Translation {
    move |state| {
        let start = *state;
        let end = to;
        *state = to;
        translation(start, end)
    }
}

/// Constructor for [`Translation`] that's relative to previous value using currying.
pub fn translation_by(by: Vec3) -> impl Fn(&mut Vec3) -> Translation {
    move |state| {
        let start = *state;
        let end = *state + by;
        *state += by;
        translation(start, end)
    }
}

/// [`Interpolator`] for [`Transform`]'s rotation using the [`Quat::slerp`] function.
#[derive(Debug, Default, Clone, PartialEq, Reflect)]
// #[reflect(InterpolatorTransform)]
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

/// Constructor for [`Rotation`]
pub fn rotation(start: Quat, end: Quat) -> Rotation {
    Rotation { start, end }
}

/// Constructor for [`Rotation`] that's relative to previous value using currying.
pub fn rotation_to(to: Quat) -> impl Fn(&mut Quat) -> Rotation {
    move |state| {
        let start = *state;
        let end = to;
        *state = to;
        rotation(start, end)
    }
}

/// Constructor for [`Rotation`] that's relative to previous value using currying.
pub fn rotation_by(by: Quat) -> impl Fn(&mut Quat) -> Rotation {
    move |state| {
        let start = *state;
        let end = *state + by;
        *state = state.mul_quat(by);
        rotation(start, end)
    }
}

/// [`Interpolator`] for [`Transform`]'s scale
#[derive(Debug, Default, Clone, PartialEq, Reflect)]
// #[reflect(InterpolatorTransform)]
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

/// Constructor for [`Scale`]
pub fn scale(start: Vec3, end: Vec3) -> Scale {
    Scale { start, end }
}

/// Constructor for [`Scale`] that's relative to previous value using currying.
pub fn scale_to(to: Vec3) -> impl Fn(&mut Vec3) -> Scale {
    move |state| {
        let start = *state;
        let end = to;
        *state = to;
        scale(start, end)
    }
}

/// Constructor for [`Scale`] that's relative to previous value using currying.
pub fn scale_by(by: Vec3) -> impl Fn(&mut Vec3) -> Scale {
    move |state| {
        let start = *state;
        let end = *state + by;
        *state += by;
        scale(start, end)
    }
}

/// [`Interpolator`] for [`Transform`]'s rotation at Z axis.
/// Usually used for 2D rotation.
#[derive(Debug, Default, Clone, PartialEq, Reflect)]
// #[reflect(InterpolatorTransform)]
pub struct AngleZ {
    #[allow(missing_docs)]
    pub start: f32,
    #[allow(missing_docs)]
    pub end: f32,
}
impl Interpolator for AngleZ {
    type Item = Transform;

    fn interpolate(&self, item: &mut Self::Item, value: f32) {
        let angle = (self.end - self.start).mul_add(value, self.start);
        item.rotation = Quat::from_rotation_z(angle);
    }
}

/// Constructor for [`AngleZ`]
pub fn angle_z(start: f32, end: f32) -> AngleZ {
    AngleZ { start, end }
}

/// Constructor for [`AngleZ`] that's relative to previous value using currying.
pub fn angle_z_to(to: f32) -> impl Fn(&mut f32) -> AngleZ {
    move |state| {
        let start = *state;
        let end = to;
        *state = to;
        angle_z(start, end)
    }
}

/// Constructor for [`AngleZ`] that's relative to previous value using currying.
pub fn angle_z_by(by: f32) -> impl Fn(&mut f32) -> AngleZ {
    move |state| {
        let start = *state;
        let end = *state + by;
        *state += by;
        angle_z(start, end)
    }
}

// #[cfg(feature = "bevy_sprite")]
// type ReflectInterpolatorSprite = ReflectInterpolator<Sprite>;

/// [`Interpolator`] for [`Sprite`]'s color
#[cfg(feature = "bevy_sprite")]
#[derive(Debug, Default, Clone, PartialEq, Reflect)]
// #[reflect(InterpolatorSprite)]
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

/// Constructor for [`SpriteColor`]
#[cfg(feature = "bevy_sprite")]
pub fn sprite_color(start: Color, end: Color) -> SpriteColor {
    SpriteColor { start, end }
}

/// Constructor for [`SpriteColor`] that's relative to previous value using currying.
#[cfg(feature = "bevy_sprite")]
pub fn sprite_color_to(to: Color) -> impl Fn(&mut Color) -> SpriteColor {
    move |state| {
        let start = *state;
        let end = to;
        *state = to;
        sprite_color(start, end)
    }
}

// #[cfg(feature = "bevy_sprite")]
// type ReflectInterpolatorColorMaterial =
//     ReflectInterpolator<bevy::sprite::ColorMaterial>;

/// [`Interpolator`] for [`Sprite`]'s [`ColorMaterial`]
#[cfg(feature = "bevy_sprite")]
#[derive(Debug, Default, Clone, PartialEq, Reflect)]
// #[reflect(InterpolatorColorMaterial)]
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

/// Constructor for [`ColorMaterial`]
#[cfg(feature = "bevy_sprite")]
pub fn color_material(start: Color, end: Color) -> ColorMaterial {
    ColorMaterial { start, end }
}

/// Constructor for [`ColorMaterial`] that's relative to previous value using currying.
#[cfg(feature = "bevy_sprite")]
pub fn color_material_to(to: Color) -> impl Fn(&mut Color) -> ColorMaterial {
    move |state| {
        let start = *state;
        let end = to;
        *state = to;
        color_material(start, end)
    }
}

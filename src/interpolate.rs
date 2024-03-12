//! Module containg the [`Interpolator`] trait and some basic built-in interpolator

use bevy::prelude::*;

#[cfg(feature = "bevy_sprite")]
use crate::utils::color_lerp;
use crate::{tween, BevyTweenRegisterSystems};

/// [`Interpolator`] is used to specify how to interpolate an [`Self::Item`] by the
/// implementor.
///
/// # Examples
///
/// Interpolator for components.
/// ```
/// use bevy::prelude::*;
/// use bevy_tween::prelude::*;
///
/// #[derive(Component)]
/// struct MyComponent(f32);
///
/// struct InterpolateMyComponent {
///     start: f32,
///     end: f32,
/// }
///
/// impl Interpolator for InterpolateMyComponent {
///     type Item = MyComponent;
///
///     fn interpolate(&self, item: &mut Self::Item, value: f32) {
///         item.0 = self.start.lerp(self.end, value);
///     }
/// }
/// ```
///
/// Interpolator for asset.
/// ```
/// use bevy::prelude::*;
/// use bevy_tween::prelude::*;
///
/// #[derive(TypePath, Asset)]
/// struct MyAsset(f32);
///
/// struct InterpolateMyAsset {
///     start: f32,
///     end: f32,
/// }
///
/// impl Interpolator for InterpolateMyAsset {
///     type Item = MyAsset;
///
///     fn interpolate(&self, item: &mut Self::Item, value: f32) {
///         item.0 = self.start.lerp(self.end, value);
///     }
/// }
/// ```
///
/// Interpolator for resource.
/// ```
/// use bevy::prelude::*;
/// use bevy_tween::prelude::*;
///
/// #[derive(Resource)]
/// struct MyResource(f32);
///
/// struct InterpolateMyResource {
///     start: f32,
///     end: f32,
/// }
///
/// impl Interpolator for InterpolateMyResource {
///     type Item = MyResource;
///
///     fn interpolate(&self, item: &mut Self::Item, value: f32) {
///         item.0 = self.start.lerp(self.end, value);
///     }
/// }
/// ```
pub trait Interpolator: Send + Sync + 'static {
    /// Type to be interpolated.
    type Item;
    /// Interpolate an item using `value` which is typically between 0 and 1.
    /// The value should be already sampled from the [`Interpolation`]
    ///
    /// [`Interpolation`]: crate::interpolation::Interpolation
    fn interpolate(&self, item: &mut Self::Item, value: f32);
}

impl<I: 'static> Interpolator
    for Box<dyn Fn(&mut I, f32) + Send + Sync + 'static>
{
    type Item = I;

    fn interpolate(&self, item: &mut Self::Item, value: f32) {
        self(item, value)
    }
}

impl<I: 'static> Interpolator for Box<dyn Interpolator<Item = I>> {
    type Item = I;

    fn interpolate(&self, item: &mut Self::Item, value: f32) {
        (**self).interpolate(item, value)
    }
}

#[allow(unused)]
trait InterpolatorReflected: Interpolator + Reflect {}

impl<T> InterpolatorReflected for T where T: Interpolator + Reflect {}

impl<I: 'static> Interpolator for Box<dyn InterpolatorReflected<Item = I>> {
    type Item = I;

    fn interpolate(&self, item: &mut Self::Item, value: f32) {
        (**self).interpolate(item, value);
    }
}

/// Create boxed closure in order to be used with dynamic [`Interpolator`]
pub fn closure<I, F>(f: F) -> Box<dyn Fn(&mut I, f32) + Send + Sync + 'static>
where
    I: 'static,
    F: Fn(&mut I, f32) + Send + Sync + 'static,
{
    Box::new(f)
}

/// Default interpolators
pub struct DefaultInterpolatorsPlugin;
impl Plugin for DefaultInterpolatorsPlugin {
    fn build(&self, app: &mut App) {
        app.add_tween_systems((
            tween::component_tween_system::<Translation>(),
            tween::component_tween_system::<Rotation>(),
            tween::component_tween_system::<Scale>(),
        ))
        .register_type::<tween::ComponentTween<Translation>>()
        .register_type::<tween::ComponentTween<Rotation>>()
        .register_type::<tween::ComponentTween<Scale>>();

        #[cfg(feature = "bevy_sprite")]
        app.add_tween_systems(tween::component_tween_system::<SpriteColor>())
            .register_type::<tween::ComponentTween<SpriteColor>>();

        #[cfg(all(feature = "bevy_sprite", feature = "bevy_asset",))]
        app.add_tween_systems(tween::asset_tween_system::<ColorMaterial>())
            .register_type::<tween::AssetTween<ColorMaterial>>();
    }
}

/// Default dynamic interpolators
pub struct DefaultDynInterpolatorsPlugin;
impl Plugin for DefaultDynInterpolatorsPlugin {
    fn build(&self, app: &mut App) {
        app.add_tween_systems(tween::component_tween_dyn_system::<Transform>());

        #[cfg(feature = "bevy_sprite")]
        app.add_tween_systems(tween::component_tween_dyn_system::<Sprite>());

        #[cfg(all(feature = "bevy_sprite", feature = "bevy_asset",))]
        app.add_tween_systems(tween::asset_tween_dyn_system::<
            bevy::sprite::ColorMaterial,
        >());
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

/// [`Interpolator`] for [`Transform`]'s rotation at Z axis.
/// Usually used for 2D rotation.
#[derive(Debug, Default, Clone, PartialEq, Reflect)]
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

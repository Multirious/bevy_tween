#![allow(missing_docs)]

use super::system::{
    apply_asset_tween_system, apply_component_tween_system,
    apply_handle_component_tween_system, apply_resource_tween_system,
};
use super::{Set, Setter};
use crate::{TweenAppResource, TweenSystemSet};
use bevy::{
    app::{PluginGroup, PluginGroupBuilder},
    prelude::*,
};
use std::marker::PhantomData;

macro_rules! tween_system_plugin {
    (
        $(
            $(#[$attr:meta])*
            $short_name:ident,
            $plugin_name:ident,
            $system_name:ident,
            $item_trait:ident;
        )*
    ) => {
        $(
            $(#[$attr])*
            #[doc = concat!("Registers [`", stringify!($system_name), "`](super::", stringify!($system_name), ")")]
            #[derive(Debug)]
            pub struct $plugin_name<S>
            where
                S: Set,
                S::Item: $item_trait,
                S::Value: Send + Sync + 'static,
            {
                marker: PhantomData<S>,
            }

            impl<S> Plugin for $plugin_name<S>
            where
                S: Set,
                S::Item: $item_trait,
                S::Value: Send + Sync + 'static,
            {
                fn build(&self, app: &mut App) {
                    let app_resource = app
                        .world()
                        .get_resource::<TweenAppResource>()
                        .expect("`TweenAppResource` resource doesn't exist");
                    app.add_systems(
                        app_resource.schedule,
                        $system_name::<S>
                            .in_set(TweenSystemSet::ApplyTween),
                    );
                }
            }

            impl<S> Default for $plugin_name<S>
            where
                S: Set,
                S::Item: $item_trait,
                S::Value: Send + Sync + 'static,
            {
                fn default() -> Self {
                    $plugin_name {
                        marker: PhantomData,
                    }
                }
            }

            #[doc = concat!("`", stringify!($plugin_name), "::default()`")]
            pub fn $short_name<S>() -> $plugin_name<S>
            where
                S: Set,
                S::Item: $item_trait,
                S::Value: Send + Sync + 'static,
            {
                $plugin_name::default()
            }
        )*
    };
}

tween_system_plugin! {
    component, ComponentTweenPlugin, apply_component_tween_system, Component;
    resource, ResourceTweenPlugin, apply_resource_tween_system, Resource;
    asset, AssetTweenPlugin, apply_asset_tween_system, Asset;
    handle_component, HandleComponentTweenPlugin, apply_handle_component_tween_system, Asset;
}

#[derive(Debug)]
pub struct DefaultTweenItemPlugins;
impl PluginGroup for DefaultTweenItemPlugins {
    #[allow(unused)]
    #[allow(clippy::let_and_return)]
    fn build(self) -> bevy::app::PluginGroupBuilder {
        use crate::items::*;

        let p = PluginGroupBuilder::start::<DefaultTweenItemPlugins>();
        let p = p
            .add(component::<Translation>())
            .add(component::<Rotation>())
            .add(component::<Scale>())
            .add(component::<AngleZ>())
            .add(|a: &mut App| {
                a.register_type::<Setter<Translation>>()
                    .register_type::<Setter<Rotation>>()
                    .register_type::<Setter<Scale>>()
                    .register_type::<Setter<AngleZ>>();
            });

        #[cfg(feature = "bevy_sprite")]
        let p = p
            .add(component::<SpriteColor>()) // mm
            .add(|a: &mut App| {
                a.register_type::<Setter<SpriteColor>>();
            });

        #[cfg(all(feature = "bevy_sprite", feature = "bevy_asset"))]
        let p = p
            .add(asset::<ColorMaterial>())
            .add(handle_component::<ColorMaterial>())
            .add(|a: &mut App| {
                a.register_type::<Setter<ColorMaterial>>();
            });

        #[cfg(feature = "bevy_ui")]
        let p = p
            .add(component::<BackgroundColor>())
            .add(component::<BorderColor>())
            .add(|a: &mut App| {
                a.register_type::<Setter<BackgroundColor>>()
                    .register_type::<Setter<BorderColor>>();
            });
        p
    }
}

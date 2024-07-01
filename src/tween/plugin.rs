#![allow(missing_docs)]

use super::system::{
    apply_asset_tween_system, apply_component_tween_system,
    apply_handle_component_tween_system, apply_resource_tween_system,
};
use super::Setter;
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
            <$g_setter:ident, $g_item:ident, $g_value:ident>,
            $system_name:ident,
            $item_trait:ident;
        )*
    ) => {
        $(
            $(#[$attr])*
            #[doc = concat!("Registers [`", stringify!($system_name), "`](super::", stringify!($system_name), ")")]
            #[derive(Debug)]
            pub struct $plugin_name<$g_setter, $g_item, $g_value>
            where
                $g_setter: Setter<$g_item, $g_value> + Component,
                $g_item: $item_trait,
                $g_value: Send + Sync + 'static,
            {
                marker: PhantomData<($g_setter, $g_item, $g_value)>,
            }

            impl<$g_setter, $g_item, $g_value> Plugin
                for $plugin_name<$g_setter, $g_item, $g_value>
            where
                $g_setter: Setter<$g_item, $g_value> + Component,
                $g_item: $item_trait,
                $g_value: Send + Sync + 'static,
            {
                fn build(&self, app: &mut App) {
                    let app_resource = app
                        .world()
                        .get_resource::<TweenAppResource>()
                        .expect("`TweenAppResource` resource doesn't exist");
                    app.add_systems(
                        app_resource.schedule,
                        $system_name::<$g_setter, $g_item, $g_value>
                            .in_set(TweenSystemSet::ApplyTween),
                    );
                }
            }

            impl<$g_setter, $g_item, $g_value> Default
                for $plugin_name<$g_setter, $g_item, $g_value>
            where
                $g_setter: Setter<$g_item, $g_value> + Component,
                $g_item: $item_trait,
                $g_value: Send + Sync + 'static,
            {
                fn default() -> Self {
                    $plugin_name {
                        marker: PhantomData,
                    }
                }
            }

            #[doc = concat!("`", stringify!($plugin_name), "::default()`")]
            pub fn $short_name<$g_setter, $g_item, $g_value>() -> $plugin_name<$g_setter, $g_item, $g_value>
            where
                $g_setter: Setter<$g_item, $g_value> + Component,
                $g_item: $item_trait,
                $g_value: Send + Sync + 'static,
            {
                $plugin_name::default()
            }
        )*
    };
}

tween_system_plugin! {
    component, ComponentTweenPlugin, <S, C, V>, apply_component_tween_system, Component;
    resource, ResourceTweenPlugin, <S, R, V>, apply_resource_tween_system, Resource;
    asset, AssetTweenPlugin, <S, A, V>, apply_asset_tween_system, Asset;
    handle_component, HandleComponentTweenPlugin, <S, A, V>, apply_handle_component_tween_system, Asset;
}

#[derive(Debug)]
pub struct DefaultTweenSystemPlugins;
impl PluginGroup for DefaultTweenSystemPlugins {
    #[allow(unused)]
    #[allow(clippy::let_and_return)]
    fn build(self) -> bevy::app::PluginGroupBuilder {
        use super::setter::*;

        let mut pg = PluginGroupBuilder::start::<DefaultTweenSystemPlugins>();
        #[cfg(feature = "bevy_sprite")]
        let pg = pg.add(component::<SpriteColor, _, _>());
        #[cfg(all(feature = "bevy_sprite", feature = "bevy_asset"))]
        let pg = pg.add(asset::<ColorMaterial, _, _>());
        #[cfg(all(feature = "bevy_sprite", feature = "bevy_asset"))]
        let pg = pg.add(handle_component::<ColorMaterial, _, _>());
        pg
    }
}

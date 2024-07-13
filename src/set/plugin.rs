#![allow(missing_docs)]

use super::system::{
    set_asset_system, set_component_system, set_handle_component_system,
    set_resource_system,
};
use super::{BoxedSetter, Set};
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
            $box_short_name:ident,
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
                S: Set + Component,
                S::Item: $item_trait,
                S::Value: Send + Sync + 'static,
            {
                marker: PhantomData<S>,
            }

            impl<S> Plugin for $plugin_name<S>
            where
                S: Set + Component,
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
                            .in_set(TweenSystemSet::Apply),
                    );
                }
            }

            impl<S> Default for $plugin_name<S>
            where
                S: Set + Component,
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
                S: Set + Component,
                S::Item: $item_trait,
                S::Value: Send + Sync + 'static,
            {
                $plugin_name::default()
            }

            #[doc = concat!("`", stringify!($plugin_name), "::<BoxedSetter<I, V>>::default()`")]
            pub fn $box_short_name<I, V>() -> $plugin_name<BoxedSetter<I, V>>
            where
                I: $item_trait,
                V: Send + Sync + 'static,
            {
                $plugin_name::default()
            }
        )*
    };
}

tween_system_plugin! {
    component, component_boxed, SetComponentPlugin,
    set_component_system, Component;

    resource, resource_boxed, SetResourcePlugin,
    set_resource_system, Resource;

    asset, asset_boxed, SetAssetPlugin,
    set_asset_system, Asset;

    handle_component, handle_component_boxed, SetHandleComponentPlugin,
    set_handle_component_system, Asset;
}

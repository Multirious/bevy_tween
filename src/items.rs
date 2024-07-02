use bevy::{
    ecs::{component::Component, reflect::ReflectComponent},
    reflect::Reflect,
};

mod blanket_impl;

// mod reflect;

mod transform;
pub use transform::*;

#[cfg(feature = "bevy_sprite")]
mod sprite;
#[cfg(feature = "bevy_sprite")]
pub use sprite::*;

#[cfg(feature = "bevy_ui")]
mod ui;
#[cfg(feature = "bevy_ui")]
pub use ui::*;

pub trait Set: Send + Sync + 'static {
    type Item;
    type Value;
    fn set(&self, item: &mut Self::Item, value: &Self::Value);
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Setter<S>(pub S)
where
    S: Set;

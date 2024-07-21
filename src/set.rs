use bevy::prelude::*;

mod blanket_impl;

mod plugin;
pub use plugin::*;

mod system;
pub use system::*;

mod dynamic_setter;
pub use dynamic_setter::*;

mod boxed_setter;
pub use boxed_setter::*;

pub trait Set: Send + Sync + 'static {
    type Item;
    type Value;
    fn set(&self, item: &mut Self::Item, value: &Self::Value);
}

#[derive(Debug, Component, Clone, Copy, PartialEq, Reflect)]
#[reflect(Component)] // might want to use sparseset but i'm not sure yet
pub struct SetterValue<V = f32>(pub V);

use bevy::ecs::component::Component;
use std::marker::PhantomData;

mod blanket_impl;

#[cfg(feature = "bevy_sprite")]
mod sprite;
mod transform;
#[cfg(feature = "bevy_ui")]
mod ui;

#[cfg(feature = "bevy_sprite")]
pub use sprite::*;
pub use transform::*;
#[cfg(feature = "bevy_ui")]
pub use ui::*;

pub trait Set<Item, Value>: Send + Sync + 'static {
    fn set(&self, item: &mut Item, value: &Value);
}

#[derive(Component)]
pub struct Setter<S, Item, Value>(pub S, PhantomData<(Item, Value)>)
where
    S: Set<Item, Value>;

impl<S, Item, Value> Setter<S, Item, Value>
where
    S: Set<Item, Value>,
{
    fn new(set: S) -> Setter<S, Item, Value> {
        Setter(set, PhantomData)
    }
}

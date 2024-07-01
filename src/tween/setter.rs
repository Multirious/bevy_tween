use std::marker::PhantomData;

mod blanket_impl;

#[cfg(feature = "bevy_sprite")]
mod sprite;

use bevy::ecs::component::Component;
#[cfg(feature = "bevy_sprite")]
pub use sprite::*;

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

use bevy::prelude::*;

mod blanket_impl;

mod plugin;
pub use plugin::*;

mod system;
pub use system::*;

mod world_setter;
pub use world_setter::*;

pub trait Set: Send + Sync + 'static {
    type Item;
    type Value;
    fn set(&self, item: &mut Self::Item, value: &Self::Value);
}

#[derive(Debug, Component, Clone, Copy, PartialEq, Reflect)]
#[reflect(Component)] // might want to use sparseset but i'm not sure yet
pub struct SetterValue<V = f32>(pub V);

#[derive(Component)]
pub struct BoxedSetter<I, V>(Box<dyn Set<Item = I, Value = V>>);

impl<I, V> BoxedSetter<I, V>
where
    I: Send + Sync + 'static,
    V: Send + Sync + 'static,
{
    pub fn new<S>(setter: S) -> BoxedSetter<S::Item, S::Value>
    where
        S: Set,
    {
        BoxedSetter(Box::new(setter))
    }

    pub fn new_closure<F>(f: F) -> BoxedSetter<I, V>
    where
        F: Fn(&mut I, &V) + Send + Sync + 'static,
    {
        let f: Box<dyn Fn(&mut I, &V) + Send + Sync + 'static> = Box::new(f);
        BoxedSetter(Box::new(f))
    }
}

impl<I, V> Set for BoxedSetter<I, V>
where
    I: Send + Sync + 'static,
    V: Send + Sync + 'static,
{
    type Item = I;
    type Value = V;

    fn set(&self, item: &mut Self::Item, value: &Self::Value) {
        self.0.set(item, value);
    }
}

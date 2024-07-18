use super::Set;
use bevy::prelude::*;

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

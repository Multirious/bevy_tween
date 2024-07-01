use super::Set;
use std::sync::Arc;

impl<S, Item, Value> Set<Item, Value> for Box<S>
where
    S: Set<Item, Value> + ?Sized,
{
    fn set(&self, item: &mut Item, value: &Value) {
        (**self).set(item, value)
    }
}

impl<S, Item, Value> Set<Item, Value> for &'static S
where
    S: Set<Item, Value> + ?Sized,
{
    fn set(&self, item: &mut Item, value: &Value) {
        (**self).set(item, value)
    }
}

impl<S, Item, Value> Set<Item, Value> for Arc<S>
where
    S: Set<Item, Value> + ?Sized,
{
    fn set(&self, item: &mut Item, value: &Value) {
        (**self).set(item, value)
    }
}

impl<Item: 'static, Value: 'static> Set<Item, Value>
    for dyn Fn(&mut Item, &Value) + Send + Sync + 'static
{
    fn set(&self, item: &mut Item, value: &Value) {
        self(item, value)
    }
}

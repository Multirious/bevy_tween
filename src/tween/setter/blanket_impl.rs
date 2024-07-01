use super::Setter;
use std::sync::Arc;

impl<S, Item, Value> Setter<Item, Value> for Box<S>
where
    S: Setter<Item, Value> + ?Sized,
{
    fn set(&self, item: &mut Item, value: &Value) {
        (**self).set(item, value)
    }
}

impl<S, Item, Value> Setter<Item, Value> for &'static S
where
    S: Setter<Item, Value> + ?Sized,
{
    fn set(&self, item: &mut Item, value: &Value) {
        (**self).set(item, value)
    }
}

impl<S, Item, Value> Setter<Item, Value> for Arc<S>
where
    S: Setter<Item, Value> + ?Sized,
{
    fn set(&self, item: &mut Item, value: &Value) {
        (**self).set(item, value)
    }
}

impl<Item: 'static, Value: 'static> Setter<Item, Value>
    for dyn Fn(&mut Item, &Value) + Send + Sync + 'static
{
    fn set(&self, item: &mut Item, value: &Value) {
        self(item, value)
    }
}

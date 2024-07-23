use super::Set;
use std::sync::Arc;

impl<S> Set for Box<S>
where
    S: Set + ?Sized,
{
    type Item = S::Item;
    type Value = S::Value;

    fn set(&self, item: &mut Self::Item, value: &Self::Value) {
        (**self).set(item, value)
    }
}

impl<S> Set for &'static S
where
    S: Set + ?Sized,
{
    type Item = S::Item;
    type Value = S::Value;

    fn set(&self, item: &mut Self::Item, value: &Self::Value) {
        (**self).set(item, value)
    }
}

impl<S> Set for Arc<S>
where
    S: Set + ?Sized,
{
    type Item = S::Item;
    type Value = S::Value;

    fn set(&self, item: &mut Self::Item, value: &Self::Value) {
        (**self).set(item, value)
    }
}

impl<Item: 'static, Value: 'static> Set
    for dyn Fn(&mut Item, &Value) + Send + Sync + 'static
{
    type Item = Item;
    type Value = Value;

    fn set(&self, item: &mut Self::Item, value: &Self::Value) {
        self(item, value)
    }
}

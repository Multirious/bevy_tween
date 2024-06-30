mod blanket_impl;

#[cfg(feature = "bevy_sprite")]
mod sprite;

#[cfg(feature = "bevy_sprite")]
pub use sprite::*;

pub mod plugin;
pub mod system;

pub trait Setter<Item, Value>: Send + Sync + 'static {
    fn set(&self, item: &mut Item, value: &Value);
}

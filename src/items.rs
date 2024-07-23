use crate::set::Set;

macro_rules! impl_simple_setter {
    (
        $(#[$attr:meta])*
        $setter:ident,
        |$item_arg:ident: &mut $item_ty:path, $value_arg:ident: & $value_ty:path| $expr:block
    ) => {
        $(#[$attr])*
        #[derive(Debug, Clone, Copy, Reflect, Component)]
        pub struct $setter;

        impl Set for $setter {
            type Item = $item_ty;
            type Value = $value_ty;

            fn set(&self, $item_arg: &mut Self::Item, $value_arg: &Self::Value) {
                $expr
            }
        }
    }
}
use impl_simple_setter;

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

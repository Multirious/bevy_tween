#![allow(clippy::type_complexity)]

mod alter;
pub mod argument;
#[cfg(feature = "bevy_app")]
mod plugin;
mod systems;
mod tween_blend;

pub use alter::*;
#[cfg(feature = "bevy_app")]
pub use plugin::*;
pub use systems::*;
pub use tween_blend::*;

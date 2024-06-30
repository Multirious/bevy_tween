use super::Setter;
use bevy::prelude::*;

#[derive(Debug, Default, Clone, PartialEq, Component, Reflect)]
#[reflect(Component)]
pub struct SpriteColor;

impl Setter<Sprite, Color> for SpriteColor {
    fn set(&self, item: &mut Sprite, value: &Color) {
        item.color = *value;
    }
}

fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        super::apply_component_tween_system::<SpriteColor, _, _>,
    );
}

#[derive(Debug, Default, Clone, PartialEq, Component, Reflect)]
#[reflect(Component)]
pub struct ColorMaterial;

impl Setter<bevy::prelude::ColorMaterial, Color> for ColorMaterial {
    fn set(&self, item: &mut bevy::prelude::ColorMaterial, value: &Color) {
        item.color = *value;
    }
}

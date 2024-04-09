use super::*;
use bevy::reflect::FromType;
use bevy_tween::interpolate::Translation;

// fn register(app: &mut App) {
//     app.register_type_data::<Translation, ReflectTween>();
// }

// pub trait TweenUi {
//     fn target(&mut self, ui: &mut egui::Ui);
//     fn interpolator(&mut self, ui: &mut egui::Ui);
// }

// pub trait InterpolationUi {
//     fn interpolation_ui(&mut self, ui: &mut egui::Ui);
// }

// #[derive(Default)]
// pub struct ReflectList {
//     tweens: TypeIdMap<ReflectComponent>,
// }

// impl ReflectList {
//     fn from_type_registry(type_registry: &TypeRegistry) {}
// }

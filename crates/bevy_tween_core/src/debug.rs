use std::any::TypeId;

use bevy_ecs::system::Resource;
use bevy_utils::HashSet;

#[cfg(feature = "bevy_reflect")]
use bevy_ecs::reflect::ReflectResource;
#[cfg(feature = "bevy_reflect")]
use bevy_reflect::Reflect;

#[derive(Debug, Default, Resource, Clone)]
#[cfg_attr(feature = "bevy_reflect", derive(Reflect))]
#[cfg_attr(feature = "bevy_reflect", reflect(Resource))]
pub struct WillTweenList {
    prepare: HashSet<TypeId>,
    apply: HashSet<TypeId>,
}

impl WillTweenList {
    pub fn will_be_prepared<T: 'static>(&mut self) {
        self.prepare.insert(TypeId::of::<T>());
    }

    pub fn will_be_applied<T: 'static>(&mut self) {
        self.apply.insert(TypeId::of::<T>());
    }

    pub fn is_will_be_prepared<T: 'static>(&self) -> bool {
        self.prepare.contains(&TypeId::of::<T>())
    }

    pub fn is_will_be_applied<T: 'static>(&self) -> bool {
        self.apply.contains(&TypeId::of::<T>())
    }
}

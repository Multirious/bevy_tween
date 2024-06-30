#![allow(missing_docs)]
#![allow(unused)]
use bevy::prelude::*;

macro_rules! doc_entity_eq_fn {
    () => {
        "\
        # fn entity_eq(world: &World, a: Entity, b: Entity) -> bool {
        #     use bevy::utils::HashSet;
        #     let a = world.entity(a);
        #     let b = world.entity(b);
        #
        #     let a_components = a.archetype().components().collect::<HashSet<_>>();
        #     let b_components = b.archetype().components().collect::<HashSet<_>>();
        #
        #     if a_components != b_components {
        #         return false;
        #     }
        #
        #     let registry = world.resource::<AppTypeRegistry>();
        #     let registry = registry.read();
        #
        #     for component_id in a_components {
        #         let components = world.components();
        #         let type_id = components
        #             .get_info(component_id)
        #             .unwrap()
        #             .type_id()
        #             .unwrap();
        #         let Some(reflect_component) =
        #             registry.get_type_data::<ReflectComponent>(type_id)
        #         else {
        #             continue;
        #         };
        #
        #         let a_component_reflected = reflect_component.reflect(a).unwrap();
        #         let b_component_reflected = reflect_component.reflect(b).unwrap();
        #
        #         if !(a_component_reflected
        #             .reflect_partial_eq(b_component_reflected)
        #             .unwrap_or(false))
        #         {
        #             return false;
        #         }
        #     }
        #     true
        # }\
        "
    };
}
pub(crate) use doc_entity_eq_fn;

macro_rules! doc_app_test_boilerplate {
    () => {
        "\
        # use bevy_tween::prelude::*;
        # use bevy::ecs::system::CommandQueue;
        # use bevy::prelude::*;
        #
        # let mut app = App::new();
        # app.add_plugins((MinimalPlugins, DefaultTweenPlugins));
        #
        # let mut queue = CommandQueue::default();
        # let mut commands = Commands::new(&mut queue, &app.world);\
        "
    };
}
pub(crate) use doc_app_test_boilerplate;

macro_rules! doc_test_boilerplate {
    () => {
        "\
        # use bevy_tween::prelude::*;
        # use bevy::ecs::system::CommandQueue;
        # use bevy::prelude::*;
        #
        # let world = World::default();
        # let mut queue = CommandQueue::default();
        # let mut commands = Commands::new(&mut queue, &world);\
        "
    };
}
pub(crate) use doc_test_boilerplate;

fn entity_eq(world: &World, a: Entity, b: Entity) -> bool {
    use bevy::utils::HashSet;
    let a = world.entity(a);
    let b = world.entity(b);

    let a_components = a.archetype().components().collect::<HashSet<_>>();
    let b_components = b.archetype().components().collect::<HashSet<_>>();

    if a_components != b_components {
        return false;
    }

    let registry = world.resource::<AppTypeRegistry>();
    let registry = registry.read();

    for component_id in a_components {
        let components = world.components();
        let type_id = components
            .get_info(component_id)
            .unwrap()
            .type_id()
            .unwrap();
        let Some(reflect_component) =
            registry.get_type_data::<ReflectComponent>(type_id)
        else {
            continue;
        };

        let a_component_reflected = reflect_component.reflect(a).unwrap();
        let b_component_reflected = reflect_component.reflect(b).unwrap();

        if let (Some(children_a), Some(children_b)) = (
            a_component_reflected.downcast_ref::<Children>(),
            b_component_reflected.downcast_ref::<Children>(),
        ) {
            if children_a.len() != children_b.len() {
                return false;
            }
            for (child_a, child_b) in children_a.iter().zip(children_b.iter()) {
                if !entity_eq(world, *child_a, *child_b) {
                    return false;
                }
            }
        } else if !(a_component_reflected
            .reflect_partial_eq(b_component_reflected)
            .unwrap_or(false))
        {
            return false;
        }
    }
    true
}

#![allow(missing_docs)]
#![allow(unused)]
use bevy::prelude::*;

#[cfg(feature = "bevy_render")]
pub fn color_lerp(start: Color, end: Color, v: f32) -> Color {
    let Color::Rgba {
        red: start_red,
        green: start_green,
        blue: start_blue,
        alpha: start_alpha,
    } = start.as_rgba()
    else {
        unreachable!()
    };
    let Color::Rgba {
        red: end_red,
        green: end_green,
        blue: end_blue,
        alpha: end_alpha,
    } = end.as_rgba()
    else {
        unreachable!()
    };
    Color::Rgba {
        red: start_red.lerp(end_red, v),
        green: start_green.lerp(end_green, v),
        blue: start_blue.lerp(end_blue, v),
        alpha: start_alpha.lerp(end_alpha, v),
    }
}

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
        # let world = World::Default();
        # let mut queue = CommandQueue::default();
        # let mut commands = Commands::new(&mut queue, &world);\
        "
    };
}
pub(crate) use doc_test_boilerplate;

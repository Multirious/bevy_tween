use bevy::{prelude::*, utils::HashSet};

use crate::tween::TargetComponent;

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub struct AnimationTarget;

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
#[non_exhaustive]
pub struct AnimationTargetResolver;

pub fn resolve_animation_target_system(
    mut q_target: Query<
        (Entity, &mut TargetComponent),
        With<AnimationTargetResolver>,
    >,
    q_animation_target: Query<(
        Entity,
        Option<&Parent>,
        Option<&AnimationTarget>,
    )>,
    mut last_error: Local<HashSet<Entity>>,
) {
    let mut error = HashSet::new();
    q_target
        .iter_mut()
        .for_each(|(resolver_entity, mut target)| {
            let mut e = resolver_entity;
            loop {
                match q_animation_target.get(e) {
                    Ok((candidate, parent, marker)) => {
                        if marker.is_some() {
                            e = candidate;
                            break;
                        } else if let Some(parent) = parent {
                            e = parent.get()
                        } else {
                            if !last_error.contains(&resolver_entity) && !error.contains(&resolver_entity) {
                                error!(
                                    "AnimationTargetResolver {resolver_entity} cannot find target in the parent chain"
                                )
                            }
                            error.insert(resolver_entity);
                            return;
                        }
                    }
                    Err(query_error) => {
                        if !last_error.contains(&resolver_entity) && !error.contains(&resolver_entity) {
                            error!(
                                "AnimationTargetResolver {resolver_entity} got query error: {query_error}"
                            )
                        }
                        error.insert(resolver_entity);
                    },
                }
            }
            *target = TargetComponent::Entity(e);
        });
    *last_error = error;
}

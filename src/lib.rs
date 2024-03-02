use bevy::prelude::*;
use ease_functions::{EaseFunctionEnum, EaseMethod};
use std::time::Duration;

pub mod ease_functions;
pub mod lenses;
pub mod prelude {
    pub use crate::{
        ease_functions::EaseFunctionEnum, lenses, AssetTween, ComponentTween,
        ResourceTween, TweenLens, TweenPlayer, TweenPlugin,
    };
}

pub struct TweenPlugin;
impl Plugin for TweenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                tick_tween_player_system.in_set(TweenSystemSet::Tick),
                tween_player_finished_system.in_set(TweenSystemSet::Finished),
                (
                    component_tween_system::<lenses::TransformTranslationLens>,
                    component_tween_system::<lenses::TransformRotationLens>,
                )
                    .in_set(TweenSystemSet::Tween),
            ),
        )
        .configure_sets(
            Update,
            (
                TweenSystemSet::Tick,
                TweenSystemSet::Tween,
                TweenSystemSet::Finished,
            )
                .chain(),
        )
        .register_type::<TweenPlayer>();
    }
}

#[derive(Debug, SystemSet, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TweenSystemSet {
    Tick,
    Tween,
    Finished,
}

#[derive(Debug, Component, Clone, Reflect)]
#[reflect(Component)]
pub struct TweenPlayer {
    pub enabled: bool,
    pub repeat: bool,
    pub current_duration: Duration,
    pub duration_ends: Duration,
}
impl TweenPlayer {
    pub fn new(duration: Duration) -> TweenPlayer {
        TweenPlayer {
            enabled: true,
            repeat: false,
            current_duration: Duration::ZERO,
            duration_ends: duration,
        }
    }
    pub fn with_repeat(mut self, repeat: bool) -> Self {
        self.repeat = repeat;
        self
    }
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
    pub fn forward(&mut self, delta: Duration) {
        self.current_duration += delta;
    }
    pub fn backward(&mut self, delta: Duration) {
        self.current_duration -= delta;
    }
    pub fn go_to(&mut self, duration: Duration) {
        self.current_duration = duration;
    }
    pub fn restart(&mut self) {
        self.go_to(Duration::ZERO);
    }
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    pub fn disable(&mut self) {
        self.enabled = false;
    }
}
impl Default for TweenPlayer {
    fn default() -> Self {
        TweenPlayer {
            enabled: true,
            repeat: false,
            current_duration: Duration::ZERO,
            duration_ends: Duration::ZERO,
        }
    }
}

pub fn tick_tween_player_system(
    time: Res<Time<Real>>,
    mut q_tween_player: Query<&mut TweenPlayer>,
) {
    let delta = time.delta();
    q_tween_player.iter_mut().for_each(|mut tween_player| {
        if tween_player.enabled {
            tween_player.forward(delta);
        }
    })
}

pub fn tween_player_finished_system(
    mut q_tween_player: Query<&mut TweenPlayer>,
) {
    q_tween_player.iter_mut().for_each(|mut tween_player| {
        if tween_player.current_duration >= tween_player.duration_ends {
            if tween_player.repeat {
                let overflow =
                    tween_player.current_duration - tween_player.duration_ends;
                tween_player.current_duration = overflow;
            } else {
                tween_player.enabled = false;
            }
        }
    });
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum ComponentTarget {
    #[default]
    PlayerParent,
    Entity(Entity),
}
impl From<Entity> for ComponentTarget {
    fn from(value: Entity) -> Self {
        ComponentTarget::Entity(value)
    }
}

#[derive(Debug)]
pub struct TweenData<L> {
    pub max_duration: Duration,
    pub min_duration: Duration,
    pub ease_method: EaseMethod,
    pub lens: L,
}
impl<L> TweenData<L>
where
    L: TweenLens,
{
    pub fn new<E: Into<EaseMethod>>(
        duration: Duration,
        ease_method: E,
        lens: L,
    ) -> Self {
        TweenData {
            max_duration: duration,
            min_duration: Duration::ZERO,
            ease_method: ease_method.into(),
            lens,
        }
    }
    pub fn with_delay(mut self, delay: Duration) -> Self {
        self.min_duration += delay;
        self.max_duration += delay;
        self
    }
    pub fn apply(&self, component: &mut L::Thing, current_duration: Duration) {
        let max_duration = self.max_duration.as_secs_f32();
        let min_duration = self.min_duration.as_secs_f32();
        let current_duration = current_duration.as_secs_f32();

        let ratio = ((current_duration - min_duration).max(0.)
            / (max_duration - min_duration))
            .min(1.);

        self.lens.apply(component, self.ease_method.value(ratio));
    }
}
impl<L: Default> Default for TweenData<L> {
    fn default() -> Self {
        TweenData {
            max_duration: Duration::ZERO,
            min_duration: Duration::ZERO,
            ease_method: EaseMethod::EaseFunction(EaseFunctionEnum::Linear),
            lens: L::default(),
        }
    }
}

#[derive(Debug, Default, Component)]
pub struct ComponentTween<L> {
    pub tween_data: TweenData<L>,
    pub component_target: ComponentTarget,
}

impl<L> ComponentTween<L>
where
    L: TweenLens,
{
    pub fn new<E: Into<EaseMethod>>(
        duration: Duration,
        ease_method: E,
        lens: L,
    ) -> Self {
        ComponentTween {
            tween_data: TweenData::new(duration, ease_method, lens),
            component_target: ComponentTarget::PlayerParent,
        }
    }
    pub fn with_delay(self, delay: Duration) -> Self {
        let Self {
            tween_data,
            component_target,
        } = self;
        let tween_data = tween_data.with_delay(delay);
        Self {
            tween_data,
            component_target,
        }
    }
    pub fn with_target<T: Into<ComponentTarget>>(mut self, target: T) -> Self {
        self.component_target = target.into();
        self
    }
}

#[derive(Debug, Component, Default)]
pub struct ResourceTween<L> {
    pub tween_data: TweenData<L>,
}

impl<L> ResourceTween<L>
where
    L: TweenLens,
{
    pub fn new<E: Into<EaseMethod>>(
        duration: Duration,
        ease_method: E,
        lens: L,
    ) -> Self {
        ResourceTween {
            tween_data: TweenData::new(duration, ease_method, lens),
        }
    }
    pub fn with_delay(self, delay: Duration) -> Self {
        let Self { tween_data } = self;
        let tween_data = tween_data.with_delay(delay);
        Self { tween_data }
    }
}

#[derive(Debug, Component, Default)]
pub struct AssetTween<L>
where
    L: TweenLens,
    <L as TweenLens>::Thing: Asset,
{
    pub tween_data: TweenData<L>,
    pub asset: Handle<L::Thing>,
}

impl<L> AssetTween<L>
where
    L: TweenLens,
    <L as TweenLens>::Thing: Asset,
{
    pub fn new<E: Into<EaseMethod>>(
        duration: Duration,
        ease_method: E,
        lens: L,
        asset: Handle<L::Thing>,
    ) -> Self {
        AssetTween {
            tween_data: TweenData::new(duration, ease_method, lens),
            asset,
        }
    }
    pub fn with_delay(self, delay: Duration) -> Self {
        let Self { tween_data, asset } = self;
        let tween_data = tween_data.with_delay(delay);
        Self { tween_data, asset }
    }
}

pub trait TweenLens {
    type Thing;
    fn apply(&self, component: &mut Self::Thing, ease_value: f32);
}

pub fn component_tween_system<L>(
    q_tween_player: Query<(Option<&Parent>, &TweenPlayer)>,
    q_tween: Query<(Option<&Parent>, Option<&TweenPlayer>, &ComponentTween<L>)>,
    mut q_component: Query<&mut <L as TweenLens>::Thing>,
) where
    L: TweenLens + Send + Sync + 'static,
    <L as TweenLens>::Thing: Component,
{
    q_tween
        .iter()
        .for_each(|(this_parent, this_tween_player, tween)| {
            let Some((tween_player, mut target)) = search_tween_player(
                this_tween_player,
                this_parent,
                &q_tween_player,
            ) else {
                return;
            };
            match tween.component_target {
                ComponentTarget::PlayerParent => {}
                ComponentTarget::Entity(e) => target = e,
            }

            if !tween_player.enabled {
                return;
            }
            let Ok(mut target_component) = q_component.get_mut(target) else {
                return;
            };
            let current_duration = tween_player.current_duration;
            tween
                .tween_data
                .apply(&mut target_component, current_duration);
        });
}

pub fn resource_tween_system<L>(
    q_tween_player: Query<(Option<&Parent>, &TweenPlayer)>,
    q_tween: Query<(Option<&Parent>, Option<&TweenPlayer>, &ResourceTween<L>)>,
    res: Option<ResMut<L::Thing>>,
) where
    L: TweenLens + Send + Sync + 'static,
    <L as TweenLens>::Thing: Resource,
{
    let Some(mut res) = res else {
        warn!("Resource does not exists for a ResourceTween!");
        return;
    };
    q_tween
        .iter()
        .for_each(|(this_parent, this_tween_player, tween)| {
            let Some((tween_player, _)) = search_tween_player(
                this_tween_player,
                this_parent,
                &q_tween_player,
            ) else {
                return;
            };

            if !tween_player.enabled {
                return;
            }
            let current_duration = tween_player.current_duration;
            tween.tween_data.apply(&mut res, current_duration);
        });
}

pub fn asset_tween_system<L>(
    q_tween_player: Query<(Option<&Parent>, &TweenPlayer)>,
    q_tween: Query<(Option<&Parent>, Option<&TweenPlayer>, &AssetTween<L>)>,
    asset: Option<ResMut<Assets<L::Thing>>>,
) where
    L: TweenLens + Send + Sync + 'static,
    <L as TweenLens>::Thing: Asset,
{
    let Some(mut asset) = asset else {
        warn!("Asset resource does not exists for an AssetTween!");
        return;
    };
    q_tween
        .iter()
        .for_each(|(this_parent, this_tween_player, tween)| {
            let Some((tween_player, _)) = search_tween_player(
                this_tween_player,
                this_parent,
                &q_tween_player,
            ) else {
                return;
            };

            if !tween_player.enabled {
                return;
            }
            let current_duration = tween_player.current_duration;
            if let Some(a) = asset.get_mut(&tween.asset) {
                tween.tween_data.apply(a, current_duration);
            }
        });
}

/// returns tweenplayer and player's parent
fn search_tween_player<'s, 'q: 's, 't: 'q>(
    this_tween_player: Option<&'s TweenPlayer>,
    this_parent: Option<&Parent>,
    q_tween_player: &'q Query<'_, 's, (Option<&Parent>, &'t TweenPlayer)>,
) -> Option<(&'s TweenPlayer, Entity)> {
    let owner;
    let tween_player;
    match (this_tween_player, this_parent) {
        (None, Some(p)) => match q_tween_player.get(p.get()) {
            Ok((Some(pp), t)) => {
                tween_player = t;
                owner = pp.get();
            }
            _ => return None,
        },
        (Some(t), Some(p)) => {
            tween_player = t;
            owner = p.get();
        }
        _ => return None,
    };
    Some((tween_player, owner))
}

use bevy::prelude::*;
use bevy_tween::{prelude::*, tween::TargetComponent};

mod interpolate {
    use bevy::prelude::*;
    use bevy_tween::prelude::*;

    pub use bevy_tween::interpolate::*;

    pub fn custom_interpolators_plugin(app: &mut App) {
        app.add_tween_systems(
            bevy_tween::component_tween_system::<AtlasIndex>(),
        );
    }

    pub fn atlas_index(start: usize, end: usize) -> AtlasIndex {
        AtlasIndex { start, end }
    }

    pub struct AtlasIndex {
        pub start: usize,
        pub end: usize,
    }

    impl Interpolator for AtlasIndex {
        type Item = TextureAtlas;

        fn interpolate(&self, item: &mut Self::Item, value: f32) {
            let start = self.start as f32;
            let end = self.end as f32;
            item.index = start.lerp(end, value).floor() as usize;
        }
    }
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            DefaultTweenPlugins,
            interpolate::custom_interpolators_plugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    use interpolate::atlas_index;
    let texture = asset_server.load("pink_fire_ball.png");
    let layout =
        TextureAtlasLayout::from_grid(Vec2::new(32., 32.), 16, 1, None, None);
    let len = layout.len();
    let atlas_layout = texture_atlas_layouts.add(layout);
    let sprite = TargetComponent::tweener_entity();
    commands.spawn((
        SpriteSheetBundle {
            texture,
            atlas: TextureAtlas {
                layout: atlas_layout,
                index: 0,
            },
            transform: Transform::IDENTITY.with_scale(Vec3::splat(15.)),
            ..Default::default()
        },
        SpanTweenerBundle::new(Duration::from_secs(1))
            .with_repeat(Repeat::Infinitely)
            .tween_here(),
        EaseFunction::Linear,
        sprite.with(atlas_index(0, len)),
    ));

    commands.spawn(Camera2dBundle::default());
}

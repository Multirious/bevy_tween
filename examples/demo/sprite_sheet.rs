use bevy::prelude::*;
use bevy_tween::{prelude::*, tween::AnimationTarget};

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
        type Item = Sprite;

        fn interpolate(&self, item: &mut Self::Item, value: f32) {
            let Some(texture_atlas) = &mut item.texture_atlas else {
                return;
            };
            let start = self.start as f32;
            let end = self.end as f32;
            texture_atlas.index = start.lerp(end, value).floor() as usize;
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
        TextureAtlasLayout::from_grid(UVec2::new(32, 32), 16, 1, None, None);
    let len = layout.len();
    let atlas_layout = texture_atlas_layouts.add(layout);

    let sprite = AnimationTarget.into_target();
    commands
        .spawn((
            Sprite {
                image: texture,
                texture_atlas: Some(TextureAtlas::from(atlas_layout)),
                ..default()
            },
            Transform::IDENTITY.with_scale(Vec3::splat(15.)),
            AnimationTarget,
        ))
        .animation()
        .repeat(Repeat::Infinitely)
        .insert_tween_here(
            Duration::from_secs(1),
            EaseKind::Linear,
            sprite.with(atlas_index(0, len)),
        );

    commands.spawn(Camera2d);
}

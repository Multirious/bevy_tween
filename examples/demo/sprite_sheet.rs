use bevy::prelude::*;
use bevy_tween::prelude::*;

mod m {
    use bevy::{math::FloatExt, sprite::TextureAtlas};
    use bevy_tween::prelude::*;

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
            DefaultTweenPlugins
        ))
        .add_systems(Startup, setup)
        .add_tween_systems(bevy_tween::component_tween_system::<m::AtlasIndex>())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("pink_fire_ball.png");
    let layout =
        TextureAtlasLayout::from_grid(Vec2::new(32., 32.), 16, 1, None, None);
    let len = layout.len();
    let atlas_layout = texture_atlas_layouts.add(layout);
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
        TweenerBundle::new(Duration::from_secs(1))
            .with_repeat(Repeat::Infinitely)
            .tween_here(),
        EaseFunction::Linear,
        ComponentTween::new(m::AtlasIndex { start: 0, end: len }),
    ));

    commands.spawn(Camera2dBundle::default());
}

#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use inner::CardGamePlugin;

mod inner;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_startup_system(debug_setup)
        // .add_system(spawn_player)
        .add_plugin(CardGamePlugin)
        .run();
}

// fn debug_setup(mut commands: Commands) {
//     let mut camera_bundle = Camera2dBundle::default();
//     camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(10.);
//     commands.spawn(camera_bundle);
// }

// fn spawn_player(mut commands: Commands) {
//     commands.spawn(SpriteBundle {
//         sprite: Sprite {
//             color: Color::rgb(0., 0.47, 1.),
//             custom_size: Some(Vec2::new(1., 1.)),
//             ..default()
//         },
//         ..default()
//     });
// }

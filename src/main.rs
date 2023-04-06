#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod game;

use bevy::prelude::*;
use game::GamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Inner Demons".into(),
                resolution: (1000.0, 800.0).into(),
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        // .add_startup_system(debug_setup)
        // .add_system(spawn_player)
        .add_plugin(GamePlugin)
        .run();
}

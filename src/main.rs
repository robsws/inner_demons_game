#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::prelude::*;
use inner::CardGamePlugin;

mod inner;
mod settings;

use settings::Settings;

fn main() {
    let settings = Settings::from_config();
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(settings)
        .add_plugin(CardGamePlugin)
        .run();
}

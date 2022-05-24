#![allow(unused)]

use bevy::prelude::*;
use galaxy_rangers::player::PlayerPlugin;
use galaxy_rangers::setup::SetupPlugin;

const BACKGROUND_COLOR: Color = Color::rgb(0.04, 0.04, 0.04);

fn main() {
    App::new()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(WindowDescriptor {
            title: "Galaxy Rangers".to_string(),
            width: 598.0,
            height: 676.0,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(SetupPlugin)
        .add_plugin(PlayerPlugin)
        .run();
}

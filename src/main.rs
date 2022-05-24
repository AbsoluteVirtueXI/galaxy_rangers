#![allow(unused)]

use bevy::prelude::*;
use galaxy_rangers::debug::DebugPlugin;
use galaxy_rangers::movement::MovementPlugin;
use galaxy_rangers::player::PlayerPlugin;
use galaxy_rangers::setup::SetupPlugin;

const BACKGROUND_COLOR: Color = Color::rgb(0.07, 0.07, 0.07);

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
        .add_plugin(MovementPlugin)
        .add_plugin(DebugPlugin)
        .run();
}

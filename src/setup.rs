use crate::textures::GameTextures;
use crate::window::WindowSize;
use crate::PLAYER_SPRITE;
use bevy::prelude::*;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_sytem);
    }
}

fn setup_sytem(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: ResMut<Windows>,
) {
    // Camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Get window size
    let window = windows.get_primary_mut().unwrap();
    let (win_w, win_h) = (window.width(), window.height());

    // Set window position
    window.set_position(IVec2::new(1680, 0));

    // Add WindowSize resource
    let window_size = WindowSize {
        width: win_w,
        height: win_h,
    };
    commands.insert_resource(window_size);

    // Add GameTextures resource
    let game_textures = GameTextures {
        player: asset_server.load(PLAYER_SPRITE),
    };
    commands.insert_resource(game_textures);
}

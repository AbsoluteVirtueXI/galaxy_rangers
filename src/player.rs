use crate::components::{Movable, Player, Velocity};
use crate::textures::GameTextures;
use crate::window::WindowSize;
use crate::{PLAYER_SIZE, SPRITE_SCALE};
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system)
            .add_system(player_keyboard_event_system) // should .before(player_movement_system)
            .add_system(player_fire_system);
    }
}

fn player_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    win_size: Res<WindowSize>,
) {
    // min on y axis is height of window divided by 2 x -1
    let bottom = -win_size.height / 2.;
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(PLAYER_SIZE.0, PLAYER_SIZE.1)),
                ..default()
            },
            texture: game_textures.player.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, bottom + PLAYER_SIZE.1 / 2. * SPRITE_SCALE + 5., 10.),
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                ..default()
            },
            ..default()
        })
        .insert(Player)
        .insert(Movable {
            auto_despawn: false,
        })
        .insert(Velocity { x: 0., y: 0. });
}

fn player_keyboard_event_system(
    kb: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    if let Ok(mut velocity) = query.get_single_mut() {
        velocity.x = if kb.pressed(KeyCode::Left) {
            -1.
        } else if kb.pressed(KeyCode::Right) {
            1.
        } else {
            0.
        }
    }
}

fn player_fire_system(
    mut commands: Commands,
    kb: Res<Input<KeyCode>>,
    game_textures: Res<GameTextures>,
    query: Query<&Transform, With<Player>>,
) {
    if let Ok(transform) = query.get_single() {
        if kb.just_pressed(KeyCode::Space) {
            let (x, y) = (transform.translation.x, transform.translation.y);
            commands
                .spawn_bundle(SpriteBundle {
                    texture: game_textures.player_laser.clone(),
                    transform: Transform {
                        translation: Vec3::new(x, y, 0.),
                        scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                        ..default()
                    },
                    ..default()
                })
                .insert(Movable { auto_despawn: true })
                .insert(Velocity { x: 0., y: 1. });
        }
    }
}

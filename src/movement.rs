use crate::components::{Movable, Velocity};
use crate::window::WindowSize;
use crate::LASER_SIZE;
use crate::{BASE_SPEED, TIME_STEP};
use bevy::prelude::*;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(movement_system);
    }
}

fn movement_system(
    mut commands: Commands,
    window_size: Res<WindowSize>,
    mut query: Query<(Entity, &Velocity, &mut Transform, &Movable)>,
) {
    for (entity, velocity, mut transform, movable) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;

        if movable.auto_despawn
            && (translation.y > window_size.height / 2. + LASER_SIZE.1 / 2.
                || translation.y < -window_size.height / 2. - LASER_SIZE.1 / 2.
                || translation.x > window_size.width / 2. + LASER_SIZE.1 / 2.
                || translation.x < -window_size.width / 2. - LASER_SIZE.1 / 2.)
        {
            commands.entity(entity).despawn();
        }
    }
}

use bevy::prelude::*;

pub const PLAYER_SPRITE: &str = "spaceship.pod_.1.blue_.png";
pub const LASER_SPRITE: &str = "laser_a_01.png";

pub struct GameTextures {
    pub player: Handle<Image>,
    pub player_laser: Handle<Image>,
}

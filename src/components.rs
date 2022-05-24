use bevy::prelude::Component;
use bevy_inspector_egui::Inspectable;

#[derive(Component, Inspectable)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Inspectable)]
pub struct Movable {
    pub auto_despawn: bool,
}

#[derive(Component, Inspectable)]
pub struct Player;

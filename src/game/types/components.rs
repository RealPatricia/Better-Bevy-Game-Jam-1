/*
 All the primative types
*/

use bevy::{prelude::*, math::Vec2};

#[derive(Component, Default, Clone, Copy)]
pub struct Velocity(pub Vec2);

#[derive(Component, Default, Clone, Copy)]
pub struct Acceleration(pub Vec2);

#[derive(Component, Default, Clone, Copy)]
pub struct PlayerTag(pub i8);
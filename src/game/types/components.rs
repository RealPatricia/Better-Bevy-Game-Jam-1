/*
 All the primative types
*/

use bevy::{math::Vec2, prelude::*};

#[derive(Component, Default, Clone, Copy)]
pub struct Velocity(pub Vec2);

#[derive(Component, Default, Clone, Copy)]
pub struct Acceleration(pub Vec2);

#[derive(Component, Default, Clone, Copy)]
pub struct PlayerTag(pub i8);

#[derive(Component, Default, Clone, Copy)]
pub struct Thrust(pub f32);

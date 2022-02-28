use bevy::prelude::*;

pub fn bool_to_input_direction(up: bool, down: bool, left: bool, right: bool) -> Vec2
{
    let vertical: f32 = ((up as i32) - (down as i32)) as f32;
    let horizontal: f32 = ((right as i32) - (left as i32)) as f32;

    let mut direction = Vec2::new(horizontal, vertical);

    if direction != Vec2::ZERO {
        direction = direction.normalize();
    }

    direction
}

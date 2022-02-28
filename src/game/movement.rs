use bevy::prelude::*;
use super::types::components::*;

pub struct MovementPlugin;

impl Plugin for MovementPlugin
{
    fn build(&self, app: &mut App)
    {
        app
            .add_system(movement_system)
            .add_system(acceleration_system);
    }
}

fn movement_system(
    time: Res<Time>,
    mut moving_q: Query<(&mut Transform, &Velocity)>
)
{
    for (mut transform, velocity) in moving_q.iter_mut()
    {
        transform.translation += Vec3::new(velocity.0.x, velocity.0.y, 0.0) * time.delta_seconds();
    }
}

fn acceleration_system(
    time: Res<Time>,
    mut accel_q: Query<(&mut Velocity, &Acceleration)>
)
{
    for (mut velocity, acceleration) in accel_q.iter_mut()
    {
        velocity.0 += acceleration.0 * time.delta_seconds();
    }
}
// Custom Bundles to make creating specific kinds of objects easier
use bevy::{prelude::*};
use super::components::*;

#[derive(Bundle, Default, Clone)]
pub struct PlayerBundle
{
    #[bundle]
    pub ship_bundle: ShipBundle,
    pub player_tag: PlayerTag,
}

#[derive(Bundle, Default, Clone)]
pub struct ShipBundle
{
    #[bundle]
    pub object_bundle: ObjectBundle,
    pub acceleration: Acceleration,
    pub thrust: Thrust
}

#[derive(Bundle, Default, Clone)]
pub struct ObjectBundle
{
    #[bundle]
    pub sprite_bundle: SpriteBundle,
    pub velocity: Velocity
}
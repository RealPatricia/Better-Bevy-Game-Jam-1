// This module is just here to organize all of the game types, had a really bad problem with it, so hopefully this helps with the issue
mod components;
mod bundles;

use bevy::prelude::*;
use resources::*;
use prefabs::*;
use components::*;
use bundles::*;

pub struct ResourcePlugin;

impl Plugin for ResourcePlugin
{
    fn build(&self, app: &mut App)
    {
        app
            .insert_resource(ArenaSize { width: 1000.0, height: 1000.0})
            .insert_resource(PlayerPrefab(PlayerBundle
            {
                ship_bundle: ShipBundle
                {
                    object_bundle: ObjectBundle
                    {
                        sprite_bundle: SpriteBundle
                        {
                            sprite: Sprite
                            {
                                color: Color::WHITE,
                                custom_size: Some(Vec2::from([100.0, 100.0])),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            }));
    }
}

pub mod resources
{
    pub struct ArenaSize { pub width: f32, pub height: f32 }
}

pub mod prefabs
{
    use super::bundles::PlayerBundle;

    pub struct PlayerPrefab(pub PlayerBundle);
}
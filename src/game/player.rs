use bevy::prelude::*;
use super::types::{prefabs::*};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin
{
    fn build(&self, app: &mut App)
    {
        app
            .add_startup_system(player_setup);
    }
}

fn player_setup(
    mut commands: Commands,
    player_prefab: Res<PlayerPrefab>,
)
{
    let player = player_prefab.0.clone();
    commands.spawn_bundle(player);
}
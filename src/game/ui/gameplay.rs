use bevy::prelude::*;
use super::super::types::{prefabs::*, resources::*};

pub struct GamePlayPlugin;

impl Plugin for GamePlayPlugin
{
    fn build(&self, app: &mut App)
    {
        app
            .add_system_set(SystemSet::on_enter(AppState::GamePlay)
            .with_system(player_setup)
            .with_system(game_setup));
    }
}

fn game_setup()
{

}

fn player_setup(mut commands: Commands, player_prefab: Res<PlayerPrefab>)
{
    let player = player_prefab.0.clone();
    commands.spawn_bundle(player);
}
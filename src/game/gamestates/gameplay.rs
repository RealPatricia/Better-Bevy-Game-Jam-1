use bevy::prelude::*;

use super::super::types::{prefabs::*, events::*, resources::*};

pub struct GamePlayPlugin;

impl Plugin for GamePlayPlugin
{
    fn build(&self, app: &mut App)
    {
        app
            .add_system_set(SystemSet::on_enter(AppState::GamePlay)
                .with_system(player_setup)
                .with_system(game_setup))
            .add_system_set(SystemSet::on_resume(AppState::GamePlay)
                .with_system(game_setup)
                .with_system(escape_to_main_menu))
            .add_system_set(SystemSet::on_update(AppState::GamePlay)
                .with_system(pause_game)
                .with_system(can_pause_toggle));
    }
}

fn game_setup(
    mut commands: Commands
)
{
    commands.insert_resource(ClearColor(Color::BLACK))
}

fn player_setup(mut commands: Commands, player_prefab: Res<PlayerPrefab>)
{
    let player = player_prefab.0.clone();
    commands.spawn_bundle(player);
}

fn pause_game(
    mut pause: ResMut<CanUnpause>,
    keys: Res<Input<KeyCode>>,
    mut spush_event: EventWriter<StatePushEvent>
)
{
    if keys.just_pressed(KeyCode::Escape) & pause.0
    {
        pause.0 = false;
        spush_event.send(StatePushEvent(AppState::PauseMenu));
    }
}

fn can_pause_toggle(
    mut pause: ResMut<CanUnpause>,
    keys: Res<Input<KeyCode>>
)
{
    if !keys.pressed(KeyCode::Escape)
    {
        pause.0 = true;
    }
}

fn escape_to_main_menu(
    mut escape: ResMut<EscToMain>,
    mut spop_event: EventWriter<StatePopEvent>
)
{
    if escape.0
    {
        escape.0 = false;
        spop_event.send(StatePopEvent);
    }
}
use bevy::prelude::*;
use crate::game::types::events::StatePopEvent;

use super::super::types::resources::*;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_system_set(SystemSet::on_enter(AppState::PauseMenu).with_system(pause_setup))
            .add_system_set(SystemSet::on_update(AppState::PauseMenu)
            .with_system(unpause_game)
            .with_system(can_unpause_toggle)
            .with_system(escape_to_main_menu));
    }
}

fn pause_setup(
    mut commands: Commands
)
{
    commands.insert_resource(ClearColor(Color::RED));
    commands.insert_resource(EscToMain(false));
}

fn unpause_game(
    mut unpause: ResMut<CanUnpause>,
    keys: Res<Input<KeyCode>>,
    mut spop_event: EventWriter<StatePopEvent>
)
{
    if keys.just_pressed(KeyCode::Escape) & unpause.0
    {
        unpause.0 = false;
        spop_event.send(StatePopEvent);
    }
}

fn can_unpause_toggle(
    mut unpause: ResMut<CanUnpause>,
    keys: Res<Input<KeyCode>>
)
{
    if !keys.pressed(KeyCode::Escape)
    {
        unpause.0 = true;
    }
}

fn escape_to_main_menu(
    mut can_escape: ResMut<EscToMain>,
    keys: Res<Input<KeyCode>>,
    mut spop_event: EventWriter<StatePopEvent>
)
{
    if keys.just_pressed(KeyCode::Space) & !can_escape.0
    {
        can_escape.0 = true;

        spop_event.send(StatePopEvent);
    }
}
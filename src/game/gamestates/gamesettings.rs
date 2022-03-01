use bevy::prelude::*;
use super::super::types::resources::*;

pub struct GameSettingsPlugin;

impl Plugin for GameSettingsPlugin
{
    fn build(&self, app: &mut App)
    {
        app
            .add_system_set(SystemSet::on_enter(AppState::GameSettings).with_system(chapter_select_setup));
    }
}

fn chapter_select_setup(
    mut commands: Commands
)
{
    commands.insert_resource(ClearColor(Color::PURPLE));
}
use bevy::prelude::*;
use super::super::types::resources::*;

pub struct ChapterSelectPlugin;

impl Plugin for ChapterSelectPlugin
{
    fn build(&self, app: &mut App)
    {
        app
            .add_system_set(SystemSet::on_enter(AppState::ChapterSelect).with_system(chapter_select_setup));
    }
}

fn chapter_select_setup(
    mut commands: Commands
)
{
    commands.insert_resource(ClearColor(Color::ORANGE));
}
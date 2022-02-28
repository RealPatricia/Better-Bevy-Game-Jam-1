use super::super::types::resources::*;
use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(main_menu_setup))
            .add_system_set(SystemSet::on_update(AppState::MainMenu).with_system(main_menu_update))
            .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(main_menu_cleanup));
    }
}

fn main_menu_setup() {}

fn main_menu_cleanup() {}

fn main_menu_update() {}

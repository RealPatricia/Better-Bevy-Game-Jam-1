use bevy::{app::*, prelude::*};
use super::types::{events::*, resources::*};

mod splashscreen;
use splashscreen::SplashScreenPlugin;

mod mainmenu;
use mainmenu::MainMenuPlugin;

mod gameplay;
use gameplay::GamePlayPlugin;

mod pausemenu;
use pausemenu::PauseMenuPlugin;

mod gamesettings;
use gamesettings::GameSettingsPlugin;

mod shipsettings;
use shipsettings::ShipSettingsPlugin;

mod chapterselect;
use chapterselect::ChapterSelectPlugin;

pub struct GameStatePlugins;

impl PluginGroup for GameStatePlugins
{
    fn build(&mut self, group: &mut PluginGroupBuilder)
    {
        group
            .add(StateChangePlugin)
            .add(SplashScreenPlugin)
            .add(MainMenuPlugin)
            .add(GamePlayPlugin)
            .add(PauseMenuPlugin)
            .add(ChapterSelectPlugin)
            .add(ShipSettingsPlugin)
            .add(GameSettingsPlugin);
    }
}

pub struct StateChangePlugin;

impl Plugin for StateChangePlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_system(change_state)
            .add_system(push_state)
            .add_system(pop_state);
    }
}

fn change_state(
    mut app_state: ResMut<State<AppState>>,
    mut ev_state_change: EventReader<StateChangeEvent>)
{
    for state_change in ev_state_change.iter()
    {
        app_state.set(state_change.0).unwrap();
    }
}

fn push_state(
    mut app_state: ResMut<State<AppState>>,
    mut ev_state_change: EventReader<StatePushEvent>)
{
    for state_change in ev_state_change.iter()
    {
        app_state.push(state_change.0).unwrap();
    }
}

fn pop_state(
    mut app_state: ResMut<State<AppState>>,
    mut ev_state_change: EventReader<StatePopEvent>)
{
    for _ in ev_state_change.iter()
    {
        app_state.pop().unwrap();
    }
}
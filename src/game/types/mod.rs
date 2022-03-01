// This module is just here to organize all of the game types, had a really bad problem with it, so hopefully this helps with the issue
pub mod bundles;
pub mod components;
pub mod helper_functions;
pub mod events;

use bevy::prelude::*;
use resources::*;
use events::*;

pub struct ResourcePlugin;

impl Plugin for ResourcePlugin
{
    fn build(&self, app: &mut App)
    {
        app.insert_resource(ArenaSize {
            width: 1000.0,
            height: 1000.0,
        })
        .insert_resource(DebugTimer(Timer::from_seconds(3.0, true)))
        .insert_resource(CanUnpause(false))
        .add_state(AppState::SplashScreen)
        .add_event::<StateChangeEvent>()
        .add_event::<StatePushEvent>()
        .add_event::<StatePopEvent>();
    }
}

pub mod resources
{
    use bevy::core::Timer;

    pub struct ArenaSize
    {
        pub width: f32,
        pub height: f32,
    }
    pub struct DebugTimer(pub Timer);

    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum AppState
    {
        SplashScreen,
        #[default]
        MainMenu,
        GameSettings,
        ShipSettings,
        ChapterSelect,
        GamePlay,
        PauseMenu,
    }

    pub struct CanUnpause(pub bool);

    pub struct EscToMain(pub bool);
}

pub mod prefabs
{
    use super::bundles::PlayerBundle;

    pub struct PlayerPrefab(pub PlayerBundle);
}
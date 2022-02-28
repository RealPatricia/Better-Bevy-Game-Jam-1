// This module is just here to organize all of the game types, had a really bad problem with it, so hopefully this helps with the issue
pub mod bundles;
pub mod components;
pub mod helper_functions;

use bevy::prelude::*;
use bundles::*;
use components::*;
use prefabs::*;
use resources::*;

pub struct ResourcePlugin;

impl Plugin for ResourcePlugin
{
    fn build(&self, app: &mut App)
    {
        app.insert_resource(ArenaSize {
            width: 1000.0,
            height: 1000.0,
        })
        .insert_resource(DebugTimer(Timer::from_seconds(1.0, true)))
        .insert_resource(ClearColor(Color::CYAN))
        .add_state(AppState::SplashScreen)
        .insert_resource(DebugTimer(Timer::from_seconds(5.0, false)));
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

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum AppState
    {
        SplashScreen,
        MainMenu,
        GameSettings,
        ShipSettings,
        ChapterSelect,
        GamePlay,
        PauseMenu,
    }
}

pub mod prefabs
{
    use super::bundles::PlayerBundle;

    pub struct PlayerPrefab(pub PlayerBundle);
}

#[allow(dead_code)]
fn player_camera_report(
    time: Res<Time>,
    mut timer: ResMut<DebugTimer>,
    cam_query: Query<&Transform, With<Camera>>,
    player_query: Query<&Transform, With<PlayerTag>>,
)
{
    if timer.0.tick(time.delta()).just_finished() {
        for trans in cam_query.iter() {
            println!("Camera position: {}", trans.translation);
        }
        for trans in player_query.iter() {
            println!("Player position: {}", trans.translation);
        }
    }
}

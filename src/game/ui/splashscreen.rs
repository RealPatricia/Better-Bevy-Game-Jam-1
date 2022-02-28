use bevy::prelude::*;

use super::types::resources::*;

pub struct SplashScreen;

impl Plugin for SplashScreen
{
    fn build(&self, app: &mut App)
    {
        app;
    }
}
 fn temp_splash_screen(
     mut commands: Commands,
     timer: ResMut<DebugTimer>,
     time: Res<Time>,
     app_state: ResMut<State<AppState>>
 )
 {
    if timer.0.tick(time.delta()).finished()
    {
        commands.insert_resource(ClearColor(Color::BLACK));
        app_state.set(AppState::MainMenu).unwrap();
    }
 }
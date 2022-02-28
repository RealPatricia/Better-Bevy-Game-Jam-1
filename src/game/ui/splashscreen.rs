use bevy::prelude::*;

use super::super::types::resources::*;

pub struct SplashScreenPlugin;

impl Plugin for SplashScreenPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_system_set(
            SystemSet::on_update(AppState::SplashScreen).with_system(temp_splash_screen),
        );
    }
}
fn temp_splash_screen(
    mut commands: Commands,
    mut timer: ResMut<DebugTimer>,
    time: Res<Time>,
    mut app_state: ResMut<State<AppState>>,
)
{
    if timer.0.tick(time.delta()).finished() {
        commands.insert_resource(ClearColor(Color::BLACK));
        app_state.set(AppState::GamePlay).unwrap();
    }
}

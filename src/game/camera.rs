use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin
{
    fn build(&self, app: &mut App)
    {
        app
            .add_startup_system(camera_setup);
    }
}

fn camera_setup(
    mut commands: Commands
)
{
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
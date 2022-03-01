// Import app for the PluginGroupBuilder
use bevy::{app::*};

pub mod gamestates;

mod types;
use types::ResourcePlugin;

mod player;
use player::PlayerPlugin;

mod camera;
use camera::CameraPlugin;

mod movement;
use movement::MovementPlugin;
pub struct GamePlugins;

impl PluginGroup for GamePlugins
{
    fn build(&mut self, group: &mut PluginGroupBuilder)
    {
        group
            .add(ResourcePlugin)
            .add(PlayerPlugin)
            .add(CameraPlugin)
            .add(MovementPlugin);
    }
}
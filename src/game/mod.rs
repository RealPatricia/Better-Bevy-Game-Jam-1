// Import app for the PluginGroupBuilder
use bevy::{prelude::*, app::*};

mod types;
use types::ResourcePlugin;

mod player;
use player::PlayerPlugin;

mod camera;
use camera::CameraPlugin;
pub struct GamePlugins;

impl PluginGroup for GamePlugins
{
    fn build(&mut self, group: &mut PluginGroupBuilder)
    {
        group
            .add(ResourcePlugin)
            .add(PlayerPlugin)
            .add(CameraPlugin);
    }
}

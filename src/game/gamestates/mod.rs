use bevy::{app::*};

mod splashscreen;
use splashscreen::SplashScreenPlugin;

mod mainmenu;
use mainmenu::MainMenuPlugin;

mod gameplay;
use gameplay::GamePlayPlugin;

pub struct GameStatePlugins;

impl PluginGroup for GameStatePlugins
{
    fn build(&mut self, group: &mut PluginGroupBuilder)
    {
        group.add(SplashScreenPlugin)
            .add(MainMenuPlugin)
            .add(GamePlayPlugin);
    }
}
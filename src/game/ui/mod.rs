use bevy::{app::*, prelude::*};

mod splashscreen;
use splashscreen::SplashScreenPlugin;

mod mainmenu;
use mainmenu::MainMenuPlugin;

mod gameplay;
use gameplay::GamePlayPlugin;

pub struct UiPlugins;

impl PluginGroup for UiPlugins
{
    fn build(&mut self, group: &mut PluginGroupBuilder)
    {
        group.add(SplashScreenPlugin)
            .add(MainMenuPlugin)
            .add(GamePlayPlugin);
    }
}
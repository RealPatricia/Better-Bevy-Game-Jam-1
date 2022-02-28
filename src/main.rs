/*
Welcome to this untitled game jam game.It probably won't be ready
in time to be able to submit, but it was a fun project that I
learned a lot from, and hopefully you'll be able to learn something
from it as well, especially if I do my job right and properly comment
everything.
*/

#![feature(derive_default_enum)]

//Hopefully this one is obvious
use bevy::prelude::*;

/*
probably a bit excessive, but I like bundling up all of my game
code into a single PluginGroup to make everything prettier.
*/
mod game;

use game::{gamestates::GameStatePlugins, GamePlugins};

fn main()
{
    // You can thank C# (and rustfmt) for the drop brace btw. I know I do.

    // Setting up the app with the default plugins and our game plugins

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugins)
        .add_plugins(GameStatePlugins)
        .run();
}
use super::super::types::{resources::*, components::*};
use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(main_menu_setup))
            .add_system_set(SystemSet::on_update(AppState::MainMenu).with_system(test))
            .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(main_menu_cleanup));
    }
}

fn main_menu_setup(mut commands: Commands)
{
    commands.insert_resource(ClearColor(Color::BLACK));
    commands.spawn_bundle(UiCameraBundle::default());

    commands.spawn_bundle(NodeBundle
    {
        style: Style
        {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        color: Color::NONE.into(),
        ..Default::default()
    })
        .insert(MainMenuUI)
        .with_children(| parent | 
        {
            parent.spawn_bundle(NodeBundle
            {
                style: Style {
                    justify_content: JustifyContent::Center,
                    align_content: AlignContent::Center,
                    border: Rect::all(Val::Px(100.0)),
                    size: Size::new(Val::Percent(60.0), Val::Percent(60.0)),
                    ..Default::default()
                },
                color: Color::GRAY.into(),
                ..Default::default()
            });
        });
}

fn test(
    time: Res<Time>,
    mut timer: ResMut<DebugTimer>,
    mut app_state: ResMut<State<AppState>>
)
{
    if timer.0.tick(time.delta()).just_finished()
    {

        app_state.set(AppState::GamePlay).unwrap();
    }
}

fn main_menu_cleanup(
    mut commands: Commands,
    mmui_q: Query<Entity, With<MainMenuUI>>
) 
{
    for entity in mmui_q.iter()
    {
        commands.entity(entity).despawn_recursive();
    }
}
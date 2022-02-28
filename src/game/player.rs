use super::types::{bundles::*, components::*, helper_functions::*, prefabs::*, resources::*};
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin
{
    fn build(&self, app: &mut App)
    {
        app.insert_resource(PlayerPrefab(PlayerBundle {
            ship_bundle: ShipBundle {
                object_bundle: ObjectBundle {
                    sprite_bundle: SpriteBundle {
                        sprite: Sprite {
                            color: Color::WHITE,
                            custom_size: Some(Vec2::from([100.0, 100.0])),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    ..Default::default()
                },
                thrust: Thrust(120.0),
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_system_set(SystemSet::on_update(AppState::GamePlay).with_system(player_accelerate));
    }
}

fn player_accelerate(
    keys: Res<Input<KeyCode>>,
    mut player_q: Query<(&mut Acceleration, &Thrust), With<PlayerTag>>,
)
{
    //get input keys
    let up = keys.pressed(KeyCode::W);
    let down = keys.pressed(KeyCode::S);
    let left = keys.pressed(KeyCode::A);
    let right = keys.pressed(KeyCode::D);

    //calculate acceleration direction
    let accel_dir = bool_to_input_direction(up, down, left, right);

    //apply thrust
    for (mut acceleration, thrust) in player_q.iter_mut() {
        acceleration.0 = accel_dir * thrust.0;
    }
}

use bevy::prelude::*;

use super::{room::{primary_event::RoomEventHolder, Room}, structs::{EnemyShip, PlayerShip, Ship}};



type BRoom = crate::lib::primary::building::room::Room;


pub fn set_up_game (
    mut coms: Commands,
    b_room_q: Query<(&BRoom, &Transform, Entity)>,
    assets: Res<AssetServer>,

) {
    for (b_room, transform, entity) in b_room_q.iter() {
        let g_room = b_room.to_game(true);
        let g_transform = transform.clone();

        coms.spawn((
            Sprite {
                image: assets.load(b_room.image.clone()),
                ..default()
            },
            g_room,
            g_transform,
        ));

        coms.get_entity(entity).unwrap().despawn();
    }
}

pub fn run_room_events (
    mut rooms: Query<&mut Room>,

    mut player_ship: Query<&mut Ship, With<PlayerShip>>,
    mut enemy_ship: Query<&mut Ship, (With<EnemyShip>, Without<PlayerShip>)>,
    mut event_holder: ResMut<RoomEventHolder>,
) {

    while ! event_holder.events.is_empty() {
        let event = event_holder.events.remove(0);

        event.preform(&mut player_ship, &mut enemy_ship, &mut rooms);        
    }
}

pub fn add_room_events (
    mut rooms: Query<&mut Room>,
    time: Res<Time>,
    mut event_holder: ResMut<RoomEventHolder>,
) {
    for mut room in rooms.iter_mut() {
        room.preform_actions(&time, &mut event_holder);
    }
}




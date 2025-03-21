

pub mod primary;

use bevy::prelude::*;

use primary::RoomPart;




#[derive(Component)]
pub struct Room {
    pub room_parts: Vec<RoomPart> 
}

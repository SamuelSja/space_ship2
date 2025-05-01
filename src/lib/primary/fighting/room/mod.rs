
pub mod primary_part;
pub mod primary_event;

use bevy::prelude::*;

use primary_event::RoomEventHolder;
use primary_part::RoomPart;




#[derive(Component)]
pub struct Room {
    pub room_parts: Vec<RoomPart>,
    pub player: bool,
}

impl Room {
    pub fn preform_actions(&mut self, time: &Res<Time>, event_holder: &mut ResMut<RoomEventHolder>) {
        for room_part in self.room_parts.iter_mut() {
            room_part.preform_actions(time, event_holder);
        }
    }





}

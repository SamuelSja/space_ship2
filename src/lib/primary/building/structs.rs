

use bevy::prelude::*;

use super::room::Room;


/// Ghost is used when the player is dragging a room onto the ship
#[derive(Component)]
pub struct Ghost {
    pub room: Room,
    pub entity: Entity,
}

impl Ghost {
    pub fn new(room: Room, entity: Entity) -> Self {
        Self {
            room,
            entity
        }
    }
}


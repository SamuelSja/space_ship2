

use bevy::prelude::*;

use super::{resources::Storage, room::Room};


/// Ghost is used when the player is dragging a room onto the ship
#[derive(Component)]
pub struct Ghost {
    pub room: Room,
    pub prev_loc: PrevLoc,
}

impl Ghost {
    pub fn new(room: Room, prev_loc: PrevLoc) -> Self {
        Self {
            room,
            prev_loc,
        }
    }
}

pub enum PrevLoc {
    None,
    Storage(usize),
    Entity(Entity),
} 


impl PrevLoc {


    /// panics if the entity is not valid
    pub fn despawn(&self, coms: &mut Commands, storage: &mut ResMut<Storage>) {
        match self {
            Self::None => (),
            Self::Storage(index) => { storage.rooms.remove(*index); },
            Self::Entity(entity) => coms.get_entity(*entity).expect("entity not valid").despawn(),
        }
    }
}
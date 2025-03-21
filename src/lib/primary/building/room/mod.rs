

pub mod primary;

use bevy::prelude::*;
use primary::RoomPart;




use crate::lib::primary::fighting;

type FightingRoom = fighting::room::Room;



#[derive(Component)]
pub struct Room {
    pub room_parts: Vec<RoomPart>,
    pub image: String,
}


impl Room {
    pub fn to_game(&self) -> FightingRoom {
        let room_parts = self.room_parts.iter().map(|part| part.to_game()).collect::<Vec<_>>();

        FightingRoom {
            room_parts
        } 
    }

    pub fn get_info(&self) -> Vec<(String, String)> {
        let mut ans = Vec::new();

        for part in self.room_parts.iter() {
            ans.extend(part.get_info().into_iter());
        }

        ans
    }
}


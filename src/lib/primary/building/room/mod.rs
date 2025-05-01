

pub mod primary;

use bevy::prelude::*;
use primary::RoomPart;




use crate::lib::primary::fighting::{self, structs::Ship};

use super::resources::ShipInfo;

type FightingRoom = fighting::room::Room;






#[derive(Component, Clone)]
pub struct Room {
    pub room_parts: Vec<RoomPart>,
    pub image: String,
    pub size: (u32, u32),
    pub pos: Option<(u32, u32)>,
}


impl Room {


    /// This turns the Building room into a Fighting room
    pub fn to_game(&self, coms: &mut Commands, assets: &Res<AssetServer>, ship: &Res<ShipInfo>, player: bool) -> FightingRoom {
        self.build_bevy_sprite(coms, assets, ship);

        let room_parts = self.room_parts.iter().map(|part| part.to_game()).collect::<Vec<_>>();

        FightingRoom {
            room_parts,
            player,
        }
    }



    /// This creates a sprite for the room
    pub fn build_bevy_sprite(&self, coms: &mut Commands, assets: &Res<AssetServer>, ship: &Res<ShipInfo>) -> Option<Entity> {

        if let Some((x, y)) = self.pos{
            let x = ship.start_pos.0 + x as f32 * ship.tile_size.0;
            let y = ship.start_pos.1 + y as f32 * ship.tile_size.1;


            Some(coms.spawn((
                Sprite {
                    image: assets.load(self.image.clone()),

                    ..default()
                },
                Transform::from_xyz(x, y, 0.0),
            )).id())
        } else {
            None
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


use bevy::prelude::*;

use crate::lib::primary::building::room::Room;





#[derive(Component)]
pub struct ItemInfo;

#[derive(Component)]
pub struct BuildingBar;

#[derive(Component)]
pub struct FightButton;

#[derive(Component)]
pub struct ExitButton;

// #[derive(Component)]
// pub struct RoomStorageInfo {
//     room: Room, //todo need a way to access Room by id that was pressed
// }
// 
// impl RoomStorageInfo {
//     pub fn new(room: Room) -> Self {
//         Self {
//             room
//         } 
//     }
// }

#[derive(Component)]
pub struct StoredRoom {
    pub index: usize,
}

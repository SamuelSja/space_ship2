
use bevy::prelude::*;

use super::{room::{primary::RoomPart, Room}, structs::Ghost};

#[derive(Resource)]
pub struct ShipInfo {
    pub start_pos: (f32, f32),
    pub tile_size: (f32, f32),
}

impl Default for ShipInfo {
    fn default() -> Self {
        Self {
            start_pos: (0.0, 0.0),
            tile_size: (50.0, 50.0),
        }
    }
}

#[derive(Resource)]
pub struct Storage {
    pub rooms: Vec<Room>,
}

impl Default for Storage {
    fn default() -> Self {
        Self {
            rooms: vec![
                Room {
                    room_parts: vec![
                        RoomPart::Base {
                            max_health: 10.0,
                        },
                        RoomPart::Gun {
                            damage: 2.0,
                            reload: 3.0,
                        },
                    ],
                    image: String::from("images/todo.png"),
                    pos: None,
                    size: (3, 2),
                },
                Room {
                    room_parts: vec![
                        RoomPart::Base {
                            max_health: 10.0,
                        },
                        RoomPart::Shield {
                            max_shield: 4,
                            reload: 10.0,
                        },
                    ],
                    image: String::from("images/todo.png"),
                    pos: None,
                    size: (3, 2),
                },
                Room {
                    room_parts: vec![
                        RoomPart::Base {
                            max_health: 10.0,
                        },
                        RoomPart::Reactor {
                            power: 3,
                        },
                    ],
                    image: String::from("images/todo.png"),
                    pos: None,
                    size: (3, 2),
                }
            ] 
        }
    }
}




#[derive(Resource)]
pub struct Selected {
    pub val: Selectable,
}

impl Default for Selected {
    fn default() -> Self {
        Self {
            val: Selectable::None,
        }
    }
}

pub enum Selectable {
    None,
    Room(Entity),
    Ghost(Entity),
}

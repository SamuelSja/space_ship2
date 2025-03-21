
use bevy::prelude::*;

use super::room::{primary::RoomPart, Room};



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
                }
            ] 
        }
    }
}


#[derive(Resource)]
pub struct Selected {
    pub val: Option<Room>,
}

impl Default for Selected {
    fn default() -> Self {
        Self {
            val: None,
        }
    }
}

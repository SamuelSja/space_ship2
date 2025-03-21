
use bevy::prelude::*;



pub enum RoomPart {
    Base {
        health: f32,
        max_health: f32,
        power: u32,
    },
    Gun {
        damage: f32,
        reload_time: f32,
        cur_reload: f32,
    },
    Shield {
        reload_time: f32,
        max_shield: u32,
        cur_reload: f32,
    },
    Reactor {
        power: u32,
    }
}




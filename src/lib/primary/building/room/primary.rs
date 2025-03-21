

use bevy::prelude::*;

use crate::lib::primary::fighting;

type FightingRoomPart = fighting::room::primary::RoomPart;

#[derive(Clone)]
pub enum RoomPart {
    Base {
        max_health: f32,
    },
    Gun {
        damage: f32,
        reload: f32,
    },
    Shield {
        max_shield: u32,
        reload: f32,
    },
    Reactor {
        power: u32,
    }
}

impl RoomPart {

    // Todo: could probably use a macro to make this easier
    pub fn get_info(&self) -> Vec<(String, String)> {
        let mut ans = Vec::new();

        match self {
            Self::Base { max_health } => ans.push(("max_health", format!("{max_health}"))),
            Self::Gun { damage, reload } => {
                ans.push(("damage", format!("{damage}")));
                ans.push(("reload", format!("{reload}")));
            },
            Self::Shield { max_shield, reload } => {
                ans.push(("max_shield", format!("{max_shield}")));
                ans.push(("reload", format!("{reload}")));
            },
            Self::Reactor { power } => ans.push(("power", format!("{power}"))),
        }

        ans.into_iter().map(|(str1, str2)| { (String::from(str1), String::from(str2)) }).collect::<Vec<_>>()
    }

    pub fn to_game(&self) -> FightingRoomPart {
        match self {
            Self::Base { max_health } => {
                FightingRoomPart::Base {
                    health: *max_health,
                    max_health: *max_health,
                    power: 0,
                }
            },
            Self::Gun { damage, reload} => {
                FightingRoomPart::Gun {
                    damage: *damage,
                    reload_time: *reload,
                    cur_reload: *reload,
                }
            },
            Self::Shield { max_shield, reload } => {
                FightingRoomPart::Shield {
                    reload_time: *reload,
                    max_shield: *max_shield,
                    cur_reload: *reload,
                }
            },
            Self::Reactor { power } => {
                FightingRoomPart::Reactor {
                    power: *power,
                }
            }
        }
    }
}


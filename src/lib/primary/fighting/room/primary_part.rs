
use bevy::prelude::*;

use super::primary_event::{RoomEvent, RoomEventHolder};



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
        target: Option<Entity>,
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


impl RoomPart {
    pub fn get_info(&self) -> Vec<String> {
        match self {
            Self::Base { health, max_health, power } => {
                vec! [
                    format!("health: {health}"),
                    format!("max_health: {max_health}"),
                    format!("power: {power}"),
                ]
            },
            Self::Gun { damage, reload_time, cur_reload, target} => {
                vec! [
                    format!("damage: {damage}"),
                    format!("reload_time: {reload_time}"),
                    format!("cur_reload: {cur_reload}"),
                    format!("target: {target:?}"),
                ]
            },
            Self::Shield { reload_time, max_shield, cur_reload } => {
                vec! [
                    format!("reload_time: {reload_time}"),
                    format!("max_shield: {max_shield}"),
                    format!("cur_reload: {cur_reload}"),
                ]
            },
            Self::Reactor { power } => {
                vec! [
                    format!("power: {power}"),
                ]
            },
        }
    }

    pub fn preform_actions(&mut self, time: &Res<Time>, event_holder: &mut ResMut<RoomEventHolder>) {
        match self {
            Self::Base { health, max_health, power } => (),
            Self::Gun { damage, reload_time, cur_reload, target } => {

                if let Some(target) = target {

                    *cur_reload -= time.delta_secs();

                    while *cur_reload <= 0.0 {
                        event_holder.events.push(
                            RoomEvent::Damage {
                                id: *target,
                                amount: *damage,
                            }
                        );

                        *cur_reload += *reload_time;
                    }
                } else {
                    *cur_reload = (*cur_reload - time.delta_secs()).max(0.0);
                }

            },
            Self::Shield { reload_time, max_shield, cur_reload } => {

            },
            Self::Reactor { power } => {
                
            },
        }


    }


}





use bevy::prelude::*;

use crate::lib::primary::fighting::structs::{EnemyShip, PlayerShip, Ship};

use super::{primary_part::RoomPart, Room};



pub enum RoomEvent {
    Damage { id: Entity, amount: f32 }
}


impl RoomEvent {
    pub fn preform (
        &self,
        player_ship: &mut Query<&mut Ship, With<PlayerShip>>,
        enemy_ship: &mut Query<&mut Ship, (With<EnemyShip>, Without<PlayerShip>)>,
        rooms: &mut Query<&mut Room>,
    ) {

        let mut player_ship = player_ship.get_single_mut().expect("in preform in RoomEvent. Tried to access Ship, but could not because it does not exist.");
        let mut enemy_ship = enemy_ship.get_single_mut().expect("in preform in RoomEvent. Tried to access Ship, but could not because it does not exist.");


        match self {
            Self::Damage {id, amount} => {
                if let Ok(mut room) = rooms.get_mut(*id) {
                    let hit_shield = if room.player {
                        player_ship.hit_shield(*amount)
                    } else {
                        enemy_ship.hit_shield(*amount)
                    };

                    if hit_shield {
                        return;
                    }

                    // todo: could improve the efficiency by making room_parts a hashmap

                    let parts = &mut room.room_parts;

                    let mut left_over = 0.0;

                    for mut part in parts {
                        if let RoomPart::Base {max_health: _, health, power: _} = &mut part {
                            // todo: possible error. Will health change?
                            *health -= amount;

                            if *health < 0.0 {
                                left_over = -*health;
                                *health = 0.0;
                            }

                            break;
                        }
                    }

                    if room.player {
                        player_ship.health -= left_over;
                    } else {
                        enemy_ship.health -= left_over;
                    }
                }
            },
        }
    }
}




#[derive(Resource)]
pub struct RoomEventHolder {
    pub events: Vec<RoomEvent>, 
}

impl Default for RoomEventHolder {
    fn default() -> Self {
        Self {
            events: Vec::new(),
        }
    }
}











pub mod gui;
pub mod structs;
pub mod systems;
pub mod room;

use bevy::prelude::*;
use gui::GUIPlug;
use room::primary_event::RoomEventHolder;
use systems::{add_room_events, run_room_events};

use super::states::AppState;


pub struct FightingPlug;
impl Plugin for FightingPlug {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(GUIPlug)
        .init_resource::<RoomEventHolder>()
        .add_systems(Update, (
            add_room_events,
            run_room_events,
        ).run_if(in_state(AppState::Fighting)))
        ; 
    }
}

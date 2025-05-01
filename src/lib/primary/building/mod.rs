
pub mod gui;
pub mod structs;
pub mod systems;
pub mod room;
pub mod resources;

use bevy::prelude::*;
use gui::GUIPlug;
use resources::{Selected, ShipInfo, Storage};
use systems::{drop_ghost, ghost_follow, pick_up_building, pick_up_room};

use super::states::AppState;


pub struct BuildingPlug;
impl Plugin for BuildingPlug {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(GUIPlug)
        .init_resource::<Storage>()
        .init_resource::<Selected>()
        .init_resource::<ShipInfo>()
        .add_systems(Update, (
            pick_up_building,
            pick_up_room,
            ghost_follow,
            drop_ghost
        ).run_if(in_state(AppState::Building)))
        ; 
    }
}
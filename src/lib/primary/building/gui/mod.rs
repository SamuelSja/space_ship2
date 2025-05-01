





pub mod layout;
pub mod styles;
pub mod systems;
pub mod structs;


use bevy::prelude::*;

use layout::{build_gui, despawn_gui};
use systems::{fight, update_building_bar, update_item_info};
use crate::lib::primary::states::AppState;



pub struct GUIPlug;
impl Plugin for GUIPlug {
    fn build(&self, app: &mut bevy::app::App) {
        app
        .add_systems(OnEnter(AppState::Building), build_gui) 
        .add_systems(OnExit(AppState::Building), despawn_gui) 
        .add_systems(Update, (
            update_building_bar,
            update_item_info,
            fight,
        ).run_if(in_state(AppState::Building)))
        
        
        ;
    }
}



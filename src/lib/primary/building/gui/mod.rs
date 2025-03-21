





pub mod layout;
pub mod styles;
pub mod systems;
pub mod structs;


use bevy::prelude::*;

use layout::build_gui;
use systems::{/*set_up_building_bar,*/ update_building_bar, update_item_info};
use crate::lib::primary::states::AppState;



pub struct GUIPlug;
impl Plugin for GUIPlug {
    fn build(&self, app: &mut bevy::app::App) {
        app
        .add_systems(OnEnter(AppState::Building), build_gui) 
        .add_systems(Update, update_building_bar)
        .add_systems(Update, update_item_info)
        ;
    }
}




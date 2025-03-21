
pub mod gui;
pub mod structs;
pub mod systems;
pub mod room;
pub mod resources;

use bevy::prelude::*;
use gui::GUIPlug;
use resources::{Selected, Storage};


pub struct BuildingPlug;
impl Plugin for BuildingPlug {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(GUIPlug)
        .init_resource::<Storage>()
        .init_resource::<Selected>()
        ; 
    }
}


pub mod layout;
pub mod styles;
pub mod systems;
pub mod event_system;


use bevy::prelude::*;
use event_system::EventSystemPlug;

pub struct GUIPlug;
impl Plugin for GUIPlug {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(EventSystemPlug)
        ; 
    }
}
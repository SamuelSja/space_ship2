

pub mod building;
pub mod fighting;
pub mod states;
pub mod systems;

use bevy::prelude::*;
use building::BuildingPlug;
use fighting::FightingPlug;
use states::AppState;
use systems::add_camera;

pub struct PrimaryPlug;
impl Plugin for PrimaryPlug {
    fn build(&self, app: &mut App) {
        app
        .init_state::<AppState>()
        .add_plugins(BuildingPlug)
        .add_plugins(FightingPlug)
        .add_systems(Startup, add_camera)
        
        ; 
    }
}



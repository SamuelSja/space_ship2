

use bevy::prelude::*;



#[derive(Component)]
pub struct GUIEditor {
    id: Entity,
    gui_type: GUIType,
}





// todo: this
pub enum GUIType {
    Power {
        max_power: u32,
        cur_power: u32,
    },
    Aim {
        // target: Entity,
        holding: bool,
    },
}

impl GUIType {
    /// preforms the GUI check. This could be checking if the player has set a new target or check if the player has changed the power level
    pub fn check(&mut self,
        id: Entity,
        event_holder: &mut ResMut<GUIEventHolder>,
        mouse_input: &Res<ButtonInput<MouseButton>>,
    ) {
        match self {
            Self::Power {
                max_power,
                cur_power,
            } => {
                // todo: this
            },
            Self::Aim {
                // target,
                holding,
            } => {
                // todo: figure out the position of the target icon that the player needs to drag



                if mouse_input.pressed(MouseButton::Left) {
                    // todo: check mouse position
                    if true {
                        *holding = true; 
                    }
                } else {
                    if *holding {
                        // todo: find target




                    }
                }


            }
        }
    }

    /// Inserts the gui of the GUIEditor into the GUI
    pub fn build_gui(&mut self, p: &mut ChildBuilder, assets: &Res<AssetServer>) {
        match self {
            Self::Power {max_power, cur_power} => {
                // todo: this
            },
            Self::Aim {holding} => {
                // todo: this
            }
        }
    }


}



enum GUIEvent {
    SetTarget {
        id: Entity,
        target: Entity,
    },
    SetPower {
        id: Entity,
        power: u32,
    },
}

#[derive(Resource)]
struct GUIEventHolder {
    events: Vec<GUIEvent>,
}



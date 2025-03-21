

use bevy::prelude::*;



pub fn add_camera (
    mut coms: Commands,
) {
    coms.spawn(Camera2d {});
}



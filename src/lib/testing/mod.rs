




use bevy::{diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, prelude::*};









pub struct TestPlug;
impl Plugin for TestPlug {
    fn build(&self, app: &mut App) {
        app
        .add_plugins((
            FrameTimeDiagnosticsPlugin::default(),
            LogDiagnosticsPlugin::default(),
        ))
        ; 
    }
}




pub fn test_test() {
    println!("test");
}


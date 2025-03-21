


pub mod primary;
pub mod testing;


use bevy::prelude::*;
use primary::PrimaryPlug;
use testing::TestPlug;

pub fn run() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
    .add_plugins(PrimaryPlug)

    ;

    #[cfg(debug_assertions)]
    app.add_plugins(TestPlug);

    app.run();
}





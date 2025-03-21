

use bevy::prelude::*;


#[derive(States, Debug, Hash, Eq, PartialEq, Clone)]
pub enum AppState {
    Building,
    Fighting,
}

impl Default for AppState {
    fn default() -> Self {
        Self::Building
    }
}




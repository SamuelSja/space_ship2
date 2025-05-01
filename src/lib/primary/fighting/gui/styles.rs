
use bevy::prelude::*;


pub fn main_style() -> Node {
    Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        ..default() 
    }
}

pub fn bottom_bar_style() -> Node {
    Node {
        width: Val::Percent(100.0),
        height: Val::Percent(20.0),
        flex_direction: FlexDirection::Row,
        ..default()
    }
}




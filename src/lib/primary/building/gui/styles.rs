

use bevy::prelude::*;




pub fn main_style() -> Node {
    Node {
        width: Val::Percent(100.0), 
        height: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        ..default()
    }
}

pub fn upper_section_style() -> Node {
    Node {
        width: Val::Percent(100.0),
        height: Val::Percent(70.0),
        flex_direction: FlexDirection::Row,
        ..default()
    }
}

pub fn lower_section_style() -> Node {
    Node {
        width: Val::Percent(100.0),
        height: Val::Percent(30.0),
        flex_direction: FlexDirection::Row,
        ..default()
    }
}

pub fn item_info_style() -> Node {
    Node {
        width: Val::Percent(30.0),
        height: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        ..default()
    }
}


pub fn building_bar_style() -> Node {
    Node {
        width: Val::Percent(70.0),
        height: Val::Percent(100.0),
        flex_direction: FlexDirection::Row,
        overflow: Overflow::scroll_x(),
        ..default()
    }
}

pub fn button_box_style() -> Node {
    Node {
        width: Val::Percent(30.0),
        height: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        align_content: AlignContent::SpaceAround,
        ..default()
    }
}

pub fn button_style() -> Node {
    Node {
        width: Val::Percent(80.0),
        height: Val::Percent(30.0),
        align_content: AlignContent::SpaceAround,
        ..default()
    }
}

pub fn storage_item_style() -> Node {
    Node {
        width: Val::Auto,
        height: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        align_content: AlignContent::SpaceAround,
        ..default()
    }
}

pub fn storage_info_style() -> Node {
    Node {
        width: Val::Percent(100.0),
        height: Val::Percent(10.0),
        ..default()
    }
}




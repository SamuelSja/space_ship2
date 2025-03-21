
use bevy::{ecs::query::QueryData, prelude::*};

use crate::lib::primary::building::resources::{Selected, Storage};

use super::{layout::insert_storage_item, structs::{BuildingBar, ItemInfo}};


pub fn update_building_bar (
    mut coms: Commands,
    building_bar_q: Query<Entity, With<BuildingBar>>,
    storage: Res<Storage>,
    assets: Res<AssetServer>,
) {
    for entity in building_bar_q.iter() {

        // Unwrap: entity exists because we are not doing anything weird
        let mut entity = coms.get_entity(entity).unwrap();
        entity.despawn_descendants();
        entity.with_children(|p| {
            for room in &storage.rooms {
                insert_storage_item(p, &assets, &room);
            }

        });
    }
}

// pub fn update_building_bar (
//     mut coms: Commands,
//     building_bar_q: Query<Entity, With<BuildingBar>>,
//     storage: Res<Storage>,
//     assets: Res<AssetServer>,
// ) {
//     for entity in building_bar_q.iter() {
//         // Unwrap: entity exists because we are not doing anything weird
//         let mut entity = coms.get_entity(entity).unwrap();
//         entity.with_children(|p| {
//             for room in &storage.rooms {
//                 insert_storage_item(p, &assets, &room);
//             }
// 
//         });
//     }
// }
// 
// pub fn set_up_building_bar (
//     coms: Commands,
//     building_bar_q: Query<Entity, With<BuildingBar>>,
//     storage: Res<Storage>,
//     assets: Res<AssetServer>,
// ) {
//     update_building_bar(coms, building_bar_q, storage, assets);
// }


pub fn update_item_info (
    mut text_q: Query<&mut Text, With<ItemInfo>>,
    selected: Res<Selected>,
) {

    
    if let Some(selected) = &selected.val {
        for mut text in text_q.iter_mut() {
            let data = selected.get_info(); 

            let mut ans = String::from("");
            for item in data {
                ans += &format!("{}: {}", item.0, item.1);
            }

            text.0 = ans;
        }
    } else {
        for mut text in text_q.iter_mut() {
            text.0 = String::from("");
        }
    }
}





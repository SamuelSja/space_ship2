
use bevy::prelude::*;

use crate::lib::primary::building::room::Room;
use crate::lib::primary::building::systems::pick_up_building;
// use crate::lib::primary::building::systems::test_click;

use super::styles::*;
use super::structs::*;


pub fn build_gui (
    mut coms: Commands,
    assets: Res<AssetServer>,
) {
    println!("build gui");

    coms.spawn((main_style(), BuildingRoot {}))
    .with_children(|p| {
        p.spawn(upper_section_style())
        .with_children(|p| {
            p.spawn(item_info_style())
            .insert(BackgroundColor(Color::linear_rgb(1.0, 0.0, 0.0)))

            .with_children(|p| {
                p.spawn(Text::new("<todo>"))
                .insert(ItemInfo {});
            })

            ;
        });

        p.spawn(lower_section_style())
        .with_children(|p| {
            p.spawn(building_bar_style())
            .insert(BuildingBar {})
            .insert(BackgroundColor(Color::linear_rgb(0.0, 1.0, 0.0)))

            ;
            p.spawn(button_box_style())
            .with_children(|p| {
                // Todo: could be DRYer
                p.spawn(button_style())
                .insert((
                    Button,
                    FightButton,
                ))
                .with_children(|p| {
                    p.spawn(Text::new("Fight"));
                });

                // Todo: could be DRYer
                p.spawn(button_style())
                .insert((
                    Button,
                    ExitButton,
                ))
                .with_children(|p| {
                    p.spawn(Text::new("Exit"));
                });
            })
            
            .insert(BackgroundColor(Color::linear_rgb(0.0, 0.0, 1.0)))

            ;
        });
    });

}


pub fn despawn_gui (
    mut coms: Commands,
    root_q: Query<Entity, With<BuildingRoot>>,
) {
    for entity in root_q.iter() {
        coms.get_entity(entity).unwrap().despawn_recursive()
    }  
}


pub fn insert_storage_item(p: &mut ChildBuilder<'_>, assets: &Res<AssetServer>, room: &Room, index: usize) {
    let entity = p.spawn(storage_item_style())
    .insert(room.clone())
    .insert(StoredRoom { index })
    .insert(Button::default())


    // .insert(Mesh2d)
    // .observe(pick_up_building)


    // .observe(test_click)


    .with_children(|p| {

        p.spawn(ImageNode::new(assets.load(room.image.clone())));

        for (key, val) in room.get_info() {
            p.spawn(storage_info_style())
            .insert(Text::new(format!("{}: {}", key, val)))
            
            ;
        }
    }).id();



}







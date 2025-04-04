

use bevy::{input::mouse::{MouseButtonInput, MouseMotion}, prelude::*, window::PrimaryWindow};

use super::{gui::structs::StoredRoom, resources::{Selectable, Selected, ShipInfo, Storage}, room::Room, structs::Ghost};


// pub fn test_click (
//     clicked: Trigger<Pointer<Click>>,
// ) {
//     println!("clicked");
// }

/// picks up a building from the gui and turns it into a Ghost
pub fn pick_up_building (
    button_q: Query<(&Room, Entity, &Interaction), (With<StoredRoom>, Changed<Interaction>)>,
    mut coms: Commands,
    assets: Res<AssetServer>,
    mut selected: ResMut<Selected>,
) {
    for (room, entity, interaction) in button_q.iter() {
        if let Interaction::Pressed = interaction {
            let entity = coms.spawn((
                Sprite {
                    image: assets.load(room.image.clone()),
                    ..default()
                },
                Ghost::new(room.clone(), entity),
            )).id();

            selected.val = Selectable::Ghost(entity);
        }
    }
}

/// makes ghosts follow the mouse
pub fn ghost_follow (
    mut ghost_q: Query<(&Ghost, &mut Transform)>,
    mut window_q: Query<&Window, With<PrimaryWindow>>,
    rooms_q: Query<&Room>,
    ship_info: Res<ShipInfo>,
) {
    if let Ok(window) = window_q.get_single() {
        for (ghost, mut transform) in ghost_q.iter_mut() {
            if let Some(mut mouse_pos) = window.cursor_position() {

                mouse_pos.x -= window.width() / 2.0;
                mouse_pos.y -= window.height() / 2.0;
                mouse_pos.y *= -1.0;

                let pos = find_pos(&ship_info, ghost, &Vec3::new(mouse_pos.x, mouse_pos.y, 0.0), &rooms_q);

                if let Some(pos) = pos {
                    let (x, y) = ship_to_global(&ship_info, pos);
                    transform.translation = Vec3::new(x, y, 0.0);
                } else {
                    transform.translation = Vec3::new(mouse_pos.x, mouse_pos.y, 0.0);
                }
            }
        }
    }
}


/// Drops the ghost onto the ship
pub fn drop_ghost (
    mut coms: Commands,
    mut ghost_q: Query<(&Ghost, &Transform, Entity)>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    ship_info: Res<ShipInfo>,
    rooms_q: Query<&Room>,
    assets: Res<AssetServer>,
) {
    if mouse_input.just_released(MouseButton::Left) {
        for (ghost, transform, entity) in ghost_q.iter() {

            let pos = find_pos(&ship_info, ghost, &transform.translation, &rooms_q);

            if let Some((x, y)) = pos {
                // todo: remove room from storage

                let mut room = ghost.room.clone();
                room.pos = Some((x, y));


                let (x, y) = ship_to_global(&ship_info, (x, y));
                let image_name = room.image.clone();

                coms.spawn((
                    Sprite {
                        image: assets.load(image_name),
                        ..default()
                    },
                    Transform::from_xyz(x, y, 0.0),
                    room,
                ));

            }

            // unwrap: for loop requires entity
            coms.get_entity(entity).unwrap().despawn();
        }
    }
}

/// Maps the ship position((u32, u32)) to the transform position((f32, f32))
pub fn ship_to_global (
    ship_info: &Res<ShipInfo>,
    pos: (u32, u32),
) -> (f32, f32) {

    let (x, y) = pos;

    let pos = ship_info.start_pos;

    let size = ship_info.tile_size;

    let x = pos.0 + size.0 * (x as f32);
    let y = pos.1 + size.1 * (y as f32);

    (x, y)
}

/// gives the snap location if any
pub fn find_pos (
    ship_info: &Res<ShipInfo>,
    ghost: &Ghost,
    translation: &Vec3,
    rooms_q: &Query<&Room>,
) -> Option<(u32, u32)> {
    let size = ship_info.tile_size;
    let start = ship_info.start_pos;

    let x_start = (translation.x - start.0) / size.0;
    let y_start = (translation.y - start.1) / size.1;

    if x_start < -0.5 || y_start < -0.5 {
        return None;
    }

    let x_start = (x_start + 0.5) as u32;
    let y_start = (y_start + 0.5) as u32; 
        
    let start = (x_start, y_start); 
    let size = ghost.room.size;
    let end = (start.0 + size.0, start.1 + size.1);

    let mut open = true;

    for room in rooms_q.iter() {
        if let Some(pos) = room.pos {
            let size = room.size;
            let other_end = (pos.0 + size.0, pos.1 + size.1);

            open &= ! aabb2d(start, end, pos, other_end);
        }
    }

    if ! open {
        None
    } else {
        Some(start)
    }
}

/// Checks if 2d u32 recs are colliding (true if colliding)
pub fn aabb2d(first_start: (u32, u32), first_end: (u32, u32), second_start: (u32, u32), second_end: (u32, u32)) -> bool {
    aabb1d(first_start.0, first_end.0, second_start.0, second_end.0)
    && aabb1d(first_start.1, first_end.1, second_start.1, second_end.1)

}

/// Checks if 1d u32 recs are colliding (true if colliding)
pub fn aabb1d(first_start: u32, first_end: u32, second_start: u32, second_end: u32) -> bool {
    first_start <= second_start && second_start < first_end 
    || second_start <= first_start && first_start < second_end
}

mod test {

    use super::*;
}





// pub fn drag_building (
//     drag: Trigger<Pointer<Drag>>,
//     mut ghost_q: Query<(&mut Transform, &Ghost, Entity)>,
// ) {
// 
// 
//     let targ = drag.target;
// 
//     for (mut transform, ghost, entity) in ghost_q.iter_mut() {
//         if targ != entity {
//             continue;
//         }
// 
//         let drag_dist = drag.distance;
// 
//         transform.translation += Vec3::new(drag_dist.x, drag_dist.y, 0.0);
//     }
// 
//     // drag.distance
// }
// 
// 
// pub fn place_building (
//     place: Trigger<Pointer<DragDrop>>,
//     mut coms: Commands,
// ) {
// 
// }












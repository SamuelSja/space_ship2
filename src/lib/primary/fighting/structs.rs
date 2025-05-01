
use bevy::prelude::*;



#[derive(Component)]
pub struct Ship {
    pub health: f32,
    pub shield: f32,

    pub pos: (f32, f32),
    pub tile_size: (f32, f32),
}

impl Ship {
    pub fn hit_shield(&mut self, amount: f32) -> bool {
        if self.shield <= 0.0 {
            return false;
        }

        self.shield -= amount;
        self.shield = self.shield.max(0.0);

        true
    }
}


#[derive(Component)]
pub struct PlayerShip;

#[derive(Component)]
pub struct EnemyShip;
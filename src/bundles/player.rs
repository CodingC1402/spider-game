use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::{Collider, Sensor};

use crate::components::player::*;

use super::physics::RigidBodyBundle;

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    pub sprite: SpriteSheetBundle,
    pub physic: RigidBodyBundle,
    pub player: Player,
    pub jump: PlayerJump,
    pub info: PlayerInfo,
    pub movement: PlayerMovement,
    pub name: Name,
}

#[derive(Bundle, Default)]
pub struct PlayerFootBundle {
    pub transform: TransformBundle,
    pub collider: Collider,
    pub sensor: Sensor,
    pub foot_comp: PlayerFoot,
    pub name: Name,
}

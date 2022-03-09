use bevy::prelude::*;

pub struct BulletCollisionEvent {
    pub translation: Vec3,
    pub collision_type: CollisionType,
}

pub enum CollisionType {
    Aircraft,
    Paratrooper,
}

pub struct GunshotEvent;

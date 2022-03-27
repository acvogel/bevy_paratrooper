use bevy::prelude::*;

pub struct BulletCollisionEvent {
    pub translation: Vec3,
    pub collision_type: CollisionType,
}

pub struct GunExplosionEvent {
    pub translation: Vec3,
}

#[derive(PartialEq)]
pub enum CollisionType {
    Aircraft,
    Paratrooper,
}

pub struct GunshotEvent;

pub struct LandingEvent;

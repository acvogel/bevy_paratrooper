use bevy::prelude::*;

pub struct BulletCollisionEvent {
    pub translation: Vec3,
    pub collision_type: CollisionType,
}

#[derive(PartialEq)]
pub enum CollisionType {
    Aircraft,
    Paratrooper,
}

pub struct GunshotEvent;

pub struct LandingEvent;

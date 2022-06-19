use bevy::prelude::*;

pub struct BulletCollisionEvent {
    pub translation: Vec3,
    pub collision_type: CollisionType,
    pub bullet_entity: Entity,
    pub target_entity: Entity,
}

pub struct GunExplosionEvent {
    pub translation: Vec3,
}

#[derive(PartialEq)]
pub enum CollisionType {
    Aircraft,
    Paratrooper,
    Parachute,
}

pub struct GunshotEvent;

pub struct LandingEvent(pub Entity);

/// Animation events.
pub struct ExplosionEvent {
    pub transform: Transform,
}

pub struct GibEvent {
    pub transform: Transform,
}

pub struct EventPlugin;

impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ExplosionEvent>()
            .add_event::<GibEvent>()
            .add_event::<GunExplosionEvent>()
            .add_event::<BulletCollisionEvent>()
            .add_event::<GunshotEvent>()
            .add_event::<LandingEvent>();
    }
}

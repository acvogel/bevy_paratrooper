use bevy::prelude::*;
//use bevy_rapier2d::prelude::ContactEvent;
use bevy_rapier2d::prelude::*;

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

/// Helper to match starting collisions to entities
pub fn entities_collision_started(contact_event: ContactEvent, e1: Entity, e2: Entity) -> bool {
    if let ContactEvent::Started(handle1, handle2) = contact_event {
        handle1.entity() == e1 && handle2.entity() == e2
            || handle2.entity() == e1 && handle1.entity() == e2
    } else {
        false
    }
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

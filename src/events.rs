use bevy::prelude::*;

#[derive(Event)]
pub struct BulletCollisionEvent {
    pub translation: Vec3,
    pub collision_type: CollisionType,
    pub bullet_entity: Entity,
    pub target_entity: Entity,
}

#[derive(Event)]
pub struct GunExplosionEvent {
    pub translation: Vec3,
}

#[derive(PartialEq, Event)]
pub enum CollisionType {
    Aircraft,
    Paratrooper,
    Parachute,
    Bomb,
}

#[derive(Event)]
pub struct GunshotEvent;

#[derive(Event)]
pub struct LandingEvent(pub Entity);

#[derive(Event)]
pub struct BombDropEvent;

/// Animation events.
#[derive(Event)]
pub struct ExplosionEvent {
    pub transform: Transform,
    pub explosion_type: ExplosionType,
}

#[derive(Component, Clone, Copy, PartialEq)]
pub enum ExplosionType {
    Aircraft,
    Bullet,
    Bomb,
}

#[derive(Event)]
pub struct GibEvent {
    pub transform: Transform,
}

pub struct EventPlugin;

impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ExplosionEvent>()
            .add_event::<BombDropEvent>()
            .add_event::<GibEvent>()
            .add_event::<GunExplosionEvent>()
            .add_event::<BulletCollisionEvent>()
            .add_event::<GunshotEvent>()
            .add_event::<LandingEvent>();
    }
}

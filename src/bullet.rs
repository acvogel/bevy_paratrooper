use crate::aircraft::Aircraft;
use crate::bomber::Bomb;
use crate::consts::{OUT_OF_BOUNDS_X, OUT_OF_BOUNDS_Y};
use crate::events::*;
use crate::gun::Gun;
use crate::paratrooper::{Parachute, Paratrooper};
use crate::{consts, AppState};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::collections::HashSet;

#[derive(Component, Default)]
pub struct Bullet;

struct BulletTextures {
    bullet_handle: Handle<Image>,
}

fn setup_bullets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(BulletTextures {
        bullet_handle: asset_server.load("images/bullet.png"),
    });
}

fn shoot_gun(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Gun, &Transform)>,
    time: Res<Time>,
    mut event_writer: EventWriter<GunshotEvent>,
    bullet_textures: Res<BulletTextures>,
) {
    if keyboard_input.pressed(KeyCode::Space) {
        for (mut gun, transform) in query.iter_mut() {
            if time.seconds_since_startup() - gun.last_fired > consts::GUN_COOLDOWN {
                event_writer.send(GunshotEvent);
                gun.last_fired = time.seconds_since_startup();

                // Spawn bullet
                let mut bullet_transform = *transform;
                bullet_transform.translation.z -= 0.1;
                bullet_transform.translation += 30. * bullet_transform.local_y();

                let sprite_bundle = SpriteBundle {
                    texture: bullet_textures.bullet_handle.clone(),
                    sprite: Sprite {
                        custom_size: Some(Vec2::splat(24.)),
                        ..Default::default()
                    },
                    transform: bullet_transform,
                    ..Default::default()
                };

                // velocity vector is local_y
                let local_y = bullet_transform.local_y();
                let velocity_vector = consts::BULLET_SPEED * Vec2::new(local_y.x, local_y.y);

                commands
                    .spawn()
                    .insert_bundle(sprite_bundle)
                    .insert(RigidBody::Dynamic)
                    .insert(bullet_transform)
                    .insert(Velocity {
                        linvel: velocity_vector, //[velocity_vector.x, velocity_vector.y],
                        angvel: 0.0,
                    })
                    .insert(Collider::cuboid(12., 12.))
                    .insert(
                        ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_STATIC,
                    )
                    .insert(ActiveEvents::COLLISION_EVENTS)
                    .insert(CollisionGroups::new(0b0010, 0b1101))
                    .insert(LockedAxes::ROTATION_LOCKED)
                    .insert(Bullet);
            }
        }
    }
}

/// Generates BulletCollisionEvents
fn bullet_collision_system(
    mut collision_events: EventReader<CollisionEvent>,
    mut event_writer: EventWriter<BulletCollisionEvent>,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
    paratrooper_query: Query<(Entity, &Transform), With<Paratrooper>>,
    aircraft_query: Query<(Entity, &Transform), With<Aircraft>>,
    parachute_query: Query<(Entity, &Transform), With<Parachute>>,
    bomb_query: Query<&Transform, With<Bomb>>,
) {
    let mut bullet_handles = HashSet::new();
    for (bullet, _transform) in bullet_query.iter() {
        bullet_handles.insert(bullet);
    }
    for collision_event in collision_events.iter() {
        if let CollisionEvent::Started(entity1, entity2, _) = collision_event {
            if bullet_handles.contains(entity1) || bullet_handles.contains(entity2) {
                let (&bullet_entity, &target_entity) = if bullet_handles.contains(entity1) {
                    (entity1, entity2)
                } else {
                    (entity2, entity1)
                };

                if let Ok(bomb_transform) = bomb_query.get(target_entity) {
                    event_writer.send(BulletCollisionEvent {
                        collision_type: CollisionType::Bomb,
                        translation: bomb_transform.translation,
                        bullet_entity,
                        target_entity,
                    })
                }

                // Aircraft
                for (aircraft_entity, aircraft_transform) in aircraft_query.iter() {
                    if aircraft_entity == target_entity {
                        event_writer.send(BulletCollisionEvent {
                            collision_type: CollisionType::Aircraft,
                            translation: aircraft_transform.translation,
                            bullet_entity,
                            target_entity,
                        });
                    }
                }

                // Parachutes
                for (parachute_entity, parachute_transform) in parachute_query.iter() {
                    if parachute_entity == target_entity {
                        event_writer.send(BulletCollisionEvent {
                            collision_type: CollisionType::Parachute,
                            translation: parachute_transform.translation,
                            bullet_entity,
                            target_entity,
                        });
                    }
                }

                // Paratroopers
                for (paratrooper_entity, paratrooper_transform) in paratrooper_query.iter() {
                    if paratrooper_entity == target_entity {
                        event_writer.send(BulletCollisionEvent {
                            collision_type: CollisionType::Paratrooper,
                            translation: paratrooper_transform.translation,
                            bullet_entity,
                            target_entity,
                        });
                    }
                }
            }
        }
    }
}

fn bullet_collision_listener(
    mut commands: Commands,
    query: Query<&Transform, With<Bullet>>,
    mut event_reader: EventReader<BulletCollisionEvent>,
    mut event_writer: EventWriter<ExplosionEvent>,
) {
    for event in event_reader.iter() {
        if event.collision_type == CollisionType::Aircraft {
            if let Ok(transform) = query.get(event.bullet_entity) {
                event_writer.send(ExplosionEvent {
                    transform: *transform,
                    explosion_type: ExplosionType::Bullet,
                });
                commands.entity(event.bullet_entity).despawn_recursive();
            }
        }
    }
}

/// Remove "out-of-bounds" bullets
fn despawn_escaped_bullets(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Bullet>>,
) {
    for (entity, transform) in query.iter() {
        if transform.translation.x.abs() > OUT_OF_BOUNDS_X
            || transform.translation.y.abs() > OUT_OF_BOUNDS_Y
        {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn despawn_all_bullets(mut commands: Commands, query: Query<Entity, With<Bullet>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_bullets)
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(shoot_gun)
                    .with_system(bullet_collision_system)
                    .with_system(bullet_collision_listener)
                    .with_system(despawn_escaped_bullets),
            )
            .add_system_set(SystemSet::on_exit(AppState::InGame).with_system(despawn_all_bullets));
    }
}

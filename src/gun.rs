use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::collections::HashSet;

use crate::paratrooper::Paratrooper;
use crate::{consts, GunExplosionEvent};

const ANGULAR_VELOCITY: f32 = 3.0;

#[derive(Component)]
pub struct Gun {
    pub last_fired: f64,
}

#[derive(Component)]
pub struct GunBase;

pub fn setup_gun_base(mut commands: Commands) {
    let h = 64.;
    let w = 64.;
    let y = consts::GROUND_Y + 0.5 * h;
    let sprite_bundle = SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(-0.1, 0.1, 0.1),
            custom_size: Some(Vec2::new(w, h)),
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(0., y, 2.)),
        ..Default::default()
    };
    let body = RigidBodyBundle {
        body_type: RigidBodyTypeComponent(RigidBodyType::Static),
        position: [0., y].into(),
        ..Default::default()
    };
    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(0.5 * w, 0.5 * h).into(),
        ..Default::default()
    };
    commands
        .spawn_bundle(sprite_bundle)
        .insert_bundle(body)
        .insert_bundle(collider)
        .insert(GunBase);
}

pub fn setup_gun_rapier(mut commands: Commands) {
    let y = consts::GROUND_Y + 64.;
    let sprite_size = Vec2::new(20., 60.);
    let sprite_bundle = SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.32, 0.36, 0.41),
            custom_size: Some(sprite_size),
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(0., y, 1.)),
        ..Default::default()
    };
    let body_bundle = RigidBodyBundle {
        body_type: RigidBodyTypeComponent(RigidBodyType::KinematicVelocityBased),
        position: [0., y].into(),
        mass_properties: RigidBodyMassPropsFlags::TRANSLATION_LOCKED.into(),
        ..Default::default()
    };
    let collider_bundle = ColliderBundle {
        shape: ColliderShape::cuboid(0.5 * sprite_size.x, 0.5 * sprite_size.y).into(),
        collider_type: ColliderType::Sensor.into(),
        ..Default::default()
    };
    commands
        .spawn_bundle(body_bundle)
        .insert(RigidBodyPositionSync::Discrete)
        .insert_bundle(sprite_bundle)
        .insert_bundle(collider_bundle)
        .insert(Gun { last_fired: 0. });
}

fn move_gun_rapier(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut RigidBodyVelocityComponent, &RigidBodyPositionComponent), With<Gun>>,
) {
    let angular_velocity = ANGULAR_VELOCITY;
    let boundary_angle = -std::f32::consts::PI / 2.5; // right boundary
    for (mut rb_vel, rb_pos) in query.iter_mut() {
        let gun_angle = rb_pos.position.rotation.angle();
        if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
            if gun_angle < -1. * boundary_angle {
                rb_vel.angvel = angular_velocity;
            }
        } else if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
            if gun_angle > boundary_angle {
                rb_vel.angvel = -angular_velocity;
            }
        }
    }
}

fn gun_collision_system(
    mut event_reader: EventReader<IntersectionEvent>,
    mut event_writer: EventWriter<GunExplosionEvent>,
    gun_query: Query<(Entity, &Transform), With<Gun>>,
    paratrooper_query: Query<Entity, With<Paratrooper>>,
) {
    let mut paratrooper_entities = HashSet::new();
    for paratrooper_entity in paratrooper_query.iter() {
        paratrooper_entities.insert(paratrooper_entity);
    }
    for (gun_entity, gun_transform) in gun_query.iter() {
        for event in event_reader.iter() {
            if (event.collider1.entity() == gun_entity
                && paratrooper_entities.contains(&event.collider2.entity()))
                || (event.collider2.entity() == gun_entity
                    && paratrooper_entities.contains(&event.collider1.entity()))
            {
                // Game over.
                info!("gun explosion event!! {:?}", event);
                event_writer.send(GunExplosionEvent {
                    translation: gun_transform.translation,
                });
            }
        }
    }
}

pub struct GunPlugin;

impl Plugin for GunPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_gun_rapier)
            .add_startup_system(setup_gun_base)
            .add_system(move_gun_rapier)
            .add_system(gun_collision_system);
    }
}

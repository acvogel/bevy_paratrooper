use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use std::collections::HashSet;

use crate::menu::AttackState;
use crate::paratrooper::Paratrooper;
use crate::{consts, AppState, GunExplosionEvent};

const ANGULAR_VELOCITY: f32 = 2.5;

#[derive(Component)]
pub struct Gun {
    pub last_fired: f64,
}

#[derive(Component)]
pub struct GunBase;

#[derive(Component)]
pub struct GunMount;

const GUN_BASE_X: f32 = 64.;
const GUN_BASE_Y: f32 = 38.;

const GUN_MOUNT_X: f32 = 24.;
const GUN_MOUNT_Y: f32 = 30.;

const GUN_HEIGHT: f32 = 40.;
const GUN_WIDTH: f32 = 10.;

pub fn setup_gun_base(mut commands: Commands) {
    let h = GUN_BASE_Y;
    let w = GUN_BASE_X;
    let y = consts::GROUND_Y + 0.5 * h;
    // todo replace with lyon shape?
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
        material: ColliderMaterial {
            restitution: 0.,
            restitution_combine_rule: CoefficientCombineRule::Min,
            friction: 0.0,
            friction_combine_rule: CoefficientCombineRule::Min,
            ..Default::default()
        }
        .into(),
        ..Default::default()
    };
    commands
        .spawn_bundle(sprite_bundle)
        .insert_bundle(body)
        .insert_bundle(collider)
        .insert(GunBase);
}

fn gun_mount_circle_shape() -> ShapeBundle {
    let mount_circle_shape = shapes::Circle {
        radius: GUN_MOUNT_X / 2.,
        center: Vec2::ZERO,
    };
    GeometryBuilder::build_as(
        &mount_circle_shape,
        DrawMode::Fill(FillMode::color(Color::PINK)),
        Transform::from_xyz(0., GUN_MOUNT_Y / 2.0, 2.0),
    )
}

fn gun_mount_rectangle_shape() -> ShapeBundle {
    let mount_rectangle_shape = shapes::Rectangle {
        extents: Vec2::new(GUN_MOUNT_X, GUN_MOUNT_Y),
        origin: RectangleOrigin::Center,
    };
    let rectangle_y = consts::GROUND_Y + GUN_BASE_Y + 0.5 * GUN_MOUNT_Y;
    GeometryBuilder::build_as(
        &mount_rectangle_shape,
        DrawMode::Fill(FillMode::color(Color::PINK)),
        Transform::from_xyz(0., rectangle_y, 2.0),
    )
}

pub fn setup_gun_mount(mut commands: Commands) {
    commands
        .spawn_bundle(gun_mount_rectangle_shape())
        .with_children(|parent| {
            parent.spawn_bundle(gun_mount_circle_shape());
        })
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyTypeComponent(RigidBodyType::Static),
            position: Vec2::new(0., consts::GROUND_Y + GUN_BASE_Y + 0.5 * GUN_MOUNT_Y).into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(0.5 * GUN_MOUNT_X, 0.5 * GUN_MOUNT_Y).into(),
            collider_type: ColliderType::Sensor.into(),
            ..Default::default()
        })
        .insert(GunMount);
}

pub fn setup_gun_barrel(mut commands: Commands) {
    let y = consts::GROUND_Y + GUN_BASE_Y + GUN_MOUNT_Y + 0.5 * GUN_HEIGHT;
    let sprite_size = Vec2::new(GUN_WIDTH, GUN_HEIGHT);
    let sprite_bundle = SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.32, 0.36, 0.41),
            custom_size: Some(sprite_size),
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(0., y, 1.)),
        ..Default::default()
    };
    // center of mass calculation.
    let body_bundle = RigidBodyBundle {
        body_type: RigidBodyTypeComponent(RigidBodyType::KinematicVelocityBased),
        position: [0., y].into(),
        mass_properties: RigidBodyMassPropsFlags::TRANSLATION_LOCKED.into(),
        ..Default::default()
    };
    let collider_bundle = ColliderBundle {
        shape: ColliderShape::cuboid(0.5 * sprite_size.x, 0.5 * sprite_size.y).into(),
        collider_type: ColliderType::Sensor.into(),
        mass_properties: MassProperties {
            local_com: Vec2::new(0., -GUN_HEIGHT / 2.0).into(),
            inv_mass: 0.1,
            inv_principal_inertia_sqrt: 0.1,
        }
        .into(),
        ..Default::default()
    };
    commands
        .spawn_bundle(body_bundle)
        .insert(RigidBodyPositionSync::Discrete)
        .insert_bundle(sprite_bundle)
        .insert_bundle(collider_bundle)
        .insert(Gun { last_fired: 0. });
}

fn move_gun(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut RigidBodyVelocityComponent, &RigidBodyPositionComponent), With<Gun>>,
) {
    let angular_velocity = ANGULAR_VELOCITY;
    let boundary_angle = -std::f32::consts::PI / 2.9; // right boundary
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
    gun_mount_query: Query<(Entity, &Transform), With<GunMount>>,
    paratrooper_query: Query<Entity, With<Paratrooper>>,
) {
    let mut paratrooper_entities = HashSet::new();
    for paratrooper_entity in paratrooper_query.iter() {
        paratrooper_entities.insert(paratrooper_entity);
    }
    let (gun_mount_entity, gun_mount_transform) = gun_mount_query.get_single().unwrap();
    for (gun_entity, gun_transform) in gun_query.iter() {
        for event in event_reader.iter() {
            if ((event.collider1.entity() == gun_entity
                || event.collider1.entity() == gun_mount_entity)
                && paratrooper_entities.contains(&event.collider2.entity()))
                || ((event.collider2.entity() == gun_entity
                    || event.collider2.entity() == gun_mount_entity)
                    && paratrooper_entities.contains(&event.collider1.entity()))
            {
                // Game over.
                event_writer.send(GunExplosionEvent {
                    translation: gun_transform.translation.clone(),
                });
                event_writer.send(GunExplosionEvent {
                    translation: gun_mount_transform.translation.clone(),
                });
            }
        }
    }
}

pub struct GunPlugin;

impl Plugin for GunPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::MainMenu)
                .with_system(setup_gun_base)
                .with_system(setup_gun_mount)
                .with_system(setup_gun_barrel),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame(AttackState::Air))
                .with_system(move_gun)
                .with_system(gun_collision_system),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame(AttackState::Ground))
                .with_system(move_gun)
                .with_system(gun_collision_system),
        );
    }
}

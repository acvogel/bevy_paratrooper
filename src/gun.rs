use bevy::prelude::*;
use bevy::sprite::Anchor::BottomCenter;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::FillMode;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use std::collections::HashSet;

use crate::bomber::Bomb;
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
const GUN_BASE_Y: f32 = 50.;

const GUN_MOUNT_X: f32 = 24.;
const GUN_MOUNT_Y: f32 = 18.;

const GUN_HEIGHT: f32 = 35.;
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
    commands
        .spawn_bundle(sprite_bundle)
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(0.5 * w, 0.5 * h))
        .insert(Friction {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        })
        .insert(Restitution {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        })
        .insert(ActiveEvents::COLLISION_EVENTS)
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
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(0.5 * GUN_MOUNT_X, 0.5 * GUN_MOUNT_Y))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .with_children(|parent| {
            parent.spawn().insert_bundle(gun_mount_circle_shape());
        })
        .insert(GunMount);
}

pub fn setup_gun_barrel(mut commands: Commands) {
    let y = consts::GROUND_Y + GUN_BASE_Y + GUN_MOUNT_Y; //+ 0.5 * GUN_HEIGHT;
    let sprite_size = Vec2::new(GUN_WIDTH, GUN_HEIGHT);
    let sprite_bundle = SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.32, 0.36, 0.41),
            custom_size: Some(sprite_size),
            anchor: BottomCenter,
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(0., y, 1.)),
        ..Default::default()
    };
    commands
        .spawn()
        .insert_bundle(sprite_bundle)
        .insert(RigidBody::KinematicVelocityBased)
        .insert(Collider::cuboid(0.5 * sprite_size.x, 0.5 * sprite_size.y))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Velocity::default())
        .insert(MassProperties {
            mass: 1.0,
            principal_inertia: 0.1,
            ..Default::default()
        })
        .insert(Gun { last_fired: 0. });
}

/// Move gun with keyboard, within bounds.
fn move_gun(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &Transform), With<Gun>>,
) {
    let angular_velocity = ANGULAR_VELOCITY;
    let boundary_angle = std::f32::consts::PI / 2.9; // right boundary
    for (mut velocity, transform) in query.iter_mut() {
        let (gun_axis, gun_angle) = transform.rotation.to_axis_angle();
        if (keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left))
            && (gun_axis.z <= 0. || gun_angle < boundary_angle)
        {
            velocity.angvel = angular_velocity;
        } else if (keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right))
            && (gun_axis.z >= 0. || gun_angle < boundary_angle)
        {
            velocity.angvel = -angular_velocity;
        } else {
            velocity.angvel = 0.
        }
    }
}

/// Paratrooper or bombing collision explodes gun
fn gun_collision_system(
    mut event_reader: EventReader<CollisionEvent>,
    mut event_writer: EventWriter<GunExplosionEvent>,
    gun_query: Query<(Entity, &Transform), With<Gun>>,
    gun_mount_query: Query<(Entity, &Transform), With<GunMount>>,
    paratrooper_query: Query<Entity, With<Paratrooper>>,
    bomb_query: Query<Entity, With<Bomb>>,
) {
    let (gun_entity, gun_transform) = gun_query.get_single().expect("No gun entity.");
    let (_gun_mount_entity, gun_mount_transform) =
        gun_mount_query.get_single().expect("No gun mount.");
    for &collision_event in event_reader.iter() {
        if let CollisionEvent::Started(entity1, entity2, _) = collision_event {
            if entity1 == gun_entity || entity2 == gun_entity {
                let other_entity = if entity1 == gun_entity {
                    entity2
                } else {
                    entity1
                };

                if bomb_query.contains(other_entity) || paratrooper_query.contains(other_entity) {
                    // Game over.
                    event_writer.send(GunExplosionEvent {
                        translation: gun_transform.translation,
                    });
                    event_writer.send(GunExplosionEvent {
                        translation: gun_mount_transform.translation,
                    });
                }
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
            SystemSet::on_update(AppState::InGame)
                .with_system(move_gun)
                .with_system(gun_collision_system),
        );
    }
}

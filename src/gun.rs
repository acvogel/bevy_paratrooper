use bevy::prelude::*;
use bevy::sprite::Anchor::BottomCenter;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use std::collections::HashSet;

use crate::bomber::Bomb;
use crate::paratrooper::Paratrooper;
use crate::{consts, AppState, GunExplosionEvent};

const ANGULAR_VELOCITY: f32 = 2.5;

/// Right-side angle boundary
const BOUNDARY_ANGLE: f32 = std::f32::consts::PI / 2.9;

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
    let sprite_bundle = SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(-0.1, 0.1, 0.1),
            custom_size: Some(Vec2::new(w, h)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0., y, 2.)),
        ..default()
    };
    commands
        .spawn(sprite_bundle)
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
        .insert(CollisionGroups::new(Group::GROUP_4, Group::ALL))
        .insert(GunBase);
}

pub fn setup_gun_mount(mut commands: Commands) {
    let mount_rectangle_shape = shapes::Rectangle {
        extents: Vec2::new(GUN_MOUNT_X, GUN_MOUNT_Y),
        origin: RectangleOrigin::Center,
    };
    let rectangle_y = consts::GROUND_Y + GUN_BASE_Y + 0.5 * GUN_MOUNT_Y;

    let mount_circle_shape = shapes::Circle {
        radius: GUN_MOUNT_X / 2.,
        center: Vec2::ZERO,
    };

    commands
        .spawn(ShapeBundle {
            path: GeometryBuilder::build_as(&mount_rectangle_shape),
            ..default()
        })
        .insert(Fill::color(Color::PINK))
        .insert(Transform::from_xyz(0., rectangle_y, 2.0))
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(0.5 * GUN_MOUNT_X, 0.5 * GUN_MOUNT_Y))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(CollisionGroups::new(Group::GROUP_4, Group::ALL))
        .with_children(|parent| {
            parent
                .spawn(ShapeBundle {
                    path: GeometryBuilder::build_as(&mount_circle_shape),
                    ..default()
                })
                .insert(Fill::color(Color::PINK))
                .insert(Transform::from_xyz(0., GUN_MOUNT_Y / 2.0, 2.0));
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
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0., y, 1.)),
        ..default()
    };
    commands
        .spawn(sprite_bundle)
        .insert(RigidBody::KinematicVelocityBased)
        .insert(Collider::cuboid(0.5 * sprite_size.x, 0.5 * sprite_size.y))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(CollisionGroups::new(Group::GROUP_4, Group::ALL))
        .insert(Velocity::default())
        .insert(AdditionalMassProperties::MassProperties(MassProperties {
            mass: 1.0,
            principal_inertia: 0.1,
            ..default()
        }))
        .insert(Gun { last_fired: 0. });
}

/// Move gun with keyboard or gamepad, within bounds.
fn move_gun(
    gamepads: Res<Gamepads>,
    button_inputs: Res<ButtonInput<GamepadButton>>,
    keyboard_inputs: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &Transform), With<Gun>>,
) {
    // Resolve keyboard and gamepad inputs
    let keyboard_left = keyboard_inputs.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
    let keyboard_right = keyboard_inputs.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);
    let gamepad_right = gamepads
        .iter()
        .find(|&gamepad| {
            button_inputs.pressed(GamepadButton::new(gamepad, GamepadButtonType::DPadRight))
        })
        .is_some();
    let gamepad_left = gamepads
        .iter()
        .find(|&gamepad| {
            button_inputs.pressed(GamepadButton::new(gamepad, GamepadButtonType::DPadLeft))
        })
        .is_some();
    let any_left = keyboard_left || gamepad_left;
    let any_right = keyboard_right || gamepad_right;

    // Rotate the gun
    let (mut velocity, transform) = query.get_single_mut().expect("Gun entity not found!");
    let (gun_axis, gun_angle) = transform.rotation.to_axis_angle();
    velocity.angvel = if any_left && any_right {
        0.
    } else if any_left && (gun_axis.z <= 0. || gun_angle < BOUNDARY_ANGLE) {
        ANGULAR_VELOCITY
    } else if any_right && (gun_axis.z >= 0. || gun_angle < BOUNDARY_ANGLE) {
        -ANGULAR_VELOCITY
    } else {
        0.
    }
}

/// Bomb colliding with any part of gun assembly causes explosion
fn gun_bomb_collision_system(
    mut event_reader: EventReader<CollisionEvent>,
    mut event_writer: EventWriter<GunExplosionEvent>,
    gun_query: Query<(Entity, &Transform), With<Gun>>,
    gun_mount_query: Query<(Entity, &Transform), With<GunMount>>,
    gun_base_query: Query<(Entity, &Transform), With<GunBase>>,
    bombs_query: Query<Entity, With<Bomb>>,
) {
    let (gun_entity, gun_transform) = gun_query.get_single().expect("No gun entity.");
    let (gun_mount_entity, gun_mount_transform) =
        gun_mount_query.get_single().expect("No gun mount entity.");
    let (gun_base_entity, _gun_base_transform) =
        gun_base_query.get_single().expect("No gun base entity.");
    let targets = HashSet::from([gun_entity, gun_mount_entity, gun_base_entity]);
    for &collision_event in event_reader.read() {
        if let CollisionEvent::Started(entity1, entity2, _flags) = collision_event {
            let is_bomb_collision = bombs_query.contains(entity1) || bombs_query.contains(entity2);
            let is_gun_collision = targets.contains(&entity1) || targets.contains(&entity2);
            if is_bomb_collision && is_gun_collision {
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

/// Landed paratrooper colliding with gun or mount causes explosion
fn gun_paratrooper_collision_system(
    mut event_reader: EventReader<CollisionEvent>,
    mut event_writer: EventWriter<GunExplosionEvent>,
    gun_query: Query<(Entity, &Transform), With<Gun>>,
    gun_mount_query: Query<(Entity, &Transform), With<GunMount>>,
    paratroopers_query: Query<Entity, With<Paratrooper>>,
) {
    let (gun_entity, gun_transform) = gun_query.get_single().expect("No gun entity.");
    let (gun_mount_entity, gun_mount_transform) =
        gun_mount_query.get_single().expect("No gun mount entity.");
    let targets = HashSet::from([gun_entity, gun_mount_entity]);
    for &collision_event in event_reader.read() {
        if let CollisionEvent::Started(entity1, entity2, _flags) = collision_event {
            let is_bomb_collision =
                paratroopers_query.contains(entity1) || paratroopers_query.contains(entity2);
            let is_gun_collision = targets.contains(&entity1) || targets.contains(&entity2);
            if is_bomb_collision && is_gun_collision {
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

/// Stop gun rotation.
fn stop_gun(mut gun_query: Query<&mut Velocity, With<Gun>>) {
    let mut velocity = gun_query.get_single_mut().expect("Gun velocity not found.");
    velocity.angvel = 0.;
}

pub struct GunPlugin;

impl Plugin for GunPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::MainMenu),
            (setup_gun_base, setup_gun_mount, setup_gun_barrel),
        )
        .add_systems(
            Update,
            (
                move_gun,
                gun_bomb_collision_system,
                gun_paratrooper_collision_system,
            )
                .run_if(in_state(AppState::InGame)),
        )
        .add_systems(OnEnter(AppState::GameOver), stop_gun);
    }
}

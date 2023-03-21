use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::consts::{OUT_OF_BOUNDS_X, OUT_OF_BOUNDS_Y, WINDOW_WIDTH};
use crate::{AppState, BulletCollisionEvent, ExplosionEvent, ExplosionType};
use rand::Rng;

pub const AIRCRAFT_SPEED: f32 = 80.;
pub const AIRCRAFT_SCALE: f32 = 0.3;
pub const AIRCRAFT_SPAWN_PROBABILITY: f32 = 0.008;
pub const SPAWN_LEFT_X: f32 = -WINDOW_WIDTH / 2.0 - 40.;
pub const SPAWN_RIGHT_X: f32 = WINDOW_WIDTH / 2.0 + 40.;
pub const SPAWN_Y_MIN: f32 = 100.;
pub const SPAWN_Y_MAX: f32 = 350.;
const PARATROOPER_STICK_SIZE: usize = 5; // Max number of paratroopers dropped per aircraft

#[derive(Component)]
pub struct Aircraft {
    pub paratroopers: usize,
}
impl Default for Aircraft {
    fn default() -> Aircraft {
        Aircraft {
            paratroopers: PARATROOPER_STICK_SIZE,
        }
    }
}

#[derive(Resource)]
struct AircraftTextures {
    image_handle: Handle<Image>,
}

fn spawn_aircraft_system(mut commands: Commands, aircraft_textures: Res<AircraftTextures>) {
    let mut rng = rand::thread_rng();
    if rng.gen_range(0.0..1.0) < AIRCRAFT_SPAWN_PROBABILITY {
        let y = rng.gen_range(SPAWN_Y_MIN..SPAWN_Y_MAX);
        let heading_right = rng.gen_bool(0.5);
        let speed = rng.gen_range(0.8..1.3) * AIRCRAFT_SPEED;
        let multiplier = if heading_right { 1.0 } else { -1.0 };
        let velocity = multiplier * speed;
        let transform = if heading_right {
            Transform::from_translation(Vec3::new(SPAWN_LEFT_X, y, 3.))
        } else {
            Transform::from_translation(Vec3::new(SPAWN_RIGHT_X, y, 3.))
        }
        .with_scale(Vec3::new(AIRCRAFT_SCALE, AIRCRAFT_SCALE, 1.));

        let sprite_bundle = SpriteBundle {
            // 412 x 114 pixels. 0.3 scale.
            texture: aircraft_textures.image_handle.clone(),
            sprite: Sprite {
                flip_x: !heading_right,
                ..Default::default()
            },
            ..Default::default()
        };

        commands
            .spawn(sprite_bundle)
            .insert(transform)
            .insert(RigidBody::Dynamic)
            .insert(Collider::cuboid(412. / 2.0, 114. / 2.0))
            .insert(Sensor)
            .insert(ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_STATIC)
            .insert(CollisionGroups::new(
                Group::from_bits(0b0100).unwrap(),
                Group::from_bits(0b1110).unwrap(),
            ))
            .insert(LockedAxes::TRANSLATION_LOCKED_Y)
            .insert(Velocity {
                linvel: Vec2::new(velocity, 0.),
                angvel: 0.0,
            })
            .insert(Aircraft::default());
    }
}

fn setup_aircraft_system(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    commands.insert_resource(AircraftTextures {
        image_handle: asset_server.load("images/paraplane1.png"),
    });
}

fn despawn_all_aircraft(mut commands: Commands, query: Query<Entity, With<Aircraft>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn despawn_escaped_aircraft(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Aircraft>>,
) {
    for (entity, transform) in query.iter() {
        if transform.translation.x.abs() > OUT_OF_BOUNDS_X
            || transform.translation.y.abs() > OUT_OF_BOUNDS_Y
        {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn bullet_collision_system(
    mut commands: Commands,
    aircraft_query: Query<(Entity, &Transform), With<Aircraft>>,
    mut event_reader: EventReader<BulletCollisionEvent>,
    mut event_writer: EventWriter<ExplosionEvent>,
) {
    for event in event_reader.iter() {
        if let Ok((aircraft_entity, aircraft_transform)) = aircraft_query.get(event.target_entity) {
            event_writer.send(ExplosionEvent {
                transform: (*aircraft_transform).with_scale(Vec3::ONE),
                explosion_type: ExplosionType::Aircraft,
            });
            commands.entity(aircraft_entity).despawn_recursive();
        }
    }
}

pub struct AircraftPlugin;

impl Plugin for AircraftPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_aircraft_system)
            .add_systems(
                (
                    spawn_aircraft_system,
                    bullet_collision_system,
                    despawn_escaped_aircraft,
                )
                    .in_set(OnUpdate(AppState::InGame)),
            )
            .add_system(despawn_all_aircraft.in_schedule(OnExit(AppState::InGame)));
    }
}

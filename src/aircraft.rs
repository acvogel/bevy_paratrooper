use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{AppState, BulletCollisionEvent, ExplosionEvent};
use rand::Rng;

#[derive(Component)]
pub struct Aircraft;

const AIRCRAFT_SPEED: f32 = 40.;
const AIRCRAFT_SPAWN_PROBABILITY: f32 = 0.008;
const SPAWN_LEFT_X: f32 = -600.;
const SPAWN_RIGHT_X: f32 = 600.;

struct AircraftTextures {
    image_handle: Handle<Image>,
}

fn spawn_aircraft_system(mut commands: Commands, aircraft_textures: Res<AircraftTextures>) {
    let mut rng = rand::thread_rng();
    if rng.gen_range(0.0..1.0) < AIRCRAFT_SPAWN_PROBABILITY {
        let y = rng.gen_range(0.0..350.0);
        let heading_right = rng.gen_bool(0.5);
        let speed = rng.gen_range(0.8..1.3) * AIRCRAFT_SPEED;
        let multiplier = if heading_right { 1.0 } else { -1.0 };
        let velocity = multiplier * speed;
        let transform = if heading_right {
            Transform::from_translation(Vec3::new(SPAWN_LEFT_X, y, 5.))
        } else {
            Transform::from_translation(Vec3::new(SPAWN_RIGHT_X, y, 5.))
        }
        .with_scale(Vec3::splat(0.3));

        let sprite_bundle = SpriteBundle {
            // 412 x 114 pixels. 0.3 scale.
            texture: aircraft_textures.image_handle.clone(),
            sprite: Sprite {
                flip_x: !heading_right,
                ..Default::default()
            },
            transform: transform.with_scale(Vec3::splat(0.3)),

            ..Default::default()
        };

        let rigid_body_bundle = RigidBodyBundle {
            body_type: RigidBodyTypeComponent(RigidBodyType::Dynamic),
            position: [transform.translation.x, transform.translation.y].into(),
            velocity: RigidBodyVelocity {
                linvel: Vec2::new(velocity, 0.0).into(),
                angvel: 0.0,
            }
            .into(),
            mass_properties: RigidBodyMassProps {
                flags: RigidBodyMassPropsFlags::TRANSLATION_LOCKED_Y,
                local_mprops: MassProperties::new(Vec2::ZERO.into(), 10.0, 1.0),
                ..Default::default()
            }
            .into(),
            ..Default::default()
        };

        // 412 x 114 asset. 0.3 scale yields (123.6, 34.2)
        let collider_bundle = ColliderBundle {
            collider_type: ColliderType::Sensor.into(),
            shape: ColliderShape::cuboid(123.6 / 2.0, 34.2 / 2.0).into(),
            flags: ColliderFlags {
                collision_groups: InteractionGroups::new(0b0100, 0b1110),

                active_collision_types: ActiveCollisionTypes::all(),
                active_events: ActiveEvents::all(),
                ..Default::default()
            }
            .into(),
            ..Default::default()
        };

        commands
            .spawn_bundle(rigid_body_bundle)
            .insert(RigidBodyPositionSync::Discrete)
            .insert_bundle(collider_bundle)
            .insert_bundle(sprite_bundle)
            .insert(Aircraft);
    }
}

fn setup_aircraft_system(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    commands.insert_resource(AircraftTextures {
        image_handle: asset_server.load("paraplane1.png"),
    });
}

fn despawn_all_aircraft(mut commands: Commands, query: Query<Entity, With<Aircraft>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
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
                transform: aircraft_transform.clone(),
            });
            commands.entity(aircraft_entity).despawn_recursive();
        }
    }
}

pub struct AircraftPlugin;

impl Plugin for AircraftPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::InGame).with_system(setup_aircraft_system),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(spawn_aircraft_system)
                .with_system(bullet_collision_system),
        )
        .add_system_set(SystemSet::on_exit(AppState::InGame).with_system(despawn_all_aircraft));
    }
}

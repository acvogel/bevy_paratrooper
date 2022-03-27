use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

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

fn spawn_aircraft_system_rapier(mut commands: Commands, aircraft_textures: Res<AircraftTextures>) {
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

//fn spawn_aircraft_system(mut commands: Commands, aircraft_textures: Res<AircraftTextures>) {
//    if should_spawn_aircraft() {
//        let aircraft = create_aircraft();
//
//        commands
//            .spawn_bundle(SpriteBundle {
//                // 412 x 114 pixels. 0.3 scale.
//                texture: aircraft_textures.image_handle.clone(),
//                sprite: Sprite {
//                    //custom_size: Some(Vec2::new(412., 114.)),
//                    flip_x: aircraft.velocity_x < 0.,
//                    ..Default::default()
//                },
//                transform: aircraft.position.with_scale(Vec3::splat(0.3)),
//
//                ..Default::default()
//            })
//            .insert(aircraft);
//    }
//}

//fn create_aircraft() -> Aircraft {
//    let mut rng = rand::thread_rng();
//    let y = rng.gen_range(0. ..350.); // TODO clearance lanes to avoid existing planes, unless same direction
//    let heading_right = rng.gen_bool(0.5);
//    let speed = rng.gen_range(0.8..1.3) * AIRCRAFT_SPEED;
//    if heading_right {
//        Aircraft {
//            velocity_x: speed,
//            position: Transform::from_translation(Vec3::new(SPAWN_LEFT_X, y, 5.)),
//        }
//    } else {
//        Aircraft {
//            velocity_x: -speed,
//            position: Transform::from_translation(Vec3::new(SPAWN_RIGHT_X, y, 5.)),
//        }
//    }
//}

//fn fly_aircraft(time: Res<Time>, mut query: Query<(&mut Aircraft, &mut Transform)>) {
//    for (mut aircraft, mut transform) in query.iter_mut() {
//        transform.translation.x += aircraft.velocity_x * time.delta_seconds();
//        aircraft.position.translation.x += aircraft.velocity_x * time.delta_seconds();
//    }
//}

///// detect when they have left the screen, count those in score.
//fn despawn_aircraft(
//    mut commands: Commands,
//    mut score: ResMut<Score>,
//    mut query: Query<(Entity, &Aircraft, &Transform)>,
//) {
//    for (e, _aircraft, transform) in query.iter_mut() {
//        if transform.translation.x >= SPAWN_RIGHT_X + 1.
//            || transform.translation.x <= SPAWN_LEFT_X - 1.
//        {
//            score.aircraft_escapes += 1;
//            commands.entity(e).despawn();
//            info!("ESCAPE {}", score.aircraft_escapes);
//        }
//    }
//}

fn setup_aircraft_system(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    commands.insert_resource(AircraftTextures {
        image_handle: asset_server.load("gfx/planes/paraplane1.png"),
    });
}

/// Listen for bullet collisions, despawn airplanes if hit.
//fn aircraft_collision_system(
//    mut commands: Commands,
//    mut event_reader: EventReader<BulletCollisionEvent>,
//    aircraft: Query<Aircraft>,
//) {
//    // filter to aircraft collisions, then get corresponding aircraft and despawn.
//    for event in event_reader.iter() {
//        if event.collision_type == CollisionType::Aircraft {
//
//        }
//    }
//}

pub struct AircraftPlugin;

impl Plugin for AircraftPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_aircraft_system)
            .add_system(spawn_aircraft_system_rapier);
        //.add_system(aircraft_collision_system);
        //.add_system(spawn_aircraft_system)
        //.add_system(despawn_aircraft)
        //.add_system(fly_aircraft);
    }
}

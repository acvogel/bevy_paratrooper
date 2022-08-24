use crate::aircraft::Aircraft;
use crate::terrain::Ground;
use crate::{
    AppState, BulletCollisionEvent, CollisionType, ExplosionEvent, ExplosionType, GibEvent,
    LandingEvent,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

const PARATROOPER_SPAWN_PROBABILITY: f32 = 0.007;
const PARACHUTE_SPAWN_PROBABILITY: f32 = 0.01;
const PARACHUTE_DAMPING: f32 = 1.0; // 100% air resistance
const MIN_PARACHUTE_VELOCITY: f32 = -70.; // meters / second
const PARACHUTE_GRAVITY_SCALE: f32 = 2.0;
const PARATROOPER_SCALE: f32 = 0.5;

// 31 x 49 texture, scaled
pub const PARATROOPER_X: f32 = PARATROOPER_SCALE * 31.;
pub const PARATROOPER_Y: f32 = PARATROOPER_SCALE * 49.;

const PARATROOPER_COLLISION_MEMBERSHIP: u32 = 0b0001;
pub const PARATROOPER_COLLISION_FILTER: u32 = 0b1110;

const PARATROOPER_SPAWN_X_MAX: f32 = 400.;
const PARATROOPER_SPAWN_X_MIN: f32 = 50.;
const PARATROOPER_SPAWN_VELOCITY: f32 = -100.;

const PARATROOPER_Z: f32 = 2.0;

#[derive(Component)]
pub struct Paratrooper {
    pub state: ParatrooperState,
    pub has_deployed_chute: bool,
}
impl Default for Paratrooper {
    fn default() -> Paratrooper {
        Paratrooper {
            state: ParatrooperState::Falling,
            has_deployed_chute: false,
        }
    }
}

#[derive(Component)]
pub struct Parachute;

#[derive(PartialEq)]
pub enum ParatrooperState {
    Falling,
    Floating,
    Landed,
    Assault,
}
struct ParatrooperTextures {
    pub body_handle: Handle<Image>,      // 31 x 49
    pub parachute_handle: Handle<Image>, // 89 x 86
}

/// Load paratrooper textures
fn setup_paratroopers(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ParatrooperTextures {
        body_handle: asset_server.load("images/paratrooperfly1_body.png"),
        parachute_handle: asset_server.load("images/paratrooperfly1_parachute.png"),
    });
}

///// Spawn several paratroopers for debugging assaults
//#[allow(dead_code)]
//fn spawn_paratroopers_debug(
//    mut commands: Commands,
//    paratrooper_textures: Res<ParatrooperTextures>,
//) {
//    let y = 100.;
//    let positions = [
//        Vec2::new(60., y),
//        Vec2::new(70., y),
//        Vec2::new(80., y),
//        Vec2::new(90., y),
//    ];
//    for position in positions {
//        commands
//            .spawn_bundle(paratrooper_rigid_body_bundle(position))
//            .insert(RigidBodyPositionSync::Discrete)
//            .insert_bundle(paratrooper_collider_bundle())
//            .insert_bundle(paratrooper_sprite_bundle(&paratrooper_textures))
//            .insert(Paratrooper::default());
//    }
//}

fn paratrooper_sprite_bundle(paratrooper_textures: &Res<ParatrooperTextures>) -> SpriteBundle {
    SpriteBundle {
        texture: paratrooper_textures.body_handle.clone(),
        ..Default::default()
    }
}

// Dynamic parachutes version
fn spawn_paratroopers(
    mut commands: Commands,
    paratrooper_textures: Res<ParatrooperTextures>,
    mut query: Query<(&mut Aircraft, &Transform, &Velocity)>,
) {
    let mut rng = rand::thread_rng();
    for (mut aircraft, transform, velocity) in query.iter_mut() {
        let pos_x = transform.translation.x.abs();
        if aircraft.paratroopers > 0
            && pos_x < PARATROOPER_SPAWN_X_MAX
            && pos_x > PARATROOPER_SPAWN_X_MIN
            && rng.gen_range(0.0..1.0) < PARATROOPER_SPAWN_PROBABILITY
        {
            aircraft.paratroopers -= 1;
            // Offset to back of plane
            let heading = velocity.linvel.x.signum();
            let paratrooper_pos = Vec2::new(
                transform.translation.x - heading * 35.,
                transform.translation.y - 25.,
            );

            commands
                .spawn()
                .insert_bundle(paratrooper_sprite_bundle(&paratrooper_textures))
                .insert(Transform {
                    translation: Vec3::new(paratrooper_pos.x, paratrooper_pos.y, PARATROOPER_Z),
                    rotation: Quat::IDENTITY,
                    scale: Vec3::new(PARATROOPER_SCALE, PARATROOPER_SCALE, 1.),
                })
                .insert(RigidBody::Dynamic)
                .insert(Sensor(false))
                .insert(Damping::default())
                .insert(GravityScale(1.0))
                .insert(LockedAxes::ROTATION_LOCKED)
                .insert(MassProperties {
                    mass: 10.0,
                    principal_inertia: 0.5,
                    ..Default::default()
                })
                .insert(Velocity {
                    linvel: Vec2::new(0., PARATROOPER_SPAWN_VELOCITY),
                    angvel: 0.,
                })
                .insert(Collider::cuboid(PARATROOPER_X / 2., PARATROOPER_Y / 2.))
                .insert(Friction {
                    coefficient: 0.0,
                    combine_rule: CoefficientCombineRule::Min,
                })
                .insert(Restitution {
                    coefficient: 0.,
                    combine_rule: CoefficientCombineRule::Min,
                })
                .insert(CollisionGroups::new(
                    PARATROOPER_COLLISION_MEMBERSHIP,
                    PARATROOPER_COLLISION_FILTER,
                ))
                .insert(ActiveEvents::COLLISION_EVENTS)
                .insert(Paratrooper::default());
        }
    }
}

/// Handle bullet <-> parachute/trooper collisions
fn bullet_collision_system(
    mut commands: Commands,
    parachute_query: Query<(Entity, &Transform), With<Parachute>>,
    mut paratrooper_query: Query<(
        Entity,
        &mut Paratrooper,
        &Transform,
        &mut Damping,
        &mut GravityScale,
        &mut Velocity,
        Option<&Children>,
    )>,
    mut event_reader: EventReader<BulletCollisionEvent>,
    mut event_writer: EventWriter<GibEvent>,
) {
    for event in event_reader.iter() {
        match event.collision_type {
            CollisionType::Paratrooper => {
                if let Ok((
                    paratrooper_entity,
                    _paratrooper,
                    transform,
                    _damping,
                    _gravity,
                    _rb_vel,
                    _children,
                )) = paratrooper_query.get(event.target_entity)
                {
                    event_writer.send(GibEvent {
                        transform: (*transform).with_scale(Vec3::new(
                            PARATROOPER_SCALE,
                            PARATROOPER_SCALE,
                            1.,
                        )),
                    });
                    commands.entity(paratrooper_entity).despawn_recursive();
                }
            }
            CollisionType::Parachute => {
                if let Ok((parachute_entity, _transform)) = parachute_query.get(event.target_entity)
                {
                    // Reset falling physics
                    for (
                        _paratrooper_entity,
                        mut paratrooper,
                        _transform,
                        mut damping,
                        mut gravity,
                        mut velocity,
                        children,
                    ) in paratrooper_query.iter_mut()
                    {
                        if let Some(children) = children {
                            for &child in children.iter() {
                                if child == parachute_entity {
                                    damping.linear_damping = 0.0;
                                    gravity.0 = 1.0;
                                    // Give them a boost down
                                    velocity.linvel.y =
                                        (1.5 * MIN_PARACHUTE_VELOCITY).min(velocity.linvel.y);
                                    paratrooper.state = ParatrooperState::Falling;
                                }
                            }
                        }
                    }

                    commands.entity(parachute_entity).despawn_recursive();
                }
            }
            _ => (),
        }
    }
}

/// Detect paratrooper landings
fn paratrooper_landing_system(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut paratrooper_query: Query<(
        Entity,
        &mut Paratrooper,
        &Transform,
        &mut Velocity,
        &MassProperties,
        Option<&Children>,
    )>,
    ground_query: Query<Entity, With<Ground>>,
    mut event_writer: EventWriter<LandingEvent>,
    mut gib_event_writer: EventWriter<GibEvent>,
) {
    for collision_event in collision_events.iter() {
        for ground_entity in ground_query.iter() {
            for (
                paratrooper_entity,
                mut paratrooper,
                &transform,
                mut _velocity,
                _rb_mprops,
                children_option,
            ) in paratrooper_query.iter_mut()
            {
                if let &CollisionEvent::Started(entity1, entity2, _) = collision_event {
                    // Ground / Paratrooper contact
                    if (paratrooper_entity == entity1 && ground_entity == entity2)
                        || (ground_entity == entity1 && paratrooper_entity == entity2)
                    {
                        // Crash landing
                        if paratrooper.state == ParatrooperState::Falling {
                            gib_event_writer.send(GibEvent {
                                transform: transform.with_scale(Vec3::new(
                                    PARATROOPER_SCALE,
                                    PARATROOPER_SCALE,
                                    1.0,
                                )),
                            });
                            commands.entity(paratrooper_entity).despawn_recursive();
                        }

                        if paratrooper.state != ParatrooperState::Landed {
                            paratrooper.state = ParatrooperState::Landed;
                            event_writer.send(LandingEvent(paratrooper_entity));
                        }

                        // Despawn and remove velocity damping. Assume only parachute children.
                        if let Some(children) = children_option {
                            for child in children.iter() {
                                commands.entity(*child).despawn_recursive();
                            }
                        } else {
                            // Hit the ground with no Parachute
                            gib_event_writer.send(GibEvent {
                                transform: transform.with_scale(Vec3::new(
                                    PARATROOPER_SCALE,
                                    PARATROOPER_SCALE,
                                    1.0,
                                )),
                            });
                            commands.entity(paratrooper_entity).despawn_recursive();
                        }
                    }
                }
            }
        }
    }
}

fn despawn_paratrooper_system(mut commands: Commands, query: Query<Entity, With<Paratrooper>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn spawn_parachutes(
    mut commands: Commands,
    textures: Res<ParatrooperTextures>,
    mut paratrooper_query: Query<(
        Entity,
        &mut Paratrooper,
        &mut Velocity,
        &MassProperties,
        &mut Damping,
        &mut GravityScale,
    )>,
) {
    let mut rng = rand::thread_rng();
    for (paratrooper_entity, mut paratrooper, mut velocity, _rb_mprops, mut damping, mut gravity) in
        paratrooper_query.iter_mut()
    {
        if !paratrooper.has_deployed_chute
            && paratrooper.state == ParatrooperState::Falling
            && rng.gen_range(0.0..1.0) < PARACHUTE_SPAWN_PROBABILITY
        {
            paratrooper.has_deployed_chute = true;
            paratrooper.state = ParatrooperState::Floating;

            // Spawn parachute
            let parachute_entity = commands
                .spawn()
                .insert_bundle(SpriteBundle {
                    texture: textures.parachute_handle.clone(),
                    // TODO scale to reasonable size
                    transform: Transform::from_translation(Vec3::new(0., 49.0, 0.)),
                    ..Default::default()
                })
                .insert(Collider::cuboid(31. / 4., 49. / 4.))
                .insert(Sensor(true))
                .insert(CollisionGroups::new(0b0001, 0b1110))
                //.insert(Transform::from_xyz(0.0, 30.0, 0.0)) // todo own offset separate from sprite?
                .insert(Parachute)
                .id();

            // Attach to paratrooper
            commands
                .entity(paratrooper_entity)
                .push_children(&[parachute_entity]);

            // Add air resistance drag
            damping.linear_damping = PARACHUTE_DAMPING;

            // Cap y velocity
            velocity.linvel.y = velocity.linvel.y.max(MIN_PARACHUTE_VELOCITY);

            // Reduce gravity
            gravity.0 = PARACHUTE_GRAVITY_SCALE;
        }
    }
}

pub struct ParatrooperPlugin;

impl Plugin for ParatrooperPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_paratroopers)
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(paratrooper_landing_system)
                    .with_system(bullet_collision_system)
                    .with_system(spawn_paratroopers)
                    .with_system(spawn_parachutes),
            )
            .add_system_set(
                SystemSet::on_exit(AppState::InGame).with_system(despawn_paratrooper_system),
            );
    }
}

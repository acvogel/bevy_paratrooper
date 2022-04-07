use crate::aircraft::Aircraft;
use crate::gun::GunBase;
use crate::menu::AttackState;
use crate::terrain::Ground;
use crate::{AppState, BulletCollisionEvent, CollisionType, ExplosionEvent, LandingEvent};
use bevy::prelude::*;
use bevy_rapier2d::prelude::ContactEvent::Started;
use bevy_rapier2d::prelude::*;
use rand::Rng;

const PARATROOPER_WALK_SPEED: f32 = 50.;
const PARATROOPER_SPAWN_PROBABILITY: f32 = 0.003;
const PARACHUTE_SPAWN_PROBABILITY: f32 = 0.005;
const PARACHUTE_DAMPING: f32 = 1.0; // 100% air resistance
const MIN_PARACHUTE_VELOCITY: f32 = -20.; // meters / second
const PARACHUTE_GRAVITY_SCALE: f32 = 0.5;
const PARATROOPER_SCALE: f32 = 0.4;
const PARATROOPER_ASSAULT_MIN: usize = 4;

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
    Climbing,
    Assault,
}

#[derive(Clone)]
enum AssaultRole {
    Base,
    Climber,
    SecondBase,
    Sapper,
}

enum JobStatus {
    Waiting,
    InProgress,
    Completed,
}

#[derive(Component)]
pub struct Assaulter {
    role: AssaultRole,
    status: JobStatus,
}

struct ParatrooperTextures {
    pub body_handle: Handle<Image>,      // 31 x 49
    pub parachute_handle: Handle<Image>, // 89 x 86
}

fn setup_paratroopers(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ParatrooperTextures {
        body_handle: asset_server.load("paratrooperfly1_body.png"),
        parachute_handle: asset_server.load("paratrooperfly1_parachute.png"),
    });
}

// Dynamic parachutes version
fn spawn_paratroopers(
    mut commands: Commands,
    paratrooper_textures: Res<ParatrooperTextures>,
    mut query: Query<(
        &Aircraft,
        &Transform,
        &RigidBodyPositionComponent,
        &RigidBodyVelocityComponent,
    )>,
) {
    let mut rng = rand::thread_rng();
    for (_aircraft, transform, rb_pos, rb_vel) in query.iter_mut() {
        if rng.gen_range(0.0..1.0) < PARATROOPER_SPAWN_PROBABILITY {
            let mut paratrooper_transform =
                transform.clone().with_scale(Vec3::splat(PARATROOPER_SCALE));

            // Offset to back of plane
            let multiplier = if rb_vel.linvel.x > 0.0 { 1.0 } else { -1.0 };

            paratrooper_transform.translation.x -= multiplier * 35.0;
            paratrooper_transform.translation.y -= 25.;

            let paratrooper_pos = [
                rb_pos.position.translation.x - multiplier * 35.,
                rb_pos.position.translation.y - 25.,
            ];

            //let sprite_size = Vec2::new(31., 49.);
            let sprite_bundle = SpriteBundle {
                texture: paratrooper_textures.body_handle.clone(),
                transform: paratrooper_transform,
                ..Default::default()
            };

            let collider = ColliderBundle {
                shape: ColliderShape::cuboid(31. / 8., 49. / 8.).into(), // XXX bad shape?
                flags: ColliderFlags {
                    // No collisions with other paratroopers (group 0)
                    collision_groups: InteractionGroups::new(0b0001, 0b1110),
                    active_collision_types: ActiveCollisionTypes::all(),
                    active_events: ActiveEvents::all(),
                    ..Default::default()
                }
                .into(),
                material: ColliderMaterial {
                    restitution: 0.,
                    restitution_combine_rule: CoefficientCombineRule::Min,
                    ..Default::default()
                }
                .into(),
                ..Default::default()
            };
            let rigid_body = RigidBodyBundle {
                body_type: RigidBodyTypeComponent(RigidBodyType::Dynamic),
                position: paratrooper_pos.into(),
                mass_properties: RigidBodyMassProps {
                    flags: RigidBodyMassPropsFlags::ROTATION_LOCKED,
                    local_mprops: MassProperties::new(Vec2::ZERO.into(), 10.0, 0.5).into(),
                    ..Default::default()
                }
                .into(),
                ..Default::default()
            };

            let paratrooper = Paratrooper {
                state: ParatrooperState::Falling,
                has_deployed_chute: false,
            };
            commands
                .spawn_bundle(rigid_body)
                .insert(RigidBodyPositionSync::Discrete)
                .insert_bundle(collider)
                .insert_bundle(sprite_bundle)
                .insert(paratrooper);
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
        &mut RigidBodyDampingComponent,
        &mut RigidBodyForcesComponent,
        &mut RigidBodyVelocityComponent,
        Option<&Children>,
    )>,
    mut event_reader: EventReader<BulletCollisionEvent>,
    mut event_writer: EventWriter<ExplosionEvent>,
) {
    for event in event_reader.iter() {
        match event.collision_type {
            CollisionType::Paratrooper => {
                if let Ok((
                    paratrooper_entity,
                    _paratrooper,
                    transform,
                    _rb_damp,
                    _rb_force,
                    _rb_vel,
                    _children,
                )) = paratrooper_query.get(event.target_entity)
                {
                    event_writer.send(ExplosionEvent {
                        transform: transform.clone(),
                    });
                    commands.entity(paratrooper_entity).despawn_recursive();
                }
            }
            CollisionType::Parachute => {
                if let Ok((parachute_entity, transform)) = parachute_query.get(event.target_entity)
                {
                    event_writer.send(ExplosionEvent {
                        transform: transform.clone(),
                    });
                    // Reset falling physics
                    for (
                        _paratrooper_entity,
                        mut paratrooper,
                        _transform,
                        mut rb_damping,
                        mut rb_forces,
                        mut rb_vel,
                        children,
                    ) in paratrooper_query.iter_mut()
                    {
                        if let Some(children) = children {
                            for &child in children.iter() {
                                if child == parachute_entity {
                                    rb_damping.linear_damping = 0.0;
                                    rb_forces.gravity_scale = 1.0;
                                    // Give them a boost down
                                    rb_vel.linvel.y =
                                        (1.5 * MIN_PARACHUTE_VELOCITY).min(rb_vel.linvel.y);
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

/// Listen for collisions with the gun base, then try to jump with impulse or velocity.
fn paratrooper_climbing_system(
    mut contact_events: EventReader<ContactEvent>,
    gun_base_query: Query<Entity, With<GunBase>>,
    mut paratrooper_query: Query<(Entity, &mut Paratrooper, &mut RigidBodyVelocityComponent)>,
) {
    let gun_base_entity = gun_base_query.get_single().unwrap();
    for contact_events in contact_events.iter() {
        for (paratrooper_entity, mut paratrooper, mut rb_vel) in paratrooper_query.iter_mut() {
            if let Started(handle1, handle2) = contact_events {
                if (handle1.entity() == paratrooper_entity && handle2.entity() == gun_base_entity)
                    || (handle2.entity() == paratrooper_entity
                        && handle1.entity() == gun_base_entity)
                {
                    info!("Jumping!");
                    paratrooper.state = ParatrooperState::Climbing;
                    rb_vel.linvel = Vec2::new(0., 50.).into(); // boosts them up to the top.
                }
            }
        }
    }
}

/// Waits for 4 landed paratroopers on one side of the gun
fn detect_assault_system(
    mut commands: Commands,
    mut paratrooper_query: Query<(Entity, &mut Paratrooper, &RigidBodyPositionComponent)>,
    gun_base_query: Query<&RigidBodyPositionComponent, With<GunBase>>,
    mut app_state: ResMut<State<AppState>>,
) {
    let gun_base_rb_pos = gun_base_query.get_single().unwrap();
    let mut landed_paratroopers: Vec<(Entity, Mut<'_, Paratrooper>, &RigidBodyPositionComponent)> =
        paratrooper_query
            .iter_mut()
            .filter(|(_e, paratrooper, _rb_pos)| paratrooper.state == ParatrooperState::Landed)
            .collect();

    let (landed_left_paratroopers, landed_right_paratroopers): (Vec<_>, Vec<_>) =
        landed_paratroopers
            .iter_mut()
            .partition(|(_e, _p, p_rb_pos)| {
                p_rb_pos.position.translation.x <= gun_base_rb_pos.position.translation.x
            });

    // Check if sufficient paratroopers on one side
    let assault_troops = if landed_left_paratroopers.len() >= PARATROOPER_ASSAULT_MIN {
        info!("left assault");
        Some(landed_left_paratroopers)
    } else if landed_right_paratroopers.len() >= PARATROOPER_ASSAULT_MIN {
        info!("right assault");
        Some(landed_right_paratroopers)
    } else {
        None
    };

    if let Some(mut assault_troops) = assault_troops {
        info!("Assault!!!");
        // Set troopers to Assault mode
        assault_troops.sort_by(|(_e1, _p1, rb_pos1), (_e2, _p2, rb_pos2)| {
            rb_pos1
                .position
                .translation
                .x
                .abs()
                .partial_cmp(&rb_pos2.position.translation.x.abs())
                .unwrap()
        });
        let roles = vec![
            AssaultRole::Base,
            AssaultRole::Climber,
            AssaultRole::SecondBase,
            AssaultRole::Sapper,
        ];
        let active_assault_troops = assault_troops.iter_mut().take(roles.len());
        for ((entity, paratrooper, _rb_pos), role) in active_assault_troops.zip(roles) {
            // Update paratrooper state
            paratrooper.state = ParatrooperState::Assault;
            commands.entity(*entity).insert(Assaulter {
                role: role,
                status: JobStatus::Waiting,
            });
            // TODO Enable collider with other paratroopers for climbing
        }

        // Change game state
        app_state
            .set(AppState::InGame(AttackState::Ground))
            .unwrap();
    }
}

/// Handles pyramid building progress
fn assault_collision_system(
    mut contact_events: EventReader<ContactEvent>,
    gun_base_query: Query<Entity, With<GunBase>>,
    mut paratrooper_query: Query<(
        Entity,
        &mut Assaulter,
        &mut RigidBodyTypeComponent,
        &mut RigidBodyVelocityComponent,
    )>,
) {
    let gun_base_entity = gun_base_query.get_single().unwrap();
    for contact_event in contact_events.iter() {
        if let Started(handle1, handle2) = contact_event {
            // Gun base collision
            if handle1.entity() == gun_base_entity || handle2.entity() == gun_base_entity {
                let other_entity = if handle1.entity() == gun_base_entity {
                    handle2.entity()
                } else {
                    handle1.entity()
                };
                for (paratrooper_entity, mut assaulter, mut rb_type, mut rb_vel) in
                    paratrooper_query.iter_mut()
                {
                    if other_entity == paratrooper_entity {
                        match *assaulter {
                            Assaulter {
                                role: AssaultRole::Base,
                                status: JobStatus::InProgress,
                            } => {
                                info!("Assaulter Base is in place.");
                                assaulter.status = JobStatus::Completed;
                                *rb_type = RigidBodyTypeComponent(RigidBodyType::Static);
                            }
                            Assaulter {
                                role: AssaultRole::Climber,
                                status: JobStatus::InProgress,
                            } => {
                                info!("Assaulter Climber is in place.");
                                assaulter.status = JobStatus::Completed;
                                rb_vel.linvel = Vec2::ZERO.into();
                            }
                            Assaulter {
                                role: AssaultRole::Sapper,
                                status: JobStatus::InProgress,
                            } => {
                                info!("Assaulter Sapper has touched the base");
                                // teleport to top of gun base, or jump
                            }
                            _ => (),
                        }
                    }
                }
            }
        }
    }
}

/// Moves paratroopers in ground attack
fn paratrooper_assault_system(
    mut query: Query<(
        &Paratrooper,
        &mut Assaulter,
        &mut RigidBodyTypeComponent,
        &RigidBodyPositionComponent,
        &mut RigidBodyVelocityComponent,
    )>,
) {
    for (_paratrooper, mut assaulter, mut _rb_type, rb_pos, mut rb_vel) in query.iter_mut() {
        let heading = -1.0 * rb_pos.position.translation.x.signum();
        match *assaulter {
            Assaulter {
                role: AssaultRole::Base,
                status: JobStatus::Waiting,
            } => {
                info!("Starting Base assault");
                assaulter.status = JobStatus::InProgress;
                rb_vel.linvel = Vec2::new(heading * PARATROOPER_WALK_SPEED, 0.0).into();
                break;
            }
            Assaulter {
                role: AssaultRole::Base,
                status: JobStatus::InProgress,
            } => {
                break;
            }
            Assaulter {
                role: AssaultRole::Climber,
                status: JobStatus::Waiting,
            } => {
                info!("Starting Climber assault");
                assaulter.status = JobStatus::InProgress;
                rb_vel.linvel = Vec2::new(heading * PARATROOPER_WALK_SPEED, 0.0).into();
                break;
            }
            Assaulter {
                role: AssaultRole::Climber,
                status: JobStatus::InProgress,
            } => (),
            Assaulter {
                role: AssaultRole::SecondBase,
                status: JobStatus::Waiting,
            } => {
                // start walking
                assaulter.status = JobStatus::InProgress;
                ()
            }
            _ => (),
        }
    }
}

// Detect paratrooper landings
fn paratrooper_landing_system(
    mut commands: Commands,
    mut contact_events: EventReader<ContactEvent>,
    mut paratrooper_query: Query<(
        Entity,
        &mut Paratrooper,
        &Transform,
        &mut RigidBodyVelocityComponent,
        &RigidBodyMassPropsComponent,
        Option<&Children>,
    )>,
    ground_query: Query<Entity, With<Ground>>,
    mut event_writer: EventWriter<LandingEvent>,
    mut explosion_event_writer: EventWriter<ExplosionEvent>,
) {
    for contact_event in contact_events.iter() {
        for ground_entity in ground_query.iter() {
            for (
                paratrooper_entity,
                mut paratrooper,
                transform,
                mut _rb_vel,
                _rb_mprops,
                children_option,
            ) in paratrooper_query.iter_mut()
            {
                if let ContactEvent::Started(handle1, handle2) = contact_event {
                    // Ground / Paratrooper contact
                    if (paratrooper_entity == handle1.entity() && ground_entity == handle2.entity())
                        || (ground_entity == handle1.entity()
                            && paratrooper_entity == handle2.entity())
                    {
                        // Crash landing
                        // TODO check for velocity?
                        if paratrooper.state == ParatrooperState::Falling {
                            explosion_event_writer.send(ExplosionEvent {
                                transform: transform.clone(),
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
                            // TODO GibEvent
                            explosion_event_writer.send(ExplosionEvent {
                                transform: transform.clone(),
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
        &mut RigidBodyVelocityComponent,
        &RigidBodyMassPropsComponent,
        &mut RigidBodyDampingComponent,
        &mut RigidBodyForcesComponent,
    )>,
) {
    let mut rng = rand::thread_rng();
    for (
        paratrooper_entity,
        mut paratrooper,
        mut rb_vel,
        _rb_mprops,
        mut rb_damping,
        mut rb_forces,
    ) in paratrooper_query.iter_mut()
    {
        if !paratrooper.has_deployed_chute
            && paratrooper.state == ParatrooperState::Falling
            && rng.gen_range(0.0..1.0) < PARACHUTE_SPAWN_PROBABILITY
        {
            paratrooper.has_deployed_chute = true;
            paratrooper.state = ParatrooperState::Floating;

            // Spawn parachute
            let parachute_entity = commands
                .spawn_bundle(SpriteBundle {
                    texture: textures.parachute_handle.clone(),
                    // TODO scale to reasonable size
                    transform: Transform::from_translation(Vec3::new(0., 49.0, 0.)),
                    ..Default::default()
                })
                .insert_bundle(ColliderBundle {
                    shape: ColliderShape::cuboid(31. / 4., 49. / 4.).into(), // XXX bad shape?
                    collider_type: ColliderType::Sensor.into(),
                    flags: ColliderFlags {
                        // No collisions with other paratroopers (group 0)
                        collision_groups: InteractionGroups::new(0b0001, 0b1110),
                        active_collision_types: ActiveCollisionTypes::all(),
                        active_events: ActiveEvents::all(),
                        ..Default::default()
                    }
                    .into(),
                    position: (Vec2::new(0., 30.), 0.).into(),
                    ..Default::default()
                })
                .insert(Parachute)
                .id();

            // Attach to paratrooper
            commands
                .entity(paratrooper_entity)
                .push_children(&[parachute_entity]);

            // Add air resistance drag
            rb_damping.linear_damping = PARACHUTE_DAMPING;

            // Cap y velocity
            rb_vel.linvel.y = rb_vel.linvel.y.max(MIN_PARACHUTE_VELOCITY);

            // Reduce gravity
            rb_forces.gravity_scale = PARACHUTE_GRAVITY_SCALE;
        }
    }
}

pub struct ParatrooperPlugin;

impl Plugin for ParatrooperPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_paratroopers)
            .add_system_set(
                SystemSet::on_update(AppState::InGame(AttackState::Air))
                    .with_system(paratrooper_landing_system)
                    .with_system(bullet_collision_system)
                    .with_system(paratrooper_climbing_system)
                    .with_system(spawn_paratroopers)
                    .with_system(spawn_parachutes)
                    .with_system(detect_assault_system),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame(AttackState::Ground))
                    .with_system(paratrooper_landing_system)
                    .with_system(bullet_collision_system)
                    .with_system(paratrooper_assault_system)
                    .with_system(assault_collision_system),
            )
            .add_system_set(
                SystemSet::on_exit(AppState::InGame(AttackState::Ground))
                    .with_system(despawn_paratrooper_system),
            );
    }
}

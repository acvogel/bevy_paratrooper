use crate::aircraft::Aircraft;
use crate::menu::AttackState;
use crate::terrain::Ground;
use crate::{AppState, BulletCollisionEvent, CollisionType, ExplosionEvent, LandingEvent};
use bevy::prelude::*;
//use bevy_rapier2d::prelude::ContactEvent::Started;
use bevy_rapier2d::prelude::*;
use rand::Rng;

const PARATROOPER_SPAWN_PROBABILITY: f32 = 0.003;
const PARACHUTE_SPAWN_PROBABILITY: f32 = 0.005;
const PARACHUTE_DAMPING: f32 = 1.0; // 100% air resistance
                                    //const MIN_PARACHUTE_VELOCITY: f32 = -20.; // meters / second
const MIN_PARACHUTE_VELOCITY: f32 = -60.; // meters / second
                                          //const PARACHUTE_GRAVITY_SCALE: f32 = 0.5; // XXX speed up falling for testing
const PARACHUTE_GRAVITY_SCALE: f32 = 2.0;
const PARATROOPER_SCALE: f32 = 0.4;

const PARATROOPER_COLLISION_MEMBERSHIP: u32 = 0b0001;
pub const PARATROOPER_COLLISION_FILTER: u32 = 0b1110;

const PARATROOPER_SPAWN_X_MAX: f32 = 400.;
const PARATROOPER_SPAWN_X_MIN: f32 = 20.;

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
    // XXX maybe slow to roll rng for each aircraft for each frame?
    for (_aircraft, transform, rb_pos, rb_vel) in query.iter_mut() {
        let pos_x = rb_pos.position.translation.x.abs();
        if pos_x < PARATROOPER_SPAWN_X_MAX
            && pos_x > PARATROOPER_SPAWN_X_MIN
            && rng.gen_range(0.0..1.0) < PARATROOPER_SPAWN_PROBABILITY
        {
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
                    collision_groups: InteractionGroups::new(
                        PARATROOPER_COLLISION_MEMBERSHIP,
                        PARATROOPER_COLLISION_FILTER,
                    ),
                    active_collision_types: ActiveCollisionTypes::all(),
                    active_events: ActiveEvents::all(),
                    ..Default::default()
                }
                .into(),
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
                    //.with_system(paratrooper_climbing_system)
                    .with_system(spawn_paratroopers)
                    .with_system(spawn_parachutes),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame(AttackState::Ground))
                    .with_system(paratrooper_landing_system)
                    .with_system(bullet_collision_system),
            )
            .add_system_set(
                SystemSet::on_exit(AppState::InGame(AttackState::Ground))
                    .with_system(despawn_paratrooper_system),
            );
    }
}

use crate::aircraft::Aircraft;
use crate::terrain::Ground;
use crate::{AppState, BulletCollisionEvent, CollisionType, LandingEvent};
use bevy::prelude::*;
use bevy_rapier2d::na::Isometry2;
use bevy_rapier2d::prelude::*;
use rand::Rng;

const PARATROOPER_WALK_SPEED: f32 = 10.;
const PARATROOPER_SPAWN_PROBABILITY: f32 = 0.003;
const PARACHUTE_SPAWN_PROBABILITY: f32 = 0.005;
const PARACHUTE_DAMPING: f32 = 1.0; // 100% air resistance
const MIN_PARACHUTE_VELOCITY: f32 = -10.; // meters / second

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
    Walking,
}

struct ParatrooperTextures {
    //pub paratrooper_handle: Handle<Image>, // 89 x 123
    pub body_handle: Handle<Image>,      // 31 x 49
    pub parachute_handle: Handle<Image>, // 89 x 86
}

fn setup_paratroopers(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ParatrooperTextures {
        //paratrooper_handle: asset_server.load("gfx/paratroopers/paratrooperfly1.png"),
        body_handle: asset_server.load("paratrooperfly1_body.png"),
        parachute_handle: asset_server.load("paratrooperfly1_parachute.png"),
    });
}

// Dynamic parachutes version
fn spawn_paratroopers(
    mut commands: Commands,
    paratrooper_textures: Res<ParatrooperTextures>,
    mut query: Query<(&Aircraft, &Transform)>,
) {
    let mut rng = rand::thread_rng();
    for (_aircraft, transform) in query.iter_mut() {
        if rng.gen_range(0.0..1.0) < PARATROOPER_SPAWN_PROBABILITY {
            let mut paratrooper_transform = transform.clone();
            // Offset to back of plane
            // TODO depend on aircraft velocity
            paratrooper_transform.translation.x -= 35.0;
            paratrooper_transform.translation.y -= 25.;
            let sprite_size = Vec2::new(31., 49.);
            let sprite_bundle = SpriteBundle {
                texture: paratrooper_textures.body_handle.clone(),
                sprite: Sprite {
                    custom_size: Some(sprite_size),
                    ..Default::default()
                },
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
                // TODO need world physics scale here? Transform is in pixel space.
                position: Isometry2::translation(
                    paratrooper_transform.translation.x,
                    paratrooper_transform.translation.y,
                )
                .into(),
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
    mut bullet_event_writer: EventWriter<BulletCollisionEvent>,
) {
    for contact_event in contact_events.iter() {
        for ground_entity in ground_query.iter() {
            for (
                paratrooper_entity,
                mut paratrooper,
                transform,
                mut rb_vel,
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
                        if paratrooper.state != ParatrooperState::Walking {
                            paratrooper.state = ParatrooperState::Walking;
                            event_writer.send(LandingEvent);

                            // Walk towards gun.
                            let multiplier = if transform.translation.x > 0.0 {
                                -1.
                            } else {
                                1.
                            };
                            rb_vel.linvel =
                                Vec2::new(multiplier * PARATROOPER_WALK_SPEED, 0.0).into();
                        }

                        // Despawn and remove velocity damping. Assume only parachute children.
                        if let Some(children) = children_option {
                            for child in children.iter() {
                                commands.entity(*child).despawn_recursive();
                            }
                        } else {
                            // TODO new event type for falling Collision. Should gib rather than explode.
                            bullet_event_writer.send(BulletCollisionEvent {
                                collision_type: CollisionType::Paratrooper,
                                translation: transform.translation,
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
    mut paratrooper_query: Query<(
        Entity,
        &mut Paratrooper,
        &mut RigidBodyVelocityComponent,
        &RigidBodyMassPropsComponent,
        &mut RigidBodyDampingComponent,
    )>,
    textures: Res<ParatrooperTextures>,
) {
    let mut rng = rand::thread_rng();
    for (paratrooper_entity, mut paratrooper, mut rb_vel, _rb_mprops, mut rb_damping) in
        paratrooper_query.iter_mut()
    {
        if !paratrooper.has_deployed_chute
            && paratrooper.state == ParatrooperState::Falling
            && rng.gen_range(0.0..1.0) < PARACHUTE_SPAWN_PROBABILITY
        {
            paratrooper.has_deployed_chute = true;

            // Spawn parachute
            let parachute_entity = commands
                .spawn_bundle(SpriteBundle {
                    texture: textures.parachute_handle.clone(),
                    // TODO scale to reasonable size
                    transform: Transform::from_translation(Vec3::new(0., 49.0, 0.)),
                    ..Default::default()
                })
                // TODO collider
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
                    .with_system(spawn_paratroopers)
                    .with_system(spawn_parachutes),
            )
            .add_system_set(
                SystemSet::on_exit(AppState::InGame).with_system(despawn_paratrooper_system),
            )
            .add_event::<LandingEvent>();
    }
}

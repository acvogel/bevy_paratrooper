use crate::aircraft::Aircraft;
use crate::terrain::Ground;
use crate::LandingEvent;
use bevy::prelude::*;
use bevy_rapier2d::na::Isometry2;
use bevy_rapier2d::prelude::*;
use rand::Rng;

const PARATROOPER_WALK_SPEED: f32 = 10.;
const PARATROOPER_SPAWN_PROBABILITY: f32 = 0.003;

#[derive(Component)]
pub struct Paratrooper {
    pub state: ParatrooperState,
}

#[derive(PartialEq)]
pub enum ParatrooperState {
    Falling,
    Walking,
}

fn setup_paratroopers(asset_server: Res<AssetServer>) {
    let _handle: Handle<Image> = asset_server.load("gfx/paratroopers/paratrooperfly1.png");
}

fn spawn_paratroopers(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<(&Aircraft, &Transform)>,
) {
    let mut rng = rand::thread_rng();
    for (_aircraft, transform) in query.iter_mut() {
        if rng.gen_range(0.0..1.0) < PARATROOPER_SPAWN_PROBABILITY {
            let paratrooper_transform = transform.clone();
            let sprite_size = Vec2::new(89., 123.);
            let sprite_bundle = SpriteBundle {
                texture: asset_server.load("gfx/paratroopers/paratrooperfly1.png"),
                sprite: Sprite {
                    custom_size: Some(sprite_size),
                    ..Default::default()
                },
                transform: paratrooper_transform,
                ..Default::default()
            };

            let collider = ColliderBundle {
                shape: ColliderShape::cuboid(89. / 8., 123. / 8.).into(), // XXX bad shape?
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
    mut contact_events: EventReader<ContactEvent>,
    mut paratrooper_query: Query<(
        Entity,
        &mut Paratrooper,
        &Transform,
        &mut RigidBodyVelocityComponent,
        &RigidBodyMassPropsComponent,
    )>,
    ground_query: Query<Entity, With<Ground>>,
    mut event_writer: EventWriter<LandingEvent>,
) {
    for contact_event in contact_events.iter() {
        for ground_entity in ground_query.iter() {
            for (paratrooper_entity, mut paratrooper, transform, mut rb_vel, _rb_mprops) in
                paratrooper_query.iter_mut()
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
                    }
                }
            }
        }
    }
}

pub struct ParatrooperPlugin;

impl Plugin for ParatrooperPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_paratroopers)
            .add_system(paratrooper_landing_system)
            .add_system(spawn_paratroopers);
    }
}

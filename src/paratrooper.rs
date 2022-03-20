use crate::aircraft::Aircraft;
use crate::score::Score;
use crate::terrain::Ground;
use crate::{consts, LandingEvent};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

const PARATROOPER_VELOCITY: f32 = 50.;
const PARATROOPER_WALK_SPEED: f32 = 10.;
const PARATROOPER_SPAWN_PROBABILITY: f32 = 0.003;
//const PARATROOPER_SIZE: Vec2 = Vec2::new(89., 123.);

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
            // XXX lazy copied scale from aircraft.rs
            let paratrooper_transform =
                Transform::from_translation(transform.translation).with_scale(Vec3::splat(0.3));
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

            let paratrooper = Paratrooper {
                state: ParatrooperState::Falling,
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
            let body = RigidBodyBundle {
                body_type: RigidBodyTypeComponent(RigidBodyType::Dynamic),
                position: [
                    paratrooper_transform.translation.x,
                    paratrooper_transform.translation.y,
                ]
                .into(),
                mass_properties: RigidBodyMassProps {
                    flags: RigidBodyMassPropsFlags::ROTATION_LOCKED,
                    local_mprops: MassProperties::new(Vec2::ZERO.into(), 10.0, 0.5).into(),
                    ..Default::default()
                }
                .into(),
                ..Default::default()
            };

            commands
                .spawn_bundle(body)
                .insert_bundle(collider)
                .insert(paratrooper)
                //.insert(ColliderDebugRender::with_id(1))
                .insert_bundle(sprite_bundle)
                .insert(ColliderPositionSync::Discrete);
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
    ground_query: Query<(Entity, &Ground)>,
    mut event_writer: EventWriter<LandingEvent>,
) {
    let ground_entity = ground_query
        .iter()
        .next()
        .expect("No ground entity spawned!")
        .0;
    for contact_event in contact_events.iter() {
        for (paratrooper_entity, mut paratrooper, transform, mut rb_vel, _rb_mprops) in
            paratrooper_query.iter_mut()
        {
            if let ContactEvent::Started(handle1, handle2) = contact_event {
                // Ground / Paratrooper contact
                if (paratrooper_entity == handle1.entity() && ground_entity == handle2.entity())
                    || (ground_entity == handle1.entity() && paratrooper_entity == handle2.entity())
                {
                    if paratrooper.state != ParatrooperState::Walking {
                        info!("Landing event {:?}", contact_event);
                        paratrooper.state = ParatrooperState::Walking;
                        event_writer.send(LandingEvent);

                        // Walk towards gun.
                        let multiplier = if transform.translation.x > 0.0 {
                            -1.
                        } else {
                            1.
                        };
                        rb_vel.linvel = Vec2::new(multiplier * PARATROOPER_WALK_SPEED, 0.0).into();
                    }
                }
            }
        }
    }
}

//    time: Res<Time>,
//    mut score: ResMut<Score>,
//    mut query: Query<(&mut Paratrooper, &mut Transform)>,
//) {
//    // XXX will need to move the sprite bundle to match the rigid body position?
//    for (mut paratrooper, mut transform) in query.iter_mut() {
//        match paratrooper.state {
//            ParatrooperState::Falling => {
//                let drop = PARATROOPER_VELOCITY * time.delta_seconds();
//                let min_y = consts::GROUND_Y + 0.25 * paratrooper.display_size.y;
//                transform.translation.y = min_y.max(transform.translation.y - drop);
//                // No longer falling on the ground
//                if (transform.translation.y - min_y).abs() < f32::EPSILON {
//                    info!("paratrooper landed");
//                    paratrooper.state = ParatrooperState::Walking;
//                    score.paratroopers_landed += 1; // todo add event for score update
//                }
//            }
//            ParatrooperState::Walking => {
//                if transform.translation.x > 0. {
//                    transform.translation.x -= PARATROOPER_WALK_SPEED * time.delta_seconds();
//                } else {
//                    transform.translation.x += PARATROOPER_WALK_SPEED * time.delta_seconds();
//                }
//            }
//        }
//    }
//}

pub struct ParatrooperPlugin;

impl Plugin for ParatrooperPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_paratroopers)
            .add_system(paratrooper_landing_system)
            .add_system(spawn_paratroopers);
    }
}

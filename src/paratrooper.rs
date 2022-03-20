use crate::aircraft::Aircraft;
use crate::consts;
use crate::score::Score;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

const PARATROOPER_VELOCITY: f32 = 50.;
const PARATROOPER_WALK_SPEED: f32 = 10.;
const PARATROOPER_SPAWN_PROBABILITY: f32 = 0.003;
//const PARATROOPER_SIZE: Vec2 = Vec2::new(89., 123.);

#[derive(Component)]
pub struct Paratrooper {
    state: ParatrooperState,
    display_size: Vec2,
}

enum ParatrooperState {
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

            let paratrooper = Paratrooper {
                state: ParatrooperState::Falling,
                display_size: sprite_size,
            };

            let collider = ColliderBundle {
                shape: ColliderShape::cuboid(89. / 8., 123. / 8.).into(), // XXX bad shape?
                flags: ColliderFlags {
                    // No collisions with other paratroopers (group 0)
                    collision_groups: InteractionGroups::new(0b0001, 0b1110),
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
                .insert(ColliderPositionSync::Discrete)
                .insert_bundle(sprite_bundle);
        }
    }
}

// Detect landings
// Either throw a paratrooper landing event, or just do it all in the handler here.
// Impulse set velocity toward gun.
// May want to turn on collisions, dunno about if too many land at once what we do.
// then a gun collision check to form the bridge
//fn detect_landings()

//fn paratrooper_physics(
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
            .add_system(spawn_paratroopers);
        //.add_system(paratrooper_physics);
    }
}

use crate::aircraft::Aircraft;
use crate::consts;
use crate::score::Score;
use bevy::prelude::*;
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
            let mut paratrooper_transform = transform.clone();
            paratrooper_transform.scale = Vec3::splat(0.5);
            let sprite_size = Vec2::new(89., 123.);
            commands
                .spawn_bundle(SpriteBundle {
                    texture: asset_server.load("gfx/paratroopers/paratrooperfly1.png"),
                    sprite: Sprite {
                        custom_size: Some(sprite_size),
                        ..Default::default()
                    },
                    transform: paratrooper_transform,
                    ..Default::default()
                })
                .insert(Paratrooper {
                    state: ParatrooperState::Falling,
                    display_size: sprite_size,
                });
        }
    }
}

fn paratrooper_physics(
    time: Res<Time>,
    mut score: ResMut<Score>,
    mut query: Query<(&mut Paratrooper, &mut Transform)>,
) {
    for (mut paratrooper, mut transform) in query.iter_mut() {
        match paratrooper.state {
            ParatrooperState::Falling => {
                let drop = PARATROOPER_VELOCITY * time.delta_seconds();
                let min_y = consts::GROUND_Y + 0.25 * paratrooper.display_size.y;
                transform.translation.y = min_y.max(transform.translation.y - drop);
                // No longer falling on the ground
                if (transform.translation.y - min_y).abs() < 0.0000001 {
                    info!("paratrooper landed");
                    paratrooper.state = ParatrooperState::Walking;
                    score.paratroopers_landed += 1;
                }
            }
            ParatrooperState::Walking => {
                if transform.translation.x > 0. {
                    transform.translation.x -= PARATROOPER_WALK_SPEED * time.delta_seconds();
                } else {
                    transform.translation.x += PARATROOPER_WALK_SPEED * time.delta_seconds();
                }
            }
        }
    }
}

pub struct ParatrooperPlugin;

impl Plugin for ParatrooperPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_paratroopers)
            .add_system(spawn_paratroopers)
            .add_system(paratrooper_physics);
    }
}

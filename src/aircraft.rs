use bevy::prelude::*;

use rand::Rng;

use crate::score::Score;

#[derive(Component, Default)]
pub struct Aircraft {
    velocity_x: f32,
    position: Transform,
}

const AIRCRAFT_SPEED: f32 = 40.;
const AIRCRAFT_SPAWN_PROBABILITY: f32 = 0.008;
const SPAWN_LEFT_X: f32 = -600.;
const SPAWN_RIGHT_X: f32 = 600.;

fn should_spawn_aircraft() -> bool {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0..1.0) < AIRCRAFT_SPAWN_PROBABILITY
}

fn spawn_aircraft_system(mut commands: Commands, _time: Res<Time>, asset_server: Res<AssetServer>) {
    if should_spawn_aircraft() {
        let aircraft = create_aircraft();
        commands
            .spawn_bundle(SpriteBundle {
                // 412 x 114 pixels. 0.3 scale.
                texture: asset_server.load("gfx/planes/paraplane1.png"), // XXX handle
                sprite: Sprite {
                    //custom_size: Some(Vec2::new(412., 114.)),
                    flip_x: aircraft.velocity_x < 0.,
                    ..Default::default()
                },
                transform: aircraft.position.with_scale(Vec3::splat(0.3)),

                ..Default::default()
            })
            .insert(aircraft);
    }
}

fn create_aircraft() -> Aircraft {
    let mut rng = rand::thread_rng();
    let y = rng.gen_range(0. ..350.); // TODO clearance lanes to avoid existing planes, unless same direction
    let heading_right = rng.gen_bool(0.5);
    let speed = rng.gen_range(0.8..1.3) * AIRCRAFT_SPEED;
    if heading_right {
        Aircraft {
            velocity_x: speed,
            position: Transform::from_translation(Vec3::new(SPAWN_LEFT_X, y, 5.)),
        }
    } else {
        Aircraft {
            velocity_x: -speed,
            position: Transform::from_translation(Vec3::new(SPAWN_RIGHT_X, y, 5.)),
        }
    }
}

fn fly_aircraft(time: Res<Time>, mut query: Query<(&mut Aircraft, &mut Transform)>) {
    for (mut aircraft, mut transform) in query.iter_mut() {
        transform.translation.x += aircraft.velocity_x * time.delta_seconds();
        aircraft.position.translation.x += aircraft.velocity_x * time.delta_seconds();
    }
}

/// detect when they have left the screen, count those in score.
fn despawn_aircraft(
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut query: Query<(Entity, &Aircraft, &Transform)>,
) {
    for (e, _aircraft, transform) in query.iter_mut() {
        if transform.translation.x >= SPAWN_RIGHT_X + 1.
            || transform.translation.x <= SPAWN_LEFT_X - 1.
        {
            score.aircraft_escapes += 1;
            commands.entity(e).despawn();
            info!("ESCAPE {}", score.aircraft_escapes);
        }
    }
}

fn setup_aircraft_system(mut _commands: Commands, _asset_server: ResMut<AssetServer>) {
    //commands.insert_resource(asset_server.load("gfx/planes/paraplane1.png") as Handle<Image>);
}

pub struct AircraftPlugin;

impl Plugin for AircraftPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_aircraft_system)
            .add_system(spawn_aircraft_system)
            .add_system(despawn_aircraft)
            .add_system(fly_aircraft);
    }
}

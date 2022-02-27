use bevy::prelude::*;

use rand::Rng;

use crate::score::Score;

#[derive(Component, Default)]
pub struct Aircraft {
    velocity_x: f32,
    position: Transform,
}

const AIRCRAFT_SPEED: f32 = 40.;
const SPAWN_PROBABILITY: f32 = 0.008;
const SPAWN_LEFT_X: f32 = -640.;
const SPAWN_RIGHT_X: f32 = 640.;

fn should_spawn_aircraft() -> bool {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0..1.0) < SPAWN_PROBABILITY
}

fn spawn_aircraft_system(mut commands: Commands, _time: Res<Time>) {
    if should_spawn_aircraft() {
        let aircraft = create_aircraft();
        info!(
            "Spawning velocity_x {} position {}",
            aircraft.velocity_x, aircraft.position.translation
        );
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(1.0, 0., 0.),
                    custom_size: Some(Vec2::new(30., 10.)),
                    ..Default::default()
                },
                transform: aircraft.position,
                ..Default::default()
            })
            .insert(aircraft);
    }
}

fn create_aircraft() -> Aircraft {
    let mut rng = rand::thread_rng();
    let y = rng.gen_range(-100. ..350.);
    let heading_right = rng.gen_bool(0.5);
    let speed = rng.gen_range(0.8..1.4) * AIRCRAFT_SPEED;
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

pub struct AircraftPlugin;

impl Plugin for AircraftPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_aircraft_system)
            .add_system(despawn_aircraft)
            .add_system(fly_aircraft);
    }
}

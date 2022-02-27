use bevy::prelude::*;

use rand::Rng;

#[derive(Component, Default)]
pub struct Aircraft {
    velocity_x: f32,
}

//struct Fleet {
//    aircraft: Vec<Aircraft>,
//}

const AIRCRAFT_SPEED: f32 = 20.;
const SPAWN_PROBABILITY: f32 = 0.012;

fn should_spawn_aircraft() -> bool {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0..1.0) < SPAWN_PROBABILITY
}

fn spawn_aircraft(mut commands: Commands, _time: Res<Time>) {
    if should_spawn_aircraft() {
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(1.0, 0., 0.),
                    custom_size: Some(Vec2::new(30., 10.)),
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::new(0., 100., 5.)),
                ..Default::default()
            })
            .insert(Aircraft {
                velocity_x: AIRCRAFT_SPEED,
            });
    }
}

fn fly_aircraft(time: Res<Time>, mut query: Query<(&Aircraft, &mut Transform)>) {
    for (aircraft, mut transform) in query.iter_mut() {
        transform.translation.x += aircraft.velocity_x * time.delta_seconds();
    }
}

// detect when they have left the screen, count those in score.
//fn despawn_aircraft()

pub struct AircraftPlugin;

impl Plugin for AircraftPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_aircraft).add_system(fly_aircraft);
    }
}

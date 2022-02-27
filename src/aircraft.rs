use bevy::prelude::*;

// what about a global cooldown resource for how many aircraft?

#[derive(Component, Default)]
pub struct Aircraft {
    velocity_x: f32,
}

fn spawn_aircraft(mut commands: Commands, _time: Res<Time>) {
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
        .insert(Aircraft { velocity_x: 20. });
}

fn fly_aircraft(time: Res<Time>, mut query: Query<(&Aircraft, &mut Transform)>) {
    for (aircraft, mut transform) in query.iter_mut() {
        transform.translation.x += aircraft.velocity_x * time.delta_seconds();
    }
}

pub struct AircraftPlugin;

impl Plugin for AircraftPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_aircraft)
            .add_system(fly_aircraft);
    }
}

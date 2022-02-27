use bevy::prelude::*;
use bevy_rapier2d::na::Quaternion;

#[derive(Component)]
pub struct Gun;

#[derive(Component)]
pub struct GunState {
    transform: Transform,
}

pub fn setup_gun_base(mut commands: Commands) {
    let y = -201. + 32.; // (-600/2 - 500 + 64/2)
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(-0.1, 0.1, 0.1),
            custom_size: Some(Vec2::new(64., 64.)),
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(0., y, 1.)),
        ..Default::default()
    });
}

pub fn setup_gun(mut commands: Commands) {
    // make the transform, insert gun and/or gunstate ?
    let y = -201. + 32. + 40.;
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.0, 0., 0.),
            custom_size: Some(Vec2::new(20., 60.)),
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(0., y, 0.)),
        ..Default::default()
    })
        .insert(Gun);
}

fn move_gun(keyboard_input: Res<Input<KeyCode>>, mut query: Query<(&Gun, &mut Transform)>) {
    let radians_per_tick = std::f32::consts::PI / 32.;
    let mut rotation = 0.;
    if keyboard_input.pressed(KeyCode::A) {
        rotation += radians_per_tick;
    }
    if keyboard_input.pressed(KeyCode::D) {
        rotation -= radians_per_tick;
    }

    if (rotation != 0.) {
        for (gun, mut transform) in query.iter_mut() {
            transform.rotate(Quat::from_rotation_z(rotation));
        }
    }
}


pub struct GunPlugin;

impl Plugin for GunPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_gun)
            .add_startup_system(setup_gun_base)
        .add_system(move_gun);
    }
}
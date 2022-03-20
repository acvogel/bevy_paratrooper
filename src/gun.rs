use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::consts;

#[derive(Component)]
pub struct Gun {
    pub last_fired: f64,
}

#[derive(Component)]
pub struct GunBase;

pub fn setup_gun_base(mut commands: Commands) {
    let h = 64.;
    let w = 64.;
    let y = consts::GROUND_Y + 0.5 * h;
    let sprite_bundle = SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(-0.1, 0.1, 0.1),
            custom_size: Some(Vec2::new(w, h)),
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(0., y, 2.)),
        ..Default::default()
    };
    let body = RigidBodyBundle {
        body_type: RigidBodyTypeComponent(RigidBodyType::Static),
        position: [0., y].into(),
        ..Default::default()
    };
    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(0.5 * w, 0.5 * h).into(),
        ..Default::default()
    };
    commands
        .spawn_bundle(sprite_bundle)
        .insert_bundle(body)
        .insert_bundle(collider)
        .insert(GunBase);
}

pub fn setup_gun(mut commands: Commands) {
    let y = consts::GROUND_Y + 64.;
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.32, 0.36, 0.41),
                custom_size: Some(Vec2::new(20., 60.)),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(0., y, 2.)),
            ..Default::default()
        })
        .insert(Gun { last_fired: 0. });
}

fn move_gun(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Gun, &mut Transform)>,
) {
    for (_gun, mut transform) in query.iter_mut() {
        let radians_per_tick = std::f32::consts::PI; // rotation speed radians / sec
        let mut rotation = 0.;
        let local_y = transform.local_y();
        if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
            if local_y.y >= 0.4 || local_y.x > 0. {
                rotation += radians_per_tick * time.delta_seconds();
            }
        }
        if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
            if local_y.y >= 0.4 || local_y.x < 0. {
                rotation -= radians_per_tick * time.delta_seconds();
            }
        }
        if rotation != 0. {
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

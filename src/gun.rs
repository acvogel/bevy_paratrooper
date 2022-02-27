use bevy::prelude::*;

#[derive(Component)]
pub struct Gun;

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
    let y = -201. + 32. + 40.;
    commands
        .spawn_bundle(SpriteBundle {
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

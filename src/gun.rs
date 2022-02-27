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

// Bullets. todo own module if it gets too big.
#[derive(Component)]
struct Bullet {
    speed: f32,
}

/// Load bullet assets
fn setup_bullets(
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("bullet-sprite.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 4, 4);
    /*let texture_atlas_handle = */
    texture_atlases.add(texture_atlas);
}

fn shoot_gun(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Gun, &Transform)>,
) {
    if keyboard_input.pressed(KeyCode::Space) {
        for (_gun, transform) in query.iter_mut() {
            let mut bullet_transform = transform.clone();
            bullet_transform.translation =
                bullet_transform.translation + 30. * bullet_transform.local_y();
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.0, 0., 1.0),
                        custom_size: Some(Vec2::new(10., 10.)),
                        ..Default::default()
                    },
                    transform: bullet_transform,
                    ..Default::default()
                })
                .insert(Bullet { speed: 100. });
        }
    }
}

fn move_bullets(time: Res<Time>, mut query: Query<(&Bullet, &mut Transform)>) {
    for (bullet, mut transform) in query.iter_mut() {
        transform.translation =
            transform.translation + time.delta_seconds() * bullet.speed * transform.local_y();
    }
}

/// Despawn bullets once off screen
fn despawn_bullets() {}

pub struct GunPlugin;

impl Plugin for GunPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_gun)
            .add_startup_system(setup_gun_base)
            .add_startup_system(setup_bullets)
            .add_system(move_gun)
            .add_system(move_bullets)
            .add_system(despawn_bullets)
            .add_system(shoot_gun);
    }
}

use crate::{BulletCollisionEvent, CollisionType, GibEvent, GunExplosionEvent, GunshotEvent};
use bevy::prelude::*;
use rand::seq::SliceRandom;

struct GunshotHandle(Handle<AudioSource>);
struct AircraftExplosionHandle(Handle<AudioSource>);
struct ScreamHandles(Vec<Handle<AudioSource>>);
struct BaseExplosionHandle(Handle<AudioSource>);

fn setup_audio_system(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    commands.insert_resource(GunshotHandle(
        asset_server.load("audio/sfx_weapon_singleshot20.wav"),
    ));
    commands.insert_resource(AircraftExplosionHandle(
        asset_server.load("audio/sfx_exp_double2.wav"),
    ));
    commands.insert_resource(BaseExplosionHandle(
        asset_server.load("audio/sfx_exp_long4.wav"),
    ));
    let mut scream_handles = Vec::new();
    for i in 1..=14 {
        let path = format!("audio/screams/sfx_deathscream_human{}.wav", i);
        scream_handles.push(asset_server.load(&path));
    }
    commands.insert_resource(ScreamHandles(scream_handles));
}

fn gunshot_listener(
    audio: Res<Audio>,
    gunshot_handle: ResMut<GunshotHandle>,
    events: EventReader<GunshotEvent>,
) {
    if !events.is_empty() {
        audio.play(gunshot_handle.0.clone());
    }
}

/// Aircraft explosion
fn explosion_listener(
    audio: Res<Audio>,
    mut events: EventReader<BulletCollisionEvent>,
    aircraft_explosion_handle: ResMut<AircraftExplosionHandle>,
) {
    for event in events.iter() {
        match event.collision_type {
            CollisionType::Aircraft => {
                audio.play(aircraft_explosion_handle.0.clone());
            }
            _ => (),
        }
    }
}

/// Paratrooper death
fn gib_listener(audio: Res<Audio>, mut events: EventReader<GibEvent>, screams: Res<ScreamHandles>) {
    for _event in events.iter() {
        let handle = screams.0.choose(&mut rand::thread_rng()).unwrap();
        audio.play(handle.clone());
    }
}

/// End of game explosion
fn base_explosion_listener(
    audio: Res<Audio>,
    events: EventReader<GunExplosionEvent>,
    base_explosion_handle: Res<BaseExplosionHandle>,
) {
    if !events.is_empty() {
        audio.play(base_explosion_handle.0.clone());
    }
}

pub struct AudioStatePlugin;

impl Plugin for AudioStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_audio_system)
            .add_system(gunshot_listener)
            .add_system(gib_listener)
            .add_system(base_explosion_listener)
            .add_system(explosion_listener);
    }
}

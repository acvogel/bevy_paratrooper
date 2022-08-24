use crate::bomber::Bomb;
use crate::{
    AppState, BulletCollisionEvent, CollisionType, ExplosionEvent, ExplosionType, GibEvent,
    GunExplosionEvent, GunshotEvent,
};
use bevy::audio::AudioSink;
use bevy::prelude::*;
use rand::seq::SliceRandom;
use std::collections::HashMap;

struct GunshotHandle(Handle<AudioSource>);
struct AircraftExplosionHandle(Handle<AudioSource>);
struct ScreamHandles(Vec<Handle<AudioSource>>);
struct BaseExplosionHandle(Handle<AudioSource>);
struct IntroMusicHandle(Handle<AudioSource>);
struct Level1MusicHandle(Handle<AudioSource>);
struct CurrentMusic(Option<Handle<AudioSink>>);
struct BombHandles {
    falling_bomb: Handle<AudioSource>,
    explosion: Handle<AudioSource>,
}

#[derive(Component)]
struct Whistler;

/// Bomb entity -> whistling audio handle
struct Whistles(HashMap<Entity, Handle<AudioSink>>);

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

    commands.insert_resource(IntroMusicHandle(
        asset_server.load("audio/565_tocf_mono_intro.ogg"),
    ));

    commands.insert_resource(Level1MusicHandle(
        asset_server.load("audio/565_tocf_mono_level_1.ogg"),
    ));

    commands.insert_resource(BombHandles {
        falling_bomb: asset_server.load("audio/falling-bomb-41038.ogg"),
        explosion: asset_server.load("audio/bomb_explosion.wav"),
    });

    commands.insert_resource(Whistles(HashMap::new()));

    // No song playing at startup
    commands.insert_resource(CurrentMusic(None));
}

fn play_menu_music(
    audio: Res<Audio>,
    intro_music: Res<IntroMusicHandle>,
    mut current_music: ResMut<CurrentMusic>,
    audio_sinks: ResMut<Assets<AudioSink>>,
) {
    let intro_handle = audio_sinks
        .get_handle(audio.play_with_settings(intro_music.0.clone(), PlaybackSettings::LOOP));
    // Stop current music
    if let Some(current_music_sink) = &current_music.0 {
        if let Some(old_sink) = audio_sinks.get(current_music_sink) {
            old_sink.pause();
        }
    }
    current_music.0 = Some(intro_handle);
}

fn stop_menu_music(audio_sinks: ResMut<Assets<AudioSink>>, current_music: Res<CurrentMusic>) {
    if let Some(current_music_sink) = &current_music.0 {
        if let Some(old_sink) = audio_sinks.get(current_music_sink) {
            old_sink.pause();
        }
    }
}

#[allow(dead_code)]
fn play_level_music(
    audio: Res<Audio>,
    level_music: Res<Level1MusicHandle>,
    mut current_music: ResMut<CurrentMusic>,
    audio_sinks: ResMut<Assets<AudioSink>>,
) {
    let level_handle = audio_sinks
        .get_handle(audio.play_with_settings(level_music.0.clone(), PlaybackSettings::LOOP));
    if let Some(current_music_sink) = &current_music.0 {
        if let Some(old_sink) = audio_sinks.get(current_music_sink) {
            old_sink.pause();
        }
    }
    current_music.0 = Some(level_handle);
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

/// Starts whistling on Bomb spawn
fn bomb_spawned_listener(
    mut commands: Commands,
    audio: Res<Audio>,
    bomb_audio: Res<BombHandles>,
    bomb_query: Query<Entity, Added<Bomb>>,
    mut whistles: ResMut<Whistles>,
    audio_sinks: Res<Assets<AudioSink>>,
) {
    for entity in bomb_query.iter() {
        let audio_sink = audio_sinks.get_handle(audio.play(bomb_audio.falling_bomb.clone()));
        whistles.0.insert(entity, audio_sink);
        commands.entity(entity).insert(Whistler);
    }
}

/// Stop whistling when bombs despawn
fn whistler_despawn_listener(
    removed_whistlers: RemovedComponents<Whistler>,
    whistles: Res<Whistles>,
    audio_sinks: Res<Assets<AudioSink>>,
) {
    for entity in removed_whistlers.iter() {
        if let Some(whistle_handle) = whistles.0.get(&entity) {
            if let Some(sink) = audio_sinks.get(whistle_handle) {
                sink.stop();
            }
        }
    }
}

/// Spawn bomb explosion sound
fn bomb_explosion_listener(
    mut events: EventReader<ExplosionEvent>,
    audio: Res<Audio>,
    bomb_audio: Res<BombHandles>,
) {
    for _event in events
        .iter()
        .filter(|&e| e.explosion_type == ExplosionType::Bomb)
    {
        audio.play(bomb_audio.explosion.clone());
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
            CollisionType::Aircraft /*| CollisionType::Bomb*/ => {
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
            .add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(play_menu_music))
            .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(stop_menu_music))
            // XXX
            //.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(play_level_music))
            .add_system(gunshot_listener)
            .add_system(gib_listener)
            .add_system(base_explosion_listener)
            .add_system(bomb_spawned_listener)
            .add_system(bomb_explosion_listener)
            .add_system(explosion_listener)
            .add_system_to_stage(CoreStage::PostUpdate, whistler_despawn_listener);
    }
}

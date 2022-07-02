use crate::{
    AppState, BulletCollisionEvent, CollisionType, GibEvent, GunExplosionEvent, GunshotEvent,
};
use bevy::audio::AudioSink;
use bevy::prelude::*;
use rand::seq::SliceRandom;

struct GunshotHandle(Handle<AudioSource>);
struct AircraftExplosionHandle(Handle<AudioSource>);
struct ScreamHandles(Vec<Handle<AudioSource>>);
struct BaseExplosionHandle(Handle<AudioSource>);
struct IntroMusicHandle(Handle<AudioSource>);
struct Level1MusicHandle(Handle<AudioSource>);
struct CurrentMusic(Option<Handle<AudioSink>>);

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
            .add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(play_menu_music))
            .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(play_level_music))
            .add_system(gunshot_listener)
            .add_system(gib_listener)
            .add_system(base_explosion_listener)
            .add_system(explosion_listener);
    }
}

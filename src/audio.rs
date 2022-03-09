use crate::GunshotEvent;
use bevy::asset::Asset;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioPlugin, AudioSource};

struct AudioState {
    gunshot_handle: Handle<AudioSource>,
}

fn setup_audio_system(mut commands: Commands, mut asset_server: ResMut<AssetServer>) {
    let gunshot_handle = asset_server.load("audio/gunshotjbudden_P9IlJlC.mp3");
    let audio_state = AudioState {
        gunshot_handle: gunshot_handle,
    };
    commands.insert_resource(audio_state);
}

fn gunshot_listener(
    audio: Res<Audio>,
    audio_state: ResMut<AudioState>,
    mut events: EventReader<GunshotEvent>,
) {
    for _gunshot in events.iter() {
        audio.play(audio_state.gunshot_handle.clone());
    }
}

pub struct AudioStatePlugin;

impl Plugin for AudioStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_audio_system)
            .add_system(gunshot_listener);
    }
}

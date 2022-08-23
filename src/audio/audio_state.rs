use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub enum AudioEventAction {
    PlayOnce,
}
pub struct AudioEvent {
    pub action: AudioEventAction,
    pub source: Option<Handle<AudioSource>>,
}

pub struct AudioStatePlugin;

// This plugin is responsible to control the game audio
impl Plugin for AudioStatePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(AudioPlugin)
        .add_event::<AudioEvent>()
        .add_system_set(SystemSet::on_enter(GameState::Running).with_system(setup_audio))
        .add_system_set(
            SystemSet::on_update(GameState::Running).with_system(update_audio),
        )
        ;
    }
}

fn setup_audio(audio: Res<Audio>) {
    audio.set_volume(0.3);
    audio.resume();
}

fn update_audio(mut audio_events: EventReader<AudioEvent>, audio: Res<Audio>) {
    for audio_event in audio_events.iter() {
        match audio_event.action {
            AudioEventAction::PlayOnce => {
                audio.play(audio_event.source.clone().unwrap());
            }
        }
    }
}

use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};
use rand::seq::SliceRandom;

use crate::{
    game::assets::{HandleMap, SfxKey},
    screen::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(play_sfx);
    app.insert_resource(SfxPlaying { states: vec![] });
    app.observe(play_looping_sfx);
    app.observe(stop_looping_sfx);
    app.observe(stop_all_looping_sfx);
}

#[derive(Resource)]
pub struct SfxPlaying {
    pub states: Vec<(SfxKey, bool)>,
}

#[derive(Component, PartialEq)]
pub struct LoopingSfx(pub SfxKey);

fn play_looping_sfx(
    trigger: Trigger<PlayLoopingSfx>,
    mut commands: Commands,
    sfx_handles: Res<HandleMap<SfxKey>>,
    mut sfx_playing: ResMut<SfxPlaying>,
) {
    let sfx_key = match trigger.event() {
        PlayLoopingSfx::Key(key) => *key,
    };

    let is_tick = matches!(
        sfx_key,
        SfxKey::Ticking1
            | SfxKey::Ticking2
            | SfxKey::Ticking3
            | SfxKey::Ticking4
            | SfxKey::Ticking5
            | SfxKey::Ticking6
    );

    let handle = sfx_handles[&sfx_key].clone_weak();
    let state = sfx_playing
        .states
        .iter_mut()
        .find(|(key, _)| *key == sfx_key);

    let mut play = false;
    if state.is_none() {
        sfx_playing.states.push((sfx_key, true));
        play = true;
    } else {
        let (_, playing) = state.unwrap();
        if !*playing {
            *playing = true;
            play = true;
        }
    }

    if play {
        commands.spawn((
            AudioSourceBundle {
                source: handle,
                settings: PlaybackSettings {
                    mode: PlaybackMode::Loop,
                    volume: if is_tick {
                        Volume::new(2.0)
                    } else {
                        Volume::new(1.0)
                    },
                    ..default()
                },
            },
            StateScoped(Screen::Playing),
            LoopingSfx(sfx_key),
        ));
    }
}

fn stop_looping_sfx(
    trigger: Trigger<StopLoopingSfx>,
    mut commands: Commands,
    audio: Query<(Entity, &LoopingSfx)>,
    mut sfx_playing: ResMut<SfxPlaying>,
) {
    let sfx_key = match trigger.event() {
        StopLoopingSfx::Key(key) => *key,
    };

    let state = sfx_playing
        .states
        .iter_mut()
        .find(|(key, _)| *key == sfx_key);

    if let Some((_, playing)) = state {
        *playing = false;

        for (entity, sfx) in audio.iter() {
            if sfx == &LoopingSfx(sfx_key) {
                sfx_playing.states.retain(|(key, _)| *key != sfx_key);
                let e = commands.get_entity(entity);
                if let Some(e) = e {
                    e.despawn_recursive();
                }
            }
        }
    }
}

fn stop_all_looping_sfx(
    _trigger: Trigger<StopAllLoopingSfx>,
    mut commands: Commands,
    audio: Query<(Entity, &LoopingSfx)>,
    mut sfx_playing: ResMut<SfxPlaying>,
) {
    let mut s = sfx_playing.states.clone();
    let states = sfx_playing.states.iter_mut();

    for (sfx_key, playing) in states {
        *playing = false;

        for (entity, sfx) in audio.iter() {
            if sfx == &LoopingSfx(*sfx_key) {
                s.retain(|(key, _)| *key != *sfx_key);
                let e = commands.get_entity(entity);
                if let Some(e) = e {
                    e.despawn_recursive();
                }
            }
        }
    }
}

fn play_sfx(
    trigger: Trigger<PlaySfx>,
    mut commands: Commands,
    sfx_handles: Res<HandleMap<SfxKey>>,
) {
    let sfx_key = match trigger.event() {
        PlaySfx::Key(key) => *key,
        PlaySfx::RandomStep => random_step(),
    };
    commands.spawn(AudioSourceBundle {
        source: sfx_handles[&sfx_key].clone_weak(),
        settings: PlaybackSettings {
            mode: PlaybackMode::Despawn,
            ..default()
        },
    });
}

/// Trigger this event to play a single sound effect.
#[derive(Event)]
pub enum PlaySfx {
    Key(SfxKey),
    RandomStep,
}

#[derive(Event)]
pub enum PlayLoopingSfx {
    Key(SfxKey),
}

#[derive(Event)]
pub enum StopLoopingSfx {
    Key(SfxKey),
}

#[derive(Event)]
pub struct StopAllLoopingSfx;

fn random_step() -> SfxKey {
    [SfxKey::Step1, SfxKey::Step2, SfxKey::Step3, SfxKey::Step4]
        .choose(&mut rand::thread_rng())
        .copied()
        .unwrap()
}

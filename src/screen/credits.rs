//! A credits screen that can be accessed from the title screen.

use bevy::prelude::*;

use super::Screen;
use crate::{
    game::{
        assets::{HandleMap, ImageKey, SoundtrackKey},
        audio::soundtrack::PlaySoundtrack,
    },
    ui::prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Credits), enter_credits);

    app.add_systems(
        Update,
        handle_credits_action.run_if(in_state(Screen::Credits)),
    );
    app.register_type::<CreditsAction>();
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
enum CreditsAction {
    Back,
}

fn enter_credits(mut commands: Commands, image_handles: Res<HandleMap<ImageKey>>) {
    commands.spawn((
        SpriteBundle {
            texture: image_handles[&ImageKey::TitleBackground].clone_weak(),
            transform: Transform {
                translation: Vec3::new(0.0, -110.0, -100.0),
                ..default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(1280.0, 1280.0)),
                ..default()
            },
            ..default()
        },
        StateScoped(Screen::Credits),
    ));

    commands
        .ui_root()
        .insert(StateScoped(Screen::Credits))
        .with_children(|children| {
            children.header("Made by");
            children.label("Joda - Game Design and Programming");
            children.label("Hildeblue - Game Design, Art, and Asset Curation");

            children.header("3rd Party Assets");
            children.label("Bevy logo - All rights reserved by the Bevy Foundation. Permission granted for splash screen use when unmodified.");

            children.label("Soundtrack: Main Menu - Clock Tower by DSTechnician");
            children.label("Soundtrack: Gameplay - Fast Jazz by Wavemaster");
            children.label("Soundtrack: Game Over / Credits - Going Forward by u_i8dn2oht5o on Pixabay");

            children.label("SFX: Oil Fill - Pour Water Glug 1 by floraphobic");
            children.label("SFX: All others from Pixabay - Wind-up2, Stomp, metal clanks, Metal_03, Metal Clang Sound, bicycle ringing, Metal Lever 1, Clock ticking, natural room verb");

            children.label("Font: Arrancar by daredemotypo");
            children.label("Font: Guavine by Craft Supply Co");

            children.button("Back").insert(CreditsAction::Back);
        });

    commands.trigger(PlaySoundtrack::Key(SoundtrackKey::Credits));
}

fn handle_credits_action(
    mut commands: Commands,
    mut next_screen: ResMut<NextState<Screen>>,
    mut button_query: InteractionQuery<&CreditsAction>,
) {
    for (interaction, action) in &mut button_query {
        if matches!(interaction, Interaction::Pressed) {
            match action {
                CreditsAction::Back => {
                    next_screen.set(Screen::Title);
                    commands.trigger(PlaySoundtrack::Key(SoundtrackKey::Menu));
                }
            }
        }
    }
}

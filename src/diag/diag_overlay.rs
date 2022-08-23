use bevy::{prelude::*, diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},};
use crate::game_state::GameState;
use crate::movement::MovementState;
use crate::settings::SettingsAsset;

// Tag for UI component
#[derive(Component)]
struct DiagOverlayText;

pub struct DiagOverlayPlugin;

/// Plugin
impl Plugin for DiagOverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin::default());
        app.add_system_set(
            SystemSet::on_enter(GameState::Running)
            .with_system(diag_overlay_setup)
        )
        .add_system_set(
            SystemSet::on_update(GameState::Running)
            .with_system(diag_overlay_update)
        );
    }
}

fn diag_overlay_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands.spawn_bundle(UiCameraBundle::default());

    // Text with multiple sections
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..default()
            },
            // Use `Text` directly
            text: Text {
                // Construct a `Vec` of `TextSection`s
                sections: vec![
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 12.0,
                            color: Color::ALICE_BLUE,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 12.0,
                            color: Color::YELLOW_GREEN,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 12.0,
                            color: Color::ORANGE_RED,
                        },
                    },
                ],
                ..default()
            },
            ..default()
        })
        .insert(DiagOverlayText);
}

fn diag_overlay_update(diagnostics: Res<Diagnostics>,
    mut query: Query<&mut Text, With<DiagOverlayText>>,
    movement_state: Res<MovementState>,
    settings: Res<SettingsAsset>,
) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                let render_mode =
                    if "full" != settings.graphics_settings.render_mode.as_str() {
                        format!(" {}", settings.graphics_settings.render_mode)
                    } else { "".to_string() };
                text.sections[0].value = format!("{:.2}", average);
                text.sections[1].value = format!("{}", render_mode);
                text.sections[2].value = format!("{}", if movement_state.noclip { " noclip" } else { "" });
            }
        }
    }
}

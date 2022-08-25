use crate::game_state::GameState;
use crate::inputs::{CursorLockState};
use crate::loading::{FontAssets};
use crate::menu::ButtonColors;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

// system state
#[derive(Default)]
pub struct PauseMenuState {
    pub ui_entity: Option<Entity>,
}

// plugin
pub struct PauseMenuStatePlugin;

impl Plugin for PauseMenuStatePlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(PauseMenuState::default())
        .add_system_set(SystemSet::on_enter(GameState::Paused)
            .with_system(enter_pause_menu))
        .add_system_set(SystemSet::on_update(GameState::Paused)
            .with_system(click_play_button))
        .add_system_set(SystemSet::on_exit(GameState::Paused)
            .with_system(exit_pause_menu))
        ;
    }
}

fn enter_pause_menu(
    mut commands: Commands,
    mut pause_menu_state: ResMut<PauseMenuState>,
    font_assets: Res<FontAssets>,
    button_colors: Res<ButtonColors>,
    mut rapier_conf: ResMut<RapierConfiguration>,
    mut cursor_lock_controls: ResMut<CursorLockState>,
    mut windows: ResMut<Windows>,
) {
    // pause menu ui
    pause_menu_state.ui_entity = Some(commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(160.0), Val::Px(50.0)),
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: button_colors.normal,
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "Resume".to_string(),
                        style: TextStyle {
                            font: font_assets.fira_sans.clone(),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    }],
                    alignment: Default::default(),
                },
                ..Default::default()
            });
        }).id());

    // pause physics
    rapier_conf.physics_pipeline_active = false;

    // exit cursor lock
    let window = windows.get_primary_mut().unwrap();
    if window.cursor_locked() {
        window.set_cursor_lock_mode(false);
        window.set_cursor_visibility(true);
        cursor_lock_controls.enabled = false;
    }
}

fn click_play_button(
    button_colors: Res<ButtonColors>,
    mut state: ResMut<State<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut cursor_lock_controls: ResMut<CursorLockState>,
    mut windows: ResMut<Windows>,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                state.set(GameState::Running).unwrap();
                // request cursor lock
                let window = windows.get_primary_mut().unwrap();
                window.set_cursor_lock_mode(true);
                window.set_cursor_visibility(false);
                cursor_lock_controls.enabled = true;
            }
            Interaction::Hovered => {
                *color = button_colors.hovered;
            }
            Interaction::None => {
                *color = button_colors.normal;
            }
        }
    }
}

fn exit_pause_menu(
    mut commands: Commands,
    pause_menu: Res<PauseMenuState>,
    mut rapier_conf: ResMut<RapierConfiguration>,
) {
    // despawn ui
    if let Some(ui_entity) = pause_menu.ui_entity {
        commands.entity(ui_entity).despawn_recursive();
    }

    // resume physics
    rapier_conf.physics_pipeline_active = true;
}

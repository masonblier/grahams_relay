use crate::inputs::{CursorLockState};
use crate::loading::{FontAssets,LoadingUiState,LoadingUiEvent,LoadingUiEventAction};
use crate::game_state::GameState;
use crate::menu::PauseMenuStatePlugin;
use bevy::prelude::*;

// system state
#[derive(Default)]
pub struct MainMenuState {
    pub ui_entity: Option<Entity>,
}

pub struct MenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(PauseMenuStatePlugin)
            .init_resource::<ButtonColors>()
            .insert_resource(MainMenuState::default())
            .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(setup_menu))
            .add_system_set(SystemSet::on_update(GameState::Menu).with_system(click_play_button))
            .add_system_set(SystemSet::on_exit(GameState::Menu).with_system(exit_menu));
    }
}

pub struct ButtonColors {
    pub normal: UiColor,
    pub hovered: UiColor,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::rgb(0.15, 0.15, 0.15).into(),
            hovered: Color::rgb(0.25, 0.25, 0.25).into(),
        }
    }
}

fn setup_menu(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    button_colors: Res<ButtonColors>,
    mut main_menu_state: ResMut<MainMenuState>,
    mut loading_ui_events: EventWriter<LoadingUiEvent>,
) {
    // spawn menu
    main_menu_state.ui_entity = Some(commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(120.0), Val::Px(50.0)),
                        margin: UiRect::new(Val::Px(0.),Val::Px(0.),Val::Px(0.),Val::Px(0.),),
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
                                value: "Play".to_string(),
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
                });
                parent.spawn_bundle(TextBundle {
                    style: Style {
                        margin: UiRect::new(Val::Px(0.),Val::Px(0.),Val::Px(0.),Val::Percent(10.),),
                        ..Default::default()
                    },
                    text: Text {
                        sections: vec![TextSection {
                            value: "Graham's Relay".to_string(),
                            style: TextStyle {
                                font: font_assets.fira_sans.clone(),
                                font_size: 60.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        }],
                        alignment: Default::default(),
                    },
                    ..Default::default()
                });
        })
        .id());


    // hide loading ui
    loading_ui_events.send(LoadingUiEvent {
        action: LoadingUiEventAction::Hide,
        payload: None,
    });
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
    mut vis_query: Query<&mut Visibility>,
    loading_ui_state: Res<LoadingUiState>,
    main_menu_state: Res<MainMenuState>,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                state.set(GameState::CharacterLoading).unwrap();
                // request cursor lock
                let window = windows.get_primary_mut().unwrap();
                window.set_cursor_lock_mode(true);
                window.set_cursor_visibility(false);
                cursor_lock_controls.enabled = true;
                // hide menu ui
                let mut vis = vis_query.get_mut(main_menu_state.ui_entity.unwrap()).unwrap();
                vis.is_visible = false;
                vis.set_changed();
                // show loading ui
                let mut vis = vis_query.get_mut(loading_ui_state.ui_entity.unwrap()).unwrap();
                vis.is_visible = true;
                vis.set_changed();
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

fn exit_menu(
    mut commands: Commands,
    main_menu_state: Res<MainMenuState>,
) {
    // despawn ui
    if let Some(ui_entity) = main_menu_state.ui_entity {
        commands.entity(ui_entity).despawn_recursive();
    }
}

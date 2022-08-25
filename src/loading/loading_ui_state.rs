use crate::game_state::GameState;
use bevy::prelude::*;

// system state
#[derive(Default)]
pub struct LoadingUiState {
    pub font_handle: Handle<Font>,
    pub ui_entity: Option<Entity>,
}

pub enum LoadingUiEventAction {
    Hide,
    // Show,
    SetText,
}
pub struct LoadingUiEvent {
    pub action: LoadingUiEventAction,
    pub payload: Option<String>,
}

// Tags for UI components
#[derive(Component)]
struct LoadingUiText;

// plugin
pub struct LoadingUiStatePlugin;

impl Plugin for LoadingUiStatePlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(LoadingUiState::default())
        .add_event::<LoadingUiEvent>()
        .add_system_set(SystemSet::on_enter(GameState::PreLoading)
            .with_system(setup_loading_ui_interaction))
        .add_system(update_loading_ui_interaction)
        ;
    }
}

fn setup_loading_ui_interaction(
    mut commands: Commands,
    mut loading_ui_state: ResMut<LoadingUiState>,
    asset_server: Res<AssetServer>,
) {
    // pre-load font
    loading_ui_state.font_handle = asset_server.load("fonts/FiraSans-Bold.ttf");

    // loading ui
    loading_ui_state.ui_entity = Some(commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                padding: UiRect::new(Val::Percent(1.),Val::Percent(1.),Val::Percent(1.),Val::Percent(1.)),
                ..default()
            },
            color: Color::rgb(0.2, 0.2, 0.2).into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "Loading".to_string(),
                        style: TextStyle {
                            font: loading_ui_state.font_handle.clone(),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    }],
                    alignment: TextAlignment::CENTER,
                },
                ..Default::default()
            })
            .insert(LoadingUiText)
            ;
        }).id());
}

fn update_loading_ui_interaction(
    loading_ui_state: Res<LoadingUiState>,
    mut loading_ui_events: EventReader<LoadingUiEvent>,
    mut vis_query: Query<&mut Visibility>,
    mut text_query: Query<&mut Text, With<LoadingUiText>>,
) {
    for loading_ui_event in loading_ui_events.iter() {
        match &loading_ui_event.action {
            LoadingUiEventAction::Hide => {
                let mut vis = vis_query.get_mut(loading_ui_state.ui_entity.unwrap()).unwrap();
                vis.is_visible = false;
                vis.set_changed();
            },
            // LoadingUiEventAction::Show => {
            //     let mut vis = vis_query.get_mut(loading_ui_state.ui_entity.unwrap()).unwrap();
            //     vis.is_visible = true;
            //     vis.set_changed();
            // },
            LoadingUiEventAction::SetText => {
                let mut text = text_query.single_mut();
                text.sections[0].value = loading_ui_event.payload.clone().unwrap();
            }
        }

    }
}

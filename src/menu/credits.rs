use crate::loading::{FontAssets,LoadingUiEvent,LoadingUiEventAction};
use crate::game_state::GameState;
use bevy::prelude::*;

// system state
#[derive(Default)]
pub struct CreditsState {
    pub ui_entity: Option<Entity>,
    pub time_left: f32,
}


// Tags for UI components
#[derive(Component)]
struct CreditsText;

// plugin
pub struct CreditsStatePlugin;

impl Plugin for CreditsStatePlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(CreditsState::default())
        .add_system_set(SystemSet::on_enter(GameState::Credits)
            .with_system(setup_credits_interaction))
        .add_system_set(SystemSet::on_update(GameState::Credits)
            .with_system(update_credits_interaction))
        ;
    }
}

fn setup_credits_interaction(
    mut commands: Commands,
    mut credits_state: ResMut<CreditsState>,
    font_assets: Res<FontAssets>,
    mut loading_ui_events: EventWriter<LoadingUiEvent>,
) {
    // hide loading ui
    loading_ui_events.send(LoadingUiEvent {
        action: LoadingUiEventAction::Hide,
        payload: None,
    });
    // loading ui
    credits_state.ui_entity = Some(commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
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
                        value: "Thanks for playing Graham's Relay".to_string(),
                        style: TextStyle {
                            font: font_assets.fira_sans.clone(),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    }],
                    alignment: TextAlignment::CENTER,
                },
                ..Default::default()
            })
            .insert(CreditsText { })
            ;
        }).id());
    // default time
    credits_state.time_left = 5.0;
}

fn update_credits_interaction(
    mut commands: Commands,
    mut credits_state: ResMut<CreditsState>,
    mut text_query: Query<&mut Text, With<CreditsText>>,
    mut game_state: ResMut<State<GameState>>,
    time: Res<Time>,
) {
    credits_state.time_left -= time.delta_seconds();

    if credits_state.time_left > 4.0 {
        let mut text = text_query.single_mut();
        let t = (5.0 - credits_state.time_left).max(0.0).min(1.0) * 0.8 + 0.2;
        text.sections[0].style.color = Color::rgb(t, t, t);
    }
    if credits_state.time_left < 1.0 {
        let mut text = text_query.single_mut();
        let t = credits_state.time_left;
        text.sections[0].style.color = Color::rgb(t, t, t);
    }
    if credits_state.time_left < f32::EPSILON {
        commands.entity(credits_state.ui_entity.unwrap()).despawn_recursive();
        game_state.set(GameState::Menu).unwrap();
    }
}

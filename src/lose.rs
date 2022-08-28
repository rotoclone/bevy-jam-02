use crate::*;

pub struct LosePlugin;

impl Plugin for LosePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Lose).with_system(lose_setup))
            .add_system_set(
                SystemSet::on_exit(GameState::Lose)
                    .with_system(despawn_components_system::<LoseComponent>),
            )
            .add_system(restart_button_system);
    }
}

#[derive(Component)]
struct LoseComponent;

#[derive(Component)]
struct RestartButton;

/// Sets up the loss screen.
fn lose_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let title_font = asset_server.load(TITLE_FONT);

    // header text
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(50.0)),
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(0.0),
                    ..default()
                },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                ..default()
            },
            color: UiColor(Color::NONE),
            ..default()
        })
        .insert(LoseComponent)
        .with_children(|parent| {
            parent.spawn_bundle(
                TextBundle::from_section(
                    "All your plants have died.\n\nDead plants are not smart plants.",
                    TextStyle {
                        font: title_font.clone(),
                        font_size: 90.0,
                        color: Color::WHITE,
                    },
                )
                .with_text_alignment(TextAlignment::CENTER)
                .with_style(Style {
                    margin: UiRect::all(Val::Auto),
                    ..default()
                }),
            );
        });

    // restart button
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(50.0)),
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(0.0),
                    ..default()
                },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                ..default()
            },
            color: UiColor(Color::NONE),
            ..default()
        })
        .insert(LoseComponent)
        .with_children(|parent| {
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(250.0), Val::Px(100.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Auto),
                        ..default()
                    },
                    color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .insert(RestartButton)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
                        "Try again",
                        TextStyle {
                            font: title_font.clone(),
                            font_size: 50.0,
                            color: Color::SEA_GREEN,
                        },
                    ));
                });
        });
}

type InteractedRestartButtonTuple = (Changed<Interaction>, With<RestartButton>);

/// Handles interactions with the restart button.
fn restart_button_system(
    mut game_state: ResMut<State<GameState>>,
    interaction_query: Query<&Interaction, InteractedRestartButtonTuple>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            game_state.set(GameState::Game).unwrap();
        }
    }
}

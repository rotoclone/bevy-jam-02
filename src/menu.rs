use crate::*;

const MENU_TEXT: &str = include_str!("../assets/menu.txt");

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Menu).with_system(menu_setup))
            .add_system_set(
                SystemSet::on_exit(GameState::Menu)
                    .with_system(despawn_components_system::<MenuComponent>),
            )
            .add_system(start_button_system);
    }
}

#[derive(Component)]
struct MenuComponent;

#[derive(Component)]
struct StartButton;

/// Sets up the main menu screen.
fn menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let title_font = asset_server.load(TITLE_FONT);
    let main_font = asset_server.load(MAIN_FONT);

    // intro text
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(0.0),
                    ..default()
                },
                justify_content: JustifyContent::FlexEnd,
                align_items: AlignItems::FlexEnd,
                ..default()
            },
            color: UiColor(Color::NONE),
            ..default()
        })
        .insert(MenuComponent)
        .with_children(|parent| {
            parent.spawn_bundle(
                TextBundle::from_section(
                    MENU_TEXT,
                    TextStyle {
                        font: main_font.clone(),
                        font_size: 35.0,
                        color: Color::WHITE,
                    },
                )
                .with_text_alignment(TextAlignment::TOP_CENTER)
                .with_style(Style {
                    margin: UiRect {
                        left: Val::Auto,
                        right: Val::Auto,
                        top: Val::Px(20.0),
                        ..default()
                    },
                    max_size: Size {
                        width: Val::Px(WINDOW_WIDTH * 0.8),
                        ..default()
                    },
                    ..default()
                }),
            );
        });

    // title text
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(40.0)),
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
        .insert(MenuComponent)
        .with_children(|parent| {
            parent.spawn_bundle(
                TextBundle::from_section(
                    "Mr. Smartyplants",
                    TextStyle {
                        font: title_font.clone(),
                        font_size: 95.0,
                        color: Color::WHITE,
                    },
                )
                .with_text_alignment(TextAlignment::CENTER)
                .with_style(Style {
                    margin: UiRect {
                        left: Val::Auto,
                        right: Val::Auto,
                        top: Val::Px(0.0),
                        ..default()
                    },
                    ..default()
                }),
            );
        });

    // start button
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(25.0)),
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(0.0),
                    ..default()
                },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexStart,
                ..default()
            },
            color: UiColor(Color::NONE),
            ..default()
        })
        .insert(MenuComponent)
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
                .insert(StartButton)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
                        "Ok let's go",
                        TextStyle {
                            font: main_font.clone(),
                            font_size: 50.0,
                            color: Color::SEA_GREEN,
                        },
                    ));
                });
        });
}

type InteractedStartButtonTuple = (Changed<Interaction>, With<StartButton>);

/// Handles interactions with the start button.
fn start_button_system(
    mut game_state: ResMut<State<GameState>>,
    interaction_query: Query<&Interaction, InteractedStartButtonTuple>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            game_state.set(GameState::GameLoading).unwrap();
        }
    }
}

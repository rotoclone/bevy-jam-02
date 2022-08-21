use crate::*;

const TOP_BAR_COLOR: Color = Color::rgba(0.0, 0.0, 0.0, 0.5);
const TOP_BAR_HEIGHT: f32 = 40.0;

const BOTTOM_BAR_COLOR: Color = Color::rgba(0.0, 0.0, 0.0, 0.5);
const BOTTOM_BAR_HEIGHT: f32 = 50.0;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(game_setup))
            .add_system_set(
                SystemSet::on_exit(GameState::Game)
                    .with_system(despawn_components_system::<GameComponent>),
            )
            .add_system(next_season_button_system)
            .insert_resource(Season(1));
    }
}

#[derive(Component)]
struct GameComponent;

#[derive(Component)]
struct SeasonText;

#[derive(Component)]
struct NextSeasonButton;

struct Season(u32);

fn game_setup(mut commands: Commands, asset_server: Res<AssetServer>, season: Res<Season>) {
    let main_font = asset_server.load(MAIN_FONT);
    let title_font = asset_server.load(TITLE_FONT);

    // plants section
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(75.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                position: UiRect {
                    right: Val::Px(0.0),
                    ..default()
                },
                padding: UiRect {
                    top: Val::Px(TOP_BAR_HEIGHT),
                    bottom: Val::Px(BOTTOM_BAR_HEIGHT),
                    ..default()
                },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                ..default()
            },
            color: Color::rgb(0.07, 0.43, 0.0).into(),
            ..default()
        })
        .insert(GameComponent)
        .with_children(|parent| {
            parent.spawn_bundle(
                TextBundle::from_section(
                    "Plants",
                    TextStyle {
                        font: title_font.clone(),
                        font_size: 40.0,
                        color: Color::WHITE,
                    },
                )
                .with_text_alignment(TextAlignment::CENTER)
                .with_style(Style {
                    margin: UiRect {
                        left: Val::Auto,
                        right: Val::Auto,
                        top: Val::Px(5.0),
                        bottom: Val::Px(5.0),
                    },
                    ..default()
                }),
            );
        });

    // seeds section
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(25.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(0.0),
                    ..default()
                },
                padding: UiRect {
                    top: Val::Px(TOP_BAR_HEIGHT),
                    bottom: Val::Px(BOTTOM_BAR_HEIGHT),
                    ..default()
                },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                ..default()
            },
            color: Color::rgb(0.23, 0.18, 0.05).into(),
            ..default()
        })
        .insert(GameComponent)
        .with_children(|parent| {
            parent.spawn_bundle(
                TextBundle::from_section(
                    "Seeds",
                    TextStyle {
                        font: title_font.clone(),
                        font_size: 40.0,
                        color: Color::WHITE,
                    },
                )
                .with_text_alignment(TextAlignment::CENTER)
                .with_style(Style {
                    margin: UiRect {
                        left: Val::Auto,
                        right: Val::Auto,
                        top: Val::Px(5.0),
                        bottom: Val::Px(5.0),
                    },
                    ..default()
                }),
            );
        });

    // top bar
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Px(TOP_BAR_HEIGHT)),
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(0.0),
                    ..default()
                },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                ..default()
            },
            color: TOP_BAR_COLOR.into(),
            ..default()
        })
        .insert(GameComponent)
        .with_children(|parent| {
            parent
                .spawn_bundle(
                    TextBundle::from_section(
                        format!("Season {}", season.0),
                        TextStyle {
                            font: main_font.clone(),
                            font_size: 30.0,
                            color: Color::WHITE,
                        },
                    )
                    .with_text_alignment(TextAlignment::CENTER)
                    .with_style(Style {
                        margin: UiRect::all(Val::Auto),
                        ..default()
                    }),
                )
                .insert(SeasonText);
        });

    // bottom bar
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Px(BOTTOM_BAR_HEIGHT)),
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(0.0),
                    ..default()
                },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                ..default()
            },
            color: BOTTOM_BAR_COLOR.into(),
            ..default()
        })
        .insert(GameComponent)
        .with_children(|parent| {
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(200.0), Val::Px(BOTTOM_BAR_HEIGHT * 0.8)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Auto),
                        ..default()
                    },
                    color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .insert(NextSeasonButton)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
                        "Next Season",
                        TextStyle {
                            font: main_font.clone(),
                            font_size: 30.0,
                            color: Color::SEA_GREEN,
                        },
                    ));
                });
        });
}

type InteractedNextSeasonButtonTuple = (Changed<Interaction>, With<NextSeasonButton>);

/// Handles interactions with the next season button.
fn next_season_button_system(
    mut season: ResMut<Season>,
    mut season_text_query: Query<&mut Text, With<SeasonText>>,
    interaction_query: Query<&Interaction, InteractedNextSeasonButtonTuple>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            increment_season(&mut season, &mut season_text_query)
        }
    }
}

fn increment_season(
    season: &mut ResMut<Season>,
    season_text_query: &mut Query<&mut Text, With<SeasonText>>,
) {
    season.0 += 1;

    for mut season_text in season_text_query.iter_mut() {
        season_text.sections[0].value = format!("Season {}", season.0);
    }
}

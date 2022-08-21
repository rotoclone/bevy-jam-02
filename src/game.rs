use crate::*;

const TOP_BAR_COLOR: Color = Color::rgba(1.0, 1.0, 1.0, 0.05);

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
    let font = asset_server.load(MAIN_FONT);

    // top bar
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Px(40.0)),
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
                            font: font.clone(),
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

    // next season button
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
        .insert(GameComponent)
        .with_children(|parent| {
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(200.0), Val::Px(50.0)),
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
                            font: font.clone(),
                            font_size: 30.0,
                            color: Color::SEA_GREEN,
                        },
                    ));
                });
        });

    //TODO
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

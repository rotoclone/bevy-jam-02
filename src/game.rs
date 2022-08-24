use std::collections::HashMap;

use bevy::{ecs::schedule::ShouldRun, ui::FocusPolicy};

use crate::*;

const TOP_BAR_COLOR: Color = Color::rgba(0.0, 0.0, 0.0, 0.5);
const TOP_BAR_HEIGHT: f32 = 40.0;

const BOTTOM_BAR_COLOR: Color = Color::rgba(0.0, 0.0, 0.0, 0.5);
const BOTTOM_BAR_HEIGHT: f32 = 50.0;

const NUM_PLANT_SPACES: usize = 4;
const PLANT_SPACE_SIZE: f32 = 200.0;
const PLANT_SPACE_MARGIN: f32 = 10.0;

const MAX_INTELLIGENCE: usize = 10;
const MAX_PEST_RESISTANCE: usize = 10;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(game_setup))
            .add_system_set(
                SystemSet::on_exit(GameState::Game)
                    .with_system(despawn_components_system::<GameComponent>),
            )
            .add_system(next_season_button_system)
            .add_system(plant_display_system.with_run_criteria(is_set_up))
            .insert_resource(SetUp(false))
            .insert_resource(Season(1))
            .insert_resource(generate_starting_plants());
    }
}

#[derive(Component)]
struct GameComponent;

#[derive(Component)]
struct SeasonText;

#[derive(Component)]
struct NextSeasonButton;

#[derive(Component)]
struct PlantSpace(usize);

impl Plants {
    /// Gets the plant in the provided space, if there is one.
    fn with_id(&self, id: usize) -> Option<&Plant> {
        self.0.get(id)
    }
}

#[derive(Component)]
struct PlantInfo(usize);

struct Season(u32);

struct SetUp(bool);

fn generate_starting_plants() -> Plants {
    //TODO generate plant names?
    let plant_1 = Plant {
        name: "Roberto".to_string(),
        genes: vec![
            Gene::new_with_category(GeneCategory::StemColor(StemColor::Green)),
            Gene::new_with_category(GeneCategory::StemColor(StemColor::Brown)),
            Gene::new_with_category(GeneCategory::StemStyle(StemStyle::Curvy)),
            Gene::new_with_category(GeneCategory::StemStyle(StemStyle::Loopy)),
            Gene::new_with_category(GeneCategory::FruitStyle(FruitStyle::Circle)),
            Gene::new_with_category(GeneCategory::FruitStyle(FruitStyle::Square)),
            Gene::new_with_category(GeneCategory::FruitColor(FruitColor::Red)),
            Gene::new_with_category(GeneCategory::FruitColor(FruitColor::Purple)),
        ],
    };

    let plant_2 = Plant {
        name: "Jessica".to_string(),
        genes: vec![
            Gene::new_with_category(GeneCategory::StemColor(StemColor::Brown)),
            Gene::new_with_category(GeneCategory::StemColor(StemColor::Blue)),
            Gene::new_with_category(GeneCategory::StemStyle(StemStyle::Wiggly)),
            Gene::new_with_category(GeneCategory::StemStyle(StemStyle::Loopy)),
            Gene::new_with_category(GeneCategory::FruitStyle(FruitStyle::Square)),
            Gene::new_with_category(GeneCategory::FruitStyle(FruitStyle::Triangle)),
            Gene::new_with_category(GeneCategory::FruitColor(FruitColor::Red)),
            Gene::new_with_category(GeneCategory::FruitColor(FruitColor::Yellow)),
        ],
    };

    let plant_3 = Plant {
        name: "Francine".to_string(),
        genes: vec![
            Gene::new_with_category(GeneCategory::StemColor(StemColor::Green)),
            Gene::new_with_category(GeneCategory::StemColor(StemColor::Blue)),
            Gene::new_with_category(GeneCategory::StemStyle(StemStyle::Wiggly)),
            Gene::new_with_category(GeneCategory::StemStyle(StemStyle::Angular)),
            Gene::new_with_category(GeneCategory::FruitStyle(FruitStyle::Circle)),
            Gene::new_with_category(GeneCategory::FruitStyle(FruitStyle::Triangle)),
            Gene::new_with_category(GeneCategory::FruitColor(FruitColor::Purple)),
            Gene::new_with_category(GeneCategory::FruitColor(FruitColor::Yellow)),
        ],
    };

    Plants(vec![plant_1, plant_2, plant_3])
}

fn is_set_up(set_up: Res<SetUp>) -> ShouldRun {
    set_up.0.into()
}

fn game_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    season: Res<Season>,
    mut set_up: ResMut<SetUp>,
) {
    let main_font = asset_server.load(MAIN_FONT);
    let title_font = asset_server.load(TITLE_FONT);
    let computer_font = asset_server.load(COMPUTER_FONT);

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
                align_content: AlignContent::Center,
                flex_wrap: FlexWrap::Wrap,
                ..default()
            },
            color: Color::rgb(0.07, 0.43, 0.0).into(),
            ..default()
        })
        .insert(GameComponent)
        .with_children(|parent| {
            // section text
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
                    position_type: PositionType::Absolute,
                    ..default()
                }),
            );

            // wrapper
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(
                            Val::Px(
                                (PLANT_SPACE_SIZE + (PLANT_SPACE_MARGIN * 2.0))
                                    * NUM_PLANT_SPACES as f32,
                            ),
                            Val::Px((PLANT_SPACE_SIZE + (PLANT_SPACE_MARGIN * 2.0)) * 2.0),
                        ),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::FlexEnd,
                        align_content: AlignContent::Center,
                        align_self: AlignSelf::Center,
                        flex_wrap: FlexWrap::Wrap,
                        ..default()
                    },
                    color: Color::NONE.into(),
                    ..default()
                })
                .add_children(|wrapper_parent| {
                    // spaces for plants
                    for i in 0..NUM_PLANT_SPACES {
                        wrapper_parent
                            .spawn_bundle(NodeBundle {
                                style: Style {
                                    size: Size::new(
                                        Val::Px(PLANT_SPACE_SIZE),
                                        Val::Px(PLANT_SPACE_SIZE),
                                    ),
                                    position_type: PositionType::Relative,
                                    margin: UiRect::all(Val::Px(PLANT_SPACE_MARGIN)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::FlexEnd,
                                    align_self: AlignSelf::Center,
                                    ..default()
                                },
                                color: Color::rgb(0.23, 0.18, 0.05).into(),
                                ..default()
                            })
                            .insert(PlantSpace(i))
                            .insert(Interaction::None);
                    }

                    // spaces for plant info
                    for i in 0..NUM_PLANT_SPACES {
                        wrapper_parent
                            .spawn_bundle(NodeBundle {
                                style: Style {
                                    size: Size::new(
                                        Val::Px(PLANT_SPACE_SIZE),
                                        Val::Px(PLANT_SPACE_SIZE),
                                    ),
                                    position_type: PositionType::Relative,
                                    margin: UiRect::all(Val::Px(PLANT_SPACE_MARGIN)),
                                    padding: UiRect::all(Val::Px(5.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    align_self: AlignSelf::Center,
                                    ..default()
                                },
                                color: Color::BLACK.into(),
                                ..default()
                            })
                            .with_children(|plant_info_parent| {
                                plant_info_parent
                                    .spawn_bundle(
                                        TextBundle::from_section(
                                            "",
                                            TextStyle {
                                                font: computer_font.clone(),
                                                font_size: 25.0,
                                                color: Color::GREEN,
                                            },
                                        )
                                        .with_text_alignment(TextAlignment::CENTER),
                                    )
                                    .insert(PlantInfo(i));
                            });
                    }
                    //TODO
                });
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
                    position_type: PositionType::Absolute,
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

    set_up.0 = true;
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

fn plant_display_system(
    plants: Res<Plants>,
    commands: Commands,
    asset_server: Res<AssetServer>,
    plant_spaces_query: Query<(Entity, &PlantSpace)>,
    plant_info_query: Query<(&mut Text, &PlantInfo)>,
) {
    if !plants.is_changed() {
        return;
    }

    update_plant_display(
        plants,
        commands,
        asset_server,
        plant_spaces_query,
        plant_info_query,
    );
}

fn update_plant_display(
    plants: Res<Plants>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    plant_spaces_query: Query<(Entity, &PlantSpace)>,
    mut plant_info_query: Query<(&mut Text, &PlantInfo)>,
) {
    let mut plant_info_text_map = HashMap::new();
    for (text, plant_info) in plant_info_query.iter_mut() {
        plant_info_text_map.insert(plant_info.0, text);
    }

    for (entity, plant_space) in plant_spaces_query.iter() {
        let mut entity_commands = commands.entity(entity);
        entity_commands.despawn_descendants();

        if let Some(plant) = plants.with_id(plant_space.0) {
            let phenotype = plant.get_phenotype();

            // spawn plant images
            entity_commands.with_children(|parent| {
                // stem
                parent.spawn_bundle(ImageBundle {
                    image: get_image_for_stem_style(&phenotype.stem_style, &asset_server).into(),
                    color: get_color_for_stem_color(&phenotype.stem_color).into(),
                    style: Style {
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    focus_policy: FocusPolicy::Pass,
                    ..default()
                });

                // fruit
                parent.spawn_bundle(ImageBundle {
                    image: get_image_for_fruit_style(&phenotype.fruit_style, &asset_server).into(),
                    color: get_color_for_fruit_color(&phenotype.fruit_color).into(),
                    style: Style {
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    focus_policy: FocusPolicy::Pass,
                    ..default()
                });
            });

            // update plant info
            if let Some(text) = plant_info_text_map.get_mut(&plant_space.0) {
                let phenotype = plant.get_phenotype();

                let name_text = format!("Name: {}", plant.name);

                let intelligence = if phenotype.intelligence < 0 {
                    0
                } else {
                    phenotype.intelligence as usize
                };
                let intelligence_filled_bar = "#".repeat(intelligence);
                let intelligence_empty_bar =
                    " ".repeat(MAX_INTELLIGENCE.saturating_sub(intelligence));
                let intelligence_text =
                    format!("Intelligence:\n[{intelligence_filled_bar}{intelligence_empty_bar}]");

                let pest_resistance = if phenotype.pest_resistance < 0 {
                    0
                } else {
                    phenotype.pest_resistance as usize
                };
                let pest_resistance_filled_bar = "#".repeat(pest_resistance);
                let pest_resistance_empty_bar =
                    " ".repeat(MAX_PEST_RESISTANCE.saturating_sub(pest_resistance));
                let pest_resistance_text = format!(
                    "Pest Resistance:\n[{pest_resistance_filled_bar}{pest_resistance_empty_bar}]"
                );

                text.sections[0].value =
                    format!("{name_text}\n\n{intelligence_text}\n{pest_resistance_text}");
            }
        }
    }
}

fn get_image_for_stem_style(style: &StemStyle, asset_server: &Res<AssetServer>) -> Handle<Image> {
    let file_name = match style {
        StemStyle::Curvy => "stem_curvy.png",
        StemStyle::Loopy => "stem_loopy.png",
        StemStyle::Angular => "stem_angular.png",
        StemStyle::Wiggly => "stem_wiggly.png",
    };

    asset_server.load(file_name)
}

fn get_image_for_fruit_style(style: &FruitStyle, asset_server: &Res<AssetServer>) -> Handle<Image> {
    let file_name = match style {
        FruitStyle::Circle => "fruit_circle.png",
        FruitStyle::Square => "fruit_square.png",
        FruitStyle::Triangle => "fruit_triangle.png",
    };

    asset_server.load(file_name)
}

fn get_color_for_stem_color(color: &StemColor) -> Color {
    match color {
        StemColor::Brown => Color::rgb(0.32, 0.27, 0.14),
        StemColor::Green => Color::DARK_GREEN,
        StemColor::Blue => Color::NAVY,
    }
}

fn get_color_for_fruit_color(color: &FruitColor) -> Color {
    match color {
        FruitColor::Red => Color::RED,
        FruitColor::Purple => Color::PURPLE,
        FruitColor::Yellow => Color::YELLOW,
    }
}

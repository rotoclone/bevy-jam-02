use std::collections::HashMap;

use bevy::{ecs::schedule::ShouldRun, transform::TransformSystem};

use crate::*;

const TOP_BAR_COLOR: Color = Color::rgba(0.0, 0.0, 0.0, 0.5);
const TOP_BAR_HEIGHT: f32 = 40.0;

const BOTTOM_BAR_COLOR: Color = Color::rgba(0.0, 0.0, 0.0, 0.5);
const BOTTOM_BAR_HEIGHT: f32 = 50.0;

const NUM_PLANT_SPACES: usize = 4;
const PLANT_SPACE_SIZE: f32 = 200.0;
const PLANT_SPACE_MARGIN: f32 = 10.0;

const NUM_SEED_SPACES: usize = 4;
const SEED_SPACE_WIDTH: f32 = 250.0;
const SEED_SPACE_HEIGHT: f32 = 100.0;
const SEED_SPACE_MARGIN: f32 = 10.0;

const MAX_INTELLIGENCE: usize = 10;
const MAX_PEST_RESISTANCE: usize = 10;

const HELP_TEXT: &str = include_str!("../assets/help.txt");

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(game_setup))
            .add_system_set(
                SystemSet::on_exit(GameState::Game)
                    .with_system(despawn_components_system::<GameComponent>),
            )
            .add_system(next_season_button_system)
            .add_system(help_button_system)
            .add_system(plant_display_system.with_run_criteria(is_set_up))
            .add_system(seed_display_system.with_run_criteria(is_set_up))
            .add_system(being_dragged_system)
            .add_system_to_stage(
                CoreStage::PostUpdate,
                debug_globaltransform.after(TransformSystem::TransformPropagate),
            ) //TODO remove
            .add_system(draggable_pickup_system)
            .add_system(draggable_drop_system.after(being_dragged_system))
            .insert_resource(SetUp(false))
            .insert_resource(Season(1))
            .insert_resource(generate_starting_plants())
            // TODO .insert_resource(Seeds(Vec::new()));
            .insert_resource(Seeds(vec![Seed {
                parent_name_1: "some parent".to_string(),
                parent_name_2: "some other parent".to_string(),
                genes: Vec::new(),
            }]));
    }
}

//TODO remove
fn debug_globaltransform(query: Query<&GlobalTransform, With<BeingDragged>>) {
    for transform in query.iter() {
        println!("Thing at: {:?}", transform.translation());
    }
}

#[derive(Component)]
struct GameComponent;

#[derive(Component)]
struct SeasonText;

#[derive(Component)]
struct NextSeasonButton;

#[derive(Component)]
struct HelpButton;

#[derive(Component)]
struct CloseHelpButton;

#[derive(Component)]
struct HelpScreen;

#[derive(Component)]
struct PlantSpace(usize);

impl Plants {
    /// Gets the plant with the provided ID, if there is one.
    fn with_id(&self, id: usize) -> Option<&Plant> {
        self.0.get(id)
    }
}

#[derive(Component)]
struct SeedSpace(usize);

impl Seeds {
    /// Gets the seed with the provided ID, if there is one.
    fn with_id(&self, id: usize) -> Option<&Seed> {
        self.0.get(id)
    }
}

#[derive(Component)]
struct PlantInfo(usize);

#[derive(Component)]
struct PlantImage(usize);

#[derive(Component)]
struct SeedImage(usize);

#[derive(Component)]
struct CursorEntity; //TODO remove?

#[derive(Component)]
struct Draggable;

#[derive(Component)]
struct BeingDragged {
    original_position: Vec3,
}

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

    // cursor entity
    commands
        .spawn_bundle(TransformBundle::default())
        .insert(CursorEntity);

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
                flex_direction: FlexDirection::ColumnReverse, //TODO remove?
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
                        top: Val::Px(10.0),
                        bottom: Val::Px(10.0),
                    },
                    position: UiRect {
                        top: Val::Px(TOP_BAR_HEIGHT + 10.0),
                        ..default()
                    },
                    position_type: PositionType::Absolute,
                    align_self: AlignSelf::Center,
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
                flex_direction: FlexDirection::ColumnReverse,
                ..default()
            },
            color: Color::rgb(0.23, 0.18, 0.05).into(),
            ..default()
        })
        .insert(GameComponent)
        .with_children(|parent| {
            // section text
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
                        top: Val::Px(10.0),
                        bottom: Val::Px(10.0),
                    },
                    position: UiRect {
                        top: Val::Px(TOP_BAR_HEIGHT + 10.0),
                        ..default()
                    },
                    position_type: PositionType::Absolute,
                    align_self: AlignSelf::Center,
                    ..default()
                }),
            );

            // spaces for seeds
            for i in 0..NUM_SEED_SPACES {
                parent
                    .spawn_bundle(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Px(SEED_SPACE_WIDTH), Val::Px(SEED_SPACE_HEIGHT)),
                            position_type: PositionType::Relative,
                            margin: UiRect::all(Val::Px(SEED_SPACE_MARGIN)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::FlexEnd,
                            align_self: AlignSelf::Center,
                            ..default()
                        },
                        color: Color::BLACK.into(),
                        ..default()
                    })
                    .insert(SeedSpace(i))
                    .insert(Interaction::None);
            }
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
                align_items: AlignItems::Center,
                ..default()
            },
            color: TOP_BAR_COLOR.into(),
            ..default()
        })
        .insert(GameComponent)
        .with_children(|parent| {
            // season display
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

            // help button
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(200.0), Val::Px(TOP_BAR_HEIGHT * 0.8)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Auto),
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            right: Val::Px(5.0),
                            ..default()
                        },
                        ..default()
                    },
                    color: NORMAL_BUTTON.into(),
                    // hide the button cuz the help screen shows underneath the other UI for some reason
                    visibility: Visibility { is_visible: false },
                    ..default()
                })
                .insert(HelpButton)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
                        "Help",
                        TextStyle {
                            font: main_font.clone(),
                            font_size: 30.0,
                            color: Color::SEA_GREEN,
                        },
                    ));
                });
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
            // next season button
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

    // help screen
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(90.0), Val::Percent(90.0)),
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(0.0),
                    ..default()
                },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::rgba(0.0, 0.0, 0.0, 0.9).into(),
            visibility: Visibility { is_visible: false },
            ..default()
        })
        .insert(GameComponent)
        .insert(HelpScreen)
        .with_children(|parent| {
            // help text
            parent
                .spawn_bundle(
                    TextBundle::from_section(
                        HELP_TEXT,
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

            // close button
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(50.0), Val::Px(50.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Auto),
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            top: Val::Px(5.0),
                            right: Val::Px(5.0),
                            ..default()
                        },
                        ..default()
                    },
                    color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .insert(CloseHelpButton)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
                        "X",
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

type InteractedHelpButtonTuple = (Changed<Interaction>, With<HelpButton>);

/// Handles interactions with the help button
fn help_button_system(
    mut help_screen_query: Query<&mut Visibility, With<HelpScreen>>,
    interaction_query: Query<&Interaction, InteractedHelpButtonTuple>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            for mut visibility in help_screen_query.iter_mut() {
                visibility.is_visible = true;
            }
        }
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
                parent
                    .spawn_bundle(ImageBundle {
                        image: get_image_for_stem_style(&phenotype.stem_style, &asset_server)
                            .into(),
                        color: get_color_for_stem_color(&phenotype.stem_color).into(),
                        style: Style {
                            position_type: PositionType::Absolute,
                            ..default()
                        },
                        ..default()
                    })
                    .insert(Draggable)
                    .insert(Interaction::None);

                // fruit
                parent
                    .spawn_bundle(ImageBundle {
                        image: get_image_for_fruit_style(&phenotype.fruit_style, &asset_server)
                            .into(),
                        color: get_color_for_fruit_color(&phenotype.fruit_color).into(),
                        style: Style {
                            position_type: PositionType::Absolute,
                            ..default()
                        },
                        ..default()
                    })
                    .insert(Draggable)
                    .insert(Interaction::None);
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

fn seed_display_system(
    seeds: Res<Seeds>,
    commands: Commands,
    asset_server: Res<AssetServer>,
    seed_spaces_query: Query<(Entity, &SeedSpace)>,
) {
    if !seeds.is_changed() {
        return;
    }

    update_seed_display(seeds, commands, asset_server, seed_spaces_query);
}

fn update_seed_display(
    seeds: Res<Seeds>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    seed_spaces_query: Query<(Entity, &SeedSpace)>,
) {
    let computer_font = asset_server.load(COMPUTER_FONT);

    for (entity, seed_space) in seed_spaces_query.iter() {
        let mut entity_commands = commands.entity(entity);
        entity_commands.despawn_descendants();

        if let Some(seed) = seeds.with_id(seed_space.0) {
            entity_commands.with_children(|parent| {
                // seed image
                parent
                    .spawn_bundle(ImageBundle {
                        image: asset_server.load("seed.png").into(),
                        style: Style {
                            position_type: PositionType::Absolute,
                            ..default()
                        },
                        ..default()
                    })
                    .insert(SeedImage(seed_space.0))
                    .insert(Draggable)
                    .insert(Interaction::None);

                // seed description
                parent.spawn_bundle(
                    TextBundle::from_section(
                        format!("{} + {}", seed.parent_name_1, seed.parent_name_2),
                        TextStyle {
                            font: computer_font.clone(),
                            font_size: 20.0,
                            color: Color::GREEN,
                        },
                    )
                    .with_style(Style {
                        align_self: AlignSelf::FlexStart,
                        ..default()
                    }),
                );
            });
        }
    }
}

/* TODO
/// Handles updating the position of entities that are being dragged by the mouse.
fn being_dragged_system(
    cursor_position: Res<CursorPosition>,
    mut dragged_query: Query<&mut Transform, With<BeingDragged>>,
) {
    if let Some(pos) = cursor_position.0 {
        for mut transform in dragged_query.iter_mut() {
            println!("moving a thing to {pos}"); //TODO remove
            transform.translation.x = pos.x;
            transform.translation.y = pos.y;
        }
    }
}
*/

/// Handles updating the position of entities that are being dragged by the mouse.
fn being_dragged_system(
    cursor_position: Res<CursorPosition>,
    window_dimensions: Res<WindowDimensions>,
    mut dragged_query: Query<(&mut Style, &BeingDragged)>,
) {
    if let Some(pos) = cursor_position.0 {
        for (mut style, being_dragged) in dragged_query.iter_mut() {
            //TODO remove vvv
            println!("moving a thing to {pos}");
            println!("original position: {}", being_dragged.original_position);
            println!(
                "window dimensions: {},{}",
                window_dimensions.0.x, window_dimensions.0.y
            );
            //original_pos.x - pos.x = (window.x / 2)
            //TODO remove ^^^
            let original_position = upper_left_origin_to_middle_origin(
                &being_dragged.original_position.truncate(),
                &window_dimensions.0,
            );
            println!("converted original position: {}", original_position); //TODO remove
            style.position = UiRect {
                left: Val::Px(pos.x - original_position.x),
                bottom: Val::Px(pos.y - original_position.y),
                ..default()
            };
            println!(
                "calculated offset: {},{}",
                pos.x - original_position.x,
                pos.y - original_position.y
            ); //TODO remove
        }
    }
}

fn upper_left_origin_to_middle_origin(coords: &Vec2, window_dimensions: &Vec2) -> Vec2 {
    let x = coords.x - (window_dimensions.x / 2.0);
    let y = coords.y - (window_dimensions.y / 2.0);

    Vec2 { x, y }
}

type InteractedDraggableTuple = (Changed<Interaction>, With<Draggable>);

/// Handles picking up things with the mouse.
fn draggable_pickup_system(
    mut commands: Commands,
    interaction_query: Query<(&Interaction, &GlobalTransform, Entity), InteractedDraggableTuple>,
) {
    for (interaction, transform, entity) in interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            println!("picked up a thing"); //TODO remove
            commands.entity(entity).insert(BeingDragged {
                original_position: transform.translation(),
            });
        }
    }
}

/// Handles dropping things that are being dragged.
fn draggable_drop_system(
    mut commands: Commands,
    mouse_buttons: Res<Input<MouseButton>>,
    mut dragged_query: Query<(Entity, &mut Style), With<BeingDragged>>,
) {
    if !mouse_buttons.pressed(MouseButton::Left) {
        for (entity, mut style) in dragged_query.iter_mut() {
            commands.entity(entity).remove::<BeingDragged>();
            style.position = UiRect::default();
        }
    }
}

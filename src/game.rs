use std::collections::HashMap;

use bevy::ecs::schedule::ShouldRun;

use crate::*;

const TOP_BAR_COLOR: Color = Color::rgba(0.0, 0.0, 0.0, 0.5);
const TOP_BAR_HEIGHT: f32 = 40.0;

const BOTTOM_BAR_COLOR: Color = Color::rgba(0.0, 0.0, 0.0, 0.5);
const BOTTOM_BAR_HEIGHT: f32 = 50.0;

const NUM_PLANT_SPACES: usize = 4;
const PLANT_SPACE_SIZE: f32 = 200.0;
const PLANT_SPACE_HEIGHT: f32 = 300.0;
const PLANT_SPACE_MARGIN: f32 = 10.0;

const NUM_SEED_SPACES: usize = 4;
const SEED_SPACE_SIZE: f32 = 100.0;
const SEED_SPACE_MARGIN: f32 = 10.0;

const SEED_TOOLTIP_WIDTH: f32 = 200.0;
const SEED_TOOLTIP_OFFSET: f32 = -15.0;

const MAX_INTELLIGENCE: usize = 10;
const MAX_PEST_RESISTANCE: usize = 10;

const GOAL_INTELLIGENCE: i32 = 10;

const HELP_TEXT: &str = include_str!("../assets/help.txt");

const SEEDS_SECTION_WIDTH: f32 = WINDOW_WIDTH * 0.25;
const SEEDS_SECTION_HEIGHT: f32 = WINDOW_HEIGHT - TOP_BAR_HEIGHT - BOTTOM_BAR_HEIGHT;

const PLANTS_SECTION_WIDTH: f32 = WINDOW_WIDTH * 0.75;
const PLANTS_SECTION_HEIGHT: f32 = WINDOW_HEIGHT - TOP_BAR_HEIGHT - BOTTOM_BAR_HEIGHT;

const SEEDS_SECTION_START_X: f32 = -(WINDOW_WIDTH / 2.0);
const PLANTS_SECTION_START_X: f32 = -(WINDOW_WIDTH / 2.0) + SEEDS_SECTION_WIDTH;

const SECTION_MARGIN: f32 = 20.0;

const BACKGROUND_LAYER: f32 = 10.0;
const MIDDLE_LAYER: f32 = 20.0;
const PLANTS_LAYER: f32 = 30.0;
const SEEDS_LAYER: f32 = 40.0;
const TOOLTIP_LAYER: f32 = 50.0;

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
            .add_system(
                plant_display_system
                    .with_run_criteria(is_set_up)
                    .after(seed_plant_system),
            )
            .add_system(
                seed_display_system
                    .with_run_criteria(is_set_up)
                    .after(seed_plant_system),
            )
            .add_system(being_dragged_system)
            .add_system(draggable_pickup_system)
            .add_system(
                plant_splice_system
                    .after(being_dragged_system)
                    .before(draggable_drop_system),
            )
            .add_system(
                seed_plant_system
                    .after(being_dragged_system)
                    .before(draggable_drop_system),
            )
            .add_system(draggable_drop_system.after(being_dragged_system))
            .add_system(seed_tooltip_system.after(draggable_drop_system))
            .add_system(check_lose_system)
            .add_system(check_win_system.after(check_lose_system))
            .insert_resource(SetUp(false))
            .insert_resource(Season(1))
            .insert_resource(generate_starting_plants())
            .insert_resource(Seeds(Vec::new()));
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

impl Planters {
    /// Gets the planter with the provided ID, if there is one.
    fn with_id(&self, id: usize) -> Option<&Planter> {
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

    /// Removes and returns the seed with the provided ID, if there is one.
    fn take_with_id(&mut self, id: usize) -> Option<Seed> {
        if id < self.0.len() {
            Some(self.0.remove(id))
        } else {
            None
        }
    }
}

#[derive(Component)]
struct PlantImage(usize);

#[derive(Component)]
struct PlantInfo(usize);

#[derive(Component)]
struct SeedImage(usize);

#[derive(Component)]
struct SeedInfo(usize);

#[derive(Component)]
struct Interactable {
    size: Vec2,
}

#[derive(Component)]
struct Draggable;

#[derive(Component)]
struct BeingDragged {
    original_position: Vec3,
}

struct Season(u32);

struct SetUp(bool);

fn generate_starting_plants() -> Planters {
    //TODO generate plant names?
    let plant_1 = Plant {
        name: vec!["ro", "ber", "to"].into(),
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
        name: vec!["jes", "si", "ca"].into(),
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
        name: vec!["gre", "go", "ry"].into(),
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

    Planters(vec![
        Planter::Plant(plant_1),
        Planter::Plant(plant_2),
        Planter::Plant(plant_3),
        Planter::Empty,
    ])
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

    //
    // plants section
    //

    // background
    //TODO

    let plants_section_center_x = PLANTS_SECTION_START_X + (PLANTS_SECTION_WIDTH / 2.0);

    // section text
    commands.spawn_bundle(Text2dBundle {
        text: Text::from_section(
            "Plants",
            TextStyle {
                font: title_font.clone(),
                font_size: 40.0,
                color: Color::WHITE,
            },
        )
        .with_alignment(TextAlignment::CENTER),
        transform: Transform {
            translation: Vec3::new(
                plants_section_center_x,
                (PLANTS_SECTION_HEIGHT / 2.0) - SECTION_MARGIN,
                MIDDLE_LAYER,
            ),
            ..default()
        },
        ..default()
    });

    let plant_spaces_start = plants_section_center_x
        - (((PLANT_SPACE_MARGIN + PLANT_SPACE_SIZE) * NUM_PLANT_SPACES as f32) / 2.0);

    for i in 0..NUM_PLANT_SPACES {
        let x_coord = plant_spaces_start
            + ((PLANT_SPACE_MARGIN + PLANT_SPACE_SIZE) * i as f32)
            + (PLANT_SPACE_SIZE / 2.0);

        let plant_info_y_coord = (PLANT_SPACE_MARGIN / 2.0) + (PLANT_SPACE_SIZE / 2.0);

        // space for plant info
        commands.spawn_bundle(SpriteBundle {
            texture: asset_server.load("plant_info_space.png"),
            transform: Transform {
                translation: Vec3::new(x_coord, plant_info_y_coord, MIDDLE_LAYER),
                ..default()
            },
            ..default()
        });

        // plant info
        commands
            .spawn_bundle(Text2dBundle {
                text: Text::from_section(
                    "",
                    TextStyle {
                        font: computer_font.clone(),
                        font_size: 25.0,
                        color: Color::GREEN,
                    },
                )
                .with_alignment(TextAlignment::CENTER),
                transform: Transform {
                    translation: Vec3::new(x_coord, plant_info_y_coord, MIDDLE_LAYER + 1.0),
                    ..default()
                },
                ..default()
            })
            .insert(PlantInfo(i));

        // space for plant
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("plant_space.png"),
                transform: Transform {
                    translation: Vec3::new(
                        x_coord,
                        -(PLANT_SPACE_MARGIN / 2.0) - (PLANT_SPACE_HEIGHT / 2.0),
                        MIDDLE_LAYER,
                    ),
                    ..default()
                },
                ..default()
            })
            .insert(PlantSpace(i))
            .insert(Interactable {
                size: Vec2::new(PLANT_SPACE_SIZE, PLANT_SPACE_HEIGHT),
            });
    }

    //
    // seeds section
    //

    // background
    //TODO

    // section text
    commands.spawn_bundle(Text2dBundle {
        text: Text::from_section(
            "Seeds",
            TextStyle {
                font: title_font.clone(),
                font_size: 40.0,
                color: Color::WHITE,
            },
        )
        .with_alignment(TextAlignment::CENTER),
        transform: Transform {
            translation: Vec3::new(
                SEEDS_SECTION_START_X + (SEEDS_SECTION_WIDTH / 2.0),
                (SEEDS_SECTION_HEIGHT / 2.0) - SECTION_MARGIN,
                MIDDLE_LAYER,
            ),
            ..default()
        },
        ..default()
    });

    let seed_spaces_start = ((SEED_SPACE_MARGIN + SEED_SPACE_SIZE) * NUM_SEED_SPACES as f32) / 2.0;

    for i in 0..NUM_SEED_SPACES {
        let y_coord = seed_spaces_start
            - ((SEED_SPACE_MARGIN + SEED_SPACE_SIZE) * i as f32)
            - (SEED_SPACE_SIZE / 2.0);

        // space for seed
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("seed_space.png"),
                transform: Transform {
                    translation: Vec3::new(
                        SEEDS_SECTION_START_X + (SEEDS_SECTION_WIDTH / 2.0),
                        y_coord,
                        MIDDLE_LAYER,
                    ),
                    ..default()
                },
                ..default()
            })
            .insert(SeedSpace(i));
    }

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
    mut planters: ResMut<Planters>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            increment_season(&mut season, &mut season_text_query);
            planters.next_season();
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
    planters: Res<Planters>,
    commands: Commands,
    asset_server: Res<AssetServer>,
    plant_spaces_query: Query<(&Transform, &PlantSpace)>,
    plant_images_query: Query<Entity, With<PlantImage>>,
    plant_info_query: Query<(&mut Text, &PlantInfo)>,
) {
    if !planters.is_changed() {
        return;
    }

    update_plant_display(
        planters,
        commands,
        asset_server,
        plant_spaces_query,
        plant_images_query,
        plant_info_query,
    );
}

fn update_plant_display(
    planters: Res<Planters>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    plant_spaces_query: Query<(&Transform, &PlantSpace)>,
    plant_images_query: Query<Entity, With<PlantImage>>,
    mut plant_info_query: Query<(&mut Text, &PlantInfo)>,
) {
    for entity in plant_images_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    let mut plant_info_text_map = HashMap::new();
    for (text, plant_info) in plant_info_query.iter_mut() {
        plant_info_text_map.insert(plant_info.0, text);
    }

    for (transform, plant_space) in plant_spaces_query.iter() {
        if let Some(planter) = planters.with_id(plant_space.0) {
            match planter {
                Planter::Plant(plant) => {
                    let phenotype = plant.get_phenotype();

                    // spawn plant images
                    commands
                        .spawn_bundle(SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(Vec2::new(PLANT_SPACE_SIZE, PLANT_SPACE_SIZE)),
                                color: Color::NONE,
                                ..default()
                            },
                            transform: Transform {
                                translation: Vec3::new(
                                    transform.translation.x,
                                    transform.translation.y
                                        + ((PLANT_SPACE_HEIGHT - PLANT_SPACE_SIZE) / 2.0),
                                    PLANTS_LAYER,
                                ),
                                ..default()
                            },
                            ..default()
                        })
                        .insert(PlantImage(plant_space.0))
                        .insert(Draggable)
                        .insert(Interactable {
                            size: Vec2::new(200.0, 200.0),
                        })
                        .with_children(|parent| {
                            // stem
                            parent.spawn_bundle(SpriteBundle {
                                texture: get_image_for_stem_style(
                                    &phenotype.stem_style,
                                    &asset_server,
                                ),
                                sprite: Sprite {
                                    color: get_color_for_stem_color(&phenotype.stem_color),
                                    ..default()
                                },
                                ..default()
                            });

                            // fruit
                            parent.spawn_bundle(SpriteBundle {
                                texture: get_image_for_fruit_style(
                                    &phenotype.fruit_style,
                                    &asset_server,
                                ),
                                sprite: Sprite {
                                    color: get_color_for_fruit_color(&phenotype.fruit_color),
                                    ..default()
                                },
                                transform: Transform {
                                    translation: Vec3::new(0.0, 0.0, 1.0),
                                    ..default()
                                },
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
                        let intelligence_text = format!(
                            "Intelligence:\n[{intelligence_filled_bar}{intelligence_empty_bar}]"
                        );

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
                Planter::DeadPlant(dead_plant) => {
                    // spawn plant image
                    commands
                        .spawn_bundle(SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(Vec2::new(PLANT_SPACE_SIZE, PLANT_SPACE_SIZE)),
                                color: Color::NONE,
                                ..default()
                            },
                            transform: Transform {
                                translation: Vec3::new(
                                    transform.translation.x,
                                    transform.translation.y
                                        + ((PLANT_SPACE_HEIGHT - PLANT_SPACE_SIZE) / 2.0),
                                    PLANTS_LAYER,
                                ),
                                ..default()
                            },
                            ..default()
                        })
                        .insert(PlantImage(plant_space.0))
                        .insert(Interactable {
                            size: Vec2::new(200.0, 200.0),
                        })
                        .with_children(|parent| {
                            parent.spawn_bundle(SpriteBundle {
                                texture: asset_server.load("dead_plant.png"),
                                ..default()
                            });
                        });

                    // update plant info
                    if let Some(text) = plant_info_text_map.get_mut(&plant_space.0) {
                        text.sections[0].value =
                            format!("RIP {}\n\nEaten by pests", dead_plant.name);
                    }
                }
                Planter::Seed(seed) => {
                    // spawn plant image
                    commands
                        .spawn_bundle(SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(Vec2::new(PLANT_SPACE_SIZE, PLANT_SPACE_SIZE)),
                                color: Color::NONE,
                                ..default()
                            },
                            transform: Transform {
                                translation: Vec3::new(
                                    transform.translation.x,
                                    transform.translation.y
                                        + ((PLANT_SPACE_HEIGHT - PLANT_SPACE_SIZE) / 2.0),
                                    PLANTS_LAYER,
                                ),
                                ..default()
                            },
                            ..default()
                        })
                        .insert(PlantImage(plant_space.0))
                        .insert(Interactable {
                            size: Vec2::new(200.0, 200.0),
                        })
                        .with_children(|parent| {
                            parent.spawn_bundle(SpriteBundle {
                                texture: asset_server.load("planted_seed.png"),
                                ..default()
                            });
                        });

                    // update plant info
                    if let Some(text) = plant_info_text_map.get_mut(&plant_space.0) {
                        text.sections[0].value = format!(
                            "A seed made from\n{}\nand\n{}",
                            seed.parent_name_1, seed.parent_name_2
                        );
                    }
                }
                Planter::Empty => (),
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
    seed_images_query: Query<Entity, With<SeedImage>>,
    seed_info_query: Query<Entity, With<SeedInfo>>,
    seed_spaces_query: Query<(&Transform, &SeedSpace)>,
) {
    if !seeds.is_changed() {
        return;
    }

    update_seed_display(
        seeds,
        commands,
        asset_server,
        seed_images_query,
        seed_info_query,
        seed_spaces_query,
    );
}

fn update_seed_display(
    seeds: Res<Seeds>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    seed_images_query: Query<Entity, With<SeedImage>>,
    seed_info_query: Query<Entity, With<SeedInfo>>,
    seed_spaces_query: Query<(&Transform, &SeedSpace)>,
) {
    let main_font = asset_server.load(MAIN_FONT);

    for entity in seed_images_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    for entity in seed_info_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    for (transform, seed_space) in seed_spaces_query.iter() {
        if let Some(seed) = seeds.with_id(seed_space.0) {
            // seed image
            commands
                .spawn_bundle(SpriteBundle {
                    texture: asset_server.load("seed.png"),
                    transform: Transform {
                        translation: Vec3::new(
                            transform.translation.x,
                            transform.translation.y,
                            SEEDS_LAYER,
                        ),
                        ..default()
                    },
                    ..default()
                })
                .insert(SeedImage(seed_space.0))
                .insert(Draggable)
                .insert(Interactable {
                    size: Vec2::new(100.0, 100.0),
                });

            // seed info
            commands
                .spawn_bundle(SpriteBundle {
                    texture: asset_server.load("seed_tooltip_background.png"),
                    transform: Transform {
                        translation: Vec3::new(
                            transform.translation.x
                                + (SEED_SPACE_SIZE / 2.0)
                                + (SEED_TOOLTIP_WIDTH / 2.0)
                                + SEED_TOOLTIP_OFFSET,
                            transform.translation.y,
                            TOOLTIP_LAYER,
                        ),
                        ..default()
                    },
                    visibility: Visibility { is_visible: false },
                    ..default()
                })
                .insert(SeedInfo(seed_space.0))
                .with_children(|parent| {
                    parent.spawn_bundle(Text2dBundle {
                        text: Text::from_section(
                            format!("{}\n+\n{}", seed.parent_name_1, seed.parent_name_2),
                            TextStyle {
                                font: main_font.clone(),
                                font_size: 20.0,
                                color: Color::WHITE,
                            },
                        )
                        .with_alignment(TextAlignment::CENTER),
                        transform: Transform {
                            translation: Vec3::new(0.0, 0.0, 1.0),
                            ..default()
                        },
                        ..default()
                    });
                });
        }
    }
}

/// Handles showing and hiding seed tooltips
fn seed_tooltip_system(
    buttons: Res<Input<MouseButton>>,
    cursor_position: Res<CursorPosition>,
    seed_images_query: Query<(&Transform, &Interactable, &SeedImage)>,
    mut tooltip_query: Query<(&mut Visibility, &SeedInfo)>,
) {
    if buttons.pressed(MouseButton::Left) {
        // hide all tooltips
        for (mut visibility, _) in tooltip_query.iter_mut() {
            visibility.is_visible = false;
        }
    } else {
        let mut hovered_seed_id = None;
        if let Some(pos) = cursor_position.0 {
            for (transform, interactable, seed_image) in seed_images_query.iter() {
                if intersects(pos, transform.translation.truncate(), interactable.size) {
                    hovered_seed_id = Some(seed_image.0);
                    break;
                }
            }
        }

        if let Some(id) = hovered_seed_id {
            for (mut visibility, seed_info) in tooltip_query.iter_mut() {
                if seed_info.0 == id {
                    visibility.is_visible = true;
                } else {
                    visibility.is_visible = false;
                }
            }
        } else {
            // hide all tooltips
            for (mut visibility, _) in tooltip_query.iter_mut() {
                visibility.is_visible = false;
            }
        }
    }
}

/// Handles updating the position of entities that are being dragged by the mouse.
fn being_dragged_system(
    cursor_position: Res<CursorPosition>,
    mut dragged_query: Query<&mut Transform, With<BeingDragged>>,
) {
    if let Some(pos) = cursor_position.0 {
        for mut transform in dragged_query.iter_mut() {
            transform.translation.x = pos.x;
            transform.translation.y = pos.y;
        }
    }
}

/// Handles picking up things with the mouse.
fn draggable_pickup_system(
    buttons: Res<Input<MouseButton>>,
    cursor_position: Res<CursorPosition>,
    mut commands: Commands,
    draggable_query: Query<(&Transform, &Interactable, Entity), With<Draggable>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(pos) = cursor_position.0 {
            for (transform, interactable, entity) in draggable_query.iter() {
                if intersects(pos, transform.translation.truncate(), interactable.size) {
                    commands.entity(entity).insert(BeingDragged {
                        original_position: transform.translation,
                    });
                }
            }
        }
    }
}

/// Determines whether a point intersects a space
fn intersects(point: Vec2, center_point: Vec2, size: Vec2) -> bool {
    point.x >= center_point.x - (size.x / 2.0) - 1.0
        && point.x <= center_point.x + (size.x / 2.0) + 1.0
        && point.y >= center_point.y - (size.y / 2.0) - 1.0
        && point.y <= center_point.y + (size.y / 2.0) + 1.0
}

fn plant_splice_system(
    mouse_buttons: Res<Input<MouseButton>>,
    cursor_position: Res<CursorPosition>,
    planters: Res<Planters>,
    mut seeds: ResMut<Seeds>,
    dragged_plant_query: Query<&PlantImage, With<BeingDragged>>,
    plant_space_query: Query<(&Transform, &PlantSpace, &Interactable)>,
) {
    if !mouse_buttons.pressed(MouseButton::Left) {
        if let Some(pos) = cursor_position.0 {
            let dragged_plant_id = dragged_plant_query.get_single().ok().map(|image| image.0);
            let mut target_plant_id = None;
            for (transform, plant_space, interactable) in plant_space_query.iter() {
                if intersects(pos, transform.translation.truncate(), interactable.size) {
                    target_plant_id = Some(plant_space.0);
                    break;
                }
            }

            if dragged_plant_id == target_plant_id {
                return;
            }

            let dragged_plant = dragged_plant_id.and_then(|id| planters.with_id(id));
            let target_plant = target_plant_id.and_then(|id| planters.with_id(id));

            if let Some(Planter::Plant(plant_1)) = dragged_plant {
                if let Some(Planter::Plant(plant_2)) = target_plant {
                    if seeds.0.len() < NUM_SEED_SPACES {
                        let new_seed = splice_plants(plant_1, plant_2);
                        seeds.0.push(new_seed);
                    }
                }
            }
        }
    }
}

fn seed_plant_system(
    mut commands: Commands,
    mouse_buttons: Res<Input<MouseButton>>,
    cursor_position: Res<CursorPosition>,
    mut planters: ResMut<Planters>,
    mut seeds: ResMut<Seeds>,
    dragged_seed_query: Query<(Entity, &SeedImage), With<BeingDragged>>,
    plant_space_query: Query<(&Transform, &PlantSpace, &Interactable)>,
) {
    if !mouse_buttons.pressed(MouseButton::Left) {
        if let Some(pos) = cursor_position.0 {
            let mut target_planter_id = None;
            for (transform, plant_space, interactable) in plant_space_query.iter() {
                if intersects(pos, transform.translation.truncate(), interactable.size) {
                    target_planter_id = Some(plant_space.0);
                    break;
                }
            }

            if let Some(planter_id) = target_planter_id {
                for (entity, seed_image) in dragged_seed_query.iter() {
                    if let Some(seed) = seeds.take_with_id(seed_image.0) {
                        planters.0[planter_id] = Planter::Seed(seed);
                        commands.entity(entity).despawn_recursive();
                    }
                }
            }
        }
    }
}

/// Handles dropping things that are being dragged.
fn draggable_drop_system(
    mut commands: Commands,
    mouse_buttons: Res<Input<MouseButton>>,
    mut dragged_query: Query<(Entity, &mut Transform, &BeingDragged)>,
) {
    if !mouse_buttons.pressed(MouseButton::Left) {
        for (entity, mut transform, being_dragged) in dragged_query.iter_mut() {
            transform.translation = being_dragged.original_position;
            commands.entity(entity).remove::<BeingDragged>();
        }
    }
}

/// Moves to the win state if the player has won
fn check_win_system(planters: Res<Planters>, mut game_state: ResMut<State<GameState>>) {
    let has_smart_plant = planters.0.iter().any(|planter| {
        if let Planter::Plant(plant) = planter {
            plant.get_phenotype().intelligence >= GOAL_INTELLIGENCE as i32
        } else {
            false
        }
    });

    if has_smart_plant {
        game_state.overwrite_set(GameState::Win).unwrap();
    }
}

/// Moves to the lose state if the player has lost
fn check_lose_system(
    planters: Res<Planters>,
    seeds: Res<Seeds>,
    mut game_state: ResMut<State<GameState>>,
) {
    let has_plant_or_planted_seed = planters
        .0
        .iter()
        .any(|planter| matches!(planter, Planter::Plant(_)) || matches!(planter, Planter::Seed(_)));

    let has_seed = !seeds.0.is_empty();

    if !has_plant_or_planted_seed && !has_seed {
        game_state.overwrite_set(GameState::Lose).unwrap();
    }
}

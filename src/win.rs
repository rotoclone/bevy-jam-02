use rand::Rng;

use crate::*;

const PLANT_TOO_SMART_CHANCE: f32 = 0.1;

pub struct WinPlugin;

impl Plugin for WinPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Win)
                .with_system(win_setup)
                .with_system(play_victory_sound),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Win)
                .with_system(despawn_components_system::<WinComponent>),
        );
    }
}

#[derive(Component)]
struct WinComponent;

/// Sets up the win screen.
fn win_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    season: Res<Season>,
    smart_plant: Res<SmartPlant>,
    image_assets: Res<ImageAssets>,
) {
    let title_font = asset_server.load(TITLE_FONT);
    let main_font = asset_server.load(MAIN_FONT);

    // gotta have a smart plant if we're at this screen
    let plant = smart_plant.0.as_ref().unwrap();

    // header text
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(33.0)),
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
        .insert(WinComponent)
        .with_children(|parent| {
            parent.spawn_bundle(
                TextBundle::from_section(
                    format!("After {} seasons, you grew a real smart plant:", season.0),
                    TextStyle {
                        font: title_font.clone(),
                        font_size: 60.0,
                        color: Color::WHITE,
                    },
                )
                .with_text_alignment(TextAlignment::CENTER)
                .with_style(Style {
                    margin: UiRect {
                        left: Val::Auto,
                        right: Val::Auto,
                        top: Val::Px(50.0),
                        ..default()
                    },
                    ..default()
                }),
            );
        });

    // plant space
    let plant_space_transform = Transform {
        translation: Vec3::new(0.0, 70.0, MIDDLE_LAYER),
        ..default()
    };
    commands
        .spawn_bundle(SpriteBundle {
            texture: image_assets.plant_space.clone(),
            transform: plant_space_transform,
            ..default()
        })
        .insert(WinComponent);

    let phenotype = plant.get_phenotype();

    // plant image
    spawn_plant_image(
        &mut commands,
        &plant_space_transform,
        &phenotype,
        &image_assets,
        0,
        WinComponent,
    );

    // plant glasses
    commands
        .spawn_bundle(SpriteBundle {
            texture: image_assets.glasses.clone(),
            transform: Transform {
                translation: Vec3::new(
                    plant_space_transform.translation.x,
                    plant_space_transform.translation.y
                        + ((PLANT_SPACE_HEIGHT - PLANT_SPACE_SIZE) / 2.0),
                    PLANTS_LAYER + 2.0,
                ),
                ..default()
            },
            ..default()
        })
        .insert(WinComponent);

    // plant name
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section(
                plant.name.to_string(),
                TextStyle {
                    font: title_font,
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            )
            .with_alignment(TextAlignment::CENTER),
            transform: Transform {
                translation: Vec3::new(
                    0.0,
                    plant_space_transform.translation.y - (PLANT_SPACE_HEIGHT / 2.0) - 20.0,
                    MIDDLE_LAYER,
                ),
                ..default()
            },
            ..default()
        })
        .insert(WinComponent);

    // more text
    let extra_intelligence = phenotype.intelligence - GOAL_INTELLIGENCE;
    let take_credit_chance = extra_intelligence as f32 * PLANT_TOO_SMART_CHANCE;
    let end_text = if rand::thread_rng().gen::<f32>() <= take_credit_chance {
        format!("{} solved an unsolved math problem, but took credit for it themselves!\nThey used the prize money to start their own farm and you are forced to work for them. The pay is pretty good. You hate to admit it, but {} is actually way better at running a farm than you were.", plant.name, plant.name)
    } else {
        format!("{} solved an unsolved math problem, and you were able to use the prize money to buy a sweet new combine harvester.\nYour farm is saved!\n{} also gives you some tips for running your farm, which helps.", plant.name, plant.name)
    };
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(33.0)),
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
        .insert(WinComponent)
        .with_children(|parent| {
            parent.spawn_bundle(
                TextBundle::from_section(
                    end_text,
                    TextStyle {
                        font: main_font.clone(),
                        font_size: 40.0,
                        color: Color::WHITE,
                    },
                )
                .with_text_alignment(TextAlignment::CENTER)
                .with_style(Style {
                    margin: UiRect::all(Val::Auto),
                    max_size: Size {
                        width: Val::Px(WINDOW_WIDTH * 0.8),
                        ..default()
                    },
                    ..default()
                }),
            );
        });
}

fn play_victory_sound(audio_assets: Res<AudioAssets>, audio: Res<AudioChannel<ForegroundChannel>>) {
    audio.play(audio_assets.victory.clone()).with_volume(0.5);
}

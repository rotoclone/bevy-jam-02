use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};

mod menu;
use menu::*;

mod game;
use game::*;

mod win;
use win::*;

const DEV_MODE: bool = true;

const MAIN_FONT: &str = "fonts/Quicksand-Medium.ttf";
const TITLE_FONT: &str = "fonts/FredokaOne-Regular.ttf";

const NORMAL_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const HOVERED_BUTTON: Color = Color::rgb(0.35, 0.35, 0.35);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Menu,
    Game,
    End,
}

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WindowDescriptor {
            title: "Mr. Smartyplants".to_string(),
            width: 1280.0,
            height: 720.0,
            ..default()
        })
        .add_state(GameState::Menu)
        .add_startup_system(setup)
        .add_plugin(MenuPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(EndPlugin)
        .add_system(button_color_system)
        .add_plugins(DefaultPlugins);

    if DEV_MODE {
        app.add_system(bevy::window::close_on_esc)
            .add_system(world_inspector_system)
            .add_plugin(LogDiagnosticsPlugin::default())
            .add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_plugin(WorldInspectorPlugin::new())
            .insert_resource(WorldInspectorParams {
                enabled: false,
                ..default()
            });
    }

    app.run();
}

fn setup(mut commands: Commands) {
    // camera
    commands.spawn_bundle(Camera2dBundle::default());
}

type InteractedButtonTuple = (Changed<Interaction>, With<Button>);

/// Handles changing button colors when they're interacted with.
fn button_color_system(
    mut interaction_query: Query<(&Interaction, &mut UiColor), InteractedButtonTuple>,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        *color = match *interaction {
            Interaction::Clicked => PRESSED_BUTTON.into(),
            Interaction::Hovered => HOVERED_BUTTON.into(),
            Interaction::None => NORMAL_BUTTON.into(),
        }
    }
}

/// Handles showing the world inspector.
fn world_inspector_system(
    keyboard: Res<Input<KeyCode>>,
    mut inspector_params: ResMut<WorldInspectorParams>,
) {
    if keyboard.pressed(KeyCode::Equals) {
        inspector_params.enabled = true;
    }
}

/// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_components_system<T: Component>(
    to_despawn: Query<Entity, With<T>>,
    mut commands: Commands,
) {
    despawn_components(to_despawn, &mut commands);
}

fn despawn_components<T: Component>(to_despawn: Query<Entity, With<T>>, commands: &mut Commands) {
    for entity in to_despawn.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

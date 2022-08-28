use crate::*;

pub struct WinPlugin;

impl Plugin for WinPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Win).with_system(win_setup))
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
) {
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
        .insert(WinComponent)
        .with_children(|parent| {
            parent.spawn_bundle(
                TextBundle::from_section(
                    format!(
                        "After {} seasons, you grew a real smart plant:\n\n\n\n\n{}",
                        season.0,
                        smart_plant.0.as_ref().unwrap().name
                    ),
                    TextStyle {
                        font: title_font.clone(),
                        font_size: 50.0,
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
}

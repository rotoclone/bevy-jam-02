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

fn win_setup() {
    todo!() //TODO
}

use crate::*;

pub struct LosePlugin;

impl Plugin for LosePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Lose).with_system(lose_setup))
            .add_system_set(
                SystemSet::on_exit(GameState::Lose)
                    .with_system(despawn_components_system::<LoseComponent>),
            );
    }
}

#[derive(Component)]
struct LoseComponent;

fn lose_setup() {
    todo!() //TODO
}

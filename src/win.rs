use crate::*;

pub struct EndPlugin;

impl Plugin for EndPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::End).with_system(end_setup))
            .add_system_set(
                SystemSet::on_exit(GameState::End)
                    .with_system(despawn_components_system::<EndComponent>),
            );
    }
}

#[derive(Component)]
struct EndComponent;

fn end_setup() {
    todo!() //TODO
}

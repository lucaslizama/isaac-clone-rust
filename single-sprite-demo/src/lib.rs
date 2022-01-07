use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use systems::*;

mod components;
mod resources;
mod systems;

pub fn game() -> AppBuilder {
    let mut builder = App::build();

    builder
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Single Sprite Demo".to_string(),
            width: 1280.0,
            height: 720.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup.system())
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1. / 60.))
                .with_system(move_boi.system()),
        );

    builder
}

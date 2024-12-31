use bevy_app::prelude::*;
use math::Map;
use plugins::{ConnectionPlugin, NetworkPlugin, ServerInfo, ServerPlugin};

mod math;
mod plugins;

fn main() {
    let mut builder = env_logger::Builder::from_default_env();
    builder.target(env_logger::Target::Stdout).init();

    App::new()
        .insert_resource(ServerInfo {
            name: "My epic server".into(),
            ..Default::default()
        })
        .insert_resource(Map::default())
        .add_plugins((ServerPlugin, NetworkPlugin, ConnectionPlugin))
        .run();
}

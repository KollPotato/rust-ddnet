use std::time::Duration;

use bevy_app::{prelude::*, ScheduleRunnerPlugin};
use bevy_ecs::prelude::*;
use protocol::messages::game::TuneParamsChunk;

pub struct ServerPlugin;

#[derive(Resource, Clone)]
pub struct ServerInfo {
    pub name: String,
    pub game_type: String,
    pub password: Option<String>,
    pub tune_params: TuneParamsChunk,
}

impl Default for ServerInfo {
    fn default() -> Self {
        Self {
            name: "Local server".into(),
            game_type: "Custom".into(),
            password: None,
            tune_params: TuneParamsChunk::default(),
        }
    }
}

#[derive(Resource, Clone)]
pub struct ServerSettings {
    tick_rate: u32,
}

impl Default for ServerSettings {
    fn default() -> Self {
        Self { tick_rate: 50 }
    }
}

#[derive(Resource)]
pub struct Server {
    pub current_tick: u32,
}

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        let info = app
            .world_mut()
            .get_resource_or_insert_with(ServerInfo::default)
            .clone();

        app.insert_resource(info);

        let settings = app
            .world_mut()
            .get_resource_or_insert_with(ServerSettings::default)
            .clone();

        app.insert_resource(Server { current_tick: 0 });

        let tick_period = Duration::from_secs_f64(f64::from(settings.tick_rate).recip());

        app.add_plugins(ScheduleRunnerPlugin::run_loop(tick_period));

        fn increment_tick(mut server: ResMut<Server>) {
            server.current_tick += 1;
        }

        app.add_systems(Last, increment_tick);
    }
}

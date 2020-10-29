use bevy::app::{prelude::Plugin, AppBuilder};
use bevy::ecs::IntoQuerySystem;
use bevy::tasks::IoTaskPool;
use bevy::type_registry::RegisterType;

#[derive(Default)]
pub struct InMemoryAssetPlugin;

impl Plugin for InMemoryAssetPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let task_pool = app
            .resources()
            .get::<IoTaskPool>()
            .expect("IoTaskPool resource not found")
            .0
            .clone();

        let in_memory = crate::InMemoryAssetIo::preloaded();
        let asset_server = bevy::asset::AssetServer::new(in_memory, task_pool);

        app.add_stage_before(
            bevy::app::stage::PRE_UPDATE,
            bevy::asset::stage::LOAD_ASSETS,
        )
        .add_stage_after(
            bevy::app::stage::POST_UPDATE,
            bevy::asset::stage::ASSET_EVENTS,
        )
        .add_resource(asset_server)
        .register_property::<bevy::asset::HandleId>()
        .add_system_to_stage(
            bevy::app::stage::PRE_UPDATE,
            bevy::asset::free_unused_assets_system.system(),
        );
    }
}

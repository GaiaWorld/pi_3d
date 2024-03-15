use bevy_ecs::prelude::Resource;
use bevy_app::prelude::App;
use pi_bevy_asset::AssetCapacity;


pub type EnginShell = App;

pub fn asset_capacity<C: AsRef<AssetCapacity> + Resource + Default>(app: &App) -> AssetCapacity {
    if let Some(cfg) = app.world.get_resource::<C>() {
        cfg.as_ref().clone()
    } else {
        C::default().as_ref().clone()
    }
}
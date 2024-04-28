// use bevy_ecs::prelude::Resource;
use pi_world::prelude::App;
use pi_bevy_asset::AssetCapacity;


pub type EnginShell = App;

pub fn asset_capacity<C: AsRef<AssetCapacity> + Default + 'static>(app: &App) -> AssetCapacity {
    if let Some(cfg) = app.world.get_single_res::<C>() {
        cfg.as_ref().clone()
    } else {
        C::default().as_ref().clone()
    }
}
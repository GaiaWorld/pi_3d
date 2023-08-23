
use pi_engine_shell::prelude::*;
pub use load::*;
pub use base::*;

mod load;
mod base;
mod particle_system;


pub struct PluginGLTF2Res;
impl Plugin for PluginGLTF2Res {
    fn build(&self, app: &mut App) {
        let cfg = app.world.get_resource_mut::<AssetMgrConfigs>().unwrap().query::<GLTFBase>();
        app.insert_resource(ShareAssetMgr::<GLTFBase>::new(GarbageEmpty(), cfg.flag, cfg.max, cfg.timeout));
        
        let cfg = app.world.get_resource_mut::<AssetMgrConfigs>().unwrap().query::<GLTF>();
        app.insert_resource(ShareAssetMgr::<GLTF>::new(GarbageEmpty(), cfg.flag, cfg.max, cfg.timeout));

        app.insert_resource(GLTFResLoader::new());
        app.add_systems(
			Update,
            (
                sys_load_gltf_launch,
                sys_gltf_base_loaded_launch,
                sys_gltf_base_loaded_check,
                sys_gltf_analy
            ).chain().in_set(ERunStageChap::Initial)
        );
    }
}


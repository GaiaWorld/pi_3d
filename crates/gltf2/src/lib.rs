
use pi_scene_shell::prelude::*;
pub use load::*;
pub use base::*;
use pi_scene_context::scene::StageScene;

mod load;
mod base;
mod particle_system;


pub struct PluginGLTF2Res;
impl Plugin for PluginGLTF2Res {
    fn build(&self, app: &mut App) {
        let cfg = app.world.get_single_res_mut::<AssetMgrConfigs>().unwrap().query::<GLTFBin>();
        app.world.insert_single_res(ShareAssetMgr::<GLTFBin>::new(GarbageEmpty(), cfg.flag, cfg.max, cfg.timeout));

        let cfg = app.world.get_single_res_mut::<AssetMgrConfigs>().unwrap().query::<GLTFBase>();
        app.world.insert_single_res(ShareAssetMgr::<GLTFBase>::new(GarbageEmpty(), cfg.flag, cfg.max, cfg.timeout));
        
        let cfg = app.world.get_single_res_mut::<AssetMgrConfigs>().unwrap().query::<GLTF>();
        app.world.insert_single_res(ShareAssetMgr::<GLTF>::new(GarbageEmpty(), cfg.flag, cfg.max, cfg.timeout));

        app.world.insert_single_res(GLTFResLoader::new());
        // app.add_system(
		// 	Update,
        //     (
        //         sys_load_gltf_launch,
        //         sys_gltf_base_loaded_launch,
        //         sys_gltf_base_loaded_check,
        //         sys_gltf_analy
        //     ).chain().in_set(StageScene::Create)
        // );
        app.add_system(Update, sys_load_gltf_launch);
        app.add_system(Update, sys_gltf_base_loaded_launch);
        app.add_system(Update, sys_gltf_base_loaded_check);
        app.add_system(Update, sys_gltf_analy);
    }
}



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
        let cfg = app.world.get_resource_mut::<AssetMgrConfigs>().unwrap().query::<GLTFBin>();
        app.insert_resource(ShareAssetMgr::<GLTFBin>::new(GarbageEmpty(), cfg.flag, cfg.max, cfg.timeout));

        let cfg = app.world.get_resource_mut::<AssetMgrConfigs>().unwrap().query::<GLTFBase>();
        app.insert_resource(ShareAssetMgr::<GLTFBase>::new(GarbageEmpty(), cfg.flag, cfg.max, cfg.timeout));
        
        let cfg = app.world.get_resource_mut::<AssetMgrConfigs>().unwrap().query::<GLTF>();
        app.insert_resource(ShareAssetMgr::<GLTF>::new(GarbageEmpty(), cfg.flag, cfg.max, cfg.timeout));

        app.insert_resource(GLTFResLoader::new());

#[cfg(feature = "use_bevy")]
        app.add_systems(
			Update,
            (
                sys_load_gltf_launch,
                sys_gltf_base_loaded_launch,
                sys_gltf_base_loaded_check,
                sys_gltf_analy
            ).chain().in_set(StageScene::Create)
        );

#[cfg(not(feature = "use_bevy"))]
        app
        .add_systems(Update, sys_load_gltf_launch.in_set(StageScene::Create))
        .add_systems(Update, sys_gltf_base_loaded_launch.after(sys_load_gltf_launch).in_set(StageScene::Create))
        .add_systems(Update, sys_gltf_base_loaded_check.after(sys_gltf_base_loaded_launch).in_set(StageScene::Create))
        .add_systems(Update, sys_gltf_analy.after(sys_gltf_base_loaded_check).in_set(StageScene::Create))
        ;

    }
}



use pi_scene_shell::prelude::*;
pub use load::*;
pub use base::*;
use pi_scene_context::scene::StageScene;

mod load;
mod base;
pub mod particle_system;
mod factory;


#[derive(Resource, Default)]
pub struct ResGLTFRecords(pub XHashMap<Entity, Handle<GLTF>>);

pub struct PluginGLTF2Res;
impl Plugin for PluginGLTF2Res {
    fn build(&self, app: &mut App) {
        let cfg = app.world.get_resource_mut::<AssetMgrConfigs>().unwrap().query::<GLTF>();
        app.insert_resource(ShareAssetMgr::<GLTF>::new(GarbageEmpty(), cfg.flag, cfg.max, cfg.timeout));

        app.insert_resource(GLTFResLoader::new());
        app.insert_resource(ResGLTFRecords::default());

        app
        .add_systems(StageD3, sys_gltf_analy.in_set(StageScene::SceneCreate))
        ;

    }
}


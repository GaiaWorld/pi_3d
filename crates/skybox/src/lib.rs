
use pi_scene_shell::prelude::*;
use pi_scene_context::prelude::*;

use shader::SkyboxShader;

pub mod command;
pub mod shader;
pub mod texture;

// pub struct SingleSkyboxMaterial(pub MaterialID);


// pub struct InterfaceSkybox;
// impl InterfaceSkybox {
//     pub fn new_skybox(
//         app: &mut App,
//         scene: Entity
//     ) -> Entity {
//         let entity = ActionCube::new_cube(app, scene, String::from("Sky"));

//         let mut queue = CommandQueue::default();
//         let mut commands = Commands::new(&mut queue, &app.world);

//         let mat = commands.spawn_empty().id();
//         queue.apply(&mut app.world);

//         ActionMaterial::init(app, mat, KeyShaderMeta::from(SkyboxShader::KEY), EPassTag::Sky);

//         ActionMaterial::use_material(app, OpsMaterialUse::ops(entity, mat));

//         entity
//     }
// }

// impl InterfaceSkybox for EnginShell {
//     fn new_skybox(&self, scene: ObjectID) -> ObjectID {
//         let material = self.world().get_resource::<SingleSkyboxMaterial>().unwrap();

//         let entity = self.new_cube(scene);
//         self.use_material(entity, material.0.0.clone());

//         entity
//     }
// }

fn setup(
    asset_mgr: Res<ShareAssetMgr<ShaderEffectMeta>>,
) {
    ActionMaterial::regist_material_meta(&asset_mgr, KeyShaderMeta::from(SkyboxShader::KEY), SkyboxShader::meta());
}


pub struct PluginSkybox;
impl Plugin for PluginSkybox {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, setup);
    }
    // fn init(
    //     &mut self,
    //     engine: &mut EnginShell,
    //     _: &mut RunStage,
    // ) -> Result<(), ErrorPlugin> {
    //     engine.regist_material_meta(KeyShaderMeta::from(SkyboxShader::KEY), SkyboxShader::meta());
        
    //     let material = engine.new_object();
    //     engine.as_material(material, KeyShaderMeta::from(SkyboxShader::KEY), EPassTag::Sky);
    //     engine.world_mut().insert_resource(SingleSkyboxMaterial(MaterialID(material)));

    //     Ok(())
    // }
}

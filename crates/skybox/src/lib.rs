use pi_atom::Atom;
use pi_engine_shell::{plugin::{Plugin, ErrorPlugin}, engine_shell::EnginShell, run_stage::RunStage, object::{ObjectID, InterfaceObject}};
use pi_mesh_builder::cube::InterfaceCube;
use pi_render::renderer::shader::KeyShaderMeta;
use pi_scene_context::{materials::{interface::{InterfaceMaterialMeta, InterfaceMaterial}, material::{MaterialID}}, pass::EPassTag};
use shader::SkyboxShader;

pub mod command;
pub mod shader;
pub mod texture;

pub struct SingleSkyboxMaterial(pub MaterialID);


pub trait InterfaceSkybox {
    fn new_skybox(&self, scene: ObjectID) -> ObjectID;
}

impl InterfaceSkybox for EnginShell {
    fn new_skybox(&self, scene: ObjectID) -> ObjectID {
        let material = self.world().get_resource::<SingleSkyboxMaterial>().unwrap();

        let entity = self.new_cube(scene);
        self.use_material(entity, material.0.0.clone());

        entity
    }
}

pub struct PluginSkybox;
impl Plugin for PluginSkybox {
    fn init(
        &mut self,
        engine: &mut EnginShell,
        _: &mut RunStage,
    ) -> Result<(), ErrorPlugin> {
        engine.regist_material_meta(KeyShaderMeta::from(SkyboxShader::KEY), SkyboxShader::meta());
        
        let material = engine.new_object();
        engine.as_material(material, KeyShaderMeta::from(SkyboxShader::KEY), EPassTag::Sky);
        engine.world_mut().insert_resource(SingleSkyboxMaterial(MaterialID(material)));

        Ok(())
    }
}

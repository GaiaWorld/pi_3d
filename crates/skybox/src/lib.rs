use pi_atom::Atom;
use pi_engine_shell::{plugin::{Plugin, ErrorPlugin}, engine_shell::EnginShell, run_stage::RunStage, object::{ObjectID, InterfaceObject}};
use pi_scene_context::{materials::{material_meta::InterfaceMaterialMeta, material::{InterfaceMaterial, MaterialID}}, meshes::cube::InterfaceCube};
use render_shader::shader::KeyShaderEffect;
use shader::SkyboxShader;

pub mod command;
pub mod material;
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
        self.use_material(entity, material.0.clone());

        entity
    }
}

pub struct PluginSkybox;
impl Plugin for PluginSkybox {
        let position_id = engine.new_object();
        Ok(())
    }
}

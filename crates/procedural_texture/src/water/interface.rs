use pi_atom::Atom;
use pi_scene_context::{
    engine::Engine,
    materials::{
        material::{MaterialID, MaterialIDCommand, SingleMaterialIDCommandList}, bind_group::RenderBindGroupPool, shader_effect::InterfaceMaterialMeta,
    },
    object::ObjectID,
    plugin::{ErrorPlugin, Plugin},
};
use pi_ecs::prelude::Setup;
use pi_engine_shell::object::InterfaceObject;
use pi_render::rhi::device::RenderDevice;
use render_shader::shader::KeyShaderEffect;

use crate::water::shader::WaterShader;




pub trait InterfaceWaterMaterial {
    fn create_water_material(&self) -> ObjectID;
}

impl InterfaceWaterMaterial for pi_engine_shell::engine_shell::EnginShell {
    fn create_water_material(&self) -> ObjectID {
        println!("create_default_material");
        let entity = self.new_object();
        self.as_material(entity, KeyShaderEffect(Atom::from(WaterShader::KEY)));

        entity
    }
    
}

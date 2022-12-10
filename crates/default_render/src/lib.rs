
use command::{SingeDefaultMaterialCommandList, SysDefaultMaterialCommand};
use interface::InterfaceDefaultMaterial;
use pi_atom::Atom;
use pi_ecs::{prelude::{Setup}};

use pi_scene_context::{plugin::{ErrorPlugin}, engine::{self, Engine}, materials::{material::{MaterialID}, material_meta::InterfaceMaterialMeta}};
use render_shader::shader::KeyPreShader;
use shader::DefaultShader;

pub mod shader;
pub mod command;
pub mod interface;

#[derive(Debug, Clone, Copy)]
pub struct SingleIDBaseDefaultMaterial(pub MaterialID);

pub struct PluginDefaultMaterial;
impl pi_engine_shell::plugin::Plugin for PluginDefaultMaterial {
    fn init(
        &mut self,
        engine: &mut Engine,
        stages: &mut pi_engine_shell::run_stage::RunStage,
    ) -> Result<(), ErrorPlugin> {

        let world = engine.world_mut();
        SysDefaultMaterialCommand::setup(world, stages.command_stage());

        let world = engine.world_mut();
        world.insert_resource(SingeDefaultMaterialCommandList::default());

        let base_default_id = engine.create_default_material();
        let world = engine.world_mut();
        world.insert_resource(SingleIDBaseDefaultMaterial(MaterialID(base_default_id)));

        let key = KeyPreShader(Atom::from(DefaultShader::KEY));
        engine.regist_material_meta(key, DefaultShader::res());

        Ok(())
    }
}


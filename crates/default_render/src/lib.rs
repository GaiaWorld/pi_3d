use command::SysDefaultMaterialCommand;
use pi_ecs::{prelude::{Setup}};
use pi_render::rhi::device::RenderDevice;

use pi_scene_context::{plugin::{ErrorPlugin}, engine::{self, Engine}, materials::{material::{MaterialID}, bind_group::{RenderBindGroupPool, RenderBindGroupKey}}};

use self::{default_material::{SingleDefaultMaterialBindDynInfoSet}, shader::DefaultShader, default_material_sys::{DefaultMaterialUniformUpdate, DefaultMaterialFilter, DefaultModelUniformUpdate, SysDefaultMaterialPipelineKey}, command::{SingeDefaultMaterialCommandList}, pipeline::DefaultMaterialPipeline, bind_group::{IDDefaultMaterialBindGroup, SysDefaultMaterialBindGroupUpdate}, interface::InterfaceDefaultMaterial};

pub mod default_material;
pub mod default_material_sys;
pub mod shader;
pub mod command;
pub mod pipeline;
pub mod bind_group;
pub mod interface;
pub struct SingleIDBaseDefaultMaterial(pub MaterialID);

pub struct PluginDefaultMaterial;
impl pi_engine_shell::plugin::Plugin for PluginDefaultMaterial {
    fn init(
        &mut self,
        engine: &mut Engine,
        stages: &mut pi_engine_shell::run_stage::RunStage,
    ) -> Result<(), ErrorPlugin> {

        //  println!("PluginDefaultMaterial");

        let world = engine.world_mut();

        let device = world.get_resource::<RenderDevice>().unwrap().clone();
        let id_default_mat_bind_group = RenderBindGroupKey::from(IDDefaultMaterialBindGroup::LABEL);
        world.get_resource_mut::<RenderBindGroupPool>().unwrap().creat(&device, id_default_mat_bind_group.clone(), IDDefaultMaterialBindGroup::layout_entries().as_slice(), IDDefaultMaterialBindGroup::SET);

        let world = engine.world_mut();

        SysDefaultMaterialCommand::setup(world, stages.command_stage());
        DefaultModelUniformUpdate::setup(world, stages.uniform_update());
        DefaultMaterialUniformUpdate::setup(world, stages.uniform_update());
        DefaultMaterialFilter::setup(world, stages.filter_culling());
        SysDefaultMaterialBindGroupUpdate::setup(world, stages.between_uniform_update_and_filter_culling());
        SysDefaultMaterialPipelineKey::setup(world, stages.uniform_update());

        world.insert_resource(DefaultMaterialPipeline::default());
        world.insert_resource(DefaultShader::new(&device));

        world.insert_resource(SingeDefaultMaterialCommandList::default());
        world.insert_resource(SingleDefaultMaterialBindDynInfoSet::default());

        world.insert_resource(IDDefaultMaterialBindGroup(id_default_mat_bind_group));

        let base_default_id = engine.create_default_material();
        let world = engine.world_mut();
        world.insert_resource(SingleIDBaseDefaultMaterial(MaterialID(base_default_id)));

        Ok(())
    }
}


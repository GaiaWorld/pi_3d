use command::SysDefaultMaterialCommand;
use pi_ecs::{prelude::{Setup}};
use pi_engine_shell::object::InterfaceObject;
use pi_render::rhi::device::RenderDevice;

use pi_scene_context::{plugin::{ErrorPlugin}, engine::{self, Engine}, materials::{material::{MaterialID}, command::{InterfaceRenderBindGroup}}};

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
        let id_default_mat_bind_group = engine.new_object();

        let world = engine.world_mut();

        let device = world.get_resource::<RenderDevice>().unwrap().clone();
        let layout = IDDefaultMaterialBindGroup::layout(&device);
        engine.as_render_bind_group(id_default_mat_bind_group, layout, IDDefaultMaterialBindGroup::SET);

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


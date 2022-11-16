use pi_ecs::{world::World, prelude::{StageBuilder, Setup, ArchetypeId}};
use pi_render::rhi::device::RenderDevice;

use crate::{object::ObjectID, plugin::{ErrorPlugin}, engine::{self, Engine}, materials::{material::{MaterialID}, command::{SingleRenderBindGroupCommandList, RenderBindGroupCommand}}, default_render::command::SysDefaultMaterialCommand};

use self::{default_material::{SingleDefaultMaterialBindDynInfoSet}, shader::DefaultShader, default_material_sys::{DefaultMaterialUniformUpdate, DefaultMaterialFilter, DefaultModelUniformUpdate, SysDefaultMaterialPipelineKey}, command::{SingeDefaultMaterialCommandList, DefaultMaterialCommand}, pipeline::DefaultMaterialPipeline, bind_group::{IDDefaultMaterialBindGroup, SysDefaultMaterialBindGroupUpdate}, dirty::SysDirtyDefaultMaterialPropertype, interface::InterfaceDefaultMaterial};

pub mod default_material;
pub mod default_material_sys;
pub mod shader;
pub mod command;
pub mod pipeline;
pub mod bind_group;
pub mod dirty;
pub mod interface;
pub struct SingleIDBaseDefaultMaterial(pub MaterialID);

pub struct PluginDefaultMaterial;
impl crate::Plugin for PluginDefaultMaterial {
    fn init(
        &mut self,
        world: &mut pi_ecs::world::World,
        engine: &mut Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), ErrorPlugin> {
        //  println!("PluginDefaultMaterial");
        let id_default_mat_bind_group = engine.new_object();

        SysDirtyDefaultMaterialPropertype::setup(world, stages.dirty_state_stage());
        SysDefaultMaterialCommand::setup(world, stages.command_stage());
        DefaultModelUniformUpdate::setup(world, stages.uniform_update());
        DefaultMaterialUniformUpdate::setup(world, stages.uniform_update());
        DefaultMaterialFilter::setup(world, stages.filter_culling());
        SysDefaultMaterialBindGroupUpdate::setup(world, stages.between_uniform_update_and_filter_culling());
        SysDefaultMaterialPipelineKey::setup(world, stages.uniform_update());

        let device = world.get_resource::<RenderDevice>().unwrap().clone();
        world.insert_resource(DefaultMaterialPipeline::default());
        world.insert_resource(DefaultShader::new(&device));

        world.insert_resource(SingeDefaultMaterialCommandList::default());
        world.insert_resource(SingleDefaultMaterialBindDynInfoSet::default());

        let layout = IDDefaultMaterialBindGroup::layout(&device);
        let commands = world.get_resource_mut::<SingleRenderBindGroupCommandList>().unwrap();
        commands.list.push(RenderBindGroupCommand::Create(id_default_mat_bind_group, layout, IDDefaultMaterialBindGroup::SET));
        world.insert_resource(IDDefaultMaterialBindGroup(id_default_mat_bind_group));

        let base_default_id = engine.create_default_material().clone();
        world.insert_resource(SingleIDBaseDefaultMaterial(MaterialID(base_default_id)));

        Ok(())
    }
}


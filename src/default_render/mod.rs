use pi_ecs::{world::World, prelude::{StageBuilder, Setup, ArchetypeId}};
use pi_render::rhi::device::RenderDevice;

use crate::{object::ObjectID, plugin::{Plugin, ErrorPlugin}, engine::{self, Engine}, materials::{material::{MaterialID}, command::{SingleRenderBindGroupCommandList, RenderBindGroupCommand}}, default_render::command::SysDefaultMaterialCommand};

use self::{default_material::{SingleDefaultMaterialBindDynInfoSet}, shader::DefaultShader, default_material_sys::{DefaultMaterialUniformUpdate, DefaultMaterialFilter, DefaultModelUniformUpdate}, command::{SingeDefaultMaterialCommandList, DefaultMaterialCommand}, pipeline::DefaultMaterialPipeline, bind_group::{IDDefaultMaterialBindGroup, SysDefaultMaterialBindGroupUpdate}, dirty::SysDirtyDefaultMaterialPropertype, interface::InterfaceDefaultMaterial};

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
impl Plugin for PluginDefaultMaterial {
    fn init(
        engine: &mut Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), ErrorPlugin> {
        //  println!("PluginDefaultMaterial");
        let id_default_mat_bind_group = engine.new_object();
        let mut world = engine.world_mut().clone();

        SysDirtyDefaultMaterialPropertype::setup(&mut world, stages.dirty_state_stage());
        SysDefaultMaterialCommand::setup(&mut world, stages.command_stage());
        DefaultModelUniformUpdate::setup(&mut world, stages.uniform_update());
        DefaultMaterialUniformUpdate::setup(&mut world, stages.uniform_update());
        DefaultMaterialFilter::setup(&mut world, stages.filter_culling());
        SysDefaultMaterialBindGroupUpdate::setup(&mut world, stages.between_uniform_update_and_filter_culling());

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


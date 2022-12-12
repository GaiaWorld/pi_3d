use pi_scene_context::{
    engine::Engine,
    materials::{
        material::{MaterialID, MaterialIDCommand, SingleMaterialIDCommandList}, bind_group::RenderBindGroupPool,
    },
    object::ObjectID,
    plugin::{ErrorPlugin, Plugin},
};
use pi_ecs::prelude::Setup;
use pi_engine_shell::object::InterfaceObject;
use pi_render::rhi::device::RenderDevice;

use super::{
    bind_group::{IDWaterMaterialBindGroup, SysWaterMaterialBindGroupUpdate},
    command::{SysWaterMaterialCommand, SingeWaterMaterialCommandList, WaterMaterialCommand},
    material::SingleWaterMaterialBindDynInfoSet,
    material_sys::{WaterMaterialFilter, WaterMaterialUniformUpdate},
    pipeline::WaterMaterialPipeline,
    shader::WaterShader,
};

pub struct SingleIDBaseWaterMaterial(pub MaterialID);

pub struct PluginWaterMaterial;
impl Plugin for PluginWaterMaterial {
    fn init(
        &mut self,
        engine: &mut Engine,
        stages: &mut pi_engine_shell::run_stage::RunStage,
    ) -> Result<(), ErrorPlugin> {
        println!("PluginWaterMaterial");
        let id_default_mat_bind_group = engine.new_object();

        let world = engine.world_mut();

        SysWaterMaterialCommand::setup(world, stages.command_stage());
        WaterMaterialUniformUpdate::setup(world, stages.uniform_update());
        WaterMaterialFilter::setup(world, stages.filter_culling());
        SysWaterMaterialBindGroupUpdate::setup(
            world,
            stages.between_uniform_update_and_filter_culling(),
        );

        let device = world.get_resource::<RenderDevice>().unwrap().clone();
        world.insert_resource(WaterMaterialPipeline::default());
        world.insert_resource(WaterShader::new(&device));

        world.insert_resource(SingeWaterMaterialCommandList::default());
        world.insert_resource(SingleWaterMaterialBindDynInfoSet::default());

        let layout = IDWaterMaterialBindGroup::layout(&device);
        let id_default_mat_bind_group = world.get_resource_mut::<RenderBindGroupPool>().unwrap().creat(&device, layout, IDWaterMaterialBindGroup::SET);
        world.insert_resource(IDWaterMaterialBindGroup(id_default_mat_bind_group));

        let base_default_id = engine.create_water_material().clone();
        let world = engine.world_mut();
        world.insert_resource(SingleIDBaseWaterMaterial(MaterialID(base_default_id)));

        Ok(())
    }
}

pub trait InterfaceWaterMaterial {
    fn create_water_material(&self) -> ObjectID;

    fn as_water_material(&self, object: ObjectID) -> & Self;

    fn use_water_material(&self, object: ObjectID) -> & Self;
}

impl InterfaceWaterMaterial for pi_engine_shell::engine_shell::EnginShell {
    fn create_water_material(&self) -> ObjectID {
        println!("create_default_material");
        let entity = self.new_object();

        self.as_water_material(entity);

        entity
    }
    fn as_water_material(&self, object: ObjectID) -> &Self {
        let world = self.world();

        let commands = world
            .get_resource_mut::<SingeWaterMaterialCommandList>()
            .unwrap();
        commands.list.push(WaterMaterialCommand::Create(object));

        self
    }

    fn use_water_material(&self, object: ObjectID) -> &Self {
        let world = self.world();

        let base_material = world.get_resource::<SingleIDBaseWaterMaterial>().unwrap();
        let commands = world
            .get_resource_mut::<SingleMaterialIDCommandList>()
            .unwrap();
        commands
            .list
            .push(MaterialIDCommand::Use(object, base_material.0));

        self
    }
}

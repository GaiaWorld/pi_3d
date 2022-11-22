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
    bind_group::{IDCloudMaterialBindGroup, SysCloudMaterialBindGroupUpdate},
    command::{SysCloudMaterialCommand, SingeCloudMaterialCommandList, CloudMaterialCommand},
    material::SingleCloudMaterialBindDynInfoSet,
    material_sys::{CloudMaterialFilter, CloudMaterialUniformUpdate},
    pipeline::CloudMaterialPipeline,
    shader::CloudShader,
};

pub struct SingleIDBaseCloudMaterial(pub MaterialID);

pub struct PluginCloudMaterial;
impl Plugin for PluginCloudMaterial {
    fn init(
        &mut self,
        engine: &mut Engine,
        stages: &mut pi_engine_shell::run_stage::RunStage,
    ) -> Result<(), ErrorPlugin> {
        println!("PluginCloudMaterial");
        let id_default_mat_bind_group = engine.new_object();

        let world = engine.world_mut();

        SysCloudMaterialCommand::setup(world, stages.command_stage());
        CloudMaterialUniformUpdate::setup(world, stages.uniform_update());
        CloudMaterialFilter::setup(world, stages.filter_culling());
        SysCloudMaterialBindGroupUpdate::setup(
            world,
            stages.between_uniform_update_and_filter_culling(),
        );

        let device = world.get_resource::<RenderDevice>().unwrap().clone();
        world.insert_resource(CloudMaterialPipeline::default());
        world.insert_resource(CloudShader::new(&device));

        world.insert_resource(SingeCloudMaterialCommandList::default());
        world.insert_resource(SingleCloudMaterialBindDynInfoSet::default());

        let layout = IDCloudMaterialBindGroup::layout(&device);
        let id_default_mat_bind_group = world.get_resource_mut::<RenderBindGroupPool>().unwrap().creat(&device, layout, IDCloudMaterialBindGroup::SET);
        world.insert_resource(IDCloudMaterialBindGroup(id_default_mat_bind_group));

        let base_default_id = engine.create_cloud_material().clone();
        let world = engine.world_mut();
        world.insert_resource(SingleIDBaseCloudMaterial(MaterialID(base_default_id)));

        Ok(())
    }
}

pub trait InterfaceCloudMaterial {
    fn create_cloud_material(&self) -> ObjectID;

    fn as_cloud_material(&self, object: ObjectID) -> & Self;

    fn use_cloud_material(&self, object: ObjectID) -> & Self;
}

impl InterfaceCloudMaterial for pi_engine_shell::engine_shell::EnginShell {
    fn create_cloud_material(&self) -> ObjectID {
        println!("create_default_material");
        let entity = self.new_object();

        self.as_cloud_material(entity);

        entity
    }
    fn as_cloud_material(&self, object: ObjectID) -> &Self {
        let world = self.world();

        let commands = world
            .get_resource_mut::<SingeCloudMaterialCommandList>()
            .unwrap();
        commands.list.push(CloudMaterialCommand::Create(object));

        self
    }

    fn use_cloud_material(&self, object: ObjectID) -> &Self {
        let world = self.world();

        let base_material = world.get_resource::<SingleIDBaseCloudMaterial>().unwrap();
        let commands = world
            .get_resource_mut::<SingleMaterialIDCommandList>()
            .unwrap();
        commands
            .list
            .push(MaterialIDCommand::Use(object, base_material.0));

        self
    }
}

use crate::{
    engine::Engine,
    materials::{
        command::{RenderBindGroupCommand, SingleRenderBindGroupCommandList},
        material::{MaterialID, MaterialIDCommand, SingleMaterialIDCommandList},
    },
    object::ObjectID,
    plugin::{ErrorPlugin, Plugin},
    skybox::{
        bind_group::{IDSkyboxMaterialBindGroup, SysSkyboxMaterialBindGroupUpdate},
        command::SysSkyboxMaterialCommand,
        material::SingleSkyboxMaterialBindDynInfoSet,
        material_sys::{SkyboxMaterialFilter, SkyboxMaterialUniformUpdate},
        pipeline::SkyboxMaterialPipeline,
        shader::SkyboxShader,
    },
};
use pi_ecs::prelude::Setup;
use pi_render::rhi::device::RenderDevice;

use super::command::{SingeSkyboxMaterialCommandList, SkyboxMaterialCommand};

pub struct SingleIDBaseSkyboxMaterial(pub MaterialID);

pub struct PluginSkyboxMaterial;
impl Plugin for PluginSkyboxMaterial {
    fn init(
        &mut self,
        world: &mut pi_ecs::world::World,
        engine: &mut Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), ErrorPlugin> {
        println!("PluginSkyboxMaterial");
        let id_default_mat_bind_group = engine.new_object();

        SysSkyboxMaterialCommand::setup(world, stages.command_stage());
        SkyboxMaterialUniformUpdate::setup(world, stages.uniform_update());
        SkyboxMaterialFilter::setup(world, stages.filter_culling());
        SysSkyboxMaterialBindGroupUpdate::setup(
            world,
            stages.between_uniform_update_and_filter_culling(),
        );

        let device = world.get_resource::<RenderDevice>().unwrap().clone();
        world.insert_resource(SkyboxMaterialPipeline::default());
        world.insert_resource(SkyboxShader::new(&device));

        world.insert_resource(SingeSkyboxMaterialCommandList::default());
        world.insert_resource(SingleSkyboxMaterialBindDynInfoSet::default());

        let layout = IDSkyboxMaterialBindGroup::layout(&device);
        let commands = world
            .get_resource_mut::<SingleRenderBindGroupCommandList>()
            .unwrap();
        commands.list.push(RenderBindGroupCommand::Create(
            id_default_mat_bind_group,
            layout,
            IDSkyboxMaterialBindGroup::SET,
        ));
        world.insert_resource(IDSkyboxMaterialBindGroup(id_default_mat_bind_group));

        let base_default_id = engine.create_skybox_material().clone();
        world.insert_resource(SingleIDBaseSkyboxMaterial(MaterialID(base_default_id)));

        Ok(())
    }
}

pub trait InterfaceSkyboxMaterial {
    fn create_skybox_material(&self) -> ObjectID;

    fn as_skybox_material(&self, object: ObjectID) -> & Self;

    fn use_skybox_material(&self, object: ObjectID) -> & Self;
}

impl InterfaceSkyboxMaterial for crate::engine::Engine {
    fn create_skybox_material(&self) -> ObjectID {
        println!("create_default_material");
        let entity = self.new_object();

        self.as_skybox_material(entity);

        entity
    }
    fn as_skybox_material(&self, object: ObjectID) -> &Self {
        let world = self.world();

        let commands = world
            .get_resource_mut::<SingeSkyboxMaterialCommandList>()
            .unwrap();
        commands.list.push(SkyboxMaterialCommand::Create(object));

        self
    }

    fn use_skybox_material(&self, object: ObjectID) -> &Self {
        let world = self.world();

        let base_material = world.get_resource::<SingleIDBaseSkyboxMaterial>().unwrap();
        let commands = world
            .get_resource_mut::<SingleMaterialIDCommandList>()
            .unwrap();
        commands
            .list
            .push(MaterialIDCommand::Use(object, base_material.0));

        self
    }
}

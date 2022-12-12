use pi_ecs::prelude::Setup;
use pi_engine_shell::object::InterfaceObject;
use pi_render::rhi::device::RenderDevice;
use pi_scene_context::{
    engine::Engine,
    materials::{
        bind_group::RenderBindGroupPool,
        material::{MaterialID, MaterialIDCommand, SingleMaterialIDCommandList},
    },
    object::ObjectID,
    plugin::{ErrorPlugin, Plugin},
};

use crate::{
    bind_group::{IDSkeletonsBindGroup, SkeletonsBindGroupUpdate},
    command::{SingeSkeletonsCommandList, SkeletonsCommand, SysSkeletonsCommand},
    material::SingleSkeletonsBindDynInfoSet,
    material_sys::{SkeletonsFilter, SkeletonsUniformUpdate},
    pipeline::SkeletonsPipeline,
    shader::SkeletonsShader,
};

pub struct SingleIDBaseSkeletons(pub MaterialID);

pub struct PluginSkeletonsPropertype;
impl Plugin for PluginSkeletonsPropertype {
    fn init(
        &mut self,
        engine: &mut Engine,
        stages: &mut pi_engine_shell::run_stage::RunStage,
    ) -> Result<(), ErrorPlugin> {
        println!("PluginSkyboxMaterial");
        let world = engine.world_mut();

        SysSkeletonsCommand::setup(world, stages.command_stage());
        SkeletonsUniformUpdate::setup(world, stages.uniform_update());
        SkeletonsFilter::setup(world, stages.filter_culling());
        SkeletonsBindGroupUpdate::setup(world, stages.between_uniform_update_and_filter_culling());

        let device = world.get_resource::<RenderDevice>().unwrap().clone();
        world.insert_resource(SkeletonsPipeline::default());
        world.insert_resource(SkeletonsShader::new(&device));

        world.insert_resource(SingeSkeletonsCommandList::default());
        world.insert_resource(SingleSkeletonsBindDynInfoSet::default());

        let layout = IDSkeletonsBindGroup::layout(&device);
        let id_default_mat_bind_group = world
            .get_resource_mut::<RenderBindGroupPool>()
            .unwrap()
            .creat(&device, layout, IDSkeletonsBindGroup::SET);

        world.insert_resource(IDSkeletonsBindGroup(id_default_mat_bind_group));

        let base_default_id = engine.create_skeletons().clone();
        let world = engine.world_mut();
        world.insert_resource(SingleIDBaseSkeletons(MaterialID(base_default_id)));
        Ok(())
    }
}

pub trait InterfaceSkeletonsPropertype {
    fn create_skeletons(&self) -> ObjectID;
    fn use_skeletons(&self, object: ObjectID) -> &Self;
}

impl InterfaceSkeletonsPropertype for pi_engine_shell::engine_shell::EnginShell {
    fn create_skeletons(&self) -> ObjectID {
        let object = self.new_object();

        let world = self.world();

        let commands = world
            .get_resource_mut::<SingeSkeletonsCommandList>()
            .unwrap();
        commands.list.push(SkeletonsCommand::Create(object));
        
        object
    }

    fn use_skeletons(&self, object: ObjectID) -> &Self {
        let world = self.world();

        let skeletons = world.get_resource::<SingleIDBaseSkeletons>().unwrap();
        let commands = world
            .get_resource_mut::<SingleMaterialIDCommandList>()
            .unwrap();
        commands
            .list
            .push(MaterialIDCommand::Use(object, skeletons.0));

        self
    }
}

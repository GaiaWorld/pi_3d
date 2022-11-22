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
    bind_group::{IDPerlinNoiseMaterialBindGroup, SysPerlinNoiseMaterialBindGroupUpdate},
    command::{SysPerlinNoiseMaterialCommand, SingePerlinNoiseMaterialCommandList, PerlinNoiseMaterialCommand},
    material::SinglePerlinNoiseMaterialBindDynInfoSet,
    material_sys::{PerlinNoiseMaterialFilter, PerlinNoiseMaterialUniformUpdate},
    pipeline::PerlinNoiseMaterialPipeline,
    shader::PerlinNoiseShader,
};

pub struct SingleIDBasePerlinNoiseMaterial(pub MaterialID);

pub struct PluginPerlinNoiseMaterial;
impl Plugin for PluginPerlinNoiseMaterial {
    fn init(
        &mut self,
        engine: &mut Engine,
        stages: &mut pi_engine_shell::run_stage::RunStage,
    ) -> Result<(), ErrorPlugin> {
        println!("PluginPerlinNoiseMaterial");
        let id_default_mat_bind_group = engine.new_object();

        let world = engine.world_mut();

        SysPerlinNoiseMaterialCommand::setup(world, stages.command_stage());
        PerlinNoiseMaterialUniformUpdate::setup(world, stages.uniform_update());
        PerlinNoiseMaterialFilter::setup(world, stages.filter_culling());
        SysPerlinNoiseMaterialBindGroupUpdate::setup(
            world,
            stages.between_uniform_update_and_filter_culling(),
        );

        let device = world.get_resource::<RenderDevice>().unwrap().clone();
        world.insert_resource(PerlinNoiseMaterialPipeline::default());
        world.insert_resource(PerlinNoiseShader::new(&device));

        world.insert_resource(SingePerlinNoiseMaterialCommandList::default());
        world.insert_resource(SinglePerlinNoiseMaterialBindDynInfoSet::default());

        let layout = IDPerlinNoiseMaterialBindGroup::layout(&device);
        let id_default_mat_bind_group = world.get_resource_mut::<RenderBindGroupPool>().unwrap().creat(&device, layout, IDPerlinNoiseMaterialBindGroup::SET);
        world.insert_resource(IDPerlinNoiseMaterialBindGroup(id_default_mat_bind_group));

        let base_default_id = engine.create_perlin_noise_material().clone();
        let world = engine.world_mut();
        world.insert_resource(SingleIDBasePerlinNoiseMaterial(MaterialID(base_default_id)));

        Ok(())
    }
}

pub trait InterfacePerlinNoiseMaterial {
    fn create_perlin_noise_material(&self) -> ObjectID;

    fn as_perlin_noise_material(&self, object: ObjectID) -> & Self;

    fn use_perlin_noise_material(&self, object: ObjectID) -> & Self;
}

impl InterfacePerlinNoiseMaterial for pi_engine_shell::engine_shell::EnginShell {
    fn create_perlin_noise_material(&self) -> ObjectID {
        println!("create_default_material");
        let entity = self.new_object();

        self.as_perlin_noise_material(entity);

        entity
    }
    fn as_perlin_noise_material(&self, object: ObjectID) -> &Self {
        let world = self.world();

        let commands = world
            .get_resource_mut::<SingePerlinNoiseMaterialCommandList>()
            .unwrap();
        commands.list.push(PerlinNoiseMaterialCommand::Create(object));

        self
    }

    fn use_perlin_noise_material(&self, object: ObjectID) -> &Self {
        let world = self.world();

        let base_material = world.get_resource::<SingleIDBasePerlinNoiseMaterial>().unwrap();
        let commands = world
            .get_resource_mut::<SingleMaterialIDCommandList>()
            .unwrap();
        commands
            .list
            .push(MaterialIDCommand::Use(object, base_material.0));

        self
    }
}

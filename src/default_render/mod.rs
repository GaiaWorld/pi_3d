use pi_ecs::{world::World, prelude::{StageBuilder, Setup}};
use pi_render::rhi::device::RenderDevice;

use self::{default_material::DefaultMaterialPipeline, default::DefaultShader, default_material_sys::{DefaultMaterialUniformTickUpdate, DefaultMaterialTickBeforeRender}};

pub mod default_material;
pub mod default_material_sys;
pub mod default;

pub fn world_init_default_materail(
    world: &mut World,
    stage_uniform_update: &mut StageBuilder,
    stage_before_render: &mut StageBuilder,
) {
    let device = world.get_resource_mut::<RenderDevice>().unwrap().clone();
    world.insert_resource(DefaultMaterialPipeline::default());
    world.insert_resource(DefaultShader::new(&device));

    DefaultMaterialUniformTickUpdate::setup(world, stage_uniform_update);
    DefaultMaterialTickBeforeRender::setup(world, stage_before_render);
}
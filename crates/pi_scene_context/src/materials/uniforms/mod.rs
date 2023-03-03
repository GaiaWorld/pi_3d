use pi_assets::{mgr::AssetMgr, asset::GarbageEmpty};
use pi_atom::Atom;
use pi_ecs::prelude::Setup;
use pi_engine_shell::{plugin::Plugin, run_stage::ERunStageChap, assets::image_texture_load::ImageAwait};
use pi_render::{rhi::{asset::TextureRes, device::RenderDevice, RenderQueue}, renderer::{sampler::SamplerRes, buildin_data::{EDefaultTexture, DefaultTexture}}};
use pi_scene_math::Number;
use pi_share::Share;

use crate::pass::*;

use self::{
    sys_texture::{SysTextureSlot01Load, SysTextureLoad, SysTextureSlot02Load, SysTextureSlot03Load, SysTextureSlot04Load, SysTextureResReady1, SysTextureResReady2},
    sys_uniform::{
        SysMaterialMetaChange, 
        SysBindValueUpdate, SysMaterialTexturesChange, SingleUniformCommands, SysUniformComand
    },
    texture::{TextureSlot01, TextureSlot02, TextureSlot03, TextureSlot04},
    sys_pass::{SysEffectValueToModelByMaterialModify, SysEffectTexturesToModelByMaterialModify}
};

pub mod value_uniform;
pub mod texture_uniform;
// pub mod sys_mat4;
// pub mod sys_mat2;
// pub mod sys_float;
// pub mod sys_int;
// pub mod sys_uint;
// pub mod sys_vec2;
// pub mod sys_vec4;
pub mod uniform;
pub mod float;
pub mod vec2;
pub mod vec4;
pub mod mat2;
pub mod mat4;
pub mod int;
pub mod uint;
pub mod boolean;
pub mod byte;
pub mod texture;
pub mod sys_texture;
pub mod sys_uniform;
pub mod sys_pass;

pub(crate) fn update_data(
    data: &mut [Number],
    slot: usize,
    value: &[Number],
    num_count: usize,
) {
    if value.len() >= num_count {
        for i in 0..num_count {
            data[slot * num_count + i] = value[i];
        }
    }
}

pub struct PluginMaterialUniforms;
impl Plugin for PluginMaterialUniforms {
    fn init(
        &mut self,
        engine: &mut pi_engine_shell::engine_shell::EnginShell,
        stages: &mut pi_engine_shell::run_stage::RunStage,
    ) -> Result<(), pi_engine_shell::plugin::ErrorPlugin> {
        let world = engine.world_mut();

        world.insert_resource(SingleUniformCommands::default());

        // 材质设置
        SysMaterialMetaChange::<PassID01>::setup(world, stages.query_stage::<SysMaterialMetaChange::<PassID01>>(ERunStageChap::Command));
        SysMaterialMetaChange::<PassID02>::setup(world, stages.query_stage::<SysMaterialMetaChange::<PassID02>>(ERunStageChap::Command));
        SysMaterialMetaChange::<PassID03>::setup(world, stages.query_stage::<SysMaterialMetaChange::<PassID03>>(ERunStageChap::Command));
        SysMaterialMetaChange::<PassID04>::setup(world, stages.query_stage::<SysMaterialMetaChange::<PassID04>>(ERunStageChap::Command));
        SysMaterialMetaChange::<PassID05>::setup(world, stages.query_stage::<SysMaterialMetaChange::<PassID05>>(ERunStageChap::Command));
        SysMaterialMetaChange::<PassID06>::setup(world, stages.query_stage::<SysMaterialMetaChange::<PassID06>>(ERunStageChap::Command));
        SysMaterialMetaChange::<PassID07>::setup(world, stages.query_stage::<SysMaterialMetaChange::<PassID07>>(ERunStageChap::Command));
        SysMaterialMetaChange::<PassID08>::setup(world, stages.query_stage::<SysMaterialMetaChange::<PassID08>>(ERunStageChap::Command));

        SysUniformComand::setup(world, stages.query_stage::<SysUniformComand>(ERunStageChap::Command));
        SysBindValueUpdate::setup(world, stages.query_stage::<SysBindValueUpdate>(ERunStageChap::Command));
        SysMaterialTexturesChange::setup(world, stages.query_stage::<SysMaterialTexturesChange>(ERunStageChap::Command));

        // 纹理属性
        {
            // 默认纹理数据
            {
                if world.get_resource::<Share<AssetMgr<SamplerRes>>>().is_none() {
                    world.insert_resource(
                        AssetMgr::<SamplerRes>::new(GarbageEmpty(), false, 60 * 1024, 60 * 1000)
                    );
                };
                let asset_tex = if let Some(asset) = world.get_resource::<Share<AssetMgr<TextureRes>>>() {
                    asset
                } else {
                    world.insert_resource(
                        AssetMgr::<TextureRes>::new(GarbageEmpty(), false, 60 * 1024 * 1024, 60 * 1000)
                    );
                    world.get_resource::<Share<AssetMgr<TextureRes>>>().unwrap()
                };
                let device = world.get_resource::<RenderDevice>().unwrap();
                let queue = world.get_resource::<RenderQueue>().unwrap();
                let desc = wgpu::TextureViewDescriptor::default();
                // 
                let texture = DefaultTexture::create(device, queue, EDefaultTexture::White, wgpu::TextureDimension::D1);
                let texture = texture.create_view(&desc);
                asset_tex.insert(Atom::from(DefaultTexture::WHITE_1D).get_hash() as u64, TextureRes::new(1, 1, 4, texture, true));
                // 
                let texture = DefaultTexture::create(device, queue, EDefaultTexture::White, wgpu::TextureDimension::D2);
                let texture = texture.create_view(&desc);
                asset_tex.insert(Atom::from(DefaultTexture::WHITE_2D).get_hash() as u64, TextureRes::new(1, 1, 4, texture, true));
                // 
                let texture = DefaultTexture::create(device, queue, EDefaultTexture::White, wgpu::TextureDimension::D3);
                let texture = texture.create_view(&desc);
                asset_tex.insert(Atom::from(DefaultTexture::WHITE_3D).get_hash() as u64, TextureRes::new(1, 1, 4, texture, true));
                // 
                let texture = DefaultTexture::create(device, queue, EDefaultTexture::Black, wgpu::TextureDimension::D1);
                let texture = texture.create_view(&desc);
                asset_tex.insert(Atom::from(DefaultTexture::BLACK_1D).get_hash() as u64, TextureRes::new(1, 1, 4, texture, true));
                // 
                let texture = DefaultTexture::create(device, queue, EDefaultTexture::Black, wgpu::TextureDimension::D2);
                let texture = texture.create_view(&desc);
                asset_tex.insert(Atom::from(DefaultTexture::BLACK_2D).get_hash() as u64, TextureRes::new(1, 1, 4, texture, true));
                // 
                let texture = DefaultTexture::create(device, queue, EDefaultTexture::Black, wgpu::TextureDimension::D3);
                let texture = texture.create_view(&desc);
                asset_tex.insert(Atom::from(DefaultTexture::BLACK_3D).get_hash() as u64, TextureRes::new(1, 1, 4, texture, true));
            }

            world.insert_resource(ImageAwait::<TextureSlot01>::default());
            world.insert_resource(ImageAwait::<TextureSlot02>::default());
            world.insert_resource(ImageAwait::<TextureSlot03>::default());
            world.insert_resource(ImageAwait::<TextureSlot04>::default());

            SysTextureSlot01Load::setup(world, stages.query_stage::<SysTextureLoad>(ERunStageChap::Command));
            SysTextureSlot02Load::setup(world, stages.query_stage::<SysTextureLoad>(ERunStageChap::Command));
            SysTextureSlot03Load::setup(world, stages.query_stage::<SysTextureLoad>(ERunStageChap::Command));
            SysTextureSlot04Load::setup(world, stages.query_stage::<SysTextureLoad>(ERunStageChap::Command));

            SysTextureResReady1::setup(world, stages.query_stage::<SysTextureResReady1>(ERunStageChap::Command));
            SysTextureResReady2::setup(world, stages.query_stage::<SysTextureResReady2>(ERunStageChap::Command));
        }

        SysEffectValueToModelByMaterialModify::<PassID01>::setup(world, stages.query_stage::<SysEffectValueToModelByMaterialModify::<PassID01>>(ERunStageChap::Command));
        SysEffectValueToModelByMaterialModify::<PassID02>::setup(world, stages.query_stage::<SysEffectValueToModelByMaterialModify::<PassID02>>(ERunStageChap::Command));
        SysEffectValueToModelByMaterialModify::<PassID03>::setup(world, stages.query_stage::<SysEffectValueToModelByMaterialModify::<PassID03>>(ERunStageChap::Command));
        SysEffectValueToModelByMaterialModify::<PassID04>::setup(world, stages.query_stage::<SysEffectValueToModelByMaterialModify::<PassID04>>(ERunStageChap::Command));
        SysEffectValueToModelByMaterialModify::<PassID05>::setup(world, stages.query_stage::<SysEffectValueToModelByMaterialModify::<PassID05>>(ERunStageChap::Command));
        SysEffectValueToModelByMaterialModify::<PassID06>::setup(world, stages.query_stage::<SysEffectValueToModelByMaterialModify::<PassID06>>(ERunStageChap::Command));
        SysEffectValueToModelByMaterialModify::<PassID07>::setup(world, stages.query_stage::<SysEffectValueToModelByMaterialModify::<PassID07>>(ERunStageChap::Command));
        SysEffectValueToModelByMaterialModify::<PassID08>::setup(world, stages.query_stage::<SysEffectValueToModelByMaterialModify::<PassID08>>(ERunStageChap::Command));

        SysEffectTexturesToModelByMaterialModify::<PassID01>::setup(world, stages.query_stage::<SysEffectTexturesToModelByMaterialModify::<PassID01>>(ERunStageChap::Command));
        SysEffectTexturesToModelByMaterialModify::<PassID02>::setup(world, stages.query_stage::<SysEffectTexturesToModelByMaterialModify::<PassID02>>(ERunStageChap::Command));
        SysEffectTexturesToModelByMaterialModify::<PassID03>::setup(world, stages.query_stage::<SysEffectTexturesToModelByMaterialModify::<PassID03>>(ERunStageChap::Command));
        SysEffectTexturesToModelByMaterialModify::<PassID04>::setup(world, stages.query_stage::<SysEffectTexturesToModelByMaterialModify::<PassID04>>(ERunStageChap::Command));
        SysEffectTexturesToModelByMaterialModify::<PassID05>::setup(world, stages.query_stage::<SysEffectTexturesToModelByMaterialModify::<PassID05>>(ERunStageChap::Command));
        SysEffectTexturesToModelByMaterialModify::<PassID06>::setup(world, stages.query_stage::<SysEffectTexturesToModelByMaterialModify::<PassID06>>(ERunStageChap::Command));
        SysEffectTexturesToModelByMaterialModify::<PassID07>::setup(world, stages.query_stage::<SysEffectTexturesToModelByMaterialModify::<PassID07>>(ERunStageChap::Command));
        SysEffectTexturesToModelByMaterialModify::<PassID08>::setup(world, stages.query_stage::<SysEffectTexturesToModelByMaterialModify::<PassID08>>(ERunStageChap::Command));

        Ok(())
    }
}
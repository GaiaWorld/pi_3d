
use std::mem::size_of;

use pi_assets::{mgr::AssetMgr, asset::GarbageEmpty, homogeneous::HomogeneousMgr};
use pi_atom::Atom;
use pi_engine_shell::prelude::*;
use pi_hash::XHashMap;


use crate::pass::*;

use self::{
    // render_item_info::{RendererItemsModifyByMaterialChange, RendererItemsReset, RendererItemsModifyByModelChange},
    // renderer_binds_sys::{SysSceneBindUpdate,},
    renderer::*,
    graphic::RendererGraphicDesc,
    render_object::RendererID,
    sys_renderer_pre::*,
    sys_renderer::*,
    pass::*,
    render_primitive::*,
};

pub mod render_object;
pub mod opaque;
pub mod renderer;
pub mod render_mode;
pub mod render_blend;
pub mod render_depth_and_stencil;
pub mod render_primitive;
pub mod render_sort;
pub mod render_target_state;
pub mod graphic;
pub mod sys_renderer_pre;
pub mod sys_renderer;
pub mod pass;
pub mod command;
pub mod base;


#[derive(Debug, Clone, Default, Component)]
pub struct ViewerRenderersInfo {
    pub map: XHashMap<Atom, (RendererGraphicDesc, RendererID)>,
}

#[derive(Component)]
pub struct DirtyViewerRenderersInfo;

pub struct PluginRenderer;
impl Plugin for PluginRenderer {
    fn build(&self, app: &mut App) {
        app.insert_resource(RendererHasher::default())
            .insert_resource(ShareAssetMgr::<RenderRes<wgpu::TextureView>>::new(
                GarbageEmpty(), 
                false,
                60 * 1024 * 1024, 
                3 * 60 * 1000
            ));
        if app.world.get_resource::<ShareAssetMgr<RenderRes<RenderPipeline>>>().is_none() {
            app.insert_resource(ShareAssetMgr::<RenderRes::<RenderPipeline>>::new(GarbageEmpty(), false, 10 * 1024 * 1024, 60 * 1000));
        }
        let device = app.world.get_resource::<PiRenderDevice>().unwrap().0.clone();
        if app.world.get_resource::<PiSafeAtlasAllocator>().is_none() {
            let texture_assets_mgr = AssetMgr::<RenderRes<wgpu::TextureView>>::new(
                GarbageEmpty(), 
                false,
                60 * 1024 * 1024, 
                3 * 60 * 1000
            );
            let unusetexture_assets_mgr = HomogeneousMgr::<RenderRes<UnuseTexture>>::new(
                pi_assets::homogeneous::GarbageEmpty(), 
                10 * size_of::<UnuseTexture>(),
                size_of::<UnuseTexture>(),
                3 * 60 * 1000,
            );
            let atlas = SafeAtlasAllocator::new(device, texture_assets_mgr, unusetexture_assets_mgr);
            app.insert_resource(PiSafeAtlasAllocator(atlas));
        }
        
        if app.world.get_resource::<AssetDataCenterShader3D>().is_none() {
            app.insert_resource(AssetDataCenterShader3D::new(false, 10 * 1024 * 1024, 60 * 1000));
        }
        if app.world.get_resource::<AssetDataCenterPipeline3D>().is_none() {
            app.insert_resource(AssetDataCenterPipeline3D::new(false, 10 * 1024 * 1024, 60 * 1000));
        }
        if app.world.get_resource::<AssetLoaderShader3D>().is_none() {
            app.insert_resource(AssetLoaderShader3D::default());
        }
        if app.world.get_resource::<AssetLoaderPipeline3D>().is_none() {
            app.insert_resource(AssetLoaderPipeline3D::default());
        }

        app.insert_resource(ActionListCullMode::default());
        app.insert_resource(ActionListPolyginMode::default());
        app.insert_resource(ActionListFrontFace::default());
        app.add_systems(
            (
                sys_act_mesh_cull_mode,
                sys_act_mesh_polygon_mode,
                sys_act_mesh_frontface,
            ).in_set(ERunStageChap::SecondInitial)
        );
        app.add_system(
            sys_render_primitive_modify.in_set(ERunStageChap::Command)
        );

        app.add_systems(
            (
                sys_bind_buffer_apply.in_set(ERunStageChap::DrawUniformToGPU),
                sys_bind_group_loaded.in_set(ERunStageChap::DrawBindGroupsLoaded),
                sys_pass_shader_loaded.in_set(ERunStageChap::DrawShaderLoaded),
                sys_pass_pipeline_loaded.in_set(ERunStageChap::DrawPipelineLoaded),
                sys_renderer_draws_modify.in_set(ERunStageChap::Draw)
            )
        );
        app.add_systems(
            (
                sys_set0_modify_by_renderer::<Pass01, PassID01>,
                sys_set0_modify_by_renderer::<Pass02, PassID02>,
                sys_set0_modify_by_renderer::<Pass03, PassID03>,
                sys_set0_modify_by_renderer::<Pass04, PassID04>,
                sys_set0_modify_by_renderer::<Pass05, PassID05>,
                sys_set0_modify_by_renderer::<Pass06, PassID06>,
                sys_set0_modify_by_renderer::<Pass07, PassID07>,
                sys_set0_modify_by_renderer::<Pass08, PassID08>,
            ).in_set(ERunStageChap::DrawBinds)
        );
        app.add_systems(
            (

                sys_set1_modify_by_renderer::<Pass01, PassID01>,
                sys_set1_modify_by_renderer::<Pass02, PassID02>,
                sys_set1_modify_by_renderer::<Pass03, PassID03>,
                sys_set1_modify_by_renderer::<Pass04, PassID04>,
                sys_set1_modify_by_renderer::<Pass05, PassID05>,
                sys_set1_modify_by_renderer::<Pass06, PassID06>,
                sys_set1_modify_by_renderer::<Pass07, PassID07>,
                sys_set1_modify_by_renderer::<Pass08, PassID08>,
            ).in_set(ERunStageChap::DrawBinds)
        );
        app.add_systems(
            (

                sys_set2_modify_by_renderer::<Pass01, PassID01>,
                sys_set2_modify_by_renderer::<Pass02, PassID02>,
                sys_set2_modify_by_renderer::<Pass03, PassID03>,
                sys_set2_modify_by_renderer::<Pass04, PassID04>,
                sys_set2_modify_by_renderer::<Pass05, PassID05>,
                sys_set2_modify_by_renderer::<Pass06, PassID06>,
                sys_set2_modify_by_renderer::<Pass07, PassID07>,
                sys_set2_modify_by_renderer::<Pass08, PassID08>,
            ).in_set(ERunStageChap::DrawBinds)
        );

        app.add_systems(
            (
                sys_set1_modify_by_pass::<Pass01, PassID01>,
                sys_set1_modify_by_pass::<Pass02, PassID02>,
                sys_set1_modify_by_pass::<Pass03, PassID03>,
                sys_set1_modify_by_pass::<Pass04, PassID04>,
                sys_set1_modify_by_pass::<Pass05, PassID05>,
                sys_set1_modify_by_pass::<Pass06, PassID06>,
                sys_set1_modify_by_pass::<Pass07, PassID07>,
                sys_set1_modify_by_pass::<Pass08, PassID08>,
            ).in_set(ERunStageChap::DrawBinds)
        );
        
        app.add_systems(
            (
                sys_set0_modify_by_scene::<Pass01, PassID01>,
                sys_set0_modify_by_scene::<Pass02, PassID02>,
                sys_set0_modify_by_scene::<Pass03, PassID03>,
                sys_set0_modify_by_scene::<Pass04, PassID04>,
                sys_set0_modify_by_scene::<Pass05, PassID05>,
                sys_set0_modify_by_scene::<Pass06, PassID06>,
                sys_set0_modify_by_scene::<Pass07, PassID07>,
                sys_set0_modify_by_scene::<Pass08, PassID08>,
            ).in_set(ERunStageChap::DrawBinds)
        );
        
        app.add_systems(
            (

                sys_set1_modify_by_model::<Pass01, PassID01>,
                sys_set1_modify_by_model::<Pass02, PassID02>,
                sys_set1_modify_by_model::<Pass03, PassID03>,
                sys_set1_modify_by_model::<Pass04, PassID04>,
                sys_set1_modify_by_model::<Pass05, PassID05>,
                sys_set1_modify_by_model::<Pass06, PassID06>,
                sys_set1_modify_by_model::<Pass07, PassID07>,
                sys_set1_modify_by_model::<Pass08, PassID08>,
            ).in_set(ERunStageChap::DrawBinds)
        );
        
        app.add_systems(
            (

                sys_set2_modify_by_model::<Pass01, PassID01>,
                sys_set2_modify_by_model::<Pass02, PassID02>,
                sys_set2_modify_by_model::<Pass03, PassID03>,
                sys_set2_modify_by_model::<Pass04, PassID04>,
                sys_set2_modify_by_model::<Pass05, PassID05>,
                sys_set2_modify_by_model::<Pass06, PassID06>,
                sys_set2_modify_by_model::<Pass07, PassID07>,
                sys_set2_modify_by_model::<Pass08, PassID08>,
            ).in_set(ERunStageChap::DrawBinds)
        );


        app.add_systems(
            (
                sys_pass_bind_groups::<Pass01, PassID01>,
                sys_pass_bind_groups::<Pass02, PassID02>,
                sys_pass_bind_groups::<Pass03, PassID03>,
                sys_pass_bind_groups::<Pass04, PassID04>,
                sys_pass_bind_groups::<Pass05, PassID05>,
                sys_pass_bind_groups::<Pass06, PassID06>,
                sys_pass_bind_groups::<Pass07, PassID07>,
                sys_pass_bind_groups::<Pass08, PassID08>,
            ).in_set(ERunStageChap::DrawBindGroups)
        );
        
        app.add_systems(
            (
                sys_pass_shader_request_by_model::<Pass01, PassID01>,
                sys_pass_shader_request_by_model::<Pass02, PassID02>,
                sys_pass_shader_request_by_model::<Pass03, PassID03>,
                sys_pass_shader_request_by_model::<Pass04, PassID04>,
                sys_pass_shader_request_by_model::<Pass05, PassID05>,
                sys_pass_shader_request_by_model::<Pass06, PassID06>,
                sys_pass_shader_request_by_model::<Pass07, PassID07>,
                sys_pass_shader_request_by_model::<Pass08, PassID08>,
            ).in_set(ERunStageChap::DrawShader)
        );
        
        app.add_systems(
            (
                sys_pass_shader_request_by_pass::<Pass01, PassID01>,
                sys_pass_shader_request_by_pass::<Pass02, PassID02>,
                sys_pass_shader_request_by_pass::<Pass03, PassID03>,
                sys_pass_shader_request_by_pass::<Pass04, PassID04>,
                sys_pass_shader_request_by_pass::<Pass05, PassID05>,
                sys_pass_shader_request_by_pass::<Pass06, PassID06>,
                sys_pass_shader_request_by_pass::<Pass07, PassID07>,
                sys_pass_shader_request_by_pass::<Pass08, PassID08>,
            ).in_set(ERunStageChap::DrawShader)
        );
        
        app.add_systems(
            (
                sys_pass_pipeline_request_by_model::<Pass01, PassID01>,
                sys_pass_pipeline_request_by_model::<Pass02, PassID02>,
                sys_pass_pipeline_request_by_model::<Pass03, PassID03>,
                sys_pass_pipeline_request_by_model::<Pass04, PassID04>,
                sys_pass_pipeline_request_by_model::<Pass05, PassID05>,
                sys_pass_pipeline_request_by_model::<Pass06, PassID06>,
                sys_pass_pipeline_request_by_model::<Pass07, PassID07>,
                sys_pass_pipeline_request_by_model::<Pass08, PassID08>,
            ).in_set(ERunStageChap::DrawPipeline)
        );
        app.add_systems(
            (
                sys_pass_pipeline_request_by_pass::<Pass01, PassID01>,
                sys_pass_pipeline_request_by_pass::<Pass02, PassID02>,
                sys_pass_pipeline_request_by_pass::<Pass03, PassID03>,
                sys_pass_pipeline_request_by_pass::<Pass04, PassID04>,
                sys_pass_pipeline_request_by_pass::<Pass05, PassID05>,
                sys_pass_pipeline_request_by_pass::<Pass06, PassID06>,
                sys_pass_pipeline_request_by_pass::<Pass07, PassID07>,
                sys_pass_pipeline_request_by_pass::<Pass08, PassID08>,
            ).in_set(ERunStageChap::DrawPipeline)
        );
        app.add_systems(
            (
                sys_pass_draw_modify_by_model::<Pass01, PassID01>,
                sys_pass_draw_modify_by_model::<Pass02, PassID02>,
                sys_pass_draw_modify_by_model::<Pass03, PassID03>,
                sys_pass_draw_modify_by_model::<Pass04, PassID04>,
                sys_pass_draw_modify_by_model::<Pass05, PassID05>,
                sys_pass_draw_modify_by_model::<Pass06, PassID06>,
                sys_pass_draw_modify_by_model::<Pass07, PassID07>,
                sys_pass_draw_modify_by_model::<Pass08, PassID08>,
            ).in_set(ERunStageChap::DrawCall)
        );
        app.add_systems(
            (
                sys_pass_draw_modify_by_pass::<Pass01, PassID01>,
                sys_pass_draw_modify_by_pass::<Pass02, PassID02>,
                sys_pass_draw_modify_by_pass::<Pass03, PassID03>,
                sys_pass_draw_modify_by_pass::<Pass04, PassID04>,
                sys_pass_draw_modify_by_pass::<Pass05, PassID05>,
                sys_pass_draw_modify_by_pass::<Pass06, PassID06>,
                sys_pass_draw_modify_by_pass::<Pass07, PassID07>,
                sys_pass_draw_modify_by_pass::<Pass08, PassID08>,
            ).in_set(ERunStageChap::DrawCall)
        );

        // app.add_systems(
        //     (
        //         sys_bind_buffer_apply,
        //         (
        //             sys_set0_modify_by_renderer::<Pass01, PassID01>,
        //             sys_set0_modify_by_renderer::<Pass02, PassID02>,
        //             sys_set0_modify_by_renderer::<Pass03, PassID03>,
        //             sys_set0_modify_by_renderer::<Pass04, PassID04>,
        //             sys_set0_modify_by_renderer::<Pass05, PassID05>,
        //             sys_set0_modify_by_renderer::<Pass06, PassID06>,
        //             sys_set0_modify_by_renderer::<Pass07, PassID07>,
        //             sys_set0_modify_by_renderer::<Pass08, PassID08>,

        //             sys_set1_modify_by_renderer::<Pass01, PassID01>,
        //             sys_set1_modify_by_renderer::<Pass02, PassID02>,
        //             sys_set1_modify_by_renderer::<Pass03, PassID03>,
        //             sys_set1_modify_by_renderer::<Pass04, PassID04>,
        //             sys_set1_modify_by_renderer::<Pass05, PassID05>,
        //             sys_set1_modify_by_renderer::<Pass06, PassID06>,
        //             sys_set1_modify_by_renderer::<Pass07, PassID07>,
        //             sys_set1_modify_by_renderer::<Pass08, PassID08>,

        //             sys_set2_modify_by_renderer::<Pass01, PassID01>,
        //             sys_set2_modify_by_renderer::<Pass02, PassID02>,
        //             sys_set2_modify_by_renderer::<Pass03, PassID03>,
        //             sys_set2_modify_by_renderer::<Pass04, PassID04>,
        //             sys_set2_modify_by_renderer::<Pass05, PassID05>,
        //             sys_set2_modify_by_renderer::<Pass06, PassID06>,
        //             sys_set2_modify_by_renderer::<Pass07, PassID07>,
        //             sys_set2_modify_by_renderer::<Pass08, PassID08>,
        //         ),
        //         (
        //             sys_set1_modify_by_pass::<Pass01, PassID01>,
        //             sys_set1_modify_by_pass::<Pass02, PassID02>,
        //             sys_set1_modify_by_pass::<Pass03, PassID03>,
        //             sys_set1_modify_by_pass::<Pass04, PassID04>,
        //             sys_set1_modify_by_pass::<Pass05, PassID05>,
        //             sys_set1_modify_by_pass::<Pass06, PassID06>,
        //             sys_set1_modify_by_pass::<Pass07, PassID07>,
        //             sys_set1_modify_by_pass::<Pass08, PassID08>,
        //         )
        //         (

        //             sys_set0_modify_by_scene::<Pass01, PassID01>,
        //             sys_set0_modify_by_scene::<Pass02, PassID02>,
        //             sys_set0_modify_by_scene::<Pass03, PassID03>,
        //             sys_set0_modify_by_scene::<Pass04, PassID04>,
        //             sys_set0_modify_by_scene::<Pass05, PassID05>,
        //             sys_set0_modify_by_scene::<Pass06, PassID06>,
        //             sys_set0_modify_by_scene::<Pass07, PassID07>,
        //             sys_set0_modify_by_scene::<Pass08, PassID08>,

        //             sys_set1_modify_by_model::<Pass01, PassID01>,
        //             sys_set1_modify_by_model::<Pass02, PassID02>,
        //             sys_set1_modify_by_model::<Pass03, PassID03>,
        //             sys_set1_modify_by_model::<Pass04, PassID04>,
        //             sys_set1_modify_by_model::<Pass05, PassID05>,
        //             sys_set1_modify_by_model::<Pass06, PassID06>,
        //             sys_set1_modify_by_model::<Pass07, PassID07>,
        //             sys_set1_modify_by_model::<Pass08, PassID08>,

        //             sys_set2_modify_by_model::<Pass01, PassID01>,
        //             sys_set2_modify_by_model::<Pass02, PassID02>,
        //             sys_set2_modify_by_model::<Pass03, PassID03>,
        //             sys_set2_modify_by_model::<Pass04, PassID04>,
        //             sys_set2_modify_by_model::<Pass05, PassID05>,
        //             sys_set2_modify_by_model::<Pass06, PassID06>,
        //             sys_set2_modify_by_model::<Pass07, PassID07>,
        //             sys_set2_modify_by_model::<Pass08, PassID08>,
        //         ),
        //         sys_bind_group_loaded
        //         ,
        //         (
        //             sys_pass_bind_groups::<Pass01, PassID01>,
        //             sys_pass_bind_groups::<Pass02, PassID02>,
        //             sys_pass_bind_groups::<Pass03, PassID03>,
        //             sys_pass_bind_groups::<Pass04, PassID04>,
        //             sys_pass_bind_groups::<Pass05, PassID05>,
        //             sys_pass_bind_groups::<Pass06, PassID06>,
        //             sys_pass_bind_groups::<Pass07, PassID07>,
        //             sys_pass_bind_groups::<Pass08, PassID08>,
        //         ),
        //         (
        //             sys_pass_shader_request_by_model::<Pass01, PassID01>,
        //             sys_pass_shader_request_by_model::<Pass02, PassID02>,
        //             sys_pass_shader_request_by_model::<Pass03, PassID03>,
        //             sys_pass_shader_request_by_model::<Pass04, PassID04>,
        //             sys_pass_shader_request_by_model::<Pass05, PassID05>,
        //             sys_pass_shader_request_by_model::<Pass06, PassID06>,
        //             sys_pass_shader_request_by_model::<Pass07, PassID07>,
        //             sys_pass_shader_request_by_model::<Pass08, PassID08>,
        //         ),
        //         (
        //             sys_pass_shader_request_by_pass::<Pass01, PassID01>,
        //             sys_pass_shader_request_by_pass::<Pass02, PassID02>,
        //             sys_pass_shader_request_by_pass::<Pass03, PassID03>,
        //             sys_pass_shader_request_by_pass::<Pass04, PassID04>,
        //             sys_pass_shader_request_by_pass::<Pass05, PassID05>,
        //             sys_pass_shader_request_by_pass::<Pass06, PassID06>,
        //             sys_pass_shader_request_by_pass::<Pass07, PassID07>,
        //             sys_pass_shader_request_by_pass::<Pass08, PassID08>,
        //         ),
        //         sys_pass_shader_loaded
        //         ,
        //         (
        //             sys_pass_pipeline_request_by_model::<Pass01, PassID01>,
        //             sys_pass_pipeline_request_by_model::<Pass02, PassID02>,
        //             sys_pass_pipeline_request_by_model::<Pass03, PassID03>,
        //             sys_pass_pipeline_request_by_model::<Pass04, PassID04>,
        //             sys_pass_pipeline_request_by_model::<Pass05, PassID05>,
        //             sys_pass_pipeline_request_by_model::<Pass06, PassID06>,
        //             sys_pass_pipeline_request_by_model::<Pass07, PassID07>,
        //             sys_pass_pipeline_request_by_model::<Pass08, PassID08>,
        //         ),
        //         (
        //             sys_pass_pipeline_request_by_pass::<Pass01, PassID01>,
        //             sys_pass_pipeline_request_by_pass::<Pass02, PassID02>,
        //             sys_pass_pipeline_request_by_pass::<Pass03, PassID03>,
        //             sys_pass_pipeline_request_by_pass::<Pass04, PassID04>,
        //             sys_pass_pipeline_request_by_pass::<Pass05, PassID05>,
        //             sys_pass_pipeline_request_by_pass::<Pass06, PassID06>,
        //             sys_pass_pipeline_request_by_pass::<Pass07, PassID07>,
        //             sys_pass_pipeline_request_by_pass::<Pass08, PassID08>,
        //         ),
        //         sys_pass_pipeline_loaded
        //         ,
        //         (
        //             sys_pass_draw_modify_by_model::<Pass01, PassID01>,
        //             sys_pass_draw_modify_by_model::<Pass02, PassID02>,
        //             sys_pass_draw_modify_by_model::<Pass03, PassID03>,
        //             sys_pass_draw_modify_by_model::<Pass04, PassID04>,
        //             sys_pass_draw_modify_by_model::<Pass05, PassID05>,
        //             sys_pass_draw_modify_by_model::<Pass06, PassID06>,
        //             sys_pass_draw_modify_by_model::<Pass07, PassID07>,
        //             sys_pass_draw_modify_by_model::<Pass08, PassID08>,
        //         ),
        //         (
        //             sys_pass_draw_modify_by_pass::<Pass01, PassID01>,
        //             sys_pass_draw_modify_by_pass::<Pass02, PassID02>,
        //             sys_pass_draw_modify_by_pass::<Pass03, PassID03>,
        //             sys_pass_draw_modify_by_pass::<Pass04, PassID04>,
        //             sys_pass_draw_modify_by_pass::<Pass05, PassID05>,
        //             sys_pass_draw_modify_by_pass::<Pass06, PassID06>,
        //             sys_pass_draw_modify_by_pass::<Pass07, PassID07>,
        //             sys_pass_draw_modify_by_pass::<Pass08, PassID08>,
        //         ),
        //         sys_renderer_draws_modify
        //     ).in_set(ERunStageChap::Draw)
        // )
    }
}

use std::mem::replace;

use pi_atom::Atom;
use pi_ecs::prelude::{ResMut, Commands, Query, Res, EntityCommands};
use pi_ecs_macros::setup;
use pi_engine_shell::{object::{ObjectID, GameObject}, run_stage::TSystemStageInfo};
use pi_render::{rhi::device::RenderDevice, renderer::bind_buffer::BindBufferAllocator};
use pi_scene_math::Vector3;

use crate::{viewer::{ViewerViewMatrix, ViewerProjectionMatrix, ViewerTransformMatrix, ViewerGlobalPosition, ViewerDirection, ModelList, FlagModelList, ModelListAfterCulling, ViewerActive, BindViewer, command::{SingleRendererCommandList, ERendererCommand}}, renderers::{ViewerRenderersInfo, render_object::RendererID, graphic::RendererGraphicDesc, renderer::{RenderSize, RenderColorFormat, RenderColorClear, RenderDepthFormat, RenderDepthClear}}, materials::{material::MaterialID, command::{SingleMatCreateCommands, EMatCreateCommand}}, pass::{EPassTag, PassTagOrders}, flags::UniqueName};

use super::{base::{LightDirection, Light, LightingMode}, point::ShadowAngle, shadow_generator::{base::{ShadowMinZ, ShadowMaxZ, ShadowFrustumSize, ShadowEnable, ShadowBias, ShadowNormalBias, ShadowDepthScale, ShadowAtlasSize, }, ShaderShadowGenerator}};

#[derive(Default)]
pub struct SingleLightCreateCommands(pub Vec<(ObjectID, Atom)>);

pub struct SysLightCreateCommand;
impl TSystemStageInfo for SysLightCreateCommand {
} 
#[setup]
impl SysLightCreateCommand {
    #[system]
    fn sys(
        mut cmds: ResMut<SingleLightCreateCommands>,
        mut light_cmd: Commands<GameObject, Light>,
        mut unique_name_cmd: Commands<GameObject, UniqueName>,
        mut lighting_cmd: Commands<GameObject, LightingMode>,
        mut minz_cmd: Commands<GameObject, ShadowMinZ>,
        mut maxz_cmd: Commands<GameObject, ShadowMaxZ>,
        mut frustum_cmd: Commands<GameObject, ShadowFrustumSize>,
        mut direct_cmd: Commands<GameObject, LightDirection>,
        mut shadowangle_cmd: Commands<GameObject, ShadowAngle>,
        mut shadowenable_cmd: Commands<GameObject, ShadowEnable>,
        mut shadowsize_cmd: Commands<GameObject, ShadowAtlasSize>,

        mut view_cmd: Commands<GameObject, ViewerViewMatrix>,
        mut proj_cmd: Commands<GameObject, ViewerProjectionMatrix>,
        mut tran_cmd: Commands<GameObject, ViewerTransformMatrix>,
        mut gpos_cmd: Commands<GameObject, ViewerGlobalPosition>,
        mut vdir_cmd: Commands<GameObject, ViewerDirection>,
        mut list_model_cmd: Commands<GameObject, ModelList>,
        mut flag_list_model_cmd: Commands<GameObject, FlagModelList>,
        mut list_culling_cmd: Commands<GameObject, ModelListAfterCulling>,
        mut viewer_active_cmd: Commands<GameObject, ViewerActive>,
        mut viewer_bind_cmd: Commands<GameObject, BindViewer>,
        mut viewer_render_cmd: Commands<GameObject, ViewerRenderersInfo>,
        mut renderid_cmd: Commands<GameObject, RendererID>,

        mut material_cmd: Commands<GameObject, MaterialID>,
        mut rendersize_cmd: Commands<GameObject, RenderSize>,

        device: Res<RenderDevice>,
        mut dynallocator: ResMut<BindBufferAllocator>,
        mut entity_cmd: EntityCommands<GameObject>,

        mut render_cmds: ResMut<SingleRendererCommandList>,
    ) {
        let mut list = replace(&mut cmds.0, vec![]);
        list.drain(..).for_each(|(entity, name)| {
            unique_name_cmd.insert(entity, UniqueName(name));
            light_cmd.insert(entity, Light::Directional);
            lighting_cmd.insert(entity, LightingMode::Lambert);
            minz_cmd.insert(entity, ShadowMinZ::default());
            maxz_cmd.insert(entity, ShadowMaxZ::default());
            frustum_cmd.insert(entity, ShadowFrustumSize::default());
            direct_cmd.insert(entity, LightDirection::default());
            shadowangle_cmd.insert(entity, ShadowAngle::default());
            shadowenable_cmd.insert(entity, ShadowEnable(false));
            shadowsize_cmd.insert(entity, ShadowAtlasSize::default());

            view_cmd.insert(entity, ViewerViewMatrix::default());
            proj_cmd.insert(entity, ViewerProjectionMatrix::default());
            tran_cmd.insert(entity, ViewerTransformMatrix::default());
            gpos_cmd.insert(entity, ViewerGlobalPosition::default());
            vdir_cmd.insert(entity, ViewerDirection::default());
            list_model_cmd.insert(entity, ModelList::default());
            flag_list_model_cmd.insert(entity, FlagModelList::default());
            list_culling_cmd.insert(entity, ModelListAfterCulling::default());
            viewer_active_cmd.insert(entity, ViewerActive(false));
            viewer_render_cmd.insert(entity, ViewerRenderersInfo::default());

            if let Some(data) = BindViewer::new(&mut dynallocator) {
                viewer_bind_cmd.insert(entity, data);
            };
            
            

            let mat = entity_cmd.spawn();
            material_cmd.insert(entity, MaterialID(mat));
            
            let rendererid = entity_cmd.spawn();
            renderid_cmd.insert(entity, RendererID(rendererid));
    
            // let size = ShadowAtlasSize::default().0;
            // render_cmds.list.push(
            //     ERendererCommand::RenderSize(rendererid, RenderSize::new(size, size))
            // );
        });
    }
}


pub enum ELightModifyCommand {
    LightType(ObjectID, Light),
    LightingType(ObjectID, LightingMode),
    ShadowMinz(ObjectID, f32),
    ShadowMaxz(ObjectID, f32),
    ShadowFrustumSize(ObjectID, f32),
    Directional(ObjectID, Vector3),
    ShadowEnable(ObjectID, bool),
    Bias(ObjectID, f32),
    NormalBias(ObjectID, f32),
    DepthScale(ObjectID, f32),
    AtlasSize(ObjectID, u32),
}

#[derive(Default)]
pub struct SingleLightModifyCommands(pub Vec<ELightModifyCommand>);

pub struct SysLightModifyCommand;
impl TSystemStageInfo for SysLightModifyCommand {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            // SysLightCreateCommand::key()
        ]
    }
}
#[setup]
impl SysLightModifyCommand {
    #[system]
    fn sys(
        mut cmds: ResMut<SingleLightModifyCommands>,
        lights: Query<GameObject, (&Light, &LightingMode, &MaterialID, &RendererID, &UniqueName, &ShadowAtlasSize)>,
        shadowangles: Query<GameObject, &ShadowAngle>,
        mut light_cmd: Commands<GameObject, Light>,
        mut lighting_cmd: Commands<GameObject, LightingMode>,
        mut minz_cmd: Commands<GameObject, ShadowMinZ>,
        mut maxz_cmd: Commands<GameObject, ShadowMaxZ>,
        mut frustum_cmd: Commands<GameObject, ShadowFrustumSize>,
        mut direct_cmd: Commands<GameObject, LightDirection>,
        mut shadowangle_cmd: Commands<GameObject, ShadowAngle>,
        mut shadowenable_cmd: Commands<GameObject, ShadowEnable>,
        mut bias_cmd: Commands<GameObject, ShadowBias>,
        mut normal_bias_cmd: Commands<GameObject, ShadowNormalBias>,
        mut depthscale_cmd: Commands<GameObject, ShadowDepthScale>,
        mut shadowsize_cmd: Commands<GameObject, ShadowAtlasSize>,

        mut rendersize_cmd: Commands<GameObject, RenderSize>,
        mut viewer_active_cmd: Commands<GameObject, ViewerActive>,
        mut material_create_cmd: ResMut<SingleMatCreateCommands>,
        
        mut render_cmds: ResMut<SingleRendererCommandList>,
    ) {
        let mut list = replace(&mut cmds.0, vec![]);
        list.drain(..).for_each(|cmd| {
            match cmd {
                ELightModifyCommand::LightType(entity, val) => {
                    if let Some((old_val, _, _, _, _, size)) = lights.get(entity) {
                        if *old_val == val {
                            return;
                        }
                    }
                    light_cmd.insert(entity, val);
                },
                ELightModifyCommand::LightingType(entity, val) => {
                    if let Some((_, old_val, _, _, _, size)) = lights.get(entity) {
                        if *old_val == val {
                            return;
                        }
                    }
                    lighting_cmd.insert(entity, val);
                },
                ELightModifyCommand::ShadowMinz(entity, val) => {
                    minz_cmd.insert(entity, ShadowMinZ(val));
                },
                ELightModifyCommand::ShadowMaxz(entity, val) => {
                    maxz_cmd.insert(entity, ShadowMaxZ(val));
                },
                ELightModifyCommand::ShadowFrustumSize(entity, val) => {
                    frustum_cmd.insert(entity, ShadowFrustumSize(val));
                },
                ELightModifyCommand::Directional(entity, val) => {
                    direct_cmd.insert(entity, LightDirection(val));
                },
                ELightModifyCommand::ShadowEnable(entity, val) => {
                    shadowenable_cmd.insert(entity, ShadowEnable(val));
                    viewer_active_cmd.insert(entity, ViewerActive(val));

                    if let Some((_, _, id_mat, id_render, name, size)) = lights.get(entity) {
                        material_create_cmd.0.push(
                            EMatCreateCommand::Use(id_mat.0.clone(), Atom::from(ShaderShadowGenerator::KEY), EPassTag::ShadowCast)
                        );
                
                        let desc = RendererGraphicDesc {
                            pre: Some(Atom::from("Clear")),
                            curr: name.0.clone(),
                            next: None,
                            passorders: PassTagOrders::new(vec![EPassTag::ShadowCast]),
                        };
                        render_cmds.list.push(
                            ERendererCommand::Active(entity, id_render.clone(), desc)
                        );

                        let id_render = id_render.0;
                        render_cmds.list.push(
                            ERendererCommand::RenderColorFormat(id_render, RenderColorFormat(wgpu::TextureFormat::Rgba16Float))
                        );
                        render_cmds.list.push(
                            ERendererCommand::RenderColorClear(id_render, RenderColorClear(wgpu::Color { r: 0., g: 0., b: 0., a: 0. }))
                        );
                        render_cmds.list.push(
                            ERendererCommand::RenderDepthFormat(id_render, RenderDepthFormat(Some(wgpu::TextureFormat::Depth32Float)))
                        );
                        render_cmds.list.push(
                            ERendererCommand::RenderDepthClear(id_render, RenderDepthClear(0.))
                        );
                        render_cmds.list.push(
                            ERendererCommand::RenderSize(id_render, RenderSize::new(size.0, size.0))
                        );
                    }
                },
                ELightModifyCommand::Bias(entity, val) => {
                    bias_cmd.insert(entity, ShadowBias(val));
                },
                ELightModifyCommand::NormalBias(entity, val) => {
                    normal_bias_cmd.insert(entity, ShadowNormalBias(val));
                },
                ELightModifyCommand::DepthScale(entity, val) => {
                    depthscale_cmd.insert(entity, ShadowDepthScale(val));
                },
                ELightModifyCommand::AtlasSize(entity, val) => {
                    shadowsize_cmd.insert(entity, ShadowAtlasSize(val));
                    
                    if let Some((_, _, id_mat, id_render, name, size)) = lights.get(entity) {
                        let id_render = id_render.0;
                        
                        render_cmds.list.push(
                            ERendererCommand::RenderSize(id_render, RenderSize::new(val, val))
                        );
                    }
                },
            }
        });
    }
}

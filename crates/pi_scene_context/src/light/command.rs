use std::mem::replace;

use pi_atom::Atom;

use pi_engine_shell::prelude::*;
use pi_render::{rhi::device::RenderDevice, renderer::bind_buffer::BindBufferAllocator, graph::graph::RenderGraph};
use pi_scene_math::Vector3;

use crate::{viewer::{ViewerViewMatrix, ViewerProjectionMatrix, ViewerTransformMatrix, ViewerGlobalPosition, ViewerDirection, ModelList, FlagModelList, ModelListAfterCulling, ViewerActive, BindViewer, command::{SingleRendererCommandList, ERendererCommand}}, renderers::{ViewerRenderersInfo, render_object::RendererID, graphic::RendererGraphicDesc, renderer::{RenderSize, RenderColorFormat, RenderColorClear, RenderDepthFormat, RenderDepthClear}, DirtyViewerRenderersInfo}, materials::{material::MaterialID, command::{SingleMatCreateCommands, EMatCreateCommand, ActionMaterial}}, pass::{EPassTag, PassTagOrders}, flags::{UniqueName, enable::SingleEnableCommands, Enable}, commands::TCommandList};

use super::{base::{LightDirection, Light, LightingMode}, point::ShadowAngle, shadow_generator::{base::{ShadowMinZ, ShadowMaxZ, ShadowFrustumSize, ShadowEnable, ShadowBias, ShadowNormalBias, ShadowDepthScale, ShadowAtlasSize, }, ShaderShadowGenerator}};

#[derive(Default)]
pub struct SingleLightCreateCommands(pub Vec<(ObjectID, Atom)>);

pub struct ActionLight;
impl ActionLight {
    pub fn create(
        commands: &mut Commands,
        light: Entity,
        name: Atom,
        device: &PiRenderDevice,
        dynallocator: &mut BindBufferAllocator,
    ) {
        let mut lightcmd = commands.entity(light);
        lightcmd.insert(UniqueName(name));
        lightcmd.insert(Light::Directional);
        lightcmd.insert(LightingMode::Lambert);
        lightcmd.insert(ShadowMinZ::default());
        lightcmd.insert( ShadowMaxZ::default());
        lightcmd.insert(ShadowFrustumSize::default());
        lightcmd.insert(LightDirection::default());
        lightcmd.insert(ShadowAngle::default());
        lightcmd.insert(ShadowEnable(false));
        lightcmd.insert(ShadowAtlasSize::default());

        lightcmd.insert(ViewerViewMatrix::default());
        lightcmd.insert(ViewerProjectionMatrix::default());
        lightcmd.insert(ViewerTransformMatrix::default());
        lightcmd.insert(ViewerGlobalPosition::default());
        lightcmd.insert(ViewerDirection::default());
        lightcmd.insert(ModelList::default());
        lightcmd.insert(FlagModelList::default());
        lightcmd.insert(ModelListAfterCulling::default());
        lightcmd.insert(ViewerActive(false));
        lightcmd.insert(ViewerRenderersInfo::default());
        lightcmd.insert(DirtyViewerRenderersInfo);

        lightcmd.insert(Enable(true));

        if let Some(data) = BindViewer::new(dynallocator) {
            lightcmd.insert(data);
        };

        let mat = commands.spawn_empty().id();
        lightcmd.insert(MaterialID(mat));
        
        let rendererid = commands.spawn_empty().id();
        lightcmd.insert(RendererID(rendererid));
    }

    pub fn modify_param(
        commands: &mut EntityCommands,
        light: Entity,
        cmd: ELightModifyCommand,
    ) {
        match cmd {
            ELightModifyCommand::ShadowMinz(val) => {
                commands.insert(ShadowMinZ(val));
            },
            ELightModifyCommand::ShadowMaxz(val) => {
                commands.insert(ShadowMaxZ(val));
            },
            ELightModifyCommand::ShadowFrustumSize(val) => {
                commands.insert(ShadowFrustumSize(val));
            },
            ELightModifyCommand::Directional(val) => {
                commands.insert(LightDirection(val));
            },
            ELightModifyCommand::Bias(val) => {
                commands.insert(ShadowBias(val));
            },
            ELightModifyCommand::NormalBias(val) => {
                commands.insert(ShadowNormalBias(val));
            },
            ELightModifyCommand::DepthScale(val) => {
                commands.insert(ShadowDepthScale(val));
            },
            ELightModifyCommand::AtlasSize(val) => {
                commands.insert(ShadowAtlasSize(val));
            },
        }
    }

    pub fn modify_light_mode(
        commands: &mut EntityCommands,
        val: Light,
        old_val: &Light,
    ) {
        if *old_val == val {
            return;
        }
        commands.insert(val);
    }

    pub fn modify_lighting_mode(
        commands: &mut EntityCommands,
        val: Light,
        old_val: &LightingMode,
    ) {
        if *old_val == val {
            return;
        }
        commands.insert(val);
    }

    pub fn active_shadow(
        commands: &mut Commands,
        light: Entity,
        val: bool,
        old_val: (&MaterialID, &mut ViewerRenderersInfo, &UniqueName),
    ) {
        let (id_mat, mut viewer_renderers, name) = old_val;

        ActionMaterial::use_material(&mut commands.entity(id_mat.0), Atom::from(ShaderShadowGenerator::KEY), EPassTag::ShadowCast);

        let lightcmd = commands.entity(light);
        commands.insert(ShadowEnable(val));

        material_create_cmd.0.push(
            EMatCreateCommand::Use(id_mat.0.clone(), Atom::from(ShaderShadowGenerator::KEY), EPassTag::ShadowCast)
        );

        let graphic_desc = RendererGraphicDesc {
            pre: Some(Atom::from("Clear")),
            curr: name.0.clone(),
            next: None,
            passorders: PassTagOrders::new(vec![EPassTag::ShadowCast]),
        };
        if let Some((render, id_render)) = viewer_renderers.map.get(&graphic_desc.curr) {
            entity_cmd.despawn(id_render.0);
            render_graphic.remove_node(render.curr.to_string());
        }
        let id_renderer = entity_cmd.spawn();
        viewer_renderers.map.insert(graphic_desc.curr.clone(), (graphic_desc.clone(), RendererID(id_renderer)));
        
        render_cmds.list.push(
            ERendererCommand::Active(RendererID(id_renderer), graphic_desc)
        );
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

// #[derive(Default)]
// pub struct SingleLightModifyCommands(pub Vec<ELightModifyCommand>);

// pub struct SysLightModifyCommand;
// impl TSystemStageInfo for SysLightModifyCommand {
// }
// #[setup]
// impl SysLightModifyCommand {
//     #[system]
    fn sys_cmd_light_modify(
        mut cmds: ResMut<SingleLightModifyCommands>,
        mut lights: Query<GameObject, (&Light, &LightingMode, &MaterialID, &mut ViewerRenderersInfo, &UniqueName, &ShadowAtlasSize)>,
        light_enable: Query<GameObject, &Enable>,
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

        mut render_graphic: ResMut<RenderGraph>,
        mut entity_cmd: EntityCommands<GameObject>,
    ) {
        let mut list = replace(&mut cmds.0, vec![]);
        list.drain(..).for_each(|cmd| {
            match cmd {
                ELightModifyCommand::LightType(val) => {
                    if let Some((old_val, _, _, _, _, size)) = lights.get(entity) {
                        if *old_val == val {
                            return;
                        }
                    }
                    light_cmd.insert(val);
                },
                ELightModifyCommand::LightingType(val) => {
                    if let Some((_, old_val, _, _, _, size)) = lights.get(entity) {
                        if *old_val == val {
                            return;
                        }
                    }
                    lighting_cmd.insert(val);
                },
                ELightModifyCommand::ShadowMinz(val) => {
                    minz_cmd.insert(ShadowMinZ(val));
                },
                ELightModifyCommand::ShadowMaxz(val) => {
                    maxz_cmd.insert(ShadowMaxZ(val));
                },
                ELightModifyCommand::ShadowFrustumSize(val) => {
                    frustum_cmd.insert(ShadowFrustumSize(val));
                },
                ELightModifyCommand::Directional(val) => {
                    direct_cmd.insert(LightDirection(val));
                },
                ELightModifyCommand::ShadowEnable(val) => {
                    shadowenable_cmd.insert(ShadowEnable(val));

                    if let Some((_, _, id_mat, mut viewer_renderers, name, _)) = lights.get_mut(entity) {
                        material_create_cmd.0.push(
                            EMatCreateCommand::Use(id_mat.0.clone(), Atom::from(ShaderShadowGenerator::KEY), EPassTag::ShadowCast)
                        );

                        let graphic_desc = RendererGraphicDesc {
                            pre: Some(Atom::from("Clear")),
                            curr: name.0.clone(),
                            next: None,
                            passorders: PassTagOrders::new(vec![EPassTag::ShadowCast]),
                        };
                        if let Some((render, id_render)) = viewer_renderers.map.get(&graphic_desc.curr) {
                            entity_cmd.despawn(id_render.0);
                            render_graphic.remove_node(render.curr.to_string());
                        }
                        let id_renderer = entity_cmd.spawn();
                        viewer_renderers.map.insert(graphic_desc.curr.clone(), (graphic_desc.clone(), RendererID(id_renderer)));
                        
                        render_cmds.list.push(
                            ERendererCommand::Active(RendererID(id_renderer), graphic_desc)
                        );
                    }
                },
                ELightModifyCommand::Bias(val) => {
                    bias_cmd.insert(ShadowBias(val));
                },
                ELightModifyCommand::NormalBias(val) => {
                    normal_bias_cmd.insert(ShadowNormalBias(val));
                },
                ELightModifyCommand::DepthScale(val) => {
                    depthscale_cmd.insert(ShadowDepthScale(val));
                },
                ELightModifyCommand::AtlasSize(val) => {
                    shadowsize_cmd.insert(ShadowAtlasSize(val));
                },
            }
        });
    }
// }

// pub struct SysLightModifyEffectRender;
// impl TSystemStageInfo for SysLightModifyEffectRender {
//     fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
//         vec![
//             SysLightModifyCommand::key()
//         ]
//     }
// }
// #[setup]
// impl SysLightModifyEffectRender {
//     #[system]
    fn sys_light_render_modify(
        lights: Query<
            GameObject,
            (
                ObjectID, &Light, &ShadowEnable, &Enable, &ViewerRenderersInfo, &ShadowAtlasSize,
            ),
            Or<(Changed<Light>, Changed<ShadowEnable>, Changed<Enable>, Changed<DirtyViewerRenderersInfo>, Changed<ShadowAtlasSize>, )>
        >,
        mut render_cmds: ResMut<SingleRendererCommandList>,
        mut enable_cmds: Commands<GameObject, Enable>,
        mut viewer_active_cmd: Commands<GameObject, ViewerActive>,
    ) {
        lights.iter().for_each(|(light, shadowenable, enable, renderers, size)| {
            renderers.map.iter().for_each(|(k, v)| {
                let id_render = v.1.0;

                let enable = shadowenable.0 && enable.0;

                log::warn!(">>>>>>>> {:?}", enable);

                viewer_active_cmd.insert(ViewerActive(enable));
                enable_cmds.insert(id_render, Enable(enable));
                if enable {
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
            });
        });
    }
// }

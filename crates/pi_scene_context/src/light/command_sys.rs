
use pi_engine_shell::prelude::*;

use crate::{
    viewer::{prelude::*, command_sys::ActionViewer},
    renderers::{
        prelude::*,
        command_sys::*,
    },
    materials::prelude::*,
    pass::{EPassTag, PassTagOrders},
    transforms::command_sys::*,
    prelude::{GlobalEnable, ActionListDisposeReady, ActionListDisposeCan, OpsDisposeReady, LayerMask},
};

use super::{
    base::{LightDirection, Light, LightingMode},
    point::ShadowAngle,
    shadow_generator::*,
    command::*,
};

pub fn sys_create_light(
    mut cmds: ResMut<ActionListLightCreate>,
    mut commands: Commands,
    mut dynallocator: ResMut<ResBindBufferAllocator> ,
    mut render_graphic: ResMut<PiRenderGraph> ,
    mut matcreatecmds: ResMut<ActionListMaterialCreate>,
    mut matusecmds: ResMut<ActionListMaterialUse>,
    empty: Res<SingleEmptyEntity>,
    mut renderercmds: ResMut<ActionListRendererModify>,
    mut disposereadylist: ResMut<ActionListDisposeReady>,
    mut _disposecanlist: ResMut<ActionListDisposeCan>,
) {
    cmds.drain().drain(..).for_each(|OpsLightCreate(scene, entity)| {
        let mat = commands.spawn_empty().id();
        matcreatecmds.push(OpsMaterialCreate(mat, Atom::from(ShaderShadowGenerator::KEY), EPassTag::ShadowCast));
        matusecmds.push(OpsMaterialUse::ops(entity, mat));

        let id_renderer = entity;

        let mut viewer_renderers = ViewerRenderersInfo::default();
        
        let mut lightcmd = if let Some(cmd) = commands.get_entity(entity) {
            cmd
        } else {
            disposereadylist.push(OpsDisposeReady::ops(entity));
            return;
        };

        ActionLight::init(&mut lightcmd, scene);
        ActionAnime::as_anime_group_target(&mut lightcmd);

        lightcmd
            .insert(MaterialID(empty.id()))
            .insert(RendererID(id_renderer));

        if let Some(bindviewer) = BindViewer::new(&mut dynallocator) {
            lightcmd.insert(bindviewer);
        }
        
        let id_viewer = entity;

        // let graphic_desc = RendererGraphicDesc {
        //     pre: Some(final_render.clear_entity),
        //     curr: name.clone(),
        //     next: None,
        //     passorders: PassTagOrders::new(vec![EPassTag::ShadowCast]),
        // };
        let passorders = PassTagOrders::new(vec![EPassTag::ShadowCast]);

        let render_node = RenderNode::new(id_renderer);
        match render_graphic.add_node(id_renderer.to_bits().to_string(), render_node) {
            Ok(nodeid) => {
                if let Some(mut cmd) = commands.get_entity(id_renderer) {
                    cmd.insert(GraphId(nodeid));

                    // ActionRenderer::init_graphic_node(&mut render_graphic, RendererID(id_renderer), nodeid, Some(final_render.clear_node), None);

                    viewer_renderers.map.insert(entity.to_bits().to_string(), (passorders.clone(), RendererID(id_renderer)));
                    
                    ActionRenderer::init(
                        &mut cmd, id_viewer, passorders, ShadowAtlasSize::DEFAULT, ShadowAtlasSize::DEFAULT,
                        ColorFormat::Rgba32Float, DepthStencilFormat::Depth24PlusStencil8, false
                    );

                    renderercmds.push(OpsRendererCommand::AutoClearColor(id_renderer, true));
                    renderercmds.push(OpsRendererCommand::AutoClearDepth(id_renderer, true));
                    renderercmds.push(OpsRendererCommand::AutoClearStencil(id_renderer, true));
                }
            },
            Err(_) => {
                log::warn!("Light render_graphic Error!");
            },
        }
        commands.get_entity(entity).unwrap().insert(viewer_renderers);
    });
}


pub fn sys_act_light_param(
    mut cmds: ResMut<ActionListLightParam>,
    mut shadowlights: Query<(&mut ShadowMinZ, &mut ShadowMaxZ, &mut ShadowFrustumSize, &mut ShadowBias, &mut ShadowNormalBias, &mut ShadowDepthScale, &mut ShadowAtlasSize, &mut ShadowEnable, &mut ViewerSize)>,
    mut directlights: Query<&mut LightDirection>,
    mut lights: Query<(&mut Light, &mut LightingMode)>,
) {
    cmds.drain().drain(..).for_each(|cmd| {
        match cmd {
            ELightModifyCommand::ShadowMinz(entity, val) => {
                if let Ok(mut item) = shadowlights.get_mut(entity) {
                    *item.0 = ShadowMinZ(val);
                }
            },
            ELightModifyCommand::ShadowMaxz(entity, val) => {
                if let Ok(mut item) = shadowlights.get_mut(entity) {
                    *item.1 = ShadowMaxZ(val);
                }
            },
            ELightModifyCommand::ShadowFrustumSize(entity, val) => {
                if let Ok(mut item) = shadowlights.get_mut(entity) {
                    *item.2 = ShadowFrustumSize(val);
                }
            },
            ELightModifyCommand::Directional(entity, val) => {
                if let Ok(mut item) = directlights.get_mut(entity) {
                    *item = LightDirection(val);
                }
            },
            ELightModifyCommand::Bias(entity, val) => {
                if let Ok(mut item) = shadowlights.get_mut(entity) {
                    *item.3 = ShadowBias(val);
                }
            },
            ELightModifyCommand::NormalBias(entity, val) => {
                if let Ok(mut item) = shadowlights.get_mut(entity) {
                    *item.4 = ShadowNormalBias(val);
                }
            },
            ELightModifyCommand::DepthScale(entity, val) => {
                if let Ok(mut item) = shadowlights.get_mut(entity) {
                    *item.5 = ShadowDepthScale(val);
                }
            },
            ELightModifyCommand::AtlasSize(entity, val) => {
                if let Ok(mut item) = shadowlights.get_mut(entity) {
                    *item.6 = ShadowAtlasSize(val);
                    *item.8 = ViewerSize(val, val);
                }
            },
            ELightModifyCommand::LightType(entity, val) => {
                if let Ok(mut item) = lights.get_mut(entity) {
                    *item.0 = val;
                }
            },
            ELightModifyCommand::LightingType(entity, val) => {
                if let Ok(mut item) = lights.get_mut(entity) {
                    *item.1 = val;
                }
            },
            ELightModifyCommand::ShadowEnable(entity, val) => {
                if let Ok(mut item) = shadowlights.get_mut(entity) {
                    *item.7 = ShadowEnable(val);
                }
            },
        }
    });
}

pub fn sys_light_render_modify(
    mut lights: Query<
        (
            ObjectID, &Light, &ShadowEnable, &GlobalEnable, &ViewerRenderersInfo, &ShadowAtlasSize, &mut ViewerActive
        ),
        Or<(Changed<Light>, Changed<ShadowEnable>, Changed<GlobalEnable>, Changed<ShadowAtlasSize>)>
    >,
    // mut render_cmds: ResMut<SingleRendererCommandList>,
    mut renderercmds: ResMut<ActionListRendererModify>,
) {
    lights.iter_mut().for_each(|(id_light, _light, shadowenable, enable, renderers, _size, mut viewactive)| {

        renderers.map.iter().for_each(|(_, v)| {
            let id_render = v.1.0;

            let enable = shadowenable.0 && enable.0;

            // log::warn!(">>>>>>>> {:?}", enable);

            *viewactive = ViewerActive(enable);

            renderercmds.push(OpsRendererCommand::Active(id_render, enable));
        });
    });
}
pub struct ActionLight;
impl ActionLight {
    pub(crate) fn as_light(
        commands: &mut EntityCommands,
    ) {
        log::warn!("CreateLight {:?}", commands.id());
        commands
            .insert(Light::Directional)
            .insert(LightingMode::Lambert)
            .insert(LightDirection::default())
            .insert(ShadowMinZ::default())
            .insert(ShadowMaxZ::default())
            .insert(ShadowFrustumSize::default())
            .insert(ShadowAngle::default())
            .insert(ShadowEnable(false))
            .insert(ShadowAtlasSize::default())
            .insert(ShadowDepthScale::default())
            .insert(ShadowBias::default())
            .insert(ShadowNormalBias::default())
            .insert(LayerMask::default())
            ;

        ActionViewer::as_viewer(commands);
        commands.insert(ViewerSize(ShadowAtlasSize::DEFAULT, ShadowAtlasSize::DEFAULT));
    }
    pub fn init(
        commands: &mut EntityCommands,
        scene: Entity,
    ) {
        ActionTransformNode::init(commands, scene);
        ActionLight::as_light(commands);
    }

    pub fn modify(
        app: &mut App,
        _light: Entity,
        cmd: ELightModifyCommand,
    ) {
        let mut cmds = app.world.get_resource_mut::<ActionListLightParam>().unwrap();
        cmds.push(cmd);
    }
}


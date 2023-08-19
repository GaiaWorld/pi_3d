
use pi_engine_shell::prelude::*;

use crate::{
    viewer::{prelude::*, command_sys::ActionViewer},
    renderers::{
        prelude::*,
        command_sys::*,
    },
    materials::{
        prelude::*,
        command_sys::*
    },
    pass::{EPassTag, PassTagOrders},
    scene::command_sys::ActionScene,
    transforms::{command_sys::*, prelude::*},
    animation::command_sys::*, prelude::{GlobalEnable, ActionListDisposeReady, ActionListDisposeCan, OpsDisposeReady},
};

use super::{
    base::{LightDirection, Light, LightingMode},
    point::ShadowAngle,
    shadow_generator::{
        base::*,
        ShaderShadowGenerator
    },
    command::*,
};

pub fn sys_create_light(
    mut cmds: ResMut<ActionListLightCreate>,
    mut tree: ResMut<ActionListTransformNodeParent>,
    mut commands: Commands,
    mut dynallocator: ResMut<ResBindBufferAllocator> ,
    mut render_graphic: ResMut<PiRenderGraph> ,
    mut matcreatecmds: ResMut<ActionListMaterialCreate>,
    mut matusecmds: ResMut<ActionListMaterialUse>,
    empty: Res<SingleEmptyEntity>,
    mut renderercmds: ResMut<ActionListRendererModify>,
    mut disposereadylist: ResMut<ActionListDisposeReady>,
    mut disposecanlist: ResMut<ActionListDisposeCan>,
) {
    cmds.drain().drain(..).for_each(|OpsLightCreate(scene, entity, name)| {
        let mat = commands.spawn_empty().id();
        matcreatecmds.push(OpsMaterialCreate(mat, Atom::from(ShaderShadowGenerator::KEY), EPassTag::ShadowCast));
        matusecmds.push(OpsMaterialUse::ops(entity, mat));

        let id_renderer = commands.spawn_empty().id();

        let mut viewer_renderers = ViewerRenderersInfo::default();
        
        let mut lightcmd = if let Some(cmd) = commands.get_entity(entity) {
            cmd
        } else {
            disposereadylist.push(OpsDisposeReady::ops(entity));
            return;
        };

        ActionLight::init(&mut lightcmd, &mut tree, scene, name.clone());
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
        match render_graphic.add_node(name.clone(), render_node) {
            Ok(nodeid) => {
                if let Some(mut cmd) = commands.get_entity(id_renderer) {
                    cmd.insert(GraphId(nodeid));

                    // ActionRenderer::init_graphic_node(&mut render_graphic, RendererID(id_renderer), nodeid, Some(final_render.clear_node), None);

                    viewer_renderers.map.insert(name.clone(), (passorders.clone(), RendererID(id_renderer)));
                    
                    ActionRenderer::init(
                        &mut cmd, id_viewer, passorders, ShadowAtlasSize::DEFAULT, ShadowAtlasSize::DEFAULT,
                        ColorFormat::Rgba32Float, DepthStencilFormat::Depth24PlusStencil8, false
                    );

                    renderercmds.push(OpsRendererCommand::AutoClearColor(id_renderer, true));
                    renderercmds.push(OpsRendererCommand::AutoClearDepth(id_renderer, true));
                    renderercmds.push(OpsRendererCommand::AutoClearStencil(id_renderer, true));
                }
            },
            Err(_) => {},
        }
    });
}


pub fn sys_act_light_param(
    mut cmds: ResMut<ActionListLightParam>,
    mut commands: Commands,
) {
    cmds.drain().drain(..).for_each(|cmd| {
        match cmd {
            ELightModifyCommand::ShadowMinz(entity, val) => {
                if let Some(mut cmd) = commands.get_entity(entity) {
                    cmd.insert(ShadowMinZ(val));
                }
            },
            ELightModifyCommand::ShadowMaxz(entity, val) => {
                if let Some(mut cmd) = commands.get_entity(entity) {
                    cmd.insert(ShadowMaxZ(val));
                }
            },
            ELightModifyCommand::ShadowFrustumSize(entity, val) => {
                if let Some(mut cmd) = commands.get_entity(entity) {
                    cmd.insert(ShadowFrustumSize(val));
                }
            },
            ELightModifyCommand::Directional(entity, val) => {
                if let Some(mut cmd) = commands.get_entity(entity) {
                    cmd.insert(LightDirection(val));
                }
            },
            ELightModifyCommand::Bias(entity, val) => {
                if let Some(mut cmd) = commands.get_entity(entity) {
                    cmd.insert(ShadowBias(val));
                }
            },
            ELightModifyCommand::NormalBias(entity, val) => {
                if let Some(mut cmd) = commands.get_entity(entity) {
                    cmd.insert(ShadowNormalBias(val));
                }
            },
            ELightModifyCommand::DepthScale(entity, val) => {
                if let Some(mut cmd) = commands.get_entity(entity) {
                    cmd.insert(ShadowDepthScale(val));
                }
            },
            ELightModifyCommand::AtlasSize(entity, val) => {
                if let Some(mut cmd) = commands.get_entity(entity) {
                    cmd.insert(ShadowAtlasSize(val)).insert(ViewerSize(val, val));
                }
            },
            ELightModifyCommand::LightType(entity, val) => {
                if let Some(mut cmd) = commands.get_entity(entity) {
                    cmd.insert(val);
                }
            },
            ELightModifyCommand::LightingType(entity, val) => {
                if let Some(mut cmd) = commands.get_entity(entity) {
                    cmd.insert(val);
                }
            },
            ELightModifyCommand::ShadowEnable(entity, val) => {
                if let Some(mut cmd) = commands.get_entity(entity) {
                    cmd.insert(ShadowEnable(val));
                }
            },
        }
    });
}

pub fn sys_light_render_modify(
    lights: Query<
        (
            ObjectID, &Light, &ShadowEnable, &GlobalEnable, &ViewerRenderersInfo, &ShadowAtlasSize,
        ),
        Or<(Changed<Light>, Changed<ShadowEnable>, &GlobalEnable, Changed<ShadowAtlasSize>, )>
    >,
    // mut render_cmds: ResMut<SingleRendererCommandList>,
    mut commands: Commands,
    mut renderercmds: ResMut<ActionListRendererModify>,
) {
    lights.iter().for_each(|(id_light, light, shadowenable, enable, renderers, size)| {
        renderers.map.iter().for_each(|(_, v)| {
            let id_render = v.1.0;

            let enable = shadowenable.0 && enable.0;

            // log::warn!(">>>>>>>> {:?}", enable);

            if let Some(mut cmd) = commands.get_entity(id_light) {
                cmd.insert(ViewerActive(enable));
            }

            renderercmds.push(OpsRendererCommand::Active(id_render, enable));
        });
    });
}
pub struct ActionLight;
impl ActionLight {
    pub(crate) fn as_light(
        commands: &mut EntityCommands,
    ) {
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
            ;

        ActionViewer::as_viewer(commands);
        commands.insert(ViewerSize(ShadowAtlasSize::DEFAULT, ShadowAtlasSize::DEFAULT));
    }
    pub fn init(
        commands: &mut EntityCommands,
        tree: &mut ActionListTransformNodeParent,
        scene: Entity,
        name: String,
    ) {
        ActionTransformNode::init(commands, tree, scene, name);
        ActionLight::as_light(commands);
    }

    pub fn modify(
        app: &mut App,
        light: Entity,
        cmd: ELightModifyCommand,
    ) {
        let mut cmds = app.world.get_resource_mut::<ActionListLightParam>().unwrap();
        cmds.push(cmd);
    }
}


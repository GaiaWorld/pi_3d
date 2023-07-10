
use pi_engine_shell::prelude::*;
use pi_render::{renderer::bind_buffer::BindBufferAllocator};
use pi_scene_math::Vector3;

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
    animation::command_sys::*, prelude::GlobalEnable,
};

use super::{
    base::{LightDirection, Light, LightingMode},
    point::ShadowAngle,
    shadow_generator::{
        base::{ShadowMinZ, ShadowMaxZ, ShadowFrustumSize, ShadowEnable, ShadowBias, ShadowNormalBias, ShadowDepthScale, ShadowAtlasSize, },
        ShaderShadowGenerator
    }
};

pub struct OpsLightCreate(pub(crate) Entity, pub(crate) Entity, pub(crate) String);
impl OpsLightCreate {
    pub fn ops(scene: Entity, light: Entity, name: String) -> Self {
        OpsLightCreate(scene, light, name)
    }
}

pub type ActionListLightCreate = ActionList<OpsLightCreate>;
pub fn sys_act_light_create(
    mut cmds: ResMut<ActionListLightCreate>,
    mut tree: ResMut<ActionListTransformNodeParent>,
    mut commands: Commands,
    mut dynallocator: ResMut<ResBindBufferAllocator> ,
    mut render_graphic: ResMut<PiRenderGraph> ,
    mut matcreatecmds: ResMut<ActionListMaterialCreate>,
    mut matusecmds: ResMut<ActionListMaterialUse>,
    empty: Res<SingleEmptyEntity>,
    mut renderercmds: ResMut<ActionListRendererModify>,
) {
    cmds.drain().drain(..).for_each(|OpsLightCreate(scene, entity, name)| {
        let mat = commands.spawn_empty().id();
        matcreatecmds.push(OpsMaterialCreate(mat, Atom::from(ShaderShadowGenerator::KEY), EPassTag::ShadowCast));
        matusecmds.push(OpsMaterialUse::ops(entity, mat));

        let id_renderer = commands.spawn_empty().id();

        let mut viewer_renderers = ViewerRenderersInfo::default();
        let mut lightcmd = commands.entity(entity);

        ActionScene::add_to_scene(&mut lightcmd, &mut tree, scene);
        ActionTransformNode::init_for_tree(&mut lightcmd);
        ActionLight::as_light(&mut lightcmd, name.clone());

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
                commands.entity(id_renderer).insert(GraphId(nodeid));

                // ActionRenderer::init_graphic_node(&mut render_graphic, RendererID(id_renderer), nodeid, Some(final_render.clear_node), None);

                viewer_renderers.map.insert(name.clone(), (passorders.clone(), RendererID(id_renderer)));
                
                let mut commands = commands.entity(id_renderer);
                ActionRenderer::as_renderer(
                    &mut commands, id_viewer, passorders, ShadowAtlasSize::DEFAULT, ShadowAtlasSize::DEFAULT,
                    ColorFormat::Rgba32Float, DepthStencilFormat::Depth24PlusStencil8, false
                );

                renderercmds.push(OpsRendererCommand::AutoClearColor(id_renderer, true));
                renderercmds.push(OpsRendererCommand::AutoClearDepth(id_renderer, true));
                renderercmds.push(OpsRendererCommand::AutoClearStencil(id_renderer, true));
            },
            Err(_) => {},
        }
    });
}

pub type ActionListLightParam = ActionList<ELightModifyCommand>;
pub fn sys_act_light_param(
    mut cmds: ResMut<ActionListLightParam>,
    mut commands: Commands,
) {
    cmds.drain().drain(..).for_each(|cmd| {
        match cmd {
            ELightModifyCommand::ShadowMinz(entity, val) => {
                commands.entity(entity).insert(ShadowMinZ(val));
            },
            ELightModifyCommand::ShadowMaxz(entity, val) => {
                commands.entity(entity).insert(ShadowMaxZ(val));
            },
            ELightModifyCommand::ShadowFrustumSize(entity, val) => {
                commands.entity(entity).insert(ShadowFrustumSize(val));
            },
            ELightModifyCommand::Directional(entity, val) => {
                commands.entity(entity).insert(LightDirection(val));
            },
            ELightModifyCommand::Bias(entity, val) => {
                commands.entity(entity).insert(ShadowBias(val));
            },
            ELightModifyCommand::NormalBias(entity, val) => {
                commands.entity(entity).insert(ShadowNormalBias(val));
            },
            ELightModifyCommand::DepthScale(entity, val) => {
                commands.entity(entity).insert(ShadowDepthScale(val));
            },
            ELightModifyCommand::AtlasSize(entity, val) => {
                commands.entity(entity).insert(ShadowAtlasSize(val));
                commands.entity(entity).insert(ViewerSize(val, val));
            },
            ELightModifyCommand::LightType(entity, val) => {
                commands.entity(entity).insert(val);
            },
            ELightModifyCommand::LightingType(entity, val) => {
                commands.entity(entity).insert(val);
            },
            ELightModifyCommand::ShadowEnable(entity, val) => {
                commands.entity(entity).insert(ShadowEnable(val));
            },
        }
    });
}


pub struct ActionLight;
impl ActionLight {
    pub(crate) fn as_light(
        commands: &mut EntityCommands,
        name: String,
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

        ActionTransformNode::as_transform_node(commands, name);
        ActionViewer::as_viewer(commands);
        commands.insert(ViewerSize(ShadowAtlasSize::DEFAULT, ShadowAtlasSize::DEFAULT));
        ActionAnime::as_anime_group_target(commands);
    }
    pub fn create(
        app: &mut App,
        scene: Entity,
        name: String,
    ) -> Entity {
        let mut queue = CommandQueue::default();
        let mut commands = Commands::new(&mut queue, &app.world);
        let entity = commands.spawn_empty().id();

        queue.apply(&mut app.world);

        let mut cmds = app.world.get_resource_mut::<ActionListLightCreate>().unwrap();
        cmds.push(OpsLightCreate(scene, entity, name));

        entity
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


pub enum ELightModifyCommand {
    LightType(Entity, Light),
    LightingType(Entity, LightingMode),
    ShadowMinz(Entity, f32),
    ShadowMaxz(Entity, f32),
    ShadowFrustumSize(Entity, f32),
    Directional(Entity, Vector3),
    Bias(Entity, f32),
    NormalBias(Entity, f32),
    DepthScale(Entity, f32),
    AtlasSize(Entity, u32),
    ShadowEnable(Entity, bool),
}

// #[derive(Default)]
// pub struct SingleLightModifyCommands(pub Vec<ELightModifyCommand>);

// pub struct SysLightModifyCommand;
// impl TSystemStageInfo for SysLightModifyCommand {
// }
// #[setup]
// impl SysLightModifyCommand {
//     #[system]
    // fn sys_cmd_light_modify(
    //     mut cmds: ResMut<SingleLightModifyCommands>,
    //     mut lights: Query<GameObject, (&Light, &LightingMode, &MaterialID, &mut ViewerRenderersInfo, &UniqueName, &ShadowAtlasSize)>,
    //     light_enable: Query<GameObject, &Enable>,
    //     shadowangles: Query<GameObject, &ShadowAngle>,
    //     mut light_cmd: Commands<GameObject, Light>,
    //     mut lighting_cmd: Commands<GameObject, LightingMode>,
    //     mut minz_cmd: Commands<GameObject, ShadowMinZ>,
    //     mut maxz_cmd: Commands<GameObject, ShadowMaxZ>,
    //     mut frustum_cmd: Commands<GameObject, ShadowFrustumSize>,
    //     mut direct_cmd: Commands<GameObject, LightDirection>,
    //     mut shadowangle_cmd: Commands<GameObject, ShadowAngle>,
    //     mut shadowenable_cmd: Commands<GameObject, ShadowEnable>,
    //     mut bias_cmd: Commands<GameObject, ShadowBias>,
    //     mut normal_bias_cmd: Commands<GameObject, ShadowNormalBias>,
    //     mut depthscale_cmd: Commands<GameObject, ShadowDepthScale>,
    //     mut shadowsize_cmd: Commands<GameObject, ShadowAtlasSize>,

    //     mut rendersize_cmd: Commands<GameObject, RenderSize>,
    //     mut viewer_active_cmd: Commands<GameObject, ViewerActive>,
    //     mut material_create_cmd: ResMut<SingleMatCreateCommands>,
        
    //     mut render_cmds: ResMut<SingleRendererCommandList>,

    //     mut render_graphic: ResMut<RenderGraph>,
    //     mut entity_cmd: EntityCommands<GameObject>,
    // ) {
    //     let mut list = replace(&mut cmds.0, vec![]);
    //     list.drain(..).for_each(|cmd| {
    //         match cmd {
    //             ELightModifyCommand::LightType(val) => {
    //                 if let Some((old_val, _, _, _, _, size)) = lights.get(entity) {
    //                     if *old_val == val {
    //                         return;
    //                     }
    //                 }
    //                 light_cmd.insert(val);
    //             },
    //             ELightModifyCommand::LightingType(val) => {
    //                 if let Some((_, old_val, _, _, _, size)) = lights.get(entity) {
    //                     if *old_val == val {
    //                         return;
    //                     }
    //                 }
    //                 lighting_cmd.insert(val);
    //             },
    //             ELightModifyCommand::ShadowMinz(val) => {
    //                 minz_cmd.insert(ShadowMinZ(val));
    //             },
    //             ELightModifyCommand::ShadowMaxz(val) => {
    //                 maxz_cmd.insert(ShadowMaxZ(val));
    //             },
    //             ELightModifyCommand::ShadowFrustumSize(val) => {
    //                 frustum_cmd.insert(ShadowFrustumSize(val));
    //             },
    //             ELightModifyCommand::Directional(val) => {
    //                 direct_cmd.insert(LightDirection(val));
    //             },
    //             ELightModifyCommand::ShadowEnable(val) => {
    //                 shadowenable_cmd.insert(ShadowEnable(val));

    //                 if let Some((_, _, id_mat, mut viewer_renderers, name, _)) = lights.get_mut(entity) {
    //                     material_create_cmd.0.push(
    //                         EMatCreateCommand::Use(id_mat.0.clone(), Atom::from(ShaderShadowGenerator::KEY), EPassTag::ShadowCast)
    //                     );

    //                     let graphic_desc = RendererGraphicDesc {
    //                         pre: Some(Atom::from("Clear")),
    //                         curr: name.0.clone(),
    //                         next: None,
    //                         passorders: PassTagOrders::new(vec![EPassTag::ShadowCast]),
    //                     };
    //                     if let Some((render, id_render)) = viewer_renderers.map.get(&graphic_desc.curr) {
    //                         entity_cmd.despawn(id_render.0);
    //                         render_graphic.remove_node(render.curr.to_string());
    //                     }
    //                     let id_renderer = entity_cmd.spawn();
    //                     viewer_renderers.map.insert(graphic_desc.curr.clone(), (graphic_desc.clone(), RendererID(id_renderer)));
                        
    //                     render_cmds.list.push(
    //                         ERendererCommand::Active(RendererID(id_renderer), graphic_desc)
    //                     );
    //                 }
    //             },
    //             ELightModifyCommand::Bias(val) => {
    //                 bias_cmd.insert(ShadowBias(val));
    //             },
    //             ELightModifyCommand::NormalBias(val) => {
    //                 normal_bias_cmd.insert(ShadowNormalBias(val));
    //             },
    //             ELightModifyCommand::DepthScale(val) => {
    //                 depthscale_cmd.insert(ShadowDepthScale(val));
    //             },
    //             ELightModifyCommand::AtlasSize(val) => {
    //                 shadowsize_cmd.insert(ShadowAtlasSize(val));
    //             },
    //         }
    //     });
    // }
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

                let mut lightcmd = commands.entity(id_light);
                lightcmd.insert(ViewerActive(enable));

                renderercmds.push(OpsRendererCommand::Active(id_render, enable));
            });
        });
    }
// }

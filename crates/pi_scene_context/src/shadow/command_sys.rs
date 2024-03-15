use pi_scene_shell::prelude::*;

use crate::{
    light::prelude::{DirectLight, SpotLight},
    materials::prelude::*,
    renderers::prelude::*,
    viewer::prelude::*,
    flags::GlobalEnable,
    layer_mask::prelude::*,
    scene::prelude::*,
};

use super::{
    base::*,
    command::*,
    shader::ShaderShadowGenerator, direct_light::{DirectionalShadowDirection, DirectionalShadowProjection, SpotShadowProjection}
};


pub fn sys_create_shadow_generator(
    mut commands: Commands,
    mut cmds: ResMut<ActionListShadowGenerator>,
    mut direct_lights: Query<(&SceneID, &GlobalEnable, &mut LightLinkedShadowID, Option<&DirectLight>, Option<&SpotLight>, &LayerMask, &ViewerDistanceCompute), Or<(With<DirectLight>, With<SpotLight>)>>,
    mut scene_shadow: Query<&mut SceneShadowQueue>,
    mut dynallocator: ResMut<ResBindBufferAllocator> ,
    mut matcreatecmds: ResMut<ActionListMaterialCreate>,
    mut matusecmds: ResMut<ActionListMaterialUse>,
    empty: Res<SingleEmptyEntity>,
    mut disposereadylist: ResMut<ActionListDisposeReadyForRef>,
    mut _disposecanlist: ResMut<ActionListDisposeCan>,
) {
    cmds.drain().drain(..).for_each(|OpsShadowGenerator(entity, scene, light, passtag)| {
        if let (Ok(mut queueshadow), Ok((idscene, enabled, mut linkedshadow, isdirect, issopt, layermask, viewerdistance))) = (scene_shadow.get_mut(scene), direct_lights.get_mut(light)) {
            let mat = commands.spawn_empty().id();

            let mut shadowcommands = if let Some(cmd) = commands.get_entity(entity) { cmd } else {
                disposereadylist.push(OpsDisposeReadyForRef::ops(entity)); commands.entity(mat).despawn(); return;
            };

                matcreatecmds.push(OpsMaterialCreate::ops(mat, ShaderShadowGenerator::KEY));
                matusecmds.push(OpsMaterialUse::ops(entity, mat, passtag));
    
                ActionShadow::as_shadow_generator(&mut shadowcommands, idscene.0, enabled.0);
                shadowcommands
                    .insert(LinkedMaterialID(empty.id()))
                    .insert(RendererID(entity))
                    .insert(ShadowLayerMask(layermask.clone()))
                    .insert(queueshadow.0.add(entity))
                    .insert(ShadowCastPassTag(passtag))
                    .insert(viewerdistance)
                    ;
                
                if isdirect.is_some() {
                    linkedshadow.0 = Some(entity);
                    shadowcommands.insert(ShadowLinkedLightID(light));
                    shadowcommands.insert(DirectionalShadowDirection::default());
                    shadowcommands.insert(DirectionalShadowProjection::default());
                }
                if issopt.is_some() {
                    linkedshadow.0 = Some(entity);
                    shadowcommands.insert(ShadowLinkedLightID(light));
                    shadowcommands.insert(DirectionalShadowDirection::default());
                    shadowcommands.insert(SpotShadowProjection::default());
                }
    
                if let Some(bindviewer) = BindViewer::new(&mut dynallocator) { shadowcommands.insert(bindviewer); }
            }
    });
}

pub fn sys_act_shadow_generator(
    mut cmds: ResMut<ActionListShadowGeneratorParam>,
    // mut atlassize: Query< &mut ShadowAtlasSize>,
    mut bias: Query<&mut ShadowBias>,
    mut normalbias: Query<&mut ShadowNormalBias>,
    mut depthscale: Query<&mut ShadowDepthScale>,
    mut minz: Query<&mut ShadowMinZ>,
    mut maxz: Query<&mut ShadowMaxZ>,
    mut frusrumsize: Query<&mut ShadowFrustumSize>,
) {
    cmds.drain().drain(..).for_each(|cmd| {
        match cmd {
            OpsShadowGeneratorParam::ShadowMinz(entity, val) => {
                if let Ok(mut item) = minz.get_mut(entity) {
                    *item = ShadowMinZ(val);
                } else {
                    cmds.push(cmd);
                }
            },
            OpsShadowGeneratorParam::ShadowMaxz(entity, val) => {
                if let Ok(mut item) = maxz.get_mut(entity) {
                    *item = ShadowMaxZ(val);
                } else {
                    cmds.push(cmd);
                }
            },
            OpsShadowGeneratorParam::ShadowFrustumSize(entity, val) => {
                if let Ok(mut item) = frusrumsize.get_mut(entity) {
                    *item = ShadowFrustumSize(val);
                } else {
                    cmds.push(cmd);
                }
            },
            OpsShadowGeneratorParam::Bias(entity, val) => {
                if let Ok(mut item) = bias.get_mut(entity) {
                    *item = ShadowBias(val);
                } else {
                    cmds.push(cmd);
                }
            },
            OpsShadowGeneratorParam::NormalBias(entity, val) => {
                if let Ok(mut item) = normalbias.get_mut(entity) {
                    *item = ShadowNormalBias(val);
                } else {
                    cmds.push(cmd);
                }
            },
            OpsShadowGeneratorParam::DepthScale(entity, val) => {
                if let Ok(mut item) = depthscale.get_mut(entity) {
                    *item = ShadowDepthScale(val);
                } else {
                    cmds.push(cmd);
                }
            },
            // OpsShadowGeneratorParam::AtlasSize(entity, val) => {
            //     if let Ok(mut item) = atlassize.get_mut(entity) {
            //         *item = ShadowAtlasSize(val);
            //     } else {
            //         cmds.push(cmd);
            //     }
            // },
            // OpsShadowGeneratorParam::ShadowEnable(entity, val) => {
            //     // if let Ok(mut item) = items.get_mut(entity) {
            //     //     *item.0 = val;
            //     // }
            // },
        }
    });
}

// pub fn sys_act_light_render_modify(
//     mut lights: Query<
//         (
//             ObjectID, &ViewerRenderersInfo, &ShadowAtlasSize, &mut ViewerActive
//         ),
//         Changed<ShadowAtlasSize>
//     >,
//     // mut render_cmds: ResMut<SingleRendererCommandList>,
//     mut renderercmds: ResMut<ActionListRendererModify>,
// ) {
//     lights.iter_mut().for_each(|(id_light, renderers, _size, mut viewactive)| {

//         renderers.map.iter().for_each(|(_, v)| {
//             let id_render = v.1.0;

//             // let enable = enable.0;

//             // log::warn!(">>>>>>>> {:?}", enable);

//             // *viewactive = ViewerActive(enable);

//             // renderercmds.push(OpsRendererCommand::Active(id_render, enable));
//         });
//     });
// }
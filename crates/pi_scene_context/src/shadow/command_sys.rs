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
    directlights: Query<(), With<DirectLight>>,
    spotlights: Query<(), With<SpotLight>>,
    mut lights: Query<(&SceneID, &GlobalEnable, &mut LightLinkedShadowID, &LayerMask, &ViewerDistanceCompute), ()>,
    mut scene_shadow: Query<&mut SceneShadowQueue>,
    mut dynallocator: ResMut<ResBindBufferAllocator> ,
    mut matcreatecmds: ResMut<ActionListMaterialCreate>,
    mut matusecmds: ResMut<ActionListMaterialUse>,
    empty: Res<SingleEmptyEntity>,
    mut disposereadylist: ResMut<ActionListDisposeReadyForRef>,
    mut _disposecanlist: ResMut<ActionListDisposeCan>,
) {
    cmds.drain().drain(..).for_each(|OpsShadowGenerator(entity, scene, light, passtag)| {
        if let (Ok(mut queueshadow), Ok((idscene, enabled, mut linkedshadow, layermask, viewerdistance))) = (scene_shadow.get_mut(scene), lights.get_mut(light)) {
            let mat = commands.spawn_empty().id();

            let mut shadowcommands = if let Some(cmd) = commands.get_entity(entity) { cmd } else {
                disposereadylist.push(OpsDisposeReadyForRef::ops(entity)); commands.entity(mat).despawn(); return;
            };
            
            if let Some(bindviewer) = BindViewer::new(&mut dynallocator) {
                matcreatecmds.push(OpsMaterialCreate::ops(mat, ShaderShadowGenerator::KEY));
                matusecmds.push(OpsMaterialUse::ops(entity, mat, passtag));
                
                if directlights.contains(light) {
                    linkedshadow.0 = Some(entity);
                    
                    shadowcommands.insert((
                        (
                            ActionShadow::as_shadow_generator(idscene.0, enabled.0),
                            LinkedMaterialID(empty.id()),
                            RendererID(entity),
                            ShadowLayerMask(layermask.clone()),
                            queueshadow.0.add(entity),
                            ShadowCastPassTag(passtag),
                            viewerdistance.clone(),
                            bindviewer,
                        ),
                        ShadowLinkedLightID(light),
                        DirectionalShadowDirection::default(),
                        DirectionalShadowProjection::default(),
                    ));
                } else if spotlights.contains(light) {
                    linkedshadow.0 = Some(entity);
                    shadowcommands.insert((
                        (
                            ActionShadow::as_shadow_generator(idscene.0, enabled.0),
                            LinkedMaterialID(empty.id()),
                            RendererID(entity),
                            ShadowLayerMask(layermask.clone()),
                            queueshadow.0.add(entity),
                            ShadowCastPassTag(passtag),
                            viewerdistance.clone(),
                            bindviewer,
                        ),
                        ShadowLinkedLightID(light),
                        DirectionalShadowDirection::default(),
                        SpotShadowProjection::default(),
                    ));
                }
            }
        }
    });
}

pub fn sys_act_shadow_generator(
    mut cmds: ResMut<ActionListShadowGeneratorParam>,
    // mut atlassize: Query< &mut ShadowAtlasSize>,
    mut shadow: Query<&mut ShadowParam>,
) {
    cmds.drain().drain(..).for_each(|cmd| {
        match cmd {
            OpsShadowGeneratorParam::ShadowMinz(entity, val) => {
                if let Ok(mut item) = shadow.get_mut(entity) {
                    item.minz = val;
                } else {
                    cmds.push(cmd);
                }
            },
            OpsShadowGeneratorParam::ShadowMaxz(entity, val) => {
                if let Ok(mut item) = shadow.get_mut(entity) {
                    item.maxz = val;
                } else {
                    cmds.push(cmd);
                }
            },
            OpsShadowGeneratorParam::ShadowFrustumSize(entity, val) => {
                if let Ok(mut item) = shadow.get_mut(entity) {
                    item.frustum = val;
                } else {
                    cmds.push(cmd);
                }
            },
            OpsShadowGeneratorParam::Bias(entity, val) => {
                if let Ok(mut item) = shadow.get_mut(entity) {
                    item.bias = val;
                } else {
                    cmds.push(cmd);
                }
            },
            OpsShadowGeneratorParam::NormalBias(entity, val) => {
                if let Ok(mut item) = shadow.get_mut(entity) {
                    item.normalbias  = val;
                } else {
                    cmds.push(cmd);
                }
            },
            OpsShadowGeneratorParam::DepthScale(entity, val) => {
                if let Ok(mut item) = shadow.get_mut(entity) {
                    item.depthscale = val;
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

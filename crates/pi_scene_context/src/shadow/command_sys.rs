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
    // mut commands: Commands,
    mut insert: Insert<()>,
    mut alter1: Alter<(), (), (ShadowParam, ShadowAngle), ()>,
    mut alter2: Alter<(), (), (DisposeReady, DisposeCan), ()>,
    mut alter3: Alter<(),(),(SceneID, ), ()>,
    mut alter4: Alter<(), (), ActionViewerBundle>,
    mut alter5: Alter<(), (), (LinkedMaterialID, RendererID, ShadowLayerMask, SceneItemIndex, ShadowCastPassTag, ViewerDistanceCompute)>,
    mut alter6: Alter<(), (), (ShadowLinkedLightID, DirectionalShadowDirection, DirectionalShadowProjection)>,
    mut alter7: Alter<(), (), (ShadowLinkedLightID, DirectionalShadowDirection, SpotShadowProjection)>,
    mut alter8: Alter<(), (), (BindViewer,)>,
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
            let mat = insert.insert(());

            let mut shadowcommands = if alter1.get(entity).is_err() {
                disposereadylist.push(OpsDisposeReadyForRef::ops(entity)); alter1.destroy(mat); return;
            };

                matcreatecmds.push(OpsMaterialCreate::ops(mat, ShaderShadowGenerator::KEY));
                matusecmds.push(OpsMaterialUse::ops(entity, mat, passtag));
    
                ActionShadow::as_shadow_generator(entity, &mut alter1, &mut alter2,&mut alter3, &mut alter4,idscene.0, enabled.0);
                alter5.alter(entity, (
                    LinkedMaterialID(empty.id()),
                    RendererID(entity),
                    ShadowLayerMask(layermask.clone()),
                    queueshadow.0.add(entity),
                    ShadowCastPassTag(passtag),
                    viewerdistance.clone())
                );
     
                if isdirect.is_some() {
                    linkedshadow.0 = Some(entity);
                    alter6.alter(entity, 
                    (
                        ShadowLinkedLightID(light),
                        DirectionalShadowDirection::default(),
                        DirectionalShadowProjection::default(),
                    ));
                    
                }
                if issopt.is_some() {
                    linkedshadow.0 = Some(entity);
                    alter7.alter(entity, 
                        (
                        ShadowLinkedLightID(light),
                        DirectionalShadowDirection::default(),
                        SpotShadowProjection::default(),
                    ));
                }
    
                if let Some(bindviewer) = BindViewer::new(&mut dynallocator) { alter8.alter(entity, (bindviewer,)); }
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

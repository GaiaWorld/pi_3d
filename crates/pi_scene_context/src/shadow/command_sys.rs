use pi_scene_shell::{add_component, prelude::{pi_world::editor::EntityEditor, *}};

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
    mut editor: EntityEditor,
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
            let mat = editor.alloc_entity();

            if !editor.contains_entity(entity) {
                disposereadylist.push(OpsDisposeReadyForRef::ops(entity)); let _ = editor.destroy(mat); return;
            };

                matcreatecmds.push(OpsMaterialCreate::ops(mat, ShaderShadowGenerator::KEY));
                // println!("sys_create_shadow_generator: {:?}", (entity, passtag));
                matusecmds.push(OpsMaterialUse::ops(entity, mat, passtag));
    
                ActionShadow::as_shadow_generator(entity, &mut editor, idscene.0, enabled.0);
                let components  = [
                    editor.init_component::<LinkedMaterialID>(),
                    editor.init_component::<RendererID>(),
                    editor.init_component::<ShadowLayerMask>(),
                    editor.init_component::<SceneItemIndex>(),
                    editor.init_component::<ShadowCastPassTag>(),
                    editor.init_component::<ViewerDistanceCompute>(),
                ];
                editor.add_components(entity, &components).unwrap();
               
       
                *editor.get_component_unchecked_mut_by_id(entity, components[0]) = LinkedMaterialID(empty.id());
                *editor.get_component_unchecked_mut_by_id(entity, components[1]) = RendererID(entity);
                *editor.get_component_unchecked_mut_by_id(entity, components[2]) = ShadowLayerMask(layermask.clone());
                *editor.get_component_unchecked_mut_by_id(entity, components[3]) = queueshadow.0.add(entity);
                *editor.get_component_unchecked_mut_by_id(entity, components[4]) = ShadowCastPassTag(passtag);
                *editor.get_component_unchecked_mut_by_id(entity, components[5]) = viewerdistance.clone();
             
     
                if isdirect.is_some() {
                    linkedshadow.0 = Some(entity);
                    log::warn!("sys_create_shadow_generator 2");
                    let components  = [
                        editor.init_component::<ShadowLinkedLightID>(),
                        editor.init_component::<DirectionalShadowDirection>(),
                        editor.init_component::<DirectionalShadowProjection>(),
                    ];
                    editor.add_components(entity, &components).unwrap();

                   
                    *editor.get_component_unchecked_mut_by_id(entity, components[0]) = ShadowLinkedLightID(light);
                    // *editor.get_component_unchecked_mut_by_id(entity, components[1]) = DirectionalShadowDirection::default();
                    // *editor.get_component_unchecked_mut_by_id(entity, components[2]) = DirectionalShadowProjection::default();
                  
                }
                if issopt.is_some() {
                    log::warn!("sys_create_shadow_generator 3");
                    linkedshadow.0 = Some(entity);
                    let components  = [
                        editor.init_component::<ShadowLinkedLightID>(),
                        editor.init_component::<DirectionalShadowDirection>(),
                        editor.init_component::<DirectionalShadowProjection>(),
                    ];
                    editor.add_components(entity, &components).unwrap();

                    *editor.get_component_unchecked_mut_by_id(entity, components[0]) = ShadowLinkedLightID(light);
                    // *editor.get_component_unchecked_mut_by_id(entity, components[1]) = DirectionalShadowDirection::default();
                    // *editor.get_component_unchecked_mut_by_id(entity, components[2]) = SpotShadowProjection::default();
                    
                }
    
                if let Some(bindviewer) = BindViewer::new(&mut dynallocator) { add_component(&mut editor, entity, bindviewer).unwrap(); /* alter8.alter(entity, (bindviewer,)); */ }
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

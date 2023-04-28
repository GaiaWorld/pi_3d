use std::{sync::Arc, time::Instant};

use pi_assets::{mgr::AssetMgr, asset::Handle};
use pi_engine_shell::prelude::*;
use pi_share::Share;

use crate::{
    flags::SceneID,
    scene::{
        environment::{BindSceneEffect,},
    },
    viewer::{BindViewer, ModelList, ViewerActive, FlagModelList},
    skeleton::{skeleton::{BindSkinValue}, SkeletonID},
    meshes::model::BindModel,
    pass::*,
    bindgroup::*,
};

use super::{ViewerRenderersInfo, DirtyViewerRenderersInfo};

    pub fn sys_bind_buffer_apply(
        mut allocator: ResMut<ResBindBufferAllocator>,
        mut vb_allocator: ResMut<VertexBufferAllocator3D>,
        device: Res<PiRenderDevice>,
        queue: Res<PiRenderQueue>,
    ) {
        let time1 = Instant::now();

        allocator.write_buffer(&device, &queue);
        vb_allocator.update_buffer(&device, &queue);

        log::debug!("SysDynBufferAllocatorUpdate: {:?}", Instant::now() - time1);
    }

/// * 视口的数据变化时, 重新创建列表内物体相关Pass 的 Set0 数据
///   * 渲染ID变化 - 变为渲染视口
///   * 渲染列表变化 - 有新物体
///   * 渲染PassTags 变化 - 渲染过程变化
    pub fn sys_set0_modify_by_renderer<T: TPass + Component, I: TPassID + Component>(
        viewers: Query<
            (ObjectID, &ViewerActive, &SceneID, &BindViewer, &ModelList, &ViewerRenderersInfo),
            Or<(Changed<BindViewer>, Changed<ViewerActive>, Changed<FlagModelList>, Changed<DirtyViewerRenderersInfo>)>,
        >,
        renderers: Query<
            &PassTagOrders,
        >,
        scenes: Query<
            &BindSceneEffect,
        >,
        models: Query<&I>,
        device: Res<PiRenderDevice>,
        mut scene_wait: ResMut<AssetBindGroupSceneWaits>,
        mut binds_recorder: ResMut<ResBindsRecorder>,
    ) {
        let time1 = Instant::now();

        viewers.iter().for_each(|(
            id_viewer, active, id_scene, bind_viewer, list_model, list_renderer
        )| {
            // log::info!("Set0ByViewer {:?}:", active.0);
            if !active.0 {
                return;
            }
            if active.0 {
                // log::trace!("SysSet0ModifyByRendererID: {:?}", list_model.0.len());
                // log::info!("Set0ByViewer :");

                if let Ok(bind_base_effect) = scenes.get(id_scene.0) {
                    // log::info!("Set0ByViewer : 0");
                    let key = KeyBindGroupScene::new(bind_viewer.0.clone(), Some(bind_base_effect.0.clone()), &mut binds_recorder);
                    list_renderer.map.iter().for_each(|(_, (desc, id_renderer))| {
                        let pass_tags = &desc.passorders;
                        
                        // log::info!("Set0ByViewer : 1");
                        list_model.0.iter().for_each(|(id_obj, _)| {
                            // log::info!("Set0ByViewer : 2");
                            if let Ok(passid) = models.get(id_obj.clone()) {
                                // log::info!("Set0ByViewer : 3");
                                if pass_tags.1 & I::TAG == I::TAG {
                                    // log::info!("Set0ByViewer : 4");
                                    scene_wait.add(&key, passid.id());
                                }
                            }
                        });
                    });
                }
            }
        });

        log::trace!("SysSet0ModifyByRendererID: {:?}", Instant::now() - time1);
    }

/// * 场景的数据变化时, 重新创建视口列表内物体相关Pass 的 Set0 数据
///   * BindSceneEffect 变化
    pub fn sys_set0_modify_by_scene<T: TPass + Component, I: TPassID + Component>(
        renderers: Query<
            &PassTagOrders,
        >,
        viewers: Query<
            (ObjectID, &ViewerActive, &SceneID, &BindViewer, &ModelList, &ViewerRenderersInfo),
        >,
        scenes: Query<
            (ObjectID, &BindSceneEffect),
            Changed<BindSceneEffect>
        >,
        models: Query<&I>,
        device: Res<PiRenderDevice>,
        mut scene_wait: ResMut<AssetBindGroupSceneWaits>,
        mut binds_recorder: ResMut<ResBindsRecorder>,
    ) {
        let time1 = Instant::now();

        scenes.iter().for_each(|(id_scene_obj, bind_base_effect)| {
            viewers.iter().for_each(|(
                id_camera, active, id_scene, bind_viewer, list_model, list_renderer
            )| {
                // log::info!("Set0ByScene :");

                // log::trace!("SysSet0ModifyFromScene ||||||||||||||||||||||");
                if active.0 && id_scene_obj == id_scene.0 {
                    // log::info!("Set0ByScene : 0");
                    let key = KeyBindGroupScene::new(bind_viewer.0.clone(), Some(bind_base_effect.0.clone()), &mut binds_recorder);
                    list_renderer.map.iter().for_each(|(_, (desc, id_renderer))| {
                        // log::info!("Set0ByScene : 1");
                        let pass_tags = &desc.passorders;
                        list_model.0.iter().for_each(|(id_obj, _)| {
                            // log::info!("Set0ByScene : 2");
                            if let Ok(passid) = models.get(id_obj.clone()) {
                                // log::info!("Set0ByScene : 3");
                                if pass_tags.1 & I::TAG == I::TAG {
                                    // log::info!("Set0ByScene : 4");
                                    scene_wait.add(&key, passid.id());
                                }
                            }
                        });
                    });
                }
            });
        });

        log::trace!("SysSet0ModifyFromScene: {:?}", Instant::now() - time1);
    }

/// * 视口的数据变化时, 重新创建列表内物体相关Pass 的 Set1 数据
///   * 渲染ID变化
///   * 渲染列表变化
///   * 渲染PassTags 变化
    pub fn sys_set1_modify_by_renderer<T: TPass + Component, I: TPassID + Component>(
        viewers: Query<
            (ObjectID, &ViewerActive, &SceneID, &ModelList, &ViewerRenderersInfo),
            Or<(Changed<ModelList>, Changed<ViewerActive>, Changed<ViewerRenderersInfo>)>,
        >,
        renderers: Query<
            &PassTagOrders,
        >,
        models: Query<
            (
                &BindModel, &BindSkinValue, Option<&SkeletonID>,
                &I,
            ),
        >,
        passes: Query<(&PassBindEffectValue, &PassReady), With<T>>,
        device: Res<PiRenderDevice>,
        mut model_wait: ResMut<AssetBindGroupModelWaits>,
        mut binds_recorder: ResMut<ResBindsRecorder>,
    ) {
        let time1 = Instant::now();

        viewers.iter().for_each(|(
            id_camera, active, id_scene, list_model, list_renderer
        )| {
            if active.0 == false {
                return;
            }
            // log::info!("SysSet1ModifyByRendererID: {:?}", list_model.0.len());
            
            list_renderer.map.iter().for_each(|(_, (desc, id_renderer))| {
                // log::info!("SysSet1ModifyByRendererID: 1111111111111111");
                let pass_tags = &desc.passorders;
                // log::info!("SysSet1ModifyByRendererID: 2222222222222222");
                list_model.0.iter().for_each(|(id_obj, _)| {
                    if let Ok(
                        (
                            bind_model, bind_skl, id_skl,
                            passid,
                        )
                    ) = models.get(id_obj.clone()) {
                        let bind_skin = match (&bind_skl.0, id_skl) {
                            (None, None) => { None },
                            (Some(bind_skin), Some(_)) => { Some(bind_skin.clone()) },
                            _ => { return; }
                        };
                        if pass_tags.1 & I::TAG == I::TAG {
                            if let Ok((val1, ready)) = passes.get(passid.id()) {
                                if ready.val().is_some() {
                                    let key = KeyBindGroupModel::new(bind_model.0.clone(), bind_skin.clone(), val1.0.clone(), &mut binds_recorder);
                                    model_wait.add(&key, passid.id());
                                }
                            }
                        }
                    }
                });
            });
        });

        log::trace!("SysSet1ModifyByRendererID: {:?}", Instant::now() - time1);
    }
// }


/// * 物体的数据变化时, 重新创建列表内物体相关Pass 的 Set1 数据
///   * 骨骼数据变化 - 
///   * 渲染效果数据 变化
    pub fn sys_set1_modify_by_model<T: TPass + Component, I: TPassID + Component>(
        renderers: Query<
            &PassTagOrders,
        >,
        viewers: Query<
            (ObjectID, &ViewerActive, &SceneID, &BindViewer, &ModelList, &ViewerRenderersInfo),
        >,
        models: Query<
            (
                &BindModel, &BindSkinValue, Option<&SkeletonID>,
                &I,
            ),
            Or<(
                Changed<BindSkinValue>, Changed<SkeletonID>
            )>,
        >,
        passes: Query<(&PassBindEffectValue, &PassReady), With<T>>,
        // mut pass01_cmd: Commands<GameObject, PassBindGroupModel>,
        device: Res<PiRenderDevice>,
        mut model_wait: ResMut<AssetBindGroupModelWaits>,
        mut binds_recorder: ResMut<ResBindsRecorder>,
    ) {
        let time1 = Instant::now();

        viewers.iter().for_each(|(
            id_camera, active, id_scene, bind_viewer, list_model, list_renderer
        )| {
            if active.0 == false {
                return;
            }
            // log::info!("SysSet1ModifyByModel: {:?}", list_model.0.len());
            list_renderer.map.iter().for_each(|(_, (desc, id_renderer))| {
                let pass_tags = &desc.passorders;
                // log::info!("SysSet1ModifyByModel: 1111111111111111111111");
                list_model.0.iter().for_each(|(id_obj, _)| {
                    // log::info!("SysSet1ModifyByModel: 111111111");
                    if let Ok(
                        (
                            bind_model, bind_skl, id_skl,
                            passid,
                        )
                    ) = models.get(id_obj.clone()) {
                        // let bind_skl: &BindSkinValue = None;
                        // let id_skl: Option<&SkeletonID> = None;
                        // log::info!("SysSet1ModifyByModel: 22222222222222222222222222");
                        let bind_skin = match (&bind_skl.0, id_skl) {
                            (None, None) => { None },
                            (Some(bind_skin), Some(_)) => { Some(bind_skin.clone()) },
                            _ => { return; }
                        };
                        if pass_tags.1 & I::TAG == I::TAG {
                            if let Ok((val1, ready)) = passes.get(passid.id()) {
                                if ready.val().is_some() {
                                    let key = KeyBindGroupModel::new(bind_model.0.clone(), bind_skin.clone(), val1.0.clone(), &mut binds_recorder);
                                    model_wait.add(&key, passid.id());
                                }
                            }
                        }
                    }
                });
            });
        });

        log::trace!("SysSet1ModifyByModel: {:?}", Instant::now() - time1);
    }
// }

/// * 物体的数据变化时, 重新创建列表内物体相关Pass 的 Set1 数据
///   * 骨骼数据变化 - 
///   * 渲染效果数据 变化
    pub fn sys_set1_modify_by_pass<T: TPass + Component, I: TPassID + Component>(
        renderers: Query<
            &PassTagOrders,
        >,
        viewers: Query<
            (ObjectID, &ViewerActive, &SceneID, &BindViewer, &ModelList, &ViewerRenderersInfo),
        >,
        models: Query<
            (
                &BindModel, &BindSkinValue, Option<&SkeletonID>,
                &I,
            ),
        >,
        passes: Query<(&PassBindEffectValue, &PassReady, &T, &PassSource), Changed<PassReady>>,
        device: Res<PiRenderDevice>,
        mut model_wait: ResMut<AssetBindGroupModelWaits>,
        mut binds_recorder: ResMut<ResBindsRecorder>,
    ) {
        let time1 = Instant::now();

        passes.iter().for_each(|(val1, ready, _, id_model)| {
            viewers.iter().for_each(|(
                id_camera, active, id_scene, bind_viewer, list_model, list_renderer
            )| {
                if active.0 == false {
                    return;
                }
                // log::info!("SysSet1ModifyByPass: {:?}", list_model.0.len());
                list_renderer.map.iter().for_each(|(_, (desc, id_renderer))| {
                    let pass_tags = &desc.passorders;
                    if list_model.0.contains_key(&id_model.0) {
                        // log::info!("SysSet1ModifyByPass: 111111111");
                        if let Ok(
                            (
                                bind_model, bind_skl, id_skl,
                                passid,
                            )
                        ) = models.get(id_model.0.clone()) {
                            // let bind_skl: &BindSkinValue = None;
                            // let id_skl: Option<&SkeletonID> = None;
                            // log::info!("SysSet1ModifyByPass: 22222222222222222222222222");
                            let bind_skin = match (&bind_skl.0, id_skl) {
                                (None, None) => { None },
                                (Some(bind_skin), Some(_)) => { Some(bind_skin.clone()) },
                                _ => { return; }
                            };
                            if pass_tags.1 & I::TAG == I::TAG {
                                if ready.val().is_some() {
                                    let key = KeyBindGroupModel::new(bind_model.0.clone(), bind_skin.clone(), val1.0.clone(), &mut binds_recorder);
                                    model_wait.add(&key, passid.id());
                                }
                            }
                        }

                    }
                });
            });
        });

        log::trace!("SysSet1ModifyByModel: {:?}", Instant::now() - time1);
    }

/// * 视口的数据变化时, 重新创建列表内物体相关Pass 的 Set2 数据
///   * 渲染ID变化
///   * 渲染列表变化
///   * 渲染PassTags 变化
    pub fn sys_set2_modify_by_renderer<T: TPass + Component, I: TPassID + Component>(
        renderers: Query<
            &PassTagOrders,
        >,
        viewers: Query<
            (ObjectID, &ViewerActive, &SceneID, &ModelList, &ViewerRenderersInfo),
            Or<(Changed<ModelList>, Changed<ViewerRenderersInfo>, Changed<ViewerActive>)>,
        >,
        models: Query<
            &I,
        >,
        passes: Query<(&PassReady, &PassBindEffectTextures), With<T>>,
        mut commands: Commands,
        mut texturesamplers_wait: ResMut<AssetBindGroupTextureSamplersWaits>,
        mut binds_recorder: ResMut<ResBindsRecorder>,
    ) {
        let time1 = Instant::now();

        viewers.iter().for_each(|(
            id_camera, active, id_scene, list_model, list_renderer
        )| {
            if active.0 == false {
                return;
            }
            list_renderer.map.iter().for_each(|(_, (desc, id_renderer))| {
                let pass_tags = &desc.passorders;
                list_model.0.iter().for_each(|(id_obj, _)| {
                    if let Ok(passid) = models.get(id_obj.clone()) {
                        if pass_tags.1 &  I::TAG == I::TAG {
                            if let Ok((meta1, effect_texture_samplers)) = passes.get(passid.id()) {
                                if let Some(effect_texture_samplers) = effect_texture_samplers.val() {
                                    let key = KeyBindGroupTextureSamplers::new(KeyShaderSetTextureSamplers::default(), effect_texture_samplers.clone(), meta1.0.as_ref().unwrap().1.clone(), &mut binds_recorder);
                                    texturesamplers_wait.add(&key, passid.id());
                                } else { 
                                    commands.entity(passid.id()).insert(PassBindGroupTextureSamplers(None));
                                };
                            }
                        }
                    }
                });
            });
        });

        log::trace!("SysSet2ModifyByRendererID: {:?}", Instant::now() - time1);
    }

    pub fn sys_set2_modify_by_model<T: TPass + Component, I: TPassID + Component>(
        renderers: Query<
            &PassTagOrders,
        >,
        viewers: Query<
            (ObjectID, &ViewerActive, &SceneID, &ModelList, &ViewerRenderersInfo),
        >,
        passes: Query<(ObjectID, &PassSource, &PassReady, &PassBindEffectTextures), (Changed<PassBindEffectTextures>, With<T>)>,
        mut commands: Commands,
        mut texturesamplers_wait: ResMut<AssetBindGroupTextureSamplersWaits>,
        mut binds_recorder: ResMut<ResBindsRecorder>,
    ) {
        let time1 = Instant::now();

        passes.iter().for_each(|(id_pass, id_model, meta1, effect_texture_samplers)| {
            if let Some(effect_texture_samplers) = effect_texture_samplers.val() {
                let key = KeyBindGroupTextureSamplers::new(KeyShaderSetTextureSamplers::default(), effect_texture_samplers.clone(), meta1.0.as_ref().unwrap().1.clone(), &mut binds_recorder);
                texturesamplers_wait.add(&key, id_pass);
            } else { 
                commands.entity(id_pass).insert(PassBindGroupTextureSamplers(None));
            };
        });

        log::trace!("SysSet2ModifyByModel: {:?}", Instant::now() - time1);
    }

    pub fn sys_bind_group_loaded(
        device: Res<PiRenderDevice>,
        asset_mgr_bindgroup_layout: Res<ShareAssetMgr<BindGroupLayout>>,
        asset_mgr_bindgroup: Res<ShareAssetMgr<BindGroup>>,
        mut commands: Commands,
        mut scene_wait: ResMut<AssetBindGroupSceneWaits>,
        mut model_wait: ResMut<AssetBindGroupModelWaits>,
        mut texturesamplers_wait: ResMut<AssetBindGroupTextureSamplersWaits>,
    ) {
        scene_wait.0.drain().for_each(|(key, v)| {
            let key_bind_group = key.key_bind_group();
            // log::warn!("Set0Loaded : ");
            if let Some(bind_group) = create_bind_group(&key_bind_group, &device, &asset_mgr_bindgroup_layout, &asset_mgr_bindgroup) {
                let data = BindGroupScene::new(BindGroupUsage::new(0, key_bind_group, bind_group), key);
                let data = Arc::new(data);
                v.iter().for_each(|id| {
                    commands.entity(id.clone()).insert(PassBindGroupScene(Some(data.clone())));
                });
            } else {
                v.iter().for_each(|id| {
                    commands.entity(id.clone()).insert(PassBindGroupScene(None));
                });
            }
        });
        model_wait.0.drain().for_each(|(key, v)| {
            let key_bind_group = key.key_bind_group();
            if let Some(bind_group) = create_bind_group(&key_bind_group, &device, &asset_mgr_bindgroup_layout, &asset_mgr_bindgroup) {
                let data = BindGroupModel::new(BindGroupUsage::new(1, key_bind_group, bind_group), key);
                let data = Arc::new(data);
                v.iter().for_each(|id| {
                    commands.entity(id.clone()).insert(PassBindGroupModel(Some(data.clone())));
                });
            } else {
                v.iter().for_each(|id| {
                    commands.entity(id.clone()).insert(PassBindGroupModel(None));
                });
            }
        });
        texturesamplers_wait.0.drain().for_each(|(key, v)| {
            let key_bind_group = key.key_bind_group();
            if let Some(key_bind_group) = key_bind_group {
                if let Some(bind_group) = create_bind_group(&key_bind_group, &device, &asset_mgr_bindgroup_layout, &asset_mgr_bindgroup) {
                    let data = BindGroupTextureSamplers::new(key, BindGroupUsage::new(2, key_bind_group, bind_group));
                    let data = Arc::new(data);
                    v.iter().for_each(|id| {
                        commands.entity(id.clone()).insert(PassBindGroupTextureSamplers(Some(data.clone())));
                    });
                } else {
                    v.iter().for_each(|id| {
                        commands.entity(id.clone()).insert(PassBindGroupTextureSamplers(None));
                    });
                }
            } else {
                v.iter().for_each(|id| {
                    commands.entity(id.clone()).insert(PassBindGroupTextureSamplers(None));
                });
            }
        });
    }


fn create_bind_group(
    key_bind_group: &KeyBindGroup,
    device: &RenderDevice,
    asset_mgr_bindgroup_layout: &Share<AssetMgr<BindGroupLayout>>,
    asset_mgr_bindgroup: &Share<AssetMgr<BindGroup>>,
) -> Option<Handle<BindGroup>> {
    let key_u64 = key_bind_group.asset_u64();
    if let Some(bind_group) = asset_mgr_bindgroup.get(&key_u64) {
        Some(bind_group)
    } else {
        let key_bind_group_layout = key_bind_group;
        let key_layout_u64 = key_bind_group_layout.asset_u64();
        let bind_group_layout = if let Some(layout) = asset_mgr_bindgroup_layout.get(&key_layout_u64) {
            Ok(layout)
        } else {
            let layout = BindGroupLayout::new(device, &key_bind_group_layout);
            asset_mgr_bindgroup_layout.insert(key_layout_u64, layout)
        };
        if let Ok(bind_group_layout) = bind_group_layout {
            let bind_group = BindGroup::new(&device, &key_bind_group, bind_group_layout);
            if let Ok(bind_group) = asset_mgr_bindgroup.insert(key_u64, bind_group) {
                Some(bind_group)
            } else {
                None
            }
        } else {
            None
        }
    }
}

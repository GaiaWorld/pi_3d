use std::sync::Arc;

use pi_assets::{mgr::AssetMgr, asset::Handle};
use pi_engine_shell::prelude::*;
use pi_share::Share;

use crate::{
    scene::environment::BindSceneEffect,
    viewer::prelude::*,
    skeleton::prelude::*,
    meshes::prelude::*,
    pass::*,
    bindgroup::*, prelude::MaterialID,
};

use super::{ViewerRenderersInfo, DirtyViewerRenderersInfo};

    pub fn sys_bind_buffer_apply(
        mut allocator: ResMut<ResBindBufferAllocator>,
        mut vb_allocator: ResMut<VertexBufferAllocator3D>,
        device: Res<PiRenderDevice>,
        queue: Res<PiRenderQueue>,
    ) {
        // let time1 = pi_time::Instant::now();

        allocator.write_buffer(&device, &queue);
        vb_allocator.update_buffer(&device, &queue);

        // log::debug!("SysDynBufferAllocatorUpdate: {:?}", pi_time::Instant::now() - time1);
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
        scenes: Query<
            &BindSceneEffect,
        >,
        models: Query<&I>,
        mut scene_wait: ResMut<AssetBindGroupSceneWaits>,
        mut binds_recorder: ResMut<ResBindsRecorder>,
    ) {
        let time1 = pi_time::Instant::now();

        viewers.iter().for_each(|(
            _id_viewer, active, id_scene, bind_viewer, list_model, list_renderer
        )| {
            // log::info!("Set0ByViewer {:?}:", active.0);
            if active.0 == false { return; }

            // log::trace!("SysSet0ModifyByRendererID: {:?}", list_model.0.len());
            // log::info!("Set0ByViewer :");

            if let Ok(bind_base_effect) = scenes.get(id_scene.0) {
                // log::info!("Set0ByViewer : 0");
                let key = KeyBindGroupScene::new(bind_viewer.0.clone(), Some(bind_base_effect.0.clone()), &mut binds_recorder);
                list_renderer.map.iter().for_each(|(_, (passorders, _id_renderer))| {
                    let pass_tags = passorders;
                    if pass_tags.1 & I::TAG == I::TAG {
                        // log::info!("Set0ByViewer : 1");
                        list_model.0.iter().for_each(|id_obj| {
                            // log::info!("Set0ByViewer : 2");
                            if let Ok(passid) = models.get(id_obj.clone()) {
                                // log::info!("Set0ByViewer : 3");
                                // log::info!("Set0ByViewer : 4");
                                scene_wait.add(&key, passid.id());
                            }
                        });
                    }
                });
            }
        });

        log::trace!("SysSet0ModifyByRendererID: {:?}", pi_time::Instant::now() - time1);
    }

/// * 场景的数据变化时, 重新创建视口列表内物体相关Pass 的 Set0 数据
///   * BindSceneEffect 变化
    pub fn sys_set0_modify_by_scene<T: TPass + Component, I: TPassID + Component>(
        viewers: Query<
            (ObjectID, &ViewerActive, &SceneID, &BindViewer, &ModelList, &ViewerRenderersInfo),
        >,
        scenes: Query<
            (ObjectID, &BindSceneEffect),
            Changed<BindSceneEffect>
        >,
        models: Query<&I>,
        mut scene_wait: ResMut<AssetBindGroupSceneWaits>,
        mut binds_recorder: ResMut<ResBindsRecorder>,
    ) {
        let time1 = pi_time::Instant::now();

        scenes.iter().for_each(|(id_scene_obj, bind_base_effect)| {
            viewers.iter().for_each(|(
                _id_camera, active, id_scene, bind_viewer, list_model, list_renderer
            )| {
                if active.0 == false { return; }
                // log::info!("Set0ByScene :");

                // log::trace!("SysSet0ModifyFromScene ||||||||||||||||||||||");
                if id_scene_obj == id_scene.0 {
                    // log::info!("Set0ByScene : 0");
                    let key = KeyBindGroupScene::new(bind_viewer.0.clone(), Some(bind_base_effect.0.clone()), &mut binds_recorder);
                    list_renderer.map.iter().for_each(|(_, (passorders, _id_renderer))| {
                        // log::info!("Set0ByScene : 1");
                        let pass_tags = passorders;
                        if pass_tags.1 & I::TAG == I::TAG {
                            list_model.0.iter().for_each(|id_obj| {
                                // log::info!("Set0ByScene : 2");
                                if let Ok(passid) = models.get(id_obj.clone()) {
                                    // log::info!("Set0ByScene : 3");
                                    // log::info!("Set0ByScene : 4");
                                    scene_wait.add(&key, passid.id());
                                }
                            });
                        }
                    });
                }
            });
        });

        log::trace!("SysSet0ModifyFromScene: {:?}", pi_time::Instant::now() - time1);
    }

    pub fn sys_set0_modify_by_pass<T: TPass + Component, I: TPassID + Component>(
        viewers: Query<
            (&ViewerActive, &SceneID, &BindViewer, &ModelList, &ViewerRenderersInfo),
        >,
        scenes: Query<&BindSceneEffect>,
        passes: Query<(Entity, &ModelPass, &T), Changed<MaterialID>>,
        models: Query<&SceneID>,
        mut scene_wait: ResMut<AssetBindGroupSceneWaits>,
        mut binds_recorder: ResMut<ResBindsRecorder>,
    ) {
        let time1 = pi_time::Instant::now();

        passes.iter().for_each(|(idpass, idmesh, _)| {
            if let Ok(idscene) = models.get(idmesh.0) {
                if let Ok(bind_base_effect) = scenes.get(idscene.0) {
                    viewers.iter().for_each(|(
                        active, id_scene, bind_viewer, list_model, list_renderer
                    )| {
                        if active.0 == false { return; }

                        if id_scene.0 == idscene.0 && list_model.0.contains(&idmesh.0) {
                            list_renderer.map.iter().for_each(|(_, (passorders, _id_renderer))| {
                                if passorders.1 & I::TAG == I::TAG {
                                    let key = KeyBindGroupScene::new(bind_viewer.0.clone(), Some(bind_base_effect.0.clone()), &mut binds_recorder);
                                    scene_wait.add(&key, idpass);
                                }
                            });
                        }
                    });
                }
            }
        });

        log::trace!("SysSet0ModifyFromPass: {:?}", pi_time::Instant::now() - time1);
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
        models: Query<
            (
                &BindModel, &BindSkinValue, Option<&SkeletonID>,
                &I,
            ),
        >,
        passes: Query<(&PassBindEffectValue, &PassEffectReady), With<T>>,
        mut model_wait: ResMut<AssetBindGroupModelWaits>,
        mut binds_recorder: ResMut<ResBindsRecorder>,
    ) {
        let time1 = pi_time::Instant::now();

        viewers.iter().for_each(|(
            _id_camera, active, _id_scene, list_model, list_renderer
        )| {
            if active.0 == false { return; }
            // log::info!("SysSet1ModifyByRendererID: {:?}", list_model.0.len());
            // log::error!("Set1: A");
            
            list_renderer.map.iter().for_each(|(_, (passorders, _id_renderer))| {
                // log::info!("SysSet1ModifyByRendererID: 1111111111111111");
                let pass_tags = passorders;
                // log::info!("SysSet1ModifyByRendererID: 2222222222222222");
                // log::error!("Set1: A0");
                if pass_tags.1 & I::TAG == I::TAG {
                    list_model.0.iter().for_each(|id_obj| {
                        if let Ok(
                            (
                                bind_model, bind_skl, id_skl,
                                passid,
                            )
                        ) = models.get(id_obj.clone()) {
                            // log::error!("Set1: A2");
                            let bind_skin = match (&bind_skl.0, id_skl) {
                                (None, None) => { None },
                                (Some(bind_skin), Some(_)) => { Some(bind_skin.clone()) },
                                _ => { return; }
                            };
                            if let Ok((val1, ready)) = passes.get(passid.id()) {
                                if ready.val().is_some() {
                                    let key = KeyBindGroupModel::new(bind_model.0.clone(), bind_skin.clone(), val1.0.clone(), &mut binds_recorder);
                                    model_wait.add(&key, passid.id());
                                }
                            }
                        }
                    });
                }
            });
        });

        log::trace!("SysSet1ModifyByRendererID: {:?}", pi_time::Instant::now() - time1);
    }
// }


/// * 物体的数据变化时, 重新创建列表内物体相关Pass 的 Set1 数据
///   * 骨骼数据变化 - 
///   * 渲染效果数据 变化
    pub fn sys_set1_modify_by_model<T: TPass + Component, I: TPassID + Component>(
        viewers: Query<
            (ObjectID, &ViewerActive, &SceneID, &BindViewer, &ModelList, &ViewerRenderersInfo),
        >,
        models: Query<
            (
                Entity, &BindModel, &BindSkinValue, Option<&SkeletonID>, &I
            ),
            Or<(
                Changed<BindModel>, Changed<BindSkinValue>, Changed<SkeletonID>
            )>,
        >,
        passes: Query<(&PassBindEffectValue, &PassEffectReady), With<T>>,
        mut model_wait: ResMut<AssetBindGroupModelWaits>,
        mut binds_recorder: ResMut<ResBindsRecorder>,
    ) {
        let time1 = pi_time::Instant::now();

        models.iter().for_each(|(idobj, bind_model, bind_skl, id_skl, passid,)| {
                
            if let Ok((val1, ready)) = passes.get(passid.id()) {
                if ready.val().is_none() { return; }

                // log::error!("Set1: AA");
                viewers.iter().for_each(|(
                    _id_camera, active, _id_scene, _bind_viewer, list_model, list_renderer
                )| {
                    if active.0 == false { return; }

                    if list_model.0.contains(&idobj) {
                        list_renderer.map.iter().for_each(|(_, (passorders, _id_renderer))| {
                            let pass_tags = passorders;
                            if pass_tags.1 & I::TAG == I::TAG {
                                // log::error!("Set1: AA1");
                                let bind_skin = match (&bind_skl.0, id_skl) {
                                    (None, None) => { None },
                                    (Some(bind_skin), Some(_)) => { Some(bind_skin.clone()) },
                                    _ => { return; }
                                };

                                // log::error!("Set1: AA2");
                                let key = KeyBindGroupModel::new(bind_model.0.clone(), bind_skin.clone(), val1.0.clone(), &mut binds_recorder);
                                model_wait.add(&key, passid.id());
                            }
                        });
                    }
                });
            }
        });

        log::trace!("SysSet1ModifyByModel: {:?}", pi_time::Instant::now() - time1);
    }
// }

/// * 物体的数据变化时, 重新创建列表内物体相关Pass 的 Set1 数据
///   * 骨骼数据变化 - 
///   * 渲染效果数据 变化
    pub fn sys_set1_modify_by_pass<T: TPass + Component, I: TPassID + Component>(
        viewers: Query<
            (ObjectID, &ViewerActive, &SceneID, &BindViewer, &ModelList, &ViewerRenderersInfo),
        >,
        models: Query<
            (
                &BindModel, &BindSkinValue, Option<&SkeletonID>,
                &I,
            ),
        >,
        passes: Query<(&PassBindEffectValue, &PassEffectReady, &T, &ModelPass), Changed<PassEffectReady>>,
        mut model_wait: ResMut<AssetBindGroupModelWaits>,
        mut binds_recorder: ResMut<ResBindsRecorder>,
    ) {
        let time1 = pi_time::Instant::now();

        passes.iter().for_each(|(val1, ready, _, id_model)| {   
            if ready.val().is_none() { return; }

            viewers.iter().for_each(|(
                _id_camera, active, _id_scene, _bind_viewer, list_model, list_renderer
            )| {
                if active.0 == false { return; }

                // log::error!("Set1: AAA");
                // log::info!("SysSet1ModifyByPass: {:?}", list_model.0.len());
                list_renderer.map.iter().for_each(|(_, (passorders, _id_renderer))| {
                    let pass_tags = passorders;
                    if pass_tags.1 & I::TAG == I::TAG {
                        // log::error!("Set1: AAA1");
                        if list_model.0.contains(&id_model.0) {
                            // log::info!("SysSet1ModifyByPass: 111111111");
                            if let Ok(
                                (
                                    bind_model, bind_skl, id_skl, passid,
                                )
                            ) = models.get(id_model.0) {
                                // log::error!("Set1: AAA2");
                                // let bind_skl: &BindSkinValue = None;
                                // let id_skl: Option<&SkeletonID> = None;
                                // log::warn!("SysSet1ModifyByPass: 22222222222222222222222222");
                                let bind_skin = match (&bind_skl.0, id_skl) {
                                    (None, None) => { None },
                                    (Some(bind_skin), Some(_)) => { Some(bind_skin.clone()) },
                                    _ => { 
                                        // log::warn!("Skinnnnnnn");
                                        return;
                                    }
                                };
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

        log::trace!("SysSet1ModifyByModel: {:?}", pi_time::Instant::now() - time1);
    }

/// * 视口的数据变化时, 重新创建列表内物体相关Pass 的 Set2 数据
///   * 渲染ID变化
///   * 渲染列表变化
///   * 渲染PassTags 变化
    pub fn sys_set2_modify_by_renderer<T: TPass + Component, I: TPassID + Component>(
        viewers: Query<
            (ObjectID, &ViewerActive, &SceneID, &ModelList, &ViewerRenderersInfo),
            Or<(Changed<ModelList>, Changed<ViewerRenderersInfo>, Changed<ViewerActive>)>,
        >,
        models: Query<
            &I,
        >,
        passes: Query<(&PassEffectReady, &PassBindEffectTextures), With<T>>,
        mut texturesamplers_wait: ResMut<AssetBindGroupTextureSamplersWaits>,
        mut binds_recorder: ResMut<ResBindsRecorder>,
    ) {
        let time1 = pi_time::Instant::now();

        viewers.iter().for_each(|(
            _id_camera, active, _id_scene, list_model, list_renderer
        )| {
            if active.0 == false {
                return;
            }
            // log::error!("Set2: A");
            list_renderer.map.iter().for_each(|(_, (passorders, _id_renderer))| {
                let pass_tags = passorders;
                // log::error!("Set2: A1");
                list_model.0.iter().for_each(|id_obj| {
                    // log::error!("Set2: A2");
                    if let Ok(passid) = models.get(id_obj.clone()) {
                        // log::error!("Set2: A3");
                        if pass_tags.1 &  I::TAG == I::TAG {
                            if let Ok((meta1, effect_texture_samplers)) = passes.get(passid.id()) {
                                // log::error!("Set2: A4");
                                if let Some(effect_texture_samplers) = effect_texture_samplers.val() {
                                    let key = KeyBindGroupTextureSamplers::new(KeyShaderSetTextureSamplers::default(), effect_texture_samplers.clone(), meta1.0.as_ref().unwrap().1.clone(), &mut binds_recorder);
                                    texturesamplers_wait.add(&key, passid.id());
                                } else {
                                    // if let Some(mut cmd) = commands.get_entity(passid.id()) {
                                    //     cmd.insert(PassBindGroupTextureSamplers(None));
                                    // }
                                };
                            }
                        }
                    }
                });
            });
        });

        log::trace!("SysSet2ModifyByRendererID: {:?}", pi_time::Instant::now() - time1);
    }
    pub fn sys_set2_modify_by_pass<T: TPass + Component, I: TPassID + Component>(
        passes: Query<(ObjectID, &ModelPass, &PassEffectReady, &PassBindEffectTextures), (Changed<PassBindEffectTextures>, With<T>)>,
        // mut commands: Commands,
        mut texturesamplers_wait: ResMut<AssetBindGroupTextureSamplersWaits>,
        mut binds_recorder: ResMut<ResBindsRecorder>,
    ) {
        let time1 = pi_time::Instant::now();

        passes.iter().for_each(|(id_pass, _id_model, meta1, effect_texture_samplers)| {
            // log::error!("Set2: AA1");
            if let Some(effect_texture_samplers) = effect_texture_samplers.val() {
                // log::error!("Set2: AA2");
                let key = KeyBindGroupTextureSamplers::new(KeyShaderSetTextureSamplers::default(), effect_texture_samplers.clone(), meta1.0.as_ref().unwrap().1.clone(), &mut binds_recorder);
                texturesamplers_wait.add(&key, id_pass);
            } else {
                
                // if let Some(mut cmd) = commands.get_entity(id_pass) {
                //     cmd.insert(PassBindGroupTextureSamplers(None));
                // }
            };
        });

        log::trace!("SysSet2ModifyByModel: {:?}", pi_time::Instant::now() - time1);
    }

    pub fn sys_bind_group_loaded(
        device: Res<PiRenderDevice>,
        asset_mgr_bindgroup_layout: Res<ShareAssetMgr<BindGroupLayout>>,
        asset_mgr_bindgroup: Res<ShareAssetMgr<BindGroup>>,
        mut bindgroupset0s: Query<&mut PassBindGroupScene>,
        mut bindgroupset1s: Query<&mut PassBindGroupModel>,
        mut bindgroupset2s: Query<&mut PassBindGroupTextureSamplers>,
        mut scene_wait: ResMut<AssetBindGroupSceneWaits>,
        mut model_wait: ResMut<AssetBindGroupModelWaits>,
        mut texturesamplers_wait: ResMut<AssetBindGroupTextureSamplersWaits>,
    ) {
        scene_wait.0.drain().for_each(|(key, v)| {
            let key_bind_group = key.key_bind_group();
            // log::warn!("Set0Loaded : ");
            if let Some(bind_group) = create_bind_group(&key_bind_group, &device, &asset_mgr_bindgroup_layout, &asset_mgr_bindgroup) {
                let data = BindGroupScene::new(BindGroupUsage::new(0, key.binds(), bind_group), key);
                let data = Arc::new(data);
                // log::error!("create_bind_group 0: Ok");
                v.iter().for_each(|id| {
                    if let Ok(mut item) = bindgroupset0s.get_mut(*id) {
                        *item = PassBindGroupScene(Some(data.clone()));
                    }
                });
            } else {
                // log::error!("create_bind_group 0: Error");
                v.iter().for_each(|id| {
                    if let Ok(mut item) = bindgroupset0s.get_mut(*id) {
                        *item = PassBindGroupScene(None);
                    }
                });
            }
        });
        model_wait.0.drain().for_each(|(key, v)| {
            let key_bind_group = key.key_bind_group();
            if let Some(bind_group) = create_bind_group(&key_bind_group, &device, &asset_mgr_bindgroup_layout, &asset_mgr_bindgroup) {
                let data = BindGroupModel::new(BindGroupUsage::new(1, key.binds(), bind_group), key);
                let data = Arc::new(data);
                // log::error!("create_bind_group 1: Ok");
                v.iter().for_each(|id| {
                    if let Ok(mut item) = bindgroupset1s.get_mut(*id) {
                        *item = PassBindGroupModel(Some(data.clone()));
                    }
                });
            } else {
                // log::error!("create_bind_group 1: Error");
                v.iter().for_each(|id| {
                    if let Ok(mut item) = bindgroupset1s.get_mut(*id) {
                        *item = PassBindGroupModel(None);
                    }
                });
            }
        });
        texturesamplers_wait.0.drain().for_each(|(key, v)| {
            let key_bind_group = key.key_bind_group();
            if let Some(key_bind_group) = key_bind_group {
                if let Some(bind_group) = create_bind_group(&key_bind_group, &device, &asset_mgr_bindgroup_layout, &asset_mgr_bindgroup) {
                    let binds = key.binds().unwrap();
                    let data = BindGroupTextureSamplers::new(key, BindGroupUsage::new(2, binds, bind_group));
                    let data = Arc::new(data);
                    // log::error!("create_bind_group 2: Ok");
                    v.iter().for_each(|id| {
                        if let Ok(mut item) = bindgroupset2s.get_mut(*id) {
                            *item = PassBindGroupTextureSamplers(Some(data.clone()));
                        }
                    });
                } else {
                    // log::error!("create_bind_group 2: Error");
                    v.iter().for_each(|id| {
                        if let Ok(mut item) = bindgroupset2s.get_mut(*id) {
                            *item = PassBindGroupTextureSamplers(None);
                        }
                    });
                }
            } else {
                v.iter().for_each(|id| {
                    if let Ok(mut item) = bindgroupset2s.get_mut(*id) {
                        *item = PassBindGroupTextureSamplers(None);
                    }
                });
            }
        });
    }


fn create_bind_group(
    key_bind_group: &KeyBindGroup ,
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

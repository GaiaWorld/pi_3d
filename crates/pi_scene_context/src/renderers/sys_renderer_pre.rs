use std::{sync::Arc, time::Instant, marker::PhantomData};

use pi_assets::{mgr::AssetMgr, asset::Handle};
use pi_ecs::{prelude::{Query, ResMut, Commands, Res, Component}, query::{Changed, Or, With}};
use pi_ecs_macros::setup;
use pi_engine_shell::{object::{GameObject, ObjectID}, run_stage::{TSystemStageInfo}};
use pi_render::{
    rhi::{
        device::RenderDevice, asset::TextureRes, RenderQueue,
    },
    renderer::{bind_buffer::BindBufferAllocator, sampler::SamplerRes, bind_group::{BindGroupLayout, BindGroupUsage, BindGroup, KeyBindGroup}, vertex_buffer::VertexBufferAllocator},
    render_3d::{
        bind_groups::{scene::{BindGroupScene, KeyBindGroupScene}, model::{BindGroupModel, KeyBindGroupModel}, texture_sampler::{BindGroupTextureSamplers, KeyBindGroupTextureSamplers, KeyShaderSetTextureSamplers}},
    }
};
use pi_share::Share;

use crate::{
    flags::SceneID,
    scene::{
        environment::{BindSceneEffect,},
    },
    viewer::{BindViewer, ModelList, ViewerActive},
    skeleton::{skeleton::{BindSkinValue}, SkeletonID},
    meshes::model::BindModel,
    pass::*, bindgroup::{AssetBindGroupSceneWaits, AssetBindGroupModelWaits, AssetBindGroupTextureSamplersWaits},
};

use super::ViewerRenderersInfo;

pub struct SysBufferAllocatorUpdate;
impl TSystemStageInfo for SysBufferAllocatorUpdate {
    // fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
    //     vec![
    //     ]
    // }
}
#[setup]
impl SysBufferAllocatorUpdate {
    #[system]
    fn sys(
        mut allocator: ResMut<BindBufferAllocator>,
        mut vb_allocator: ResMut<VertexBufferAllocator>,
        device: Res<RenderDevice>,
        queue: Res<RenderQueue>,
    ) {
        let time1 = Instant::now();

        allocator.write_buffer(&device, &queue);
        vb_allocator.update_buffer(&device, &queue);

        log::info!("SysDynBufferAllocatorUpdate: {:?}", Instant::now() - time1);
    }
}

pub struct SysBindGroupLoad;
impl TSystemStageInfo for SysBindGroupLoad {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysSet0ModifyByRendererID::<Pass01, PassID01>::key(), SysSet0ModifyFromScene::<Pass01, PassID01>::key(), 
            SysSet1ModifyByRendererID::<Pass01, PassID01>::key(), SysSet1ModifyByModel::<Pass01, PassID01>::key(),
            SysSet2ModifyByRendererID::<Pass01, PassID01>::key(), SysSet2ModifyByModel::<Pass01, PassID01>::key()
        ]
    }
}
#[setup]
impl SysBindGroupLoad {
    #[system]
    fn sys(
        device: Res<RenderDevice>,
        asset_mgr_bindgroup_layout: Res<Share<AssetMgr<BindGroupLayout>>>,
        asset_mgr_bindgroup: Res<Share<AssetMgr<BindGroup>>>,
        mut bg_scene: Commands<GameObject, PassBindGroupScene>,
        mut bg_model: Commands<GameObject, PassBindGroupModel>,
        mut bg_texsamplers: Commands<GameObject, PassBindGroupTextureSamplers>,
        mut scene_wait: ResMut<AssetBindGroupSceneWaits>,
        mut model_wait: ResMut<AssetBindGroupModelWaits>,
        mut texturesamplers_wait: ResMut<AssetBindGroupTextureSamplersWaits>,
    ) {
        scene_wait.0.drain().for_each(|(key, v)| {
            let key_bind_group = key.key_bind_group();
            if let Some(bind_group) = create_bind_group(&key_bind_group, &device, &asset_mgr_bindgroup_layout, &asset_mgr_bindgroup) {
                let data = BindGroupScene::new(BindGroupUsage::new(0, key_bind_group, bind_group), key);
                let data = Arc::new(data);
                v.iter().for_each(|id| {
                    bg_scene.insert(id.clone(), PassBindGroupScene(Some(data.clone())));
                });
            } else {
                v.iter().for_each(|id| {
                    bg_scene.insert(id.clone(), PassBindGroupScene(None));
                });
            }
        });
        model_wait.0.drain().for_each(|(key, v)| {
            let key_bind_group = key.key_bind_group();
            if let Some(bind_group) = create_bind_group(&key_bind_group, &device, &asset_mgr_bindgroup_layout, &asset_mgr_bindgroup) {
                let data = BindGroupModel::new(BindGroupUsage::new(1, key_bind_group, bind_group), key);
                let data = Arc::new(data);
                v.iter().for_each(|id| {
                    bg_model.insert(id.clone(), PassBindGroupModel(Some(data.clone())));
                });
            } else {
                v.iter().for_each(|id| {
                    bg_model.insert(id.clone(), PassBindGroupModel(None));
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
                        bg_texsamplers.insert(id.clone(), PassBindGroupTextureSamplers(Some(data.clone())));
                    });
                } else {
                    v.iter().for_each(|id| {
                        bg_texsamplers.insert(id.clone(), PassBindGroupTextureSamplers(None));
                    });
                }
            } else {
                v.iter().for_each(|id| {
                    bg_texsamplers.insert(id.clone(), PassBindGroupTextureSamplers(None));
                });
            }
        });
    }
}
fn create_bind_group(
    key_bind_group: &KeyBindGroup,
    device: &RenderDevice,
    asset_mgr_bindgroup_layout: &Share<AssetMgr<BindGroupLayout>>,
    asset_mgr_bindgroup: &Share<AssetMgr<BindGroup>>,
) -> Option<Handle<BindGroup>> {
    let key_u64 = key_bind_group.as_u64();
    if let Some(bind_group) = asset_mgr_bindgroup.get(&key_u64) {
        Some(bind_group)
    } else {
        let key_bind_group_layout = key_bind_group.key_bind_group_layout();
        let key_layout_u64 = key_bind_group_layout.as_u64();
        let bind_group_layout = if let Some(layout) = asset_mgr_bindgroup_layout.get(&key_layout_u64) {
            Some(layout)
        } else {
            let layout = BindGroupLayout::new(device, &key_bind_group_layout);
            asset_mgr_bindgroup_layout.insert(key_layout_u64, layout)
        };
        if let Some(bind_group_layout) = bind_group_layout {
            let bind_group = BindGroup::new(&device, &key_bind_group, bind_group_layout);
            asset_mgr_bindgroup.insert(key_u64, bind_group)
        } else {
            None
        }
    }
}

/// * 视口的数据变化时, 重新创建列表内物体相关Pass 的 Set0 数据
///   * 渲染ID变化 - 变为渲染视口
///   * 渲染列表变化 - 有新物体
///   * 渲染PassTags 变化 - 渲染过程变化
pub struct SysSet0ModifyByRendererID<T: TPass + Component, I: TPassID + Component>(PhantomData<(T, I)>);
impl<T: TPass + Component, I: TPassID + Component> TSystemStageInfo for SysSet0ModifyByRendererID<T, I> {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysBufferAllocatorUpdate::key()
        ]
    }
}
#[setup]
impl<T: TPass + Component, I: TPassID + Component> SysSet0ModifyByRendererID<T, I> {
    #[system]
    pub fn sys(
        viewers: Query<
            GameObject,
            (ObjectID, &ViewerActive, &SceneID, &BindViewer, &ModelList, &ViewerRenderersInfo),
            Or<(Changed<BindViewer>, Changed<ViewerActive>, Changed<ModelList>, Changed<ViewerRenderersInfo>)>,
        >,
        renderers: Query<
            GameObject,
            &PassTagOrders,
        >,
        scenes: Query<
            GameObject,
            &BindSceneEffect,
        >,
        models: Query<GameObject, &I>,
        device: Res<RenderDevice>,
        mut scene_wait: ResMut<AssetBindGroupSceneWaits>,
    ) {
        let time1 = Instant::now();

        viewers.iter().for_each(|(
            id_viewer, active, id_scene, bind_viewer, list_model, list_renderer
        )| {
            if !active.0 {
                return;
            }
            if active.0 {
                // log::trace!("SysSet0ModifyByRendererID: {:?}", list_model.0.len());

                if let Some(bind_base_effect) = scenes.get(id_scene.0) {
                    let key = KeyBindGroupScene::new(bind_viewer.0.clone(), Some(bind_base_effect.0.clone()));
                    list_renderer.map.iter().for_each(|(_, (desc, id_renderer))| {
                        let pass_tags = &desc.passorders;
                        list_model.0.iter().for_each(|(id_obj, _)| {
                            if let Some(passid) = models.get(id_obj.clone()) {
                                if pass_tags.1 & I::TAG == I::TAG {
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
}

/// * 场景的数据变化时, 重新创建视口列表内物体相关Pass 的 Set0 数据
///   * BindSceneEffect 变化
pub struct SysSet0ModifyFromScene<T: TPass + Component, I: TPassID + Component>(PhantomData<(T, I)>);
impl<T: TPass + Component, I: TPassID + Component> TSystemStageInfo for SysSet0ModifyFromScene<T, I> {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysSet0ModifyByRendererID::<T, I>::key()
        ]
    }
}
#[setup]
impl<T: TPass + Component, I: TPassID + Component> SysSet0ModifyFromScene<T, I> {
    #[system]
    pub fn sys(
        renderers: Query<
            GameObject,
            &PassTagOrders,
        >,
        viewers: Query<
            GameObject,
            (ObjectID, &ViewerActive, &SceneID, &BindViewer, &ModelList, &ViewerRenderersInfo),
        >,
        scenes: Query<
            GameObject,
            (ObjectID, &BindSceneEffect),
            Changed<BindSceneEffect>
        >,
        models: Query<GameObject, &I>,
        device: Res<RenderDevice>,
        mut scene_wait: ResMut<AssetBindGroupSceneWaits>,
    ) {
        let time1 = Instant::now();

        scenes.iter().for_each(|(id_scene_obj, bind_base_effect)| {
            viewers.iter().for_each(|(
                id_camera, active, id_scene, bind_viewer, list_model, list_renderer
            )| {
                // log::trace!("SysSet0ModifyFromScene ||||||||||||||||||||||");
                if active.0 && id_scene_obj == id_scene.0 {
                    let key = KeyBindGroupScene::new(bind_viewer.0.clone(), Some(bind_base_effect.0.clone()));
                    list_renderer.map.iter().for_each(|(_, (desc, id_renderer))| {
                        let pass_tags = &desc.passorders;
                        list_model.0.iter().for_each(|(id_obj, _)| {
                            if let Some(passid) = models.get(id_obj.clone()) {
                                if pass_tags.1 & I::TAG == I::TAG {
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
}

/// * 视口的数据变化时, 重新创建列表内物体相关Pass 的 Set1 数据
///   * 渲染ID变化
///   * 渲染列表变化
///   * 渲染PassTags 变化
pub struct SysSet1ModifyByRendererID<T: TPass + Component, I: TPassID + Component>(PhantomData<(T, I)>);
impl<T: TPass + Component, I: TPassID + Component> TSystemStageInfo for SysSet1ModifyByRendererID<T, I> {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysBufferAllocatorUpdate::key()
        ]
    }
}
#[setup]
impl<T: TPass + Component, I: TPassID + Component> SysSet1ModifyByRendererID<T, I> {
    #[system]
    pub fn sys(
        viewers: Query<
            GameObject,
            (ObjectID, &ViewerActive, &SceneID, &ModelList, &ViewerRenderersInfo),
            Or<(Changed<ModelList>, Changed<ViewerActive>, Changed<ViewerRenderersInfo>)>,
        >,
        renderers: Query<
            GameObject,
            &PassTagOrders,
        >,
        models: Query<
            GameObject,
            (
                &BindModel, Option<&BindSkinValue>, Option<&SkeletonID>,
                &I,
            ),
        >,
        passes: Query<GameObject, &PassBindEffectValue, With<T>>,
        mut pass01_cmd: Commands<GameObject, PassBindGroupModel>,
        device: Res<RenderDevice>,
        mut model_wait: ResMut<AssetBindGroupModelWaits>,
    ) {
        let time1 = Instant::now();

        viewers.iter().for_each(|(
            id_camera, active, id_scene, list_model, list_renderer
        )| {
            if active.0 == false {
                return;
            }
            // log::trace!("SysSet1ModifyByRendererID: {:?}", list_model.0.len());
            
            list_renderer.map.iter().for_each(|(_, (desc, id_renderer))| {
                // log::trace!("SysSet1ModifyByRendererID: 1111111111111111");
                let pass_tags = &desc.passorders;
                // log::trace!("SysSet1ModifyByRendererID: 2222222222222222");
                list_model.0.iter().for_each(|(id_obj, _)| {
                    if let Some(
                        (
                            bind_model, bind_skl, id_skl,
                            passid,
                        )
                    ) = models.get(id_obj.clone()) {
                        let bind_skin = match (bind_skl, id_skl) {
                            (None, None) => { None },
                            (Some(bind_skin), Some(_)) => { Some(bind_skin.0.clone()) },
                            _ => { return; }
                        };
                        if pass_tags.1 & I::TAG == I::TAG {
                            if let Some(val1) = passes.get(passid.id()) {
                                let key = KeyBindGroupModel::new(bind_model.0.clone(), bind_skin.clone(), val1.0.clone());
                                model_wait.add(&key, passid.id());
                                log::warn!("Skin: {:?}", bind_skin);
                            }
                        }
                    }
                });
            });
        });

        log::trace!("SysSet1ModifyByRendererID: {:?}", Instant::now() - time1);
    }
}


/// * 物体的数据变化时, 重新创建列表内物体相关Pass 的 Set1 数据
///   * 骨骼数据变化 - 
///   * 渲染效果数据 变化
pub struct SysSet1ModifyByModel<T: TPass + Component, I: TPassID + Component>(PhantomData<(T, I)>);
impl<T: TPass + Component, I: TPassID + Component> TSystemStageInfo for SysSet1ModifyByModel<T, I> {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysSet1ModifyByRendererID::<T, I>::key()
        ]
    }
}
#[setup]
impl<T: TPass + Component, I: TPassID + Component> SysSet1ModifyByModel<T, I> {
    #[system]
    pub fn sys(
        renderers: Query<
            GameObject,
            &PassTagOrders,
        >,
        viewers: Query<
            GameObject,
            (ObjectID, &ViewerActive, &SceneID, &BindViewer, &ModelList, &ViewerRenderersInfo),
        >,
        models: Query<
            GameObject,
            (
                &BindModel, Option<&BindSkinValue>, Option<&SkeletonID>,
                &I,
            ),
            Or<(
                Changed<BindSkinValue>, Changed<SkeletonID>, Changed<PassDirtyBindEffectValue>
            )>,
        >,
        passes: Query<GameObject, &PassBindEffectValue, With<T>>,
        mut pass01_cmd: Commands<GameObject, PassBindGroupModel>,
        device: Res<RenderDevice>,
        mut model_wait: ResMut<AssetBindGroupModelWaits>,
    ) {
        let time1 = Instant::now();

        viewers.iter().for_each(|(
            id_camera, active, id_scene, bind_viewer, list_model, list_renderer
        )| {
            if active.0 == false {
                return;
            }
            // log::trace!("SysSet1ModifyByModel: {:?}", list_model.0.len());
            list_renderer.map.iter().for_each(|(_, (desc, id_renderer))| {
                let pass_tags = &desc.passorders;
                // log::trace!("SysSet1ModifyByModel: 1111111111111111111111");
                list_model.0.iter().for_each(|(id_obj, _)| {
                    if let Some(
                        (
                            bind_model, bind_skl, id_skl,
                            passid,
                        )
                    ) = models.get(id_obj.clone()) {
                        // let bind_skl: Option<&BindSkinValue> = None;
                        // let id_skl: Option<&SkeletonID> = None;
                        // log::trace!("SysSet1ModifyByModel: 22222222222222222222222222");
                        let bind_skin = match (bind_skl, id_skl) {
                            (None, None) => { None },
                            (Some(bind_skin), Some(_)) => { Some(bind_skin.0.clone()) },
                            _ => { return; }
                        };
                        if pass_tags.1 & I::TAG == I::TAG {
                            if let Some(val1) = passes.get(passid.id()) {
                                let key = KeyBindGroupModel::new(bind_model.0.clone(), bind_skin.clone(), val1.0.clone());
                                model_wait.add(&key, passid.id());
                            }
                            log::warn!("Skin: {:?}", bind_skin);
                        }
                    }
                });
            });
        });

        log::trace!("SysSet1ModifyByModel: {:?}", Instant::now() - time1);
    }
}

/// * 视口的数据变化时, 重新创建列表内物体相关Pass 的 Set2 数据
///   * 渲染ID变化
///   * 渲染列表变化
///   * 渲染PassTags 变化
pub struct SysSet2ModifyByRendererID<T: TPass + Component, I: TPassID + Component>(PhantomData<(T, I)>);
impl<T: TPass + Component, I: TPassID + Component> TSystemStageInfo for SysSet2ModifyByRendererID<T, I> {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysBufferAllocatorUpdate::key()
        ]
    }
}
#[setup]
impl<T: TPass + Component, I: TPassID + Component> SysSet2ModifyByRendererID<T, I> {
    #[system]
    pub fn sys(
        renderers: Query<
            GameObject,
            &PassTagOrders,
        >,
        viewers: Query<
            GameObject,
            (ObjectID, &ViewerActive, &SceneID, &ModelList, &ViewerRenderersInfo),
            Or<(Changed<ModelList>, Changed<ViewerRenderersInfo>, Changed<ViewerActive>)>,
        >,
        models: Query<
            GameObject,
            &I,
        >,
        passes: Query<GameObject, (&PassReady, &PassBindEffectTextures), With<T>>,
        mut pass01_cmd: Commands<GameObject, PassBindGroupTextureSamplers>,
        device: Res<RenderDevice>,
        mut texturesamplers_wait: ResMut<AssetBindGroupTextureSamplersWaits>,
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
                    if let Some(passid) = models.get(id_obj.clone()) {
                        if pass_tags.1 &  I::TAG == I::TAG {
                            if let Some((meta1, effect_texture_samplers)) = passes.get(passid.id()) {
                                if let Some(effect_texture_samplers) = effect_texture_samplers.val() {
                                    let key = KeyBindGroupTextureSamplers::new(KeyShaderSetTextureSamplers::default(), effect_texture_samplers.clone(), meta1.0.as_ref().unwrap().1.clone());
                                    texturesamplers_wait.add(&key, passid.id());
                                } else { 
                                    pass01_cmd.insert(passid.id(), PassBindGroupTextureSamplers(None));
                                };
                            }
                        }
                    }
                });
            });
        });

        log::trace!("SysSet2ModifyByRendererID: {:?}", Instant::now() - time1);
    }
}

pub struct SysSet2ModifyByModel<T: TPass + Component, I: TPassID + Component>(PhantomData<(T, I)>);
impl<T: TPass + Component, I: TPassID + Component> TSystemStageInfo for SysSet2ModifyByModel<T, I> {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysSet2ModifyByRendererID::<T, I>::key()
        ]
    }
}
#[setup]
impl<T: TPass + Component, I: TPassID + Component> SysSet2ModifyByModel<T, I> {
    #[system]
    pub fn sys(
        renderers: Query<
            GameObject,
            &PassTagOrders,
        >,
        viewers: Query<
            GameObject,
            (ObjectID, &ViewerActive, &SceneID, &ModelList, &ViewerRenderersInfo),
        >,
        passes: Query<GameObject, (ObjectID, &PassSource, &PassReady, &PassBindEffectTextures), (Changed<PassBindEffectTextures>, With<T>)>,
        mut pass01_cmd: Commands<GameObject, PassBindGroupTextureSamplers>,
        device: Res<RenderDevice>,
        mut texturesamplers_wait: ResMut<AssetBindGroupTextureSamplersWaits>,
    ) {
        let time1 = Instant::now();

        passes.iter().for_each(|(id_pass, id_model, meta1, effect_texture_samplers)| {
            if let Some(effect_texture_samplers) = effect_texture_samplers.val() {
                let key = KeyBindGroupTextureSamplers::new(KeyShaderSetTextureSamplers::default(), effect_texture_samplers.clone(), meta1.0.as_ref().unwrap().1.clone());
                texturesamplers_wait.add(&key, id_pass);
            } else { 
                pass01_cmd.insert(id_pass, PassBindGroupTextureSamplers(None));
            };
        });

        log::trace!("SysSet2ModifyByModel: {:?}", Instant::now() - time1);
    }
}


use std::{sync::Arc, time::Instant, marker::PhantomData};

use pi_assets::{mgr::AssetMgr};
use pi_ecs::{prelude::{Query, ResMut, Commands, Res, Component}, query::{Changed, Or, With}};
use pi_ecs_macros::setup;
use pi_engine_shell::{object::{GameObject, ObjectID}, run_stage::{TSystemStageInfo}};
use pi_render::{
    rhi::{
        device::RenderDevice, asset::TextureRes, RenderQueue, bind_group::BindGroup
    },
    renderer::{bind_buffer::BindBufferAllocator, sampler::SamplerRes, bind_group::BindGroupLayout, vertex_buffer::VertexBufferAllocator},
    render_3d::{
        bind_groups::{scene::BindGroupScene, model::BindGroupModel, texture_sampler::BindGroupTextureSamplers},
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
    pass::*,
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
        mut pass01_cmd: Commands<GameObject, PassBindGroupScene>,
        device: Res<RenderDevice>,
        asset_tex: Res<Share<AssetMgr<TextureRes>>>,
        asset_mgr_bind_group_layout:  Res<Share<AssetMgr<BindGroupLayout>>>,
        asset_mgr_bind_group: Res<Share<AssetMgr<BindGroup>>>,
    ) {
        let time1 = Instant::now();

        viewers.iter().for_each(|(
            id_viewer, active, id_scene, bind_viewer, list_model, list_renderer
        )| {
            if !active.0 {
                return;
            }
            if active.0 {
                // log::info!("SysSet0ModifyByRendererID: {:?}", list_model.0.len());

                if let Some(bind_base_effect) = scenes.get(id_scene.0) {
                    // log::info!("SysSet0ModifyByRendererID BindGroupScene");
                    if let Some(bindgroup) = BindGroupScene::new(
                        bind_viewer.0.clone(),
                        Some(bind_base_effect.0.clone()),
                        &device,
                        &asset_mgr_bind_group_layout,
                        &asset_mgr_bind_group,
                    ) {
                        // log::info!("RenderBindGroupScene::new |||||||||||||||||||||| {:?}", list_renderer);
                        list_renderer.map.iter().for_each(|(_, (desc, id_renderer))| {
                            let pass_tags = &desc.passorders;
                            list_model.0.iter().for_each(|(id_obj, _)| {
                                if let Some(passid) = models.get(id_obj.clone()) {
                                    if pass_tags.1 & I::TAG == I::TAG {
                                        pass01_cmd.insert(passid.id(), PassBindGroupScene(Some(Arc::new(bindgroup.clone()))));
                                    }
                                }
                            });
                        });
                    }
                }
            }
        });

        log::info!("SysSet0ModifyByRendererID: {:?}", Instant::now() - time1);
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
        mut pass01_cmd: Commands<GameObject, PassBindGroupScene>,
        device: Res<RenderDevice>,
        asset_mgr_bind_group_layout:  Res<Share<AssetMgr<BindGroupLayout>>>,
        asset_mgr_bind_group: Res<Share<AssetMgr<BindGroup>>>,
    ) {
        let time1 = Instant::now();

        scenes.iter().for_each(|(id_scene_obj, bind_base_effect)| {
            viewers.iter().for_each(|(
                id_camera, active, id_scene, bind_viewer, list_model, list_renderer
            )| {
                // log::info!("SysSet0ModifyFromScene ||||||||||||||||||||||");
                if active.0 && id_scene_obj == id_scene.0 {
                    // log::info!("SysSet0ModifyFromScene BindGroupScene");
                    if let Some(bindgroup) = BindGroupScene::new(
                        bind_viewer.0.clone(),
                        Some(bind_base_effect.0.clone()),
                        &device,
                        &asset_mgr_bind_group_layout,
                        &asset_mgr_bind_group,
                    ) {
                        // log::info!("RenderBindGroupScene::new ||||||||||||||||||||||");
                        list_renderer.map.iter().for_each(|(_, (desc, id_renderer))| {
                            let pass_tags = &desc.passorders;
                            list_model.0.iter().for_each(|(id_obj, _)| {
                                if let Some(passid) = models.get(id_obj.clone()) {
                                    if pass_tags.1 & I::TAG == I::TAG {
                                        pass01_cmd.insert(passid.id(), PassBindGroupScene(Some(Arc::new(bindgroup.clone()))));
                                    }
                                }
                            });
                        });
                    }
                }
            });
        });

        log::info!("SysSet0ModifyFromScene: {:?}", Instant::now() - time1);
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
        asset_sampler: Res<Share<AssetMgr<SamplerRes>>>,
        asset_mgr_bind_group_layout:  Res<Share<AssetMgr<BindGroupLayout>>>,
        asset_mgr_bind_group: Res<Share<AssetMgr<BindGroup>>>,
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
                                let data = if let Some(bindgroup) = BindGroupModel::new(
                                    bind_model.0.clone(),
                                    bind_skin.clone(),
                                    val1.0.clone(),
                                    &device,
                                    &asset_mgr_bind_group_layout,
                                    &asset_mgr_bind_group
                                ) {
                                    Some(Arc::new(bindgroup))
                                } else {
                                    None
                                };
                                pass01_cmd.insert(passid.id(), PassBindGroupModel(data));
                            }
                        }
                    }
                });
            });
        });

        log::info!("SysSet1ModifyByRendererID: {:?}", Instant::now() - time1);
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
        asset_sampler: Res<Share<AssetMgr<SamplerRes>>>,
        asset_mgr_bind_group_layout:  Res<Share<AssetMgr<BindGroupLayout>>>,
        asset_mgr_bind_group: Res<Share<AssetMgr<BindGroup>>>,
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
                    if let Some(
                        (
                            bind_model, bind_skl, id_skl,
                            passid,
                        )
                    ) = models.get(id_obj.clone()) {
                        // let bind_skl: Option<&BindSkinValue> = None;
                        // let id_skl: Option<&SkeletonID> = None;
                        // log::info!("SysSet1ModifyByModel: 22222222222222222222222222");
                        let bind_skin = match (bind_skl, id_skl) {
                            (None, None) => { None },
                            (Some(bind_skin), Some(_)) => { Some(bind_skin.0.clone()) },
                            _ => { return; }
                        };
                        if pass_tags.1 & I::TAG == I::TAG {
                            if let Some(val1) = passes.get(passid.id()) {
                                let data = if let Some(bindgroup) = BindGroupModel::new(
                                    bind_model.0.clone(),
                                    bind_skin.clone(),
                                    val1.0.clone(),
                                    &device,
                                    &asset_mgr_bind_group_layout,
                                    &asset_mgr_bind_group
                                ) {
                                    Some(Arc::new(bindgroup))
                                } else {
                                    None
                                };
                                pass01_cmd.insert(passid.id(), PassBindGroupModel(data));
                            }
                        }
                    }
                });
            });
        });

        log::info!("SysSet1ModifyByModel: {:?}", Instant::now() - time1);
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
        asset_tex: Res<Share<AssetMgr<TextureRes>>>,
        asset_sampler: Res<Share<AssetMgr<SamplerRes>>>,
        asset_mgr_bind_group_layout:  Res<Share<AssetMgr<BindGroupLayout>>>,
        asset_mgr_bind_group: Res<Share<AssetMgr<BindGroup>>>,
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
                                let data = if let Some(effect_texture_samplers) = effect_texture_samplers.val() {
                                    if let Some(bindgroup) = BindGroupTextureSamplers::new(meta1.0.as_ref().unwrap().1.clone(), effect_texture_samplers.clone(),  &device, &asset_mgr_bind_group_layout, &asset_mgr_bind_group) {
                                        Some(Arc::new(bindgroup))
                                    } else {
                                        None
                                    }
                                } else { None };
                                pass01_cmd.insert(passid.id(), PassBindGroupTextureSamplers(data));
                            }
                        }
                    }
                });
            });
        });

        log::info!("SysSet2ModifyByRendererID: {:?}", Instant::now() - time1);
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
        asset_tex: Res<Share<AssetMgr<TextureRes>>>,
        asset_sampler: Res<Share<AssetMgr<SamplerRes>>>,
        asset_mgr_bind_group_layout:  Res<Share<AssetMgr<BindGroupLayout>>>,
        asset_mgr_bind_group: Res<Share<AssetMgr<BindGroup>>>,
    ) {
        let time1 = Instant::now();

        passes.iter().for_each(|(id_pass, id_model, meta1, effect_texture_samplers)| {
            let data = if let Some(effect_texture_samplers) = effect_texture_samplers.val() {
                if let Some(bindgroup) = BindGroupTextureSamplers::new(meta1.0.as_ref().unwrap().1.clone(), effect_texture_samplers.clone(),  &device, &asset_mgr_bind_group_layout, &asset_mgr_bind_group) {
                    Some(Arc::new(bindgroup))
                } else { None }
            } else { None };
            pass01_cmd.insert(id_pass, PassBindGroupTextureSamplers(data));
        });

        // viewers.iter().for_each(|(
        //     id_camera, active,  id_scene, list_model, list_renderer
        // )| {
        //     if active.0 == false {
        //         return;
        //     }
        //     list_renderer.map.iter().for_each(|(_, (desc, id_renderer))| {
        //         let pass_tags = &desc.passorders;
        //         list_model.0.iter().for_each(|(id_obj, _)| {
        //             passes.iter().for_each(|(id_pass, meta1, effect_texture_samplers)| {
        //                 let data = if let Some(effect_texture_samplers) = effect_texture_samplers.val() {
        //                     if let Some(bindgroup) = BindGroupTextureSamplers::new(meta1.0.as_ref().unwrap().1.clone(), effect_texture_samplers.clone(),  &device, &asset_mgr_bind_group_layout, &asset_mgr_bind_group) {
        //                         Some(Arc::new(bindgroup))
        //                     } else { None }
        //                 } else { None };
        //                 pass01_cmd.insert(id_pass, PassBindGroupTextureSamplers(data));
        //             });
        //         });
        //     });
        // });

        log::info!("SysSet2ModifyByModel: {:?}", Instant::now() - time1);
    }
}


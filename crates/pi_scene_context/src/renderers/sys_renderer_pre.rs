use std::{sync::Arc, time::Instant};

use pi_assets::{mgr::AssetMgr};
use pi_ecs::{prelude::{Query, ResMut, Commands, Res}, query::{Changed, Or}};
use pi_ecs_macros::setup;
use pi_engine_shell::{object::{GameObject, ObjectID}, run_stage::{TSystemStageInfo}};
use pi_render::{
    rhi::{
        device::RenderDevice, asset::TextureRes, RenderQueue, bind_group::BindGroup
    },
    renderer::{bind_buffer::BindBufferAllocator, sampler::SamplerRes, bind_group::BindGroupLayout, vertex_buffer::VertexBufferAllocator},
    render_3d::{
        bind_groups::{scene::BindGroupScene, model::BindGroupModel, texture_sampler::BindGroupTextureSamplers},
        binds::model::skin::ShaderBindModelAboutSkinValue,
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

fn bindgroup_scene(
    pass_tags: &PassTagOrders,
    id_obj: &ObjectID,
    bindgroup: &BindGroupScene,
    pass01_cmd: &mut Commands<GameObject, Pass01BindGroupScene>,
    pass02_cmd: &mut Commands<GameObject, Pass02BindGroupScene>,
    pass03_cmd: &mut Commands<GameObject, Pass03BindGroupScene>,
    pass04_cmd: &mut Commands<GameObject, Pass04BindGroupScene>,
    pass05_cmd: &mut Commands<GameObject, Pass05BindGroupScene>,
    pass06_cmd: &mut Commands<GameObject, Pass06BindGroupScene>,
    pass07_cmd: &mut Commands<GameObject, Pass07BindGroupScene>,
    pass08_cmd: &mut Commands<GameObject, Pass08BindGroupScene>,
) {
    // log::info!("bindgroup_scene ||||||||||||||| {:?}", pass_tags);
    if pass_tags.1 & EPassTag::PASS_TAG_01 == EPassTag::PASS_TAG_01 {
        pass01_cmd.insert(id_obj.clone(), Pass01BindGroupScene(Some(bindgroup.clone())));
    }
    if pass_tags.1 & EPassTag::PASS_TAG_02 == EPassTag::PASS_TAG_02 {
        pass02_cmd.insert(id_obj.clone(), Pass02BindGroupScene(Some(bindgroup.clone())));
    }
    if pass_tags.1 & EPassTag::PASS_TAG_03 == EPassTag::PASS_TAG_03 {
        pass03_cmd.insert(id_obj.clone(), Pass03BindGroupScene(Some(bindgroup.clone())));
    }
    if pass_tags.1 & EPassTag::PASS_TAG_04 == EPassTag::PASS_TAG_04 {
        pass04_cmd.insert(id_obj.clone(), Pass04BindGroupScene(Some(bindgroup.clone())));
    }
    if pass_tags.1 & EPassTag::PASS_TAG_05 == EPassTag::PASS_TAG_05 {
        pass05_cmd.insert(id_obj.clone(), Pass05BindGroupScene(Some(bindgroup.clone())));
    }
    if pass_tags.1 & EPassTag::PASS_TAG_06 == EPassTag::PASS_TAG_06 {
        pass06_cmd.insert(id_obj.clone(), Pass06BindGroupScene(Some(bindgroup.clone())));
    }
    if pass_tags.1 & EPassTag::PASS_TAG_07 == EPassTag::PASS_TAG_07 {
        pass07_cmd.insert(id_obj.clone(), Pass07BindGroupScene(Some(bindgroup.clone())));
    }
    if pass_tags.1 & EPassTag::PASS_TAG_08 == EPassTag::PASS_TAG_08 {
        pass08_cmd.insert(id_obj.clone(), Pass08BindGroupScene(Some(bindgroup.clone())));
    }
}

fn bindgroup_model(
    id_obj: &ObjectID,
    pass_tags: &PassTagOrders,
    bind_matrix: &BindModel,
    bind_skin: Option<Arc<ShaderBindModelAboutSkinValue>>,
    val: (
        &Pass01BindEffectValue, &Pass02BindEffectValue, &Pass03BindEffectValue, &Pass04BindEffectValue,
        &Pass05BindEffectValue, &Pass06BindEffectValue, &Pass07BindEffectValue, &Pass08BindEffectValue
    ),
    pass01_cmd: &mut Commands<GameObject, Pass01BindGroupModel>,
    pass02_cmd: &mut Commands<GameObject, Pass02BindGroupModel>,
    pass03_cmd: &mut Commands<GameObject, Pass03BindGroupModel>,
    pass04_cmd: &mut Commands<GameObject, Pass04BindGroupModel>,
    pass05_cmd: &mut Commands<GameObject, Pass05BindGroupModel>,
    pass06_cmd: &mut Commands<GameObject, Pass06BindGroupModel>,
    pass07_cmd: &mut Commands<GameObject, Pass07BindGroupModel>,
    pass08_cmd: &mut Commands<GameObject, Pass08BindGroupModel>,
    device: &RenderDevice,
    asset_sampler: &Share<AssetMgr<SamplerRes>>,
    asset_mgr_bind_group_layout: &Share<AssetMgr<BindGroupLayout>>,
    asset_mgr_bind_group: &Share<AssetMgr<BindGroup>>,
) {
    let (val1, val2, val3, val4, val5, val6, val7, val8) = val;
    if pass_tags.1 & EPassTag::PASS_TAG_01 == EPassTag::PASS_TAG_01 {
        if let Some(bindgroup) = BindGroupModel::new(bind_matrix.0.clone(), bind_skin.clone(), val1.0.clone(), device, &asset_mgr_bind_group_layout, &asset_mgr_bind_group) {
            pass01_cmd.insert(id_obj.clone(), Pass01BindGroupModel(Some(bindgroup)));
        } else {
            pass01_cmd.insert(id_obj.clone(), Pass01BindGroupModel(None));
        }
}
    if pass_tags.1 & EPassTag::PASS_TAG_02 == EPassTag::PASS_TAG_02 {
        if let Some(bindgroup) = BindGroupModel::new(bind_matrix.0.clone(), bind_skin.clone(), val2.0.clone(), device, &asset_mgr_bind_group_layout, &asset_mgr_bind_group) {
            // log::info!("bindgroup_model ||||||||||||||||||||||||||||||||||||||||||||||||||||");
            pass02_cmd.insert(id_obj.clone(), Pass02BindGroupModel(Some(bindgroup)));
        } else {
            pass02_cmd.insert(id_obj.clone(), Pass02BindGroupModel(None));
        }
    }
    if pass_tags.1 & EPassTag::PASS_TAG_03 == EPassTag::PASS_TAG_03 {
        if let Some(bindgroup) = BindGroupModel::new(bind_matrix.0.clone(), bind_skin.clone(), val3.0.clone(), device, &asset_mgr_bind_group_layout, &asset_mgr_bind_group) {
            pass03_cmd.insert(id_obj.clone(), Pass03BindGroupModel(Some(bindgroup)));
        } else {
            pass03_cmd.insert(id_obj.clone(), Pass03BindGroupModel(None));
        }
    }
    if pass_tags.1 & EPassTag::PASS_TAG_04 == EPassTag::PASS_TAG_04 {
        if let Some(bindgroup) = BindGroupModel::new(bind_matrix.0.clone(), bind_skin.clone(), val4.0.clone(), device, &asset_mgr_bind_group_layout, &asset_mgr_bind_group) {
            pass04_cmd.insert(id_obj.clone(), Pass04BindGroupModel(Some(bindgroup)));
        } else {
            pass04_cmd.insert(id_obj.clone(), Pass04BindGroupModel(None));
        }
    }
    if pass_tags.1 & EPassTag::PASS_TAG_05 == EPassTag::PASS_TAG_05 {
        if let Some(bindgroup) = BindGroupModel::new(bind_matrix.0.clone(), bind_skin.clone(), val5.0.clone(), device, &asset_mgr_bind_group_layout, &asset_mgr_bind_group) {
            pass05_cmd.insert(id_obj.clone(), Pass05BindGroupModel(Some(bindgroup)));
        } else {
            pass05_cmd.insert(id_obj.clone(), Pass05BindGroupModel(None));
        }
    }
    if pass_tags.1 & EPassTag::PASS_TAG_06 == EPassTag::PASS_TAG_06 {
        if let Some(bindgroup) = BindGroupModel::new(bind_matrix.0.clone(), bind_skin.clone(), val6.0.clone(), device, &asset_mgr_bind_group_layout, &asset_mgr_bind_group) {
            pass06_cmd.insert(id_obj.clone(), Pass06BindGroupModel(Some(bindgroup)));
        } else {
            pass06_cmd.insert(id_obj.clone(), Pass06BindGroupModel(None));
        }
    }
    if pass_tags.1 & EPassTag::PASS_TAG_07 == EPassTag::PASS_TAG_07 {
        if let Some(bindgroup) = BindGroupModel::new(bind_matrix.0.clone(), bind_skin.clone(), val7.0.clone(), device, &asset_mgr_bind_group_layout, &asset_mgr_bind_group) {
            pass07_cmd.insert(id_obj.clone(), Pass07BindGroupModel(Some(bindgroup)));
        } else {
            pass07_cmd.insert(id_obj.clone(), Pass07BindGroupModel(None));
        }
    }
    if pass_tags.1 & EPassTag::PASS_TAG_08 == EPassTag::PASS_TAG_08 {
        if let Some(bindgroup) = BindGroupModel::new(bind_matrix.0.clone(), bind_skin.clone(), val8.0.clone(), device, &asset_mgr_bind_group_layout, &asset_mgr_bind_group) {
            pass08_cmd.insert(id_obj.clone(), Pass08BindGroupModel(Some(bindgroup)));
        } else {
            pass08_cmd.insert(id_obj.clone(), Pass08BindGroupModel(None));
        }
    }
}

fn bindgroup_texturesamples(
    id_obj: &ObjectID,
    pass_tags: &PassTagOrders,
    metas: (
        &Pass01Ready, &Pass02Ready, &Pass03Ready, &Pass04Ready,
        &Pass05Ready, &Pass06Ready, &Pass07Ready, &Pass08Ready,
    ),
    val: (
        &Pass01BindEffectTextures, &Pass02BindEffectTextures, &Pass03BindEffectTextures, &Pass04BindEffectTextures,
        &Pass05BindEffectTextures, &Pass06BindEffectTextures, &Pass07BindEffectTextures, &Pass08BindEffectTextures,
    ),
    pass01_cmd: &mut Commands<GameObject, Pass01BindGroupTextureSamplers>,
    pass02_cmd: &mut Commands<GameObject, Pass02BindGroupTextureSamplers>,
    pass03_cmd: &mut Commands<GameObject, Pass03BindGroupTextureSamplers>,
    pass04_cmd: &mut Commands<GameObject, Pass04BindGroupTextureSamplers>,
    pass05_cmd: &mut Commands<GameObject, Pass05BindGroupTextureSamplers>,
    pass06_cmd: &mut Commands<GameObject, Pass06BindGroupTextureSamplers>,
    pass07_cmd: &mut Commands<GameObject, Pass07BindGroupTextureSamplers>,
    pass08_cmd: &mut Commands<GameObject, Pass08BindGroupTextureSamplers>,
    device: &RenderDevice,
    asset_tex: &Share<AssetMgr<TextureRes>>,
    asset_sampler: &Share<AssetMgr<SamplerRes>>,
    asset_mgr_bind_group_layout: &Share<AssetMgr<BindGroupLayout>>,
    asset_mgr_bind_group: &Share<AssetMgr<BindGroup>>,
) {
    // log::info!("bindgroup_texturesamples ||||||||||||");
    let (meta1, meta2, meta3, meta4, meta5, meta6, meta7, meta8) = metas;
    let (val1, val2, val3, val4, val5, val6, val7, val8) = val;
    if pass_tags.1 & EPassTag::PASS_TAG_01 == EPassTag::PASS_TAG_01 {
        if let Some(effect_texture_samplers) = &val1.0 {
            if let Some(bindgroup) = BindGroupTextureSamplers::new(meta1.0.as_ref().unwrap().1.clone(), effect_texture_samplers.clone(),  device, asset_mgr_bind_group_layout, asset_mgr_bind_group) {
                pass01_cmd.insert(id_obj.clone(), Pass01BindGroupTextureSamplers(Some(bindgroup)));
            } else {
                pass01_cmd.insert(id_obj.clone(), Pass01BindGroupTextureSamplers(None));
            }
        } else {
            pass01_cmd.insert(id_obj.clone(), Pass01BindGroupTextureSamplers(None));
        } 
    }
    if pass_tags.1 & EPassTag::PASS_TAG_02 == EPassTag::PASS_TAG_02 {
        if let Some(effect_texture_samplers) = &val2.0 {
            if let Some(bindgroup) = BindGroupTextureSamplers::new(meta2.0.as_ref().unwrap().1.clone(), effect_texture_samplers.clone(),  device, asset_mgr_bind_group_layout, asset_mgr_bind_group) {
                // log::info!("bindgroup_texturesamples |||||||||||| A");
                pass02_cmd.insert(id_obj.clone(), Pass02BindGroupTextureSamplers(Some(bindgroup)));
            } else {
                // log::info!("bindgroup_texturesamples |||||||||||| B");
                pass02_cmd.insert(id_obj.clone(), Pass02BindGroupTextureSamplers(None));
            }
        } else {
            // log::info!("bindgroup_texturesamples |||||||||||| C");
            pass02_cmd.insert(id_obj.clone(), Pass02BindGroupTextureSamplers(None));
        }
    }
    if pass_tags.1 & EPassTag::PASS_TAG_03 == EPassTag::PASS_TAG_03 {
        if let Some(effect_texture_samplers) = &val3.0 {
            if let Some(bindgroup) = BindGroupTextureSamplers::new(meta3.0.as_ref().unwrap().1.clone(), effect_texture_samplers.clone(),  device, asset_mgr_bind_group_layout, asset_mgr_bind_group) {
                pass03_cmd.insert(id_obj.clone(), Pass03BindGroupTextureSamplers(Some(bindgroup)));
            } else {
                pass03_cmd.insert(id_obj.clone(), Pass03BindGroupTextureSamplers(None));
            }
        } else {
            pass03_cmd.insert(id_obj.clone(), Pass03BindGroupTextureSamplers(None));
        }
    }
    if pass_tags.1 & EPassTag::PASS_TAG_04 == EPassTag::PASS_TAG_04 {
        if let Some(effect_texture_samplers) = &val4.0 {
            if let Some(bindgroup) = BindGroupTextureSamplers::new(meta4.0.as_ref().unwrap().1.clone(), effect_texture_samplers.clone(),  device, asset_mgr_bind_group_layout, asset_mgr_bind_group) {
                pass04_cmd.insert(id_obj.clone(), Pass04BindGroupTextureSamplers(Some(bindgroup)));
            } else {
                pass04_cmd.insert(id_obj.clone(), Pass04BindGroupTextureSamplers(None));
            }
        } else {
            pass04_cmd.insert(id_obj.clone(), Pass04BindGroupTextureSamplers(None));
        }
    }
    if pass_tags.1 & EPassTag::PASS_TAG_05 == EPassTag::PASS_TAG_05 {
        if let Some(effect_texture_samplers) = &val5.0 {
            if let Some(bindgroup) = BindGroupTextureSamplers::new(meta5.0.as_ref().unwrap().1.clone(), effect_texture_samplers.clone(),  device, asset_mgr_bind_group_layout, asset_mgr_bind_group) {
                pass05_cmd.insert(id_obj.clone(), Pass05BindGroupTextureSamplers(Some(bindgroup)));
            } else {
                pass05_cmd.insert(id_obj.clone(), Pass05BindGroupTextureSamplers(None));
            }
        } else {
            pass05_cmd.insert(id_obj.clone(), Pass05BindGroupTextureSamplers(None));
        }
    }
    if pass_tags.1 & EPassTag::PASS_TAG_06 == EPassTag::PASS_TAG_06 {
        if let Some(effect_texture_samplers) = &val6.0 {
            if let Some(bindgroup) = BindGroupTextureSamplers::new(meta6.0.as_ref().unwrap().1.clone(), effect_texture_samplers.clone(),  device, asset_mgr_bind_group_layout, asset_mgr_bind_group) {
                pass06_cmd.insert(id_obj.clone(), Pass06BindGroupTextureSamplers(Some(bindgroup)));
            } else {
                pass06_cmd.insert(id_obj.clone(), Pass06BindGroupTextureSamplers(None));
            }
        } else {
            pass06_cmd.insert(id_obj.clone(), Pass06BindGroupTextureSamplers(None));
        }
    }
    if pass_tags.1 & EPassTag::PASS_TAG_07 == EPassTag::PASS_TAG_07 {
        if let Some(effect_texture_samplers) = &val7.0 {
            if let Some(bindgroup) = BindGroupTextureSamplers::new(meta7.0.as_ref().unwrap().1.clone(), effect_texture_samplers.clone(),  device, asset_mgr_bind_group_layout, asset_mgr_bind_group) {
                pass07_cmd.insert(id_obj.clone(), Pass07BindGroupTextureSamplers(Some(bindgroup)));
            } else {
                pass07_cmd.insert(id_obj.clone(), Pass07BindGroupTextureSamplers(None));
            }
        } else {
            pass07_cmd.insert(id_obj.clone(), Pass07BindGroupTextureSamplers(None));
        }
    }
    if pass_tags.1 & EPassTag::PASS_TAG_08 == EPassTag::PASS_TAG_08 {
        if let Some(effect_texture_samplers) = &val8.0 {
            if let Some(bindgroup) = BindGroupTextureSamplers::new(meta8.0.as_ref().unwrap().1.clone(), effect_texture_samplers.clone(),  device, asset_mgr_bind_group_layout, asset_mgr_bind_group) {
                pass08_cmd.insert(id_obj.clone(), Pass08BindGroupTextureSamplers(Some(bindgroup)));
            } else {
                pass08_cmd.insert(id_obj.clone(), Pass08BindGroupTextureSamplers(None));
            }
        } else {
            pass08_cmd.insert(id_obj.clone(), Pass08BindGroupTextureSamplers(None));
        }
    }
}

/// * 视口的数据变化时, 重新创建列表内物体相关Pass 的 Set0 数据
///   * 渲染ID变化 - 变为渲染视口
///   * 渲染列表变化 - 有新物体
///   * 渲染PassTags 变化 - 渲染过程变化
pub struct SysSet0ModifyByRendererID;
impl TSystemStageInfo for SysSet0ModifyByRendererID {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysBufferAllocatorUpdate::key()
        ]
    }
}
#[setup]
impl SysSet0ModifyByRendererID {
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
        mut pass01_cmd: Commands<GameObject, Pass01BindGroupScene>,
        mut pass02_cmd: Commands<GameObject, Pass02BindGroupScene>,
        mut pass03_cmd: Commands<GameObject, Pass03BindGroupScene>,
        mut pass04_cmd: Commands<GameObject, Pass04BindGroupScene>,
        mut pass05_cmd: Commands<GameObject, Pass05BindGroupScene>,
        mut pass06_cmd: Commands<GameObject, Pass06BindGroupScene>,
        mut pass07_cmd: Commands<GameObject, Pass07BindGroupScene>,
        mut pass08_cmd: Commands<GameObject, Pass08BindGroupScene>,
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
                                bindgroup_scene(
                                    pass_tags,
                                    id_obj,
                                    &bindgroup,
                                    &mut pass01_cmd,
                                    &mut pass02_cmd,
                                    &mut pass03_cmd,
                                    &mut pass04_cmd,
                                    &mut pass05_cmd,
                                    &mut pass06_cmd,
                                    &mut pass07_cmd,
                                    &mut pass08_cmd,
                                );
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
pub struct SysSet0ModifyFromScene;
impl TSystemStageInfo for SysSet0ModifyFromScene {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysBufferAllocatorUpdate::key()
        ]
    }
}
#[setup]
impl SysSet0ModifyFromScene {
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
        mut pass01_cmd: Commands<GameObject, Pass01BindGroupScene>,
        mut pass02_cmd: Commands<GameObject, Pass02BindGroupScene>,
        mut pass03_cmd: Commands<GameObject, Pass03BindGroupScene>,
        mut pass04_cmd: Commands<GameObject, Pass04BindGroupScene>,
        mut pass05_cmd: Commands<GameObject, Pass05BindGroupScene>,
        mut pass06_cmd: Commands<GameObject, Pass06BindGroupScene>,
        mut pass07_cmd: Commands<GameObject, Pass07BindGroupScene>,
        mut pass08_cmd: Commands<GameObject, Pass08BindGroupScene>,
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
                                bindgroup_scene(
                                    pass_tags,
                                    id_obj,
                                    &bindgroup,
                                    &mut pass01_cmd,
                                    &mut pass02_cmd,
                                    &mut pass03_cmd,
                                    &mut pass04_cmd,
                                    &mut pass05_cmd,
                                    &mut pass06_cmd,
                                    &mut pass07_cmd,
                                    &mut pass08_cmd,
                                );
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
pub struct SysSet1ModifyByRendererID;
impl TSystemStageInfo for SysSet1ModifyByRendererID {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysBufferAllocatorUpdate::key()
        ]
    }
}
#[setup]
impl SysSet1ModifyByRendererID {
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
                
                &Pass01BindEffectValue, &Pass02BindEffectValue, &Pass03BindEffectValue, &Pass04BindEffectValue,
                &Pass05BindEffectValue, &Pass06BindEffectValue, &Pass07BindEffectValue, &Pass08BindEffectValue
                ,
            ),
        >,
        mut pass01_cmd: Commands<GameObject, Pass01BindGroupModel>,
        mut pass02_cmd: Commands<GameObject, Pass02BindGroupModel>,
        mut pass03_cmd: Commands<GameObject, Pass03BindGroupModel>,
        mut pass04_cmd: Commands<GameObject, Pass04BindGroupModel>,
        mut pass05_cmd: Commands<GameObject, Pass05BindGroupModel>,
        mut pass06_cmd: Commands<GameObject, Pass06BindGroupModel>,
        mut pass07_cmd: Commands<GameObject, Pass07BindGroupModel>,
        mut pass08_cmd: Commands<GameObject, Pass08BindGroupModel>,
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
                            val1, val2, val3, val4,
                            val5, val6, val7, val8,
                        )
                    ) = models.get(id_obj.clone()) {
                        // log::info!("SysSet1ModifyByRendererID: 33333333333333333333");
                        // let bind_skl: Option<&BindSkinValue> = None;
                        // let id_skl: Option<&SkeletonID> = None;
                        match (bind_skl, id_skl) {
                            (None, None) => {
                                bindgroup_model(
                                    id_obj, pass_tags, bind_model, None, 
                                    (val1, val2, val3, val4, val5, val6, val7, val8,), 
                                    &mut pass01_cmd, &mut pass02_cmd, &mut pass03_cmd, &mut pass04_cmd, &mut pass05_cmd, &mut pass06_cmd, &mut pass07_cmd, &mut pass08_cmd,
                                    &device, &asset_sampler, &asset_mgr_bind_group_layout, &asset_mgr_bind_group
                                );
                            },
                            (Some(bind_skin), Some(_)) => {
                                bindgroup_model(
                                    id_obj, pass_tags, bind_model, Some(bind_skin.0.clone()), 
                                    (val1, val2, val3, val4, val5, val6, val7, val8,), 
                                    &mut pass01_cmd, &mut pass02_cmd, &mut pass03_cmd, &mut pass04_cmd, &mut pass05_cmd, &mut pass06_cmd, &mut pass07_cmd, &mut pass08_cmd,
                                    &device, &asset_sampler, &asset_mgr_bind_group_layout, &asset_mgr_bind_group
                                );
                            },
                            _ => {

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
pub struct SysSet1ModifyByModel;
impl TSystemStageInfo for SysSet1ModifyByModel {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysBufferAllocatorUpdate::key()
        ]
    }
}
#[setup]
impl SysSet1ModifyByModel {
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
                
                    &Pass01BindEffectValue, &Pass02BindEffectValue, &Pass03BindEffectValue, &Pass04BindEffectValue,
                    &Pass05BindEffectValue, &Pass06BindEffectValue, &Pass07BindEffectValue, &Pass08BindEffectValue
                ,
            ),
            Or<(
                Changed<BindSkinValue>, Changed<SkeletonID>,
                Changed<Pass01BindEffectValue>, Changed<Pass02BindEffectValue>, Changed<Pass03BindEffectValue>, Changed<Pass04BindEffectValue>,
                Changed<Pass05BindEffectValue>, Changed<Pass06BindEffectValue>, Changed<Pass07BindEffectValue>, Changed<Pass08BindEffectValue>,
            )>,
        >,
        mut pass01_cmd: Commands<GameObject, Pass01BindGroupModel>,
        mut pass02_cmd: Commands<GameObject, Pass02BindGroupModel>,
        mut pass03_cmd: Commands<GameObject, Pass03BindGroupModel>,
        mut pass04_cmd: Commands<GameObject, Pass04BindGroupModel>,
        mut pass05_cmd: Commands<GameObject, Pass05BindGroupModel>,
        mut pass06_cmd: Commands<GameObject, Pass06BindGroupModel>,
        mut pass07_cmd: Commands<GameObject, Pass07BindGroupModel>,
        mut pass08_cmd: Commands<GameObject, Pass08BindGroupModel>,
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
                            val1, val2, val3, val4,
                            val5, val6, val7, val8,
                        )
                    ) = models.get(id_obj.clone()) {
                        // let bind_skl: Option<&BindSkinValue> = None;
                        // let id_skl: Option<&SkeletonID> = None;
                        // log::info!("SysSet1ModifyByModel: 22222222222222222222222222");
                        match (bind_skl, id_skl) {
                            (None, None) => {
                                bindgroup_model(
                                    id_obj, pass_tags, bind_model, None, 
                                    (val1, val2, val3, val4, val5, val6, val7, val8,), 
                                    &mut pass01_cmd, &mut pass02_cmd, &mut pass03_cmd, &mut pass04_cmd, &mut pass05_cmd, &mut pass06_cmd, &mut pass07_cmd, &mut pass08_cmd,
                                    &device, &asset_sampler, &asset_mgr_bind_group_layout, &asset_mgr_bind_group
                                );
                            },
                            (Some(bind_skin), Some(_)) => {
                                bindgroup_model(
                                    id_obj, pass_tags, bind_model, Some(bind_skin.0.clone()), 
                                    (val1, val2, val3, val4, val5, val6, val7, val8,), 
                                    &mut pass01_cmd, &mut pass02_cmd, &mut pass03_cmd, &mut pass04_cmd, &mut pass05_cmd, &mut pass06_cmd, &mut pass07_cmd, &mut pass08_cmd,
                                    &device, &asset_sampler, &asset_mgr_bind_group_layout, &asset_mgr_bind_group
                                );
                            },
                            _ => {
                                
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
pub struct SysSet2ModifyByRendererID;
impl TSystemStageInfo for SysSet2ModifyByRendererID {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysBufferAllocatorUpdate::key()
        ]
    }
}
#[setup]
impl SysSet2ModifyByRendererID {
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
            (
                (
                    &Pass01Ready, &Pass02Ready, &Pass03Ready, &Pass04Ready, 
                    &Pass05Ready, &Pass06Ready, &Pass07Ready, &Pass08Ready,
                ),
                (
                    &Pass01BindEffectTextures, &Pass02BindEffectTextures, &Pass03BindEffectTextures, &Pass04BindEffectTextures,
                    &Pass05BindEffectTextures, &Pass06BindEffectTextures, &Pass07BindEffectTextures, &Pass08BindEffectTextures,
                )
            ),
        >,
        mut pass01_cmd: Commands<GameObject, Pass01BindGroupTextureSamplers>,
        mut pass02_cmd: Commands<GameObject, Pass02BindGroupTextureSamplers>,
        mut pass03_cmd: Commands<GameObject, Pass03BindGroupTextureSamplers>,
        mut pass04_cmd: Commands<GameObject, Pass04BindGroupTextureSamplers>,
        mut pass05_cmd: Commands<GameObject, Pass05BindGroupTextureSamplers>,
        mut pass06_cmd: Commands<GameObject, Pass06BindGroupTextureSamplers>,
        mut pass07_cmd: Commands<GameObject, Pass07BindGroupTextureSamplers>,
        mut pass08_cmd: Commands<GameObject, Pass08BindGroupTextureSamplers>,
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
                    if let Some((metas, val)) = models.get(id_obj.clone()) {
                        bindgroup_texturesamples(
                            id_obj, pass_tags, metas, val, 
                            &mut pass01_cmd, &mut pass02_cmd, &mut pass03_cmd, &mut pass04_cmd, &mut pass05_cmd, &mut pass06_cmd, &mut pass07_cmd, &mut pass08_cmd,
                            &device, &asset_tex, &asset_sampler, &asset_mgr_bind_group_layout, &asset_mgr_bind_group
                        );
                    }
                });
            });
        });

        log::info!("SysSet2ModifyByRendererID: {:?}", Instant::now() - time1);
    }
}

pub struct SysSet2ModifyByModel;
impl TSystemStageInfo for SysSet2ModifyByModel {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysBufferAllocatorUpdate::key()
        ]
    }
}
#[setup]
impl SysSet2ModifyByModel {
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
        models: Query<
            GameObject,
            (
                (
                    &Pass01Ready, &Pass02Ready, &Pass03Ready, &Pass04Ready, 
                    &Pass05Ready, &Pass06Ready, &Pass07Ready, &Pass08Ready,
                ),
                (
                    &Pass01BindEffectTextures, &Pass02BindEffectTextures, &Pass03BindEffectTextures, &Pass04BindEffectTextures,
                    &Pass05BindEffectTextures, &Pass06BindEffectTextures, &Pass07BindEffectTextures, &Pass08BindEffectTextures,
                )
            ),
            Or<(
                Changed<Pass01BindEffectTextures>, Changed<Pass02BindEffectTextures>, Changed<Pass03BindEffectTextures>, Changed<Pass04BindEffectTextures>,
                Changed<Pass05BindEffectTextures>, Changed<Pass06BindEffectTextures>, Changed<Pass07BindEffectTextures>, Changed<Pass08BindEffectTextures>,
            )>,
        >,
        mut pass01_cmd: Commands<GameObject, Pass01BindGroupTextureSamplers>,
        mut pass02_cmd: Commands<GameObject, Pass02BindGroupTextureSamplers>,
        mut pass03_cmd: Commands<GameObject, Pass03BindGroupTextureSamplers>,
        mut pass04_cmd: Commands<GameObject, Pass04BindGroupTextureSamplers>,
        mut pass05_cmd: Commands<GameObject, Pass05BindGroupTextureSamplers>,
        mut pass06_cmd: Commands<GameObject, Pass06BindGroupTextureSamplers>,
        mut pass07_cmd: Commands<GameObject, Pass07BindGroupTextureSamplers>,
        mut pass08_cmd: Commands<GameObject, Pass08BindGroupTextureSamplers>,
        device: Res<RenderDevice>,
        asset_tex: Res<Share<AssetMgr<TextureRes>>>,
        asset_sampler: Res<Share<AssetMgr<SamplerRes>>>,
        asset_mgr_bind_group_layout:  Res<Share<AssetMgr<BindGroupLayout>>>,
        asset_mgr_bind_group: Res<Share<AssetMgr<BindGroup>>>,
    ) {
        let time1 = Instant::now();

        viewers.iter().for_each(|(
            id_camera, active,  id_scene, list_model, list_renderer
        )| {
            if active.0 == false {
                return;
            }
            list_renderer.map.iter().for_each(|(_, (desc, id_renderer))| {
                let pass_tags = &desc.passorders;
                list_model.0.iter().for_each(|(id_obj, _)| {
                    if let Some((metas, val)) = models.get(id_obj.clone()) {
                        bindgroup_texturesamples(
                            id_obj, pass_tags, metas, val, 
                            &mut pass01_cmd, &mut pass02_cmd, &mut pass03_cmd, &mut pass04_cmd, &mut pass05_cmd, &mut pass06_cmd, &mut pass07_cmd, &mut pass08_cmd,
                            &device, &asset_tex, &asset_sampler, &asset_mgr_bind_group_layout, &asset_mgr_bind_group
                        );
                    }
                });
            });
        });

        log::info!("SysSet2ModifyByModel: {:?}", Instant::now() - time1);
    }
}


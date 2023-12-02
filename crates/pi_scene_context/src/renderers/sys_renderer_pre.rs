

use pi_engine_shell::prelude::*;

use crate::prelude::DisposeReady;
use crate::{
    viewer::prelude::*,
    skeleton::prelude::*,
    meshes::prelude::*,
    pass::*,
    scene::prelude::*,
};
use super::renderer::*;

pub use super::sys_bindgroup_0::*;
pub use super::sys_bindgroup_1::*;
pub use super::sys_bindgroup_2::*;
pub use super::sys_bindgroup_3::*;

pub fn sys_sets_modify_by_viewer<T: TPass + Component, I: TPassID + Component>(
    viewers: Query<
        (ObjectID, &SceneID, &ModelList, &ForceIncludeModelList, &ViewerRenderersInfo),
        Or<(Changed<BindViewer>, Changed<FlagModelList>, Changed<ForceIncludeModelList>, Changed<DirtyViewerRenderersInfo>)>,
    >,
    renderers: Query<(&RendererEnable, &PassTag)>,
    models: Query<&I>,
    mut passes: Query<(&PassSceneID, &DisposeReady, &mut PassViewerID, &mut PassRendererID), With<T>>,
) {
    let time1 = pi_time::Instant::now();

    viewers.iter().for_each(|(idviewer, id_scene, modellist, forcemodels, viewrenderinfos)| {
        // log::error!("DDD 0");
        viewrenderinfos.renderers().for_each(|idrenderer| {
            let idrenderer = *idrenderer;
            if let Ok((rendererenable, passtag)) = renderers.get(idrenderer) {
                // log::error!("DDD 1");
                if rendererenable.0 == true && *passtag == T::TAG {
                    // log::error!("DDD 2 ");
                    modellist.0.iter().for_each(|idmodel| {
                        if let Ok(passid) = models.get(*idmodel) {
                            if let Ok((passscene, disposeready, mut flagpassviewer, mut passrenderer) ) = passes.get_mut(passid.id()) {
                                if disposeready.0 == false && passscene.0 == id_scene.0 {
                                    // log::error!("DDD 3 ");
                                    if flagpassviewer.0 != idviewer { *flagpassviewer = PassViewerID(idviewer); }
                                    if passrenderer.0 != idrenderer { *passrenderer = PassRendererID(idrenderer); }
                                }
                            }
                        }
                    });
                    forcemodels.0.iter().for_each(|idmodel| {
                        if let Ok(passid) = models.get(*idmodel) {
                            if let Ok((passscene, disposeready, mut flagpassviewer, mut passrenderer) ) = passes.get_mut(passid.id()) {
                                if disposeready.0 == false && passscene.0 == id_scene.0 {
                                    if flagpassviewer.0 != idviewer { *flagpassviewer = PassViewerID(idviewer); }
                                    if passrenderer.0 != idrenderer { *passrenderer = PassRendererID(idrenderer); }
                                }
                            }
                        }
                    });
                }
            }
        });
    });

    log::trace!("SysSet0ModifyByRendererID: {:?}", pi_time::Instant::now() - time1);
}

pub fn sys_passrendererid_pass_reset<T: TPass + Component, I: TPassID + Component>(
    viewers: Query<(Entity, &SceneID, &ModelList, &ForceIncludeModelList, &ViewerRenderersInfo)>,
    renderers: Query<(&RendererEnable, &PassTag)>,
    models: Query<
        (&SceneID, &I),
    >,
    mut passes: Query<(&mut PassRendererID, &mut PassViewerID, &PassModelID), (With<T>, Changed<PassReset>)>,
) {
    passes.iter_mut().for_each(|(mut passrenderer, mut passviewer, idmodel)| {
        // log::error!("BBB 0 ");
        if let Ok((idscene, _)) = models.get(idmodel.0) {
            // log::error!("BBB 1 ");
            viewers.iter().for_each(|(idviewer, viewscene, list0, list1, viewrenderinfos)| {
                // log::error!("BBB 2 ");
                if viewscene.0 == idscene.0 {
                    viewrenderinfos.renderers().for_each(|idrenderer| {
                        let idrenderer = *idrenderer;
                        // log::error!("BBB 3 ");

                        if let Ok((rendererenable, passtag)) = renderers.get(idrenderer) {
                            // log::error!("BBB 4 ");
                            if rendererenable.0 == true && *passtag == T::TAG {
                                // log::error!("BBB 5 ");
                                if list0.0.contains(&idmodel.0) || list1.0.contains(&idmodel.0) {
                                    passrenderer.0 = idrenderer;
                                    passviewer.0 = idviewer;
                                    // log::warn!("Dirty PassRenderID While Pass Reset");
                                }
                            }
                        }
                    });
                }
            });
        }
    });
}

pub fn sys_sets_modify_by_scene_extend<T: TPass + Component, I: TPassID + Component>(
    scenes: Query<Entity, Or<(Changed<BRDFTexture>, Changed<MainCameraOpaqueTarget>, Changed<MainCameraDepthTarget>, Changed<EnvTexture>, Changed<SceneShadowRenderTarget>)>>,
    models: Query<
        (Entity, &SceneID, &I),
    >,
    mut passes: Query<&mut PassSceneForSet3, With<T>>,
) {
    let time1 = pi_time::Instant::now();

    models.iter().for_each(|(_entity, idscene, passid)| {
        if scenes.contains(idscene.0) {
            if let Ok(mut dirty) = passes.get_mut(passid.id()) {
                *dirty = PassSceneForSet3(idscene.0);
            }
        }
    });

    log::trace!("SysSet1ModifyByModel: {:?}", pi_time::Instant::now() - time1);
}

pub fn sys_sets_modify_by_model<T: TPass + Component, I: TPassID + Component>(
    models: Query<
        (
            Entity, &I
        ),
        Or<(
            Changed<BindModel>, Changed<BindSkinValue>, Changed<SkeletonID>, Added<ModelLightingIndexs>
        )>,
    >,
    mut passes: Query<&mut PassModelID, With<T>>,
) {
    let time1 = pi_time::Instant::now();

    models.iter().for_each(|(entity, passid)| {
        if let Ok(mut dirty) = passes.get_mut(passid.id()) {
            *dirty = PassModelID(entity);
        }
    });

    log::trace!("SysSet1ModifyByModel: {:?}", pi_time::Instant::now() - time1);
}

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

pub fn sys_bind_group_loaded(
    _device: Res<PiRenderDevice>,
) {
    
}


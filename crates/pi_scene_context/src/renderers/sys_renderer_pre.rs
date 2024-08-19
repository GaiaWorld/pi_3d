
use pi_scene_shell::prelude::*;

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

pub fn sys_sets_modify_by_viewer(
    viewers: Query<
        (ObjectID, &SceneID, &ModelList, &ForceIncludeModelList, &ViewerRenderersInfo),
        Or<(Changed<BindViewer>, Changed<FlagModelList>, Changed<ForceIncludeModelList>, Changed<DirtyViewerRenderersInfo>)>,
    >,
    renderers: Query<(&RendererParam, &PassTag)>,
    modelspass: Query<&PassIDs>,
    mut passes: Query<(&DisposeReady, &mut PassRendererID)>,
) {
    // let time1 = pi_time::Instant::now();

    viewers.iter().for_each(|(_idviewer, _id_scene, modellist, forcemodels, viewrenderinfos)| {
        // log::error!("DDD 0 idviewer: {:?}, rendererCount: {:?}, models: {:?}", idviewer, viewrenderinfos.0.len(), modellist.0.len());
        viewrenderinfos.renderers().for_each(|idrenderer| {
            let idrenderer = *idrenderer;

            // log::error!("DDD 1 idviewer: {:?}, idrenderer: {:?}, models: {:?}", idviewer, idrenderer, modellist.0.len());
            if let Ok((rendererenable, passtag)) = renderers.get(idrenderer) {
                // log::error!("DDD 2");
                if rendererenable.enable.0 == true {
                    // log::error!("DDD 3 idviewer: {:?}, idrenderer: {:?}, models: {:?}", idviewer, idrenderer, modellist.0.len());
                    _sets_modify_by_viewer(idrenderer, &mut passes, &modelspass, modellist, forcemodels, passtag);
                }
            }
        });
    });

    // log::trace!("SysSet0ModifyByRendererID: {:?}", pi_time::Instant::now() - time1);
}

#[inline(never)]
fn _sets_modify_by_viewer(
    idrenderer: Entity,
    // idviewer: Entity,
    passes: &mut Query<(&DisposeReady, &mut PassRendererID)>,
    // id_scene: Entity,
    models: &Query<&PassIDs>,
    modellist: &ModelList,
    forcemodels: &ForceIncludeModelList,
    passtag: &PassTag,
) {
    modellist.0.iter().for_each(|idmodel| {
        if let Ok(passid) = models.get(*idmodel) {
            __sets_modify_by_viewer(idrenderer, passid.0[passtag.index()], passes);
        }
    });
    forcemodels.0.iter().for_each(|idmodel| {
        if let Ok(passid) = models.get(*idmodel) {
            __sets_modify_by_viewer(idrenderer, passid.0[passtag.index()], passes);
        }
    });
}

#[inline(never)]
fn __sets_modify_by_viewer(
    idrenderer: Entity,
    // idviewer: Entity,
    passid: Entity,
    passes: &mut Query<(&DisposeReady, &mut PassRendererID)>,
    // id_scene: Entity,
) {
    if let Ok((disposeready, mut passrenderer) ) = passes.get_mut(passid) {
        if disposeready.0 == false {
            if passrenderer.0 != idrenderer { *passrenderer = PassRendererID(idrenderer); }
        }
        // log::warn!("__sets_modify_by_viewer {:?}", (idviewer));
    }
}

pub fn sys_passrendererid_pass_reset(
    viewers: Query<(Entity, &SceneID, &ModelList, &ForceIncludeModelList, &ViewerRenderersInfo)>,
    renderers: Query<(&RendererParam, &PassTag)>,
    model: Query<&SceneID>,
    addeds: ComponentAdded<PassReset>,
    changes: ComponentChanged<PassReset>,
    mut passes: Query<(Entity, &mut PassRendererID, &PassModelID, &PassTag)>,
) {
    let changes = addeds.iter().chain(changes.iter());
    changes.for_each(|entity| {
        if let Ok((idpass, mut passrenderer, idmodel, passpasstag)) = passes.get_mut(*entity) {
            if let Ok(idscene) = model.get(idmodel.0) {
                // log::error!("BBB 1 ");
                viewers.iter().for_each(|(_idviewer, viewscene, list0, list1, viewrenderinfos)| {
                    // if viewrenderinfos.len() == 0 { log::error!("BBB 2 viewrenderinfos {:?}", (idpass, idviewer, viewrenderinfos.len())); }
                    if idscene.0 == viewscene.0 {
                        viewrenderinfos.renderers().for_each(|idrenderer| {
                            let idrenderer = *idrenderer;
                            // log::error!("BBB 3 ");
            
                            if let Ok((rendererenable, passtag)) = renderers.get(idrenderer) {
                                // log::error!("BBB 4 ");
                                if rendererenable.enable.0 == true && passtag == passpasstag {
                                    // log::error!("BBB 5 ");
                                    if list0.0.contains(&idmodel.0) || list1.0.contains(&idmodel.0) {
                                        // log::warn!("Dirty PassRenderID While Pass Reset {:?}", (idpass, idviewer, passviewer.0 != idviewer));
                                        // passrenderer.0 = idrenderer;
                                        // passviewer.0 = idviewer;
                                        if passrenderer.0 != idrenderer { *passrenderer = PassRendererID(idrenderer); }
                                        // log::warn!("Dirty PassRenderID While Pass Reset");
                                    }
                                }
                            }
                        });
                    }
                });
            }
        }
    });
    // passes.iter_mut().for_each(|(idpass, mut passrenderer, idmodel, passpasstag)| {
    // });
}

pub fn sys_sets_modify_by_scene_extend(
    scenes: Query<(Entity, &MainCameraOpaqueTarget), Or<(Changed<BRDFTexture>, Changed<MainCameraOpaqueTarget>, Changed<MainCameraDepthTarget>, Changed<EnvTexture>, Changed<SceneShadowRenderTarget>)>>,
    mut passes: Query<(&mut PassBindGroupsDirty, &PassModelID)>,
    models: Query<(&SceneID, &PassIDs)>,
) {
    // let time1 = pi_time::Instant::now();

    // log::error!("Scene : {:?}", temp);

    scenes.iter().for_each(|(scene, _)| {
        models.iter().for_each(|(sceneid, passids)| {
            if sceneid.0 == scene {
                passids.0.iter().for_each(|idpass| {
                    if let Ok((mut dirty, _idmodel)) = passes.get_mut(*idpass) {
                        // log::error!("sys_sets_modify_by_scene_extend");
                        *dirty = PassBindGroupsDirty;
                    }
                });
            }
        })
    });

    // passes.iter_mut().for_each(|(mut dirty, idmodel)| {
    //     if let Ok(idscene) = models.get(idmodel.0) {
    //         if scenes.contains(idscene.0) {
    //             log::error!("sys_sets_modify_by_scene_extend");
    //             *dirty = PassBindGroupsDirty;
    //         }
    //     }
    // });

    // log::trace!("SysSet1ModifyByModel: {:?}", pi_time::Instant::now() - time1);
}

pub fn sys_sets_modify_by_model(
    models: Query<
        (
            Entity, &PassIDs
        ),
        Or<(
            Changed<BindModel>, Changed<BindSkinValue>, Changed<SkeletonID>, Changed<ModelLightingIndexs>
        )>,
    >,
    mut passes: Query<&mut PassBindGroupsDirty>,
) {
    // let time1 = pi_time::Instant::now();

    models.iter().for_each(|(_entity, passids)| {
        // log::error!("sys_sets_modify_by_model");
        passids.0.iter().for_each(|id| {
            if let Ok(mut dirty) = passes.get_mut(*id) { *dirty = PassBindGroupsDirty; }
        });
    });

    // log::trace!("SysSet1ModifyByModel: {:?}", pi_time::Instant::now() - time1);
}

pub fn sys_bind_buffer_apply(
    mut allocator: ResMut<ResBindBufferAllocator>,
    device: Res<PiRenderDevice>,
    queue: Res<PiRenderQueue>,
) {
    // let time1 = pi_time::Instant::now();

    allocator.write_buffer(&device, &queue);

    // log::debug!("SysDynBufferAllocatorUpdate: {:?}", pi_time::Instant::now() - time1);
}

pub fn sys_vertice_buffer_apply(
    mut vb_allocator: ResMut<VertexBufferAllocator3D>,
    device: Res<PiRenderDevice>,
    queue: Res<PiRenderQueue>,
) {
    // let time1 = pi_time::Instant::now();

    vb_allocator.update_buffer(&device, &queue);

    // log::debug!("SysDynBufferAllocatorUpdate: {:?}", pi_time::Instant::now() - time1);
}

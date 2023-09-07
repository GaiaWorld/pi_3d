use std::sync::Arc;

use pi_assets::asset::Handle;
use pi_engine_shell::prelude::*;
use crate::{
    viewer::prelude::*,
    pass::*,
    geometry::prelude::*,
    cameras::prelude::*,
    scene::prelude::*,
    transforms::prelude::*, prelude::{RenderAlignment, IndiceRenderRange, DisposeReady, GlobalEnable, InstancedMeshTransparentSortCollection},
};

use super::{
    render_primitive::*,
    base::*,
    pass::*,
    render_depth_and_stencil::*,
    render_sort::*,
    render_blend::ModelBlend,
    render_target_state::RenderTargetState,
    renderer::{Renderer, RendererEnable, RenderSize},
};


/// 渲染器搜集渲染
    pub fn sys_pass_bind_groups(
        mut passes: Query<
            (ObjectID, &ModelPass, &PassEffectReady, &PassBindGroupScene, &PassBindGroupModel, &PassBindGroupTextureSamplers, &mut PassBindGroups),
            Or<(Changed<PassEffectReady>, Changed<PassBindGroupScene>, Changed<PassBindGroupModel>, Changed<PassBindGroupTextureSamplers>)>
        >,
    ) {
        passes.iter_mut().for_each(|(_id_pass, _id_model, ready, set0, set1, set2, mut bindgroups)| {
            if let Some((_key_meta, meta)) = ready.val() {
                if let (Some(set0), Some(set1)) = (set0.val(), set1.val()) {
                    if meta.textures.len() > 0 && set2.val().is_none() {
                        if bindgroups.val().is_some() {
                            *bindgroups = PassBindGroups::new(None);
                        }
                        log::debug!("Bindgroups: {:?}", false);
                    } else {
                        *bindgroups = PassBindGroups::new(Some(
                            BindGroups3D::create(set0.clone(), set1.clone(), set2.val().clone())
                        ));
                        log::debug!("Bindgroups: {:?}", true);
                    }
                    return;
                }
            } else {
                log::debug!("Bindgroups: Ready False");
            }
            
            if bindgroups.val().is_some() {
                *bindgroups = PassBindGroups::new(None);
            }
        });
    }

/// 渲染器搜集渲染
    pub fn sys_pass_shader_request_by_model<T: TPass + Component, I: TPassID + Component>(
        models: Query<
            (
                ObjectID,
                &GeometryID, 
                &I, &RenderAlignment
            ),
            Or<(Changed<GeometryID>, Changed<RenderAlignment>)>,
        >,
        geometrys: Query<(&EVerticeExtendCodeComp, &VertexBufferLayoutsComp)>, 
        passes: Query<(&PassEffectReady, &PassBindGroups, &PassShader), With<T>>,
        mut shader_center: ResMut<AssetDataCenterShader3D>,
        mut shader_loader: ResMut<AssetLoaderShader3D>,
        device: Res<PiRenderDevice>,
    ) {
        // let time1 = pi_time::Instant::now();

        models.iter().for_each(
            |(_id_model, id_geo, passid, renderalignment)| {
                // log::debug!("SysPassShaderRequestByModel: 0");
                let id_pass = passid.id();
                let (instance, vb) = if let Ok(val) = geometrys.get(id_geo.0) {
                    val
                } else {
                    return;
                };
                if let Ok((ready, bindgroups, _old_shader)) = passes.get(id_pass.clone()) {
                    // log::debug!("SysPassShaderRequestByModel: 2");
                    // log::error!("Shader: 1");
                    if let (Some((key_meta, meta)), Some(bindgroups)) = (ready.val(), bindgroups.val()) {
                        // log::error!("Shader: 2");
                        // log::debug!("SysPassShaderRequestByModel: 3");
                        shader(
                            id_pass, meta, key_meta, &instance, vb, bindgroups, renderalignment, &mut shader_center, &mut shader_loader, &device
                        );
                    }
                }
            }
        );

        // log::debug!("SysPassShaderRequestByModel: {:?}", pi_time::Instant::now() - time1);
    }

/// 渲染器搜集渲染
    pub fn sys_pass_shader_request_by_pass<T: TPass + Component, I: TPassID + Component>(
        models: Query<
            (
                &GeometryID, &I, &RenderAlignment
            ),
        >,
        geometrys: Query<(&EVerticeExtendCodeComp, &VertexBufferLayoutsComp)>, 
        passes: Query<
            (ObjectID, &ModelPass, &PassEffectReady, &PassBindGroups, &PassShader, &T),
            Or<(Changed<PassEffectReady>, Changed<PassBindGroups>)>
        >,
        // mut commands: Commands,
        mut shader_center: ResMut<AssetDataCenterShader3D>,
        mut shader_loader: ResMut<AssetLoaderShader3D>,
        device: Res<PiRenderDevice>,
    ) {
        // let time1 = pi_time::Instant::now();

        passes.iter().for_each(|(id_pass, id_model, ready, bindgroups, _old_shader, _)| {
            // log::debug!("SysPassShaderRequestByPass: 0");
            if let (Some((key_meta, meta)), Some(bindgroups)) = (ready.val(), bindgroups.val()) {
                // log::debug!("SysPassShaderRequestByPass: 1");
                if let Ok((id_geometry, _passid, renderalignment)) = models.get(id_model.0) {
                    // log::debug!("SysPassShaderRequestByPass: 2");
                    let (instance, vb) = if let Ok(val) = geometrys.get(id_geometry.0) {
                        val
                    } else {
                        return;
                    };
                    shader(
                        id_pass, meta, key_meta, &instance, vb, bindgroups, renderalignment, &mut shader_center, &mut shader_loader, &device
                    );
                }
            }
        });

        // log::debug!("SysPassShaderRequestByPass: {:?}", pi_time::Instant::now() - time1);
    }

    pub fn sys_pass_shader_loaded(
        mut items: Query<&mut PassShader>,
        mut shader_center: ResMut<AssetDataCenterShader3D>,
        mut shader_loader: ResMut<AssetLoaderShader3D>,
    ) {
        // let time1 = pi_time::Instant::now();
        shader_center.single_create().iter().for_each(|(key, value)| {
            // log::debug!("PassShaderLoaded: 0");
            shader_loader.loaded(key, value).drain(..).for_each(|(entity, component)| {
                // log::debug!("PassShaderLoaded: 1");
                if let Ok(mut item) = items.get_mut(entity) {
                    *item = PassShader::from(component);
                }
            })
        });

        // log::trace!("SysPassShaderLoad: {:?}", pi_time::Instant::now() - time1);
    }

    pub fn sys_pass_pipeline_request_by_model<T: TPass + Component, I: TPassID + Component>(
        scenes: Query<&ScenePassRenderCfg>,
        models: Query<
            (
                &SceneID,
                (&CCullMode, &Topology, &CPolygonMode, &CFrontFace, &CUnClipDepth),
                &ModelBlend,
                &GeometryID, &I,
                (&DepthWrite, &DepthCompare, &DepthBias, &StencilFront, &StencilBack, &StencilRead, &StencilWrite)
            ),
            Or<(
                Changed<ModelBlend>, Changed<GeometryID>,
                Changed<CCullMode>, Changed<Topology>, Changed<CPolygonMode>, Changed<CFrontFace>, Changed<CUnClipDepth>,
                Changed<DepthWrite>, Changed<DepthCompare>, Changed<DepthBias>, Changed<StencilFront>, Changed<StencilBack>, Changed<StencilRead>, Changed<StencilWrite>
            )>
        >,
        geometrys: Query<&VertexBufferLayoutsComp>, 
        passes: Query<
            (&PassShader, &PassBindGroups, &PassPipeline, &T, & PassPipeline),
            With<T>
        >,
        // mut commands: Commands,
        mut pipeline_center: ResMut<AssetDataCenterPipeline3D>,
        mut pipeline_loader: ResMut<AssetLoaderPipeline3D>,
        device: Res<PiRenderDevice>,
    ) {
        // let time1 = pi_time::Instant::now();

        models.iter().for_each(| (
            idscene,
            (cull, topology, polygon, face, unclip_depth),
            blend, id_geo, passid, 
            (depth_write, compare, bias, stencil_front, stencil_back, stencil_read, stencil_write)
        ) |{
            let passcfgs = if let Ok(cfg) = scenes.get(idscene.0) {
                cfg.query(T::TAG)
            } else {
                return;
            };
            let id_pass = passid.id();
            if let Ok((shader, bindgroups, old_draw, _, _)) = passes.get(id_pass) {
                // log::debug!("SysPipeline: 0 Model");
                let vb = if let Ok(vb) = geometrys.get(id_geo.0.clone()) {
                    vb
                } else {
                    return;
                };

                // log::debug!("SysPipeline: 1 Model");
                if let (Some(shader), Some(bindgroups)) = (shader.val(), bindgroups.val()) {
                    pipeline(
                        shader, bindgroups, vb, passcfgs,
                        blend, depth_write, compare, bias, stencil_front, stencil_back, stencil_read, stencil_write,
                        cull, topology, polygon, face, unclip_depth,
                        id_pass, &mut pipeline_center, &mut pipeline_loader,
                        &device
                    );
                }
            }
        });

        // log::trace!("SysPassPipelineRequest: {:?}", pi_time::Instant::now() - time1);
    }

    pub fn sys_pass_pipeline_request_by_pass<T: TPass + Component, I: TPassID + Component>(
        scenes: Query<&ScenePassRenderCfg>,
        models: Query<
            (
                &SceneID,
                &GeometryID,
                (&CCullMode, &Topology, &CPolygonMode, &CFrontFace, &CUnClipDepth),
                &ModelBlend,
                (&DepthWrite, &DepthCompare, &DepthBias, &StencilFront, &StencilBack, &StencilRead, &StencilWrite)
            ),
        >,
        geometrys: Query<&VertexBufferLayoutsComp>, 
        passes: Query<
            (ObjectID, &ModelPass, &PassBindGroups, &PassShader, &T, & PassPipeline),
            Changed<PassShader>
        >,
        // mut commands: Commands,
        mut pipeline_center: ResMut<AssetDataCenterPipeline3D>,
        mut pipeline_loader: ResMut<AssetLoaderPipeline3D>,
        device: Res<PiRenderDevice>,
    ) {
        // let time1 = pi_time::Instant::now();

        passes.iter().for_each(|(id_pass, id_model, bindgroups, shader, _, _)| {
            // log::debug!("SysPipeline: 0 Pass");
            if let (Some(shader), Some(bindgroups)) = (shader.val(), bindgroups.val()) {
                // log::debug!("SysPipeline: 1 Pass");
                if let Ok((
                    idscene, id_geo, 
                    (cull, topology, polygon, face, unclip_depth),
                    blend,
                    (depth_write, compare, bias, stencil_front, stencil_back, stencil_read, stencil_write)
                )) = models.get(id_model.0) {
                    let passcfgs = if let Ok(cfg) = scenes.get(idscene.0) {
                        cfg.query(T::TAG)
                    } else {
                        return;
                    };

                    let vb = if let Ok(vb) = geometrys.get(id_geo.0.clone()) {
                        vb
                    } else {
                        return;
                    };
                    pipeline(
                        shader, bindgroups, vb, passcfgs,
                        blend, depth_write, compare, bias, stencil_front, stencil_back, stencil_read, stencil_write,
                        cull, topology, polygon, face, unclip_depth,
                        id_pass, &mut pipeline_center, &mut pipeline_loader,
                        &device
                    );
                }
            }
        });

        // log::trace!("SysPassPipelineRequest: {:?}", pi_time::Instant::now() - time1);
    }

    pub fn sys_pass_pipeline_loaded(
        // mut commands: Commands,
        mut passes: Query<&mut PassPipeline>,
        mut pipeline_center: ResMut<AssetDataCenterPipeline3D>,
        mut pipeline_loader: ResMut<AssetLoaderPipeline3D>,
    ) {
        // let time1 = pi_time::Instant::now();

        pipeline_center.single_create().iter().for_each(|(key, value)| {
            // log::debug!("SysPassPipeline: 0");
            pipeline_loader.loaded(key, value).drain(..).for_each(|(entity, component)| {
                // log::debug!("SysPassPipeline: 1");
                if let Ok(mut oldpipeline) = passes.get_mut(entity) {
                    *oldpipeline = PassPipeline::from(component);
                }
                // commands.entity(entity).insert(PassPipeline::from(component));
            })
        });

        // log::trace!("SysPassPipeline: {:?}", pi_time::Instant::now() - time1);
    }

    pub fn sys_pass_draw_modify_by_pass<T: TPass + Component, I: TPassID + Component>(
        models: Query<(&GeometryID, &IndiceRenderRange, &RenderGeometryEable, &InstanceSourceRefs, &DisposeReady)>,
        geometrys: Query<&RenderGeometryComp>,
        mut passes: Query<(ObjectID, &ModelPass, &PassBindGroups, &PassPipeline, &mut PassDraw, &T), Changed<PassPipeline>>,
        // mut commands: Commands,
    ) {
        // let time1 = pi_time::Instant::now();

        passes.iter_mut().for_each(|(_, id_model, bindgroups, pipeline, mut old_draw, _)| {
            if let (Some(bindgroups), Some(pipeline)) = (bindgroups.val(), pipeline.val()) {
                if let Ok((id_geo, renderindices, geoenable, instances, disposed)) = models.get(id_model.0) {
                    if geoenable.0 == false || disposed.0 == true {
                        if old_draw.val().is_some() { *old_draw = PassDraw(None); };
                        return;
                    }
        
                    if let Ok(RenderGeometryComp(Some(rendergeo))) = geometrys.get(id_geo.0.clone()) {
                        if rendergeo.isok() {
                            let draw = if instances.len() == 0 {
                                DrawObj3D::Draw( Arc::new(DrawObj {
                                    pipeline: Some(pipeline.clone()),
                                    bindgroups: bindgroups.groups(),
                                    vertices: rendergeo.vertices(),
                                    instances: rendergeo.instances(),
                                    vertex: rendergeo.vertex_range(),
                                    indices: renderindices.apply(rendergeo),
                                }))
                            } else {
                                DrawObj3D::Tmp( DrawObjTmp {
                                    pipeline: Some(pipeline.clone()),
                                    bindgroups: bindgroups.clone(),
                                    vertices: rendergeo.vertices(),
                                    instances: rendergeo.instances(),
                                    vertex: rendergeo.vertex_range(),
                                    indices: renderindices.apply(rendergeo),
                                })
                            };
                            
                            *old_draw = PassDraw(Some(draw));
                        } else {
                            *old_draw = PassDraw(None);
                        }
                        // log::warn!("PassDraw: {:?}", id_pass);
                        // log::debug!("PassDrawLoaded: 1 Pass");
                        // commands.entity(id_pass).insert(PassDraw(Some(Arc::new(draw))));
                    } else {
                        // log::warn!("PassDraw None: {:?}", id_pass);
                        *old_draw = PassDraw(None);
                        // if old_draw.0.is_some() { commands.entity(id_pass).insert(PassDraw(None)); }
                    }
                }
            } else {
                // log::warn!("PassDraw None: {:?}", id_pass);
                *old_draw = PassDraw(None);
                // if old_draw.0.is_some() { commands.entity(id_pass).insert(PassDraw(None)); }
            }
        });

        // log::trace!("SysPassDrawLoad: {:?}", pi_time::Instant::now() - time1);
    }

    pub fn sys_pass_draw_modify_by_model<T: TPass + Component, I: TPassID + Component>(
        models: Query<(&GeometryID, &I, &IndiceRenderRange, &RenderGeometryEable, &InstanceSourceRefs, &DisposeReady), Or<(Changed<RenderGeometryEable>, Changed<IndiceRenderRange>, Changed<DisposeReady>)>>,
        geometrys: Query<&RenderGeometryComp>,
        mut passes: Query<(&ModelPass, &PassBindGroups, &PassPipeline, &mut PassDraw, &T)>,
        // mut commands: Commands,
    ) {
        // let time1 = pi_time::Instant::now();

        models.iter().for_each(|(id_geo, id_pass, renderindices, geoenable, instances, disposed)| {

            if let Ok((_, bindgroups, pipeline, mut old_draw, _)) = passes.get_mut(id_pass.id()) {
                if geoenable.0 == false || disposed.0 == true { 
                    if old_draw.val().is_some() { *old_draw = PassDraw(None); };
                    return;
                }
                if let (Some(bindgroups), Some(pipeline)) = (bindgroups.val(), pipeline.val()) {
                    if let Ok(RenderGeometryComp(Some(rendergeo))) = geometrys.get(id_geo.0.clone()) {
                        if rendergeo.isok() {
                            let draw = if instances.len() == 0 {
                                DrawObj3D::Draw( Arc::new(DrawObj {
                                    pipeline: Some(pipeline.clone()),
                                    bindgroups: bindgroups.groups(),
                                    vertices: rendergeo.vertices(),
                                    instances: rendergeo.instances(),
                                    vertex: rendergeo.vertex_range(),
                                    indices: renderindices.apply(rendergeo),
                                }))
                            } else {
                                DrawObj3D::Tmp( DrawObjTmp {
                                    pipeline: Some(pipeline.clone()),
                                    bindgroups: bindgroups.clone(),
                                    vertices: rendergeo.vertices(),
                                    instances: rendergeo.instances(),
                                    vertex: rendergeo.vertex_range(),
                                    indices: renderindices.apply(rendergeo),
                                })
                            };
                            *old_draw = PassDraw(Some(draw));
                            // log::warn!("PassDraw: {:?}", id_pass.id());
                            // log::debug!("PassDrawLoaded: 1 Model");
                            // commands.entity(id_pass.id()).insert(PassDraw(Some(Arc::new(draw))));
                        } else {
                            *old_draw = PassDraw(None);
                        }
                    } else {
                        // log::warn!("PassDraw None: {:?}", id_pass.id());
                        *old_draw = PassDraw(None);
                        // if old_draw.0.is_some() { commands.entity(id_pass.id()).insert(PassDraw(None)); }
                    }
                }
            }
        });

        // log::trace!("SysPassDrawLoad: {:?}", pi_time::Instant::now() - time1);
    }

    pub fn sys_renderer_draws_modify(
        scenes: Query<&ScenePassRenderCfg>,
        mut renderers: Query<
            (
                ObjectID, &ViewerID, &mut Renderer, &PassTagOrders, &RendererEnable, &mut RenderSize
            )
        >,
        viewers: Query<
            (&SceneID, &ModelListAfterCulling, &ViewerSize, Option<&CameraViewport>, &ViewerGlobalPosition, &DisposeReady),
        >,
        models: Query<
            (
                &GlobalEnable, &DisposeReady, &GlobalTransform, &TransparentSortParam, &InstancedMeshTransparentSortCollection,
                (&PassID01, &PassID02, &PassID03, &PassID04, &PassID05, &PassID06, &PassID07, &PassID08)
            )
        >,
        passes: Query<
            (&PassDraw, &PassPipeline)
        >,
        mut record: ResMut<Performance>,
    ) {
        let time1 = pi_time::Instant::now();

        renderers.iter_mut().for_each(|(_id, id_viewer, mut renderer, passtag_orders, enable, mut rendersize)| {
            renderer.clear();
            // log::warn!("Renderer: {:?}, Camera {:?}, {:?}", id, id_viewer.0, enable.0);
            if enable.0 == false {
                return;
            }
            let mut count_vertex = 0;
            // let mut list_sort_opaque: Vec<(Arc<DrawObj>, f32, TransparentSortParam, u8, u64)> = vec![];
            // let mut list_sort_blend: Vec<(Arc<DrawObj>, f32, TransparentSortParam, u8, u64)> = vec![];
            let mut opaque_list: Vec<TmpSortDrawOpaque> = vec![];
            let mut transparent_list: Vec<TmpSortDrawTransparent> = vec![];
            let mut draws: Vec<Arc<DrawObj>> = vec![];
            if let Ok((idscene, list_model, viewersize, viewport, viewposition, disposed)) = viewers.get(id_viewer.0) {
                if disposed.0 { return; }

                if let Ok(passcfg) = scenes.get(idscene.0) {
                    *rendersize = RenderSize(viewersize.0, viewersize.1);

                    if let Some(viewport) = viewport {
                        renderer.draws.viewport = (viewport.x, viewport.y, viewport.w, viewport.h, viewport.mindepth, viewport.maxdepth);
                    } else {
                        renderer.draws.viewport = (0., 0., 1., 1., 0., 1.);
                    }
                    list_model.0.iter().for_each(|id_obj| {
                        if let Ok((globalenable, disposed, nodeposition, rendersort, instancessortinfo, passrecord)) = models.get(id_obj.clone()) {
                            // log::warn!("Renderer: A");
                            if disposed.0 == true || globalenable.0 == false { return; }
                            
                            let temp = nodeposition.position() - &viewposition.0;
                            let distance = temp.x * temp.x + temp.y * temp.y + temp.z * temp.z;
    
                            let mut index = 0;
                            for tag in passtag_orders.0.iter() {
                                let pass = tag.as_pass();

                                let is_transparent = passcfg.query(pass).blend();

                                // let list = if passcfg.query(pass).blend() {
                                //     &mut list_sort_blend
                                // } else {
                                //     &mut list_sort_opaque
                                // };

                                if pass == EPassTag::PASS_TAG_01 {
                                    if let Ok((PassDraw(Some(draw)), pipeline)) = passes.get(passrecord.0.0) {
                                        // log::warn!("Renderer: B {:?}, {:?}", tag, passrecord.0.0);
                                        // list.push((draw.clone(), distance, rendersort.clone(), index, pipeline.key()));
                                        collect_draw(
                                            is_transparent, index, pipeline.key(), distance, draw, rendersort,
                                            &mut opaque_list, &mut transparent_list, &instancessortinfo, &mut draws
                                        );
                                    }
                                }
                                else if pass == EPassTag::PASS_TAG_02 {
                                    if let Ok((PassDraw(Some(draw)), pipeline)) = passes.get(passrecord.1.0) {
                                        // log::warn!("Renderer: B {:?}, {:?}", tag, passrecord.1.0);
                                        // list.push((draw.clone(), distance, rendersort.clone(), index, pipeline.key()));
                                        collect_draw(
                                            is_transparent, index, pipeline.key(), distance, draw, rendersort,
                                            &mut opaque_list, &mut transparent_list, &instancessortinfo, &mut draws
                                        );
                                    }
                                }
                                else if pass == EPassTag::PASS_TAG_03 {
                                    if let Ok((PassDraw(Some(draw)), pipeline)) = passes.get(passrecord.2.0) {
                                        // log::warn!("Renderer: B {:?}, {:?}", tag, passrecord.2.0);
                                        // list.push((draw.clone(), distance, rendersort.clone(), index, pipeline.key()));
                                        collect_draw(
                                            is_transparent, index, pipeline.key(), distance, draw, rendersort,
                                            &mut opaque_list, &mut transparent_list, &instancessortinfo, &mut draws
                                        );
                                    }
                                }
                                else if pass == EPassTag::PASS_TAG_04 {
                                    if let Ok((PassDraw(Some(draw)), pipeline)) = passes.get(passrecord.3.0) {
                                        // log::warn!("Renderer: B {:?}, {:?}", tag, passrecord.3.0);
                                        // list.push((draw.clone(), distance, rendersort.clone(), index, pipeline.key()));
                                        collect_draw(
                                            is_transparent, index, pipeline.key(), distance, draw, rendersort,
                                            &mut opaque_list, &mut transparent_list, &instancessortinfo, &mut draws
                                        );
                                    }
                                }
                                else if pass == EPassTag::PASS_TAG_05 {
                                    if let Ok((PassDraw(Some(draw)), pipeline)) = passes.get(passrecord.4.0) {
                                        // log::warn!("Renderer: B {:?}, {:?}", tag, passrecord.4.0);
                                        // list.push((draw.clone(), distance, rendersort.clone(), index, pipeline.key()));
                                        collect_draw(
                                            is_transparent, index, pipeline.key(), distance, draw, rendersort,
                                            &mut opaque_list, &mut transparent_list, &instancessortinfo, &mut draws
                                        );
                                    }
                                }
                                else if pass == EPassTag::PASS_TAG_06 {
                                    if let Ok((PassDraw(Some(draw)), pipeline)) = passes.get(passrecord.5.0) {
                                        // log::warn!("Renderer: B {:?}, {:?}", tag, passrecord.5.0);
                                        // list.push((draw.clone(), distance, rendersort.clone(), index, pipeline.key()));
                                        collect_draw(
                                            is_transparent, index, pipeline.key(), distance, draw, rendersort,
                                            &mut opaque_list, &mut transparent_list, &instancessortinfo, &mut draws
                                        );
                                    }
                                }
                                else if pass == EPassTag::PASS_TAG_07 {
                                    if let Ok((PassDraw(Some(draw)), pipeline)) = passes.get(passrecord.6.0) {
                                        // log::warn!("Renderer: B {:?}, {:?}", tag, passrecord.6.0);
                                        // list.push((draw.clone(), distance, rendersort.clone(), index, pipeline.key()));
                                        collect_draw(
                                            is_transparent, index, pipeline.key(), distance, draw, rendersort,
                                            &mut opaque_list, &mut transparent_list, &instancessortinfo, &mut draws
                                        );
                                    }
                                }
                                else if pass == EPassTag::PASS_TAG_08 {
                                    if let Ok((PassDraw(Some(draw)), pipeline)) = passes.get(passrecord.7.0) {
                                        // log::warn!("Renderer: B {:?}, {:?}", tag, passrecord.7.0);
                                        // list.push((draw.clone(), distance, rendersort.clone(), index, pipeline.key()));
                                        collect_draw(
                                            is_transparent, index, pipeline.key(), distance, draw, rendersort,
                                            &mut opaque_list, &mut transparent_list, &instancessortinfo, &mut draws
                                        );
                                    }
                                }
    
                                index += 1;
                            }
                        }
                    });

                    opaque_list.sort();
                    transparent_list.sort();

                    opaque_list.iter().for_each(|tmp| {
                        if let Some(draw) = draws.get(tmp.idx as usize) {
                            renderer.draws.list.push(draw.clone());
                            let vertex = if let Some(indices) = &draw.indices {
                                indices.value_range().end - indices.value_range().start
                            } else { draw.vertex.end - draw.vertex.start };
                            count_vertex += (vertex * (draw.instances.end - draw.instances.start)) as usize;
                        }
                    });
                    transparent_list.iter().for_each(|tmp| {
                        if let Some(draw) = draws.get(tmp.idx as usize) {
                            renderer.draws.list.push(draw.clone());
                            let vertex = if let Some(indices) = &draw.indices {
                                indices.value_range().end - indices.value_range().start
                            } else { draw.vertex.end - draw.vertex.start };
                            count_vertex += (vertex * (draw.instances.end - draw.instances.start)) as usize;
                        }
                    });

                    // log::warn!("Renderer Draw {:?} {:?}", list_model.0.len(), renderer.draws.list.len());
                }
            }

            renderer.vertexs = count_vertex;
        });

        record.drawobjs = (pi_time::Instant::now() - time1).as_micros() as u32;
        // log::trace!("SysRendererDraws: {:?}", pi_time::Instant::now() - time1);
    }


fn shader(
    id_pass: Entity,
    meta: &Handle<ShaderEffectMeta>,
    key_meta: &Atom,
    instance: &EVerticeExtendCode,
    vb: &VertexBufferLayoutsComp,
    bindgroups: &BindGroups3D,
    renderalignment: &RenderAlignment,
    shader_center: &mut AssetDataCenterShader3D,
    shader_loader: &mut AssetLoaderShader3D,
    device: &RenderDevice,
) {
    
    // log::error!("Shader: {:?}", 2);
    let key_attributes = vb.as_key_shader_from_attributes();
    // let key_shader_defines = 0;

    let key_set_blocks = bindgroups.key_set_blocks();

    let key_shader = KeyShader3D {
        key_meta: key_meta.clone(),
        key_attributes,
        key_set_blocks,
        defines: 0,
        renderalignment: renderalignment.0
    };

    let (set0, set1, set2) = (&bindgroups.scene, &bindgroups.model, bindgroups.textures.as_ref());
    let mut vs_defines = vec![];
    vs_defines.push(set0.vs_define_code());
    vs_defines.push(set1.vs_define_code());
    let mut fs_defines = vec![];
    fs_defines.push(set0.fs_define_code());
    fs_defines.push(set1.fs_define_code());
    // let set2 = 
    if let Some(set2) = set2 {
        vs_defines.push(set2.vs_define_code());
        fs_defines.push(set2.fs_define_code());
        // Some(set2.as_ref())
    }
    //  else { None };

    if shader_center.request(&key_shader, None) {
        // log::debug!("SysPassShaderRequestByModel: 4");
        shader_loader.request(id_pass, &key_shader);
    } else {
        // log::debug!("SysPassShaderRequestByPass: 4");
        if !shader_center.check(&key_shader) {
            let shader = meta.build_2(
                &device,
                &key_meta,
                &key_shader.key_attributes,
                &instance,
                &renderalignment,
                &set1.key().key.skin,
                &vs_defines,
                &[], &[],
                &fs_defines,
                &[], &[],
            );

            // let shader = meta.build(&device, &key_shader.key_meta, &key_shader.key_attributes, &instance, set0.as_ref(), set1.as_ref(), set2, None);
            shader_center.add(&key_shader, shader, None);
        }
        shader_loader.request(id_pass, &key_shader);
    }
}

fn pipeline(
    shader: &Handle<Shader3D>,
    bindgroups: &BindGroups3D,
    vb: &VertexBufferLayoutsComp,
    passcfgs: &PassRenderInfo,
    blend: &ModelBlend,
    depth_write: &DepthWrite, compare: &DepthCompare, bias: &DepthBias, stencil_front: &StencilFront, stencil_back: &StencilBack, stencil_read: &StencilRead, stencil_write: &StencilWrite,
    cull: &CCullMode, topology: &Topology, polygon: &CPolygonMode, face: &CFrontFace, unclip_depth: &CUnClipDepth,
    id_pass: Entity,
    pipeline_center: &mut AssetDataCenterPipeline3D,
    pipeline_loader: &mut AssetLoaderPipeline3D,
    device: &RenderDevice,
) {
    
    let key_shader = shader.key().clone();
    let bind_group_layouts = bindgroups.bind_group_layouts();
    let key_bindgroup_layouts = KeyPipelineFromBindGroup(bindgroups.key_bindgroup_layouts());

    let key_vertex_layouts = KeyPipelineFromAttributes::new(vb.0.clone());

    let pass_blend = passcfgs.blend();
    let pass_color_format = passcfgs.color_format();
    let pass_depth_format = passcfgs.depth_format();
    let blend = if pass_blend { blend.clone() } else { ModelBlend::default() };
    let depth_stencil = if let Some(pass_depth_format) = pass_depth_format {
        Some(
            depth_stencil_state(
                pass_depth_format,
                depth_write, compare, bias, stencil_front, stencil_back, stencil_read, stencil_write
            )
        )
    } else { None };

    let targets = RenderTargetState::color_target(pass_color_format, &blend);
    let key_state = KeyRenderPipelineState {
        primitive: PrimitiveState::state(cull, topology, polygon, face, unclip_depth),
        target_state: targets[0].clone(),
        depth_stencil: depth_stencil,
        multisample: wgpu::MultisampleState { count: 1, mask: !0, alpha_to_coverage_enabled: false }
    };

    // log::warn!("{:?}", key_state);

    let key_pipeline = KeyPipeline3D {
        key_state,
        key_shader,
        key_bindgroup_layouts,
        key_vertex_layouts,
    };

    let key_u64 = key_pipeline.to_u64();

    if pipeline_center.request(&key_u64, None) {
        // log::debug!("SysPipeline: 3 Pass");
        // *oldpipeline = PassPipeline::new(Some(pipeline));
        pipeline_loader.request(id_pass, &key_u64);
    } else {
        // log::debug!("SysPipeline: 4 Pass");
        if !pipeline_center.check(&key_u64) {
            // log::warn!("SysPassPipeline: {:?}", key_pipeline);
            let pipeline = KeyPipeline3D::create(key_pipeline, shader.clone(), bind_group_layouts, &device);
            pipeline_center.add(&key_u64, pipeline, None);
        }
        pipeline_loader.request(id_pass, &key_u64);
    }
}

fn collect_draw(
    is_transparent: bool,
    pass: u8,
    pipeline: u64,
    distance: f32,
    draw: &DrawObj3D,
    sort_param: &TransparentSortParam,
    opaque_list: &mut Vec<TmpSortDrawOpaque>,
    transparent_list: &mut Vec<TmpSortDrawTransparent>,
    instancessortinfo: &InstancedMeshTransparentSortCollection,
    draws: &mut Vec<Arc<DrawObj>>,
) {
    if is_transparent == false {
        let index = draws.len();
        let tmpdraw =  match draw {
            DrawObj3D::Tmp(draw) => {
                Arc::new(DrawObj {
                    pipeline: draw.pipeline.clone(),
                    bindgroups: draw.bindgroups.groups(),
                    vertices: draw.vertices.clone(),
                    instances: draw.instances.clone(),
                    vertex: draw.vertex.clone(),
                    indices: draw.indices.clone(),
                })
            },
            DrawObj3D::Draw(draw) => draw.clone(),
        };
        draws.push(tmpdraw);
        opaque_list.push(TmpSortDrawOpaque { idx: index as u16, pass, distance, pipeline });
    } else {
        match draw {
            DrawObj3D::Tmp(draw) => {
                if instancessortinfo.0.len() <= 1 || draw.instances.end == 1 {
                    let index = draws.len();
                    let tmpdraw = DrawObj {
                        pipeline: draw.pipeline.clone(),
                        bindgroups: draw.bindgroups.groups(),
                        vertices: draw.vertices.clone(),
                        instances: draw.instances.clone(),
                        vertex: draw.vertex.clone(),
                        indices: draw.indices.clone(),
                    };
                    draws.push(Arc::new(tmpdraw));
                    transparent_list.push(TmpSortDrawTransparent { idx: index as u16, pass, distance, pipeline, queue: sort_param.clone() });
                } else {
                    instancessortinfo.0.iter().for_each(|(alphaindex, range)| {
                        if range.start < range.end && range.end <= draw.instances.end {
                            let tmpdraw = DrawObj {
                                pipeline: draw.pipeline.clone(),
                                bindgroups: draw.bindgroups.groups(),
                                vertices: draw.vertices.clone(),
                                instances: range.clone(),
                                vertex: draw.vertex.clone(),
                                indices: draw.indices.clone(),
                            };
                            let index = draws.len();
                            draws.push(Arc::new(tmpdraw));
                            let mut queue = sort_param.clone();
                            queue.index += *alphaindex;
                            transparent_list.push(TmpSortDrawTransparent { idx: index as u16, pass, distance, pipeline, queue });
                        }
                    });
                }
            },
            DrawObj3D::Draw(draw) => {
                let index = draws.len();
                draws.push(draw.clone());
                transparent_list.push(TmpSortDrawTransparent { idx: index as u16, pass, distance, pipeline, queue: sort_param.clone() });
            }
        };
    }
}

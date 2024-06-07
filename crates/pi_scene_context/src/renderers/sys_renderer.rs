use std::{ops::Range, sync::Arc};

use pi_scene_shell::prelude::*;
use crate::{
    bindgroup::*, flags::*, geometry::{instance::instanced_buffer::InstanceBufferAllocator, prelude::*}, meshes::prelude::*, pass::*, transforms::prelude::*, viewer::prelude::*
};

use super::{
    base::*, render_blend::*, render_depth_and_stencil::*, render_object::RenderState, render_primitive::*, render_sort::*, render_target_state::*, renderer::*
};


/// 渲染器搜集渲染
    pub fn sys_pass_bind_groups(
        mut passes: Query<
            (ObjectID, &PassModelID, &PassEffectReady, &PassBindGroupScene, &PassBindGroupModel, &PassBindGroupTextureSamplers, &PassBindGroupLightingShadow, &mut PassBindGroups),
            Or<(Changed<PassEffectReady>, Changed<PassBindGroupScene>, Changed<PassBindGroupModel>, Changed<PassBindGroupTextureSamplers>, Changed<PassBindGroupLightingShadow>)>
        >,
    ) {
        passes.iter_mut().for_each(|(_id_pass, _id_model, ready, set0, set1, set2, set_3, mut bindgroups)| {
            if let Some((_key_meta, meta)) = ready.val() {
                let set0 = match (BindDefines::need_bind_group_set0(meta.binddefines), set0.val()) {
                    (true, Some(set)) => Some(set.clone()),
                    (false, _) => None,
                    _ => {
                        if bindgroups.val().is_some() { *bindgroups = PassBindGroups::new(None); }
                        // log::warn!("Bindgroups: Set0 fail");
                        return;
                    }
                };
                let set1 = match (BindDefines::need_bind_group_set1(meta.binddefines), set1.val()) {
                    (true, Some(set)) => Some(set.clone()),
                    (false, _) => None,
                    _ => {
                        if bindgroups.val().is_some() { *bindgroups = PassBindGroups::new(None); }
                        // log::warn!("Bindgroups: Set1 fail");
                        return;
                    }
                };
                let need_set2 = meta.textures.len() > 0;
                let need_set3 = BindDefines::need_bind_group_set3(meta.binddefines);

                let textures = match (need_set2, set2.val()) {
                    (true, Some(val)) => Some(val.clone()),
                    (false, _) => None,
                    _ => {
                        if bindgroups.val().is_some() { *bindgroups = PassBindGroups::new(None); }
                        // log::warn!("Bindgroups: textures fail");
                        return;
                    }
                };
                let lightshadow = match (need_set3, set_3.val()) {
                    (true, Some(val)) => {
                        Some(val.clone())
                    },
                    (false, _) => None,
                    _ => {
                        if bindgroups.val().is_some() { *bindgroups = PassBindGroups::new(None); }
                        return;
                    }
                };

                // log::warn!("Bindgroups: {:?}", (true, bindgroups.0.is_some()));
                let data = BindGroups3D::create(set0, set1, textures, lightshadow);
                *bindgroups = PassBindGroups::new(Some(data));
            } else {
                if bindgroups.val().is_some() { *bindgroups = PassBindGroups::new(None); }
                // log::warn!("Bindgroups: Ready False");
            }
        });
    }

/// 渲染器搜集渲染
    pub fn sys_pass_shader_request_by_model(
        models: Query<
            (
                &GeometryID, &PassIDs
            ),
            Or<(Changed<GeometryID>, Changed<RenderAlignment>)>,
        >,
        mut passes: Query<&mut PassGeometryID>,
    ) {
        // let time1 = pi_time::Instant::now();

        models.iter().for_each(
            |(id_geo, passids)| {
                passids.0.iter().for_each(|id| {
                    if let Ok(mut idgeometry) = passes.get_mut(*id) { *idgeometry = PassGeometryID(id_geo.0); }
                });
            }
        );

        // log::debug!("SysPassShaderRequestByModel: {:?}", pi_time::Instant::now() - time1);
    }
    pub fn sys_pass_shader_request_by_geometry(
        models: Query<
            (&GeometryID, &PassIDs),
        >,
        geometrys: Query<(Entity, &MeshID), Changed<VertexBufferLayoutsComp>>,
        mut passes: Query<&mut PassGeometryID>,
    ) {
        // let time1 = pi_time::Instant::now();
        geometrys.iter().for_each(|(entity, idmesh)| {
            if let Ok((id_geo, passids)) = models.get(idmesh.0) {
                if entity == id_geo.0 {
                    passids.0.iter().for_each(|id| {
                        if let Ok(mut idgeometry) = passes.get_mut(*id) { *idgeometry = PassGeometryID(id_geo.0); }
                    });
                }
            }
        });

        // log::debug!("SysPassShaderRequestByModel: {:?}", pi_time::Instant::now() - time1);
    }

/// 渲染器搜集渲染
    pub fn sys_pass_shader(
        models: Query<&RenderAlignment>,
        geometrys: Query<&VertexBufferLayoutsComp>, 
        mut passes: Query<
            (ObjectID, &DisposeReady, &PassModelID, &PassGeometryID, &PassEffectReady, &PassBindGroups, &mut PassShader),
            Or<(Changed<PassBindGroups>, Changed<PassGeometryID>)>
        >,
        assets: Res<ShareAssetMgr<Shader3D>>,
        device: Res<PiRenderDevice>,
    ) {
        // let time1 = pi_time::Instant::now();

        passes.iter_mut().for_each(|(id_pass, disposeready, id_model, id_geo, ready, bindgroups, mut old_shader)| {
            if disposeready.0 == true { return; }

            // log::debug!("SysPassShaderRequestByPass: 0");
            if let (Some((key_meta, meta)), Some(bindgroups)) = (ready.val(), bindgroups.val()) {
                // log::debug!("SysPassShaderRequestByPass: 1");
                match (models.get(id_model.0), geometrys.get(id_geo.0)) {
                    (Ok(renderalignment), Ok(vb)) => {

                        let limit = device.0.limits();
                        if vb.0.attrcount as u32 <= limit.max_vertex_attributes && vb.0.desccount as u32 <= limit.max_vertex_buffers {
                            let renderalignment = renderalignment.shader_tag(false);
                            if let Ok(shader) = shader(
                                id_pass, meta, key_meta, vb, bindgroups, renderalignment, &assets, &device
                            ) {
                                // log::error!("Shader Success");
                    
                                if let Some(old) = &old_shader.0 {
                                    if old.key() != shader.key() { *old_shader = PassShader(Some(shader)) }
                                } else { *old_shader = PassShader(Some(shader)) }
                            } else {
                                // log::error!("Shader Fail");
                                if old_shader.0.is_some() {
                                    *old_shader = PassShader(None);
                                }
                            }
                        } else {
                            // log::error!("MAX_ATTRIBUTES: {}, Using Attributes: {}, MAX_BUFFER: {}, Using Buffers: {}", limit.max_vertex_attributes, vb.0.attrcount, limit.max_vertex_buffers, vb.0.desccount);
                            *old_shader = PassShader(None);
                        }
                    },
                    _ => {
                        // log::error!("Shader Fail Geometry");
                        if old_shader.0.is_some() {
                            *old_shader = PassShader(None);
                        }
                    }
                };
            }
        });

        // log::debug!("SysPassShaderRequestByPass: {:?}", pi_time::Instant::now() - time1);
    }

    pub fn sys_pass_pipeline_request_by_model(
        models: Query<
            (&DisposeReady, &PassIDs),
            Or<(
                Changed<GeometryID>,
            )>
        >,
        mut passes: Query<&mut PassPipelineStateDirty, With<PassPipeline>>,
    ) {
        // let time1 = pi_time::Instant::now();

        models.iter().for_each(|(disposeenable, passids)|{
            if disposeenable.0 == true { return; }
            passids.0.iter().for_each(|id| {
                if let Ok(mut flag ) = passes.get_mut(*id) { *flag = PassPipelineStateDirty; }
            });
        });

        // // log::trace!("SysPassPipelineRequest: {:?}", pi_time::Instant::now() - time1);
    }

    pub fn sys_pass_pipeline_request_by_renderer(
        renderers: Query<(&RendererParam, &ViewerID, &PassTag), Or<(Changed<RendererParam>, Changed<PassTag>)>>,
        viewers: Query<(&ModelList, &ForceIncludeModelList)>,
        modelspass: Query<&PassIDs>,
        mut passes: Query<&mut PassPipelineStateDirty>,
    ) {
        // let time1 = pi_time::Instant::now();
        renderers.iter().for_each(|(param, idviewer, passtag)| {
            if param.enable.0 {
                if let Ok((modellist, forcemodels)) = viewers.get(idviewer.0) {
                    modellist.0.iter().for_each(|idmodel| {
                        if let Ok(passid) = modelspass.get(*idmodel) { _pass_pipeline_request_by_renderer(passid.0[passtag.index()], &mut passes); }
                    });
                    forcemodels.0.iter().for_each(|idmodel| {
                        if let Ok(passid) = modelspass.get(*idmodel) { _pass_pipeline_request_by_renderer(passid.0[passtag.index()], &mut passes); }
                    });
                }
            }
        });
    }
    
    #[inline(never)]
    fn _pass_pipeline_request_by_renderer(
        passid: Entity,
        passes: &mut Query<&mut PassPipelineStateDirty>,
    ) {
        if let Ok( mut flag ) = passes.get_mut(passid) { *flag = PassPipelineStateDirty; }
    }

    pub fn sys_pass_pipeline(
        renderers: Query<&RendererParam>,
        models: Query<&GeometryID>,
        geometrys: Query<&VertexBufferLayoutsComp>, 
        mut passes: Query<
            (
                ObjectID, &DisposeReady, &PassModelID, &PassBindGroups, &PassShader, &mut PassPipeline, &PassRendererID,
                &RenderState,
            ),
            Or<(
                Changed<PassShader>, Changed<PassPipelineStateDirty>, Changed<PassRendererID>,
                Changed<RenderState>
            )>
        >,
        assets: ResMut<ShareAssetMgr<Pipeline3D>>,
        device: Res<PiRenderDevice>,
        mut errors: ResMut<ErrorRecord>,
    ) {
        // let time1 = pi_time::Instant::now();

        passes.iter_mut().for_each(|(
            id_pass, disposeready, id_model, bindgroups, shader, mut oldpipeline, idrenderer,
            renderstate
        )| {
            if disposeready.0 == true { return; }
            // log::warn!("SysPipeline: 0 Pass");
            if let (Some(shader), Some(bindgroups)) = (shader.val(), bindgroups.val()) {
                // log::warn!("SysPipeline: 1 Pass");
                if let Ok(id_geo) = models.get(id_model.0) {
                    // log::warn!("SysPipeline: 2 Pass {:?}", (geometrys.get(id_geo.0).is_ok(), renderers.get(idrenderer.0).is_ok()));
                    match (geometrys.get(id_geo.0), renderers.get(idrenderer.0)) {
                        (Ok(vb), Ok(param)) => {
                            let blend = renderstate.blend.clone(); // if blendenable.0 { blend.clone() } else { ModelBlend::default() };
                            if let Ok(pipeline) = pipeline(
                                shader, bindgroups, vb, &param.colorformat.0, &param.depthstencilformat.0,
                                blend, &renderstate.depth, &renderstate.stencil,
                                &renderstate.primitive,
                                id_pass, & assets, &device
                            ) {
                                // log::warn!("SysPipeline: {:?}", (colorformat, passtag, passpasstag));
                                // *oldpipeline = PassPipeline(Some(pipeline));
                                if let Some(old) = &oldpipeline.0 {
                                    if old.key() != pipeline.key() { *oldpipeline = PassPipeline(Some(pipeline)); }
                                } else { *oldpipeline = PassPipeline(Some(pipeline)); }
                            } else {
                                errors.record(id_model.0, ErrorRecord::ERROR_PASS_PIPELINE_FAIL);
                                if oldpipeline.0.is_some() { *oldpipeline = PassPipeline(None); }
                            }
                        },
                        _ => { 
                            if oldpipeline.0.is_some() { *oldpipeline = PassPipeline(None); }
                        }
                    }
                } else {
                    if oldpipeline.0.is_some() { *oldpipeline = PassPipeline(None); }
                }
            } else {
                if oldpipeline.0.is_some() { *oldpipeline = PassPipeline(None); }
            }
        });

        // // log::trace!("SysPassPipelineRequest: {:?}", pi_time::Instant::now() - time1);
    }


    pub fn sys_pass_draw_modify_by_model(
        models: Query<
            (&PassIDs), 
            Or<(Changed<RenderGeometryEable>, Changed<IndiceRenderRange>, Changed<VertexRenderRange>, Changed<DisposeReady>)>
        >,
        mut passes: Query<&mut PassDrawDirty>,
    ) {
        models.iter().for_each(|(passids)| {
            passids.0.iter().for_each(|id| {
                if let Ok(mut drawdirty) = passes.get_mut(*id) { *drawdirty = PassDrawDirty; }
            });
        });

        // // log::trace!("SysPassDrawLoad: {:?}", pi_time::Instant::now() - time1);
    }
    
    pub fn sys_pass_draw_modify_by_pass(
        models: Query<(&GeometryID, &IndiceRenderRange, &VertexRenderRange, &RenderGeometryEable, &InstanceSourceRefs, &DisposeReady)>,
        geometrys: Query<(&RenderGeometryComp, &GeometryResourceHash)>,
        mut passes: Query<(Entity, &PassModelID, &PassBindGroups, &PassPipeline, &mut PassDraw), Or<(Changed<PassPipeline>, Changed<PassDrawDirty>, Changed<PassBindGroups>, Changed<PassModelID>)>>,
        // mut commands: Commands,
    ) {
        passes.iter_mut().for_each(|(entity, id_model, bindgroups, pipeline, mut old_draw)| {
            if let (Some(bindgroups), Some(pipeline)) = (bindgroups.val(), pipeline.val()) {
                if let Ok((id_geo, renderindices, rendervertex, geoenable, instances, disposed)) = models.get(id_model.0) {
                    if geoenable.0 == false || disposed.0 == true {
                        if old_draw.val().is_some() { *old_draw = PassDraw(None); };
                        return;
                    }

                    if let Ok((RenderGeometryComp(Some(rendergeo)), geohash)) = geometrys.get(id_geo.0.clone()) {
                        if rendergeo.isok() {
                            // let draw = if instances.len() == 0 {
                            //     DrawObj3D::Draw( Arc::new(DrawObj {
                            //         pipeline: Some(pipeline.clone()),
                            //         bindgroups: bindgroups.groups(),
                            //         vertices: rendergeo.vertices(),
                            //         instances: rendergeo.instances(),
                            //         vertex: rendervertex.apply(rendergeo),
                            //         indices: renderindices.apply(rendergeo),
                            //     }))
                            // } else {
                            //     DrawObj3D::InstanceNotClip( DrawObjTmp {
                            //         pipeline: Some(pipeline.clone()),
                            //         bindgroups: bindgroups.clone(),
                            //         vertices: rendergeo.vertices(),
                            //         instances: rendergeo.instances(),
                            //         instance_memory: rendergeo.instance_memory.clone(),
                            //         vertex: rendervertex.apply(rendergeo),
                            //         indices: renderindices.apply(rendergeo),
                            //     })
                            // };
                            let draw = DrawObjTmp {
                                pipeline: pipeline.key().clone(),
                                passentity: entity,
                                bindgroupshash: BindGroups3DHashResource::from(bindgroups),
                                vertexentity: id_geo.0.clone(),
                                vertexhash: geohash.clone(),
                                instance_memory: rendergeo.instance_memory.clone(),
                                indice_range: renderindices.clone(),
                                vertex_range: rendervertex.clone(),
                            };

                            *old_draw = PassDraw(Some(draw));
                        } else {
                            *old_draw = PassDraw(None);
                        }
                        // log::debug!("PassDrawLoaded: 1 Pass");
                    } else {
                        // log::warn!("PassDraw None: {:?}", id_pass);
                        *old_draw = PassDraw(None);
                    }
                }
            } else {
                // log::warn!("PassDraw None: {:?}", id_pass);
                *old_draw = PassDraw(None);
            }
        });

        // // log::trace!("SysPassDrawLoad: {:?}", pi_time::Instant::now() - time1);
    }

    pub fn sys_renderer_draws_modify(
        mut renderers: Query< ( ObjectID, &SceneID, &ViewerID, &mut Renderer, &PassTag, &RendererParam ) >,
        viewers: Query< (&ModelListAfterCulling, &ViewerGlobalPosition, &ViewerDirection, &DisposeReady, &ViewerDistanceCompute), >,
        scenes: Query< (&BatchParamOpaque, &BatchParamTransparent) >,
        models: Query<
            (
                &GlobalEnable, &DisposeReady, &GlobalMatrix, &TransparentSortParam, &InstancedMeshTransparentSortCollection,
                &PassIDs
            )
        >,
        passes: Query<
            (&PassDraw, &PassRendererID)
        >,
        passbindgroups: Query<(&PassPipeline, &PassBindGroups)>,
        geometrys: Query<&RenderGeometryComp>,
        mut instancedcache: ResMut<InstanceBufferAllocator>,
        mut record: ResMut<Performance>,
        mut allocator: ResMut<VertexBufferAllocator3D>,
        device: Res<PiRenderDevice>,
        queue: Res<PiRenderQueue>,
    ) {
        let time1 = pi_time::Instant::now();

        renderers.iter_mut().for_each(|(_id_renderer, idscene, id_viewer, mut renderer, passtag, param)| {
            renderer.clear();
            // log::warn!("Renderer: {:?}, Camera {:?}, {:?}", _id, id_viewer.0, enable.0);
            if param.enable.0 == false {
                return;
            }
            let mut count_vertex = 0;
            // let mut list_sort_opaque: Vec<(Arc<DrawObj>, f32, TransparentSortParam, u8, u64)> = vec![];
            // let mut list_sort_blend: Vec<(Arc<DrawObj>, f32, TransparentSortParam, u8, u64)> = vec![];
            let mut opaque_list: Vec<TmpSortDrawOpaque> = vec![];
            let mut transparent_list: Vec<TmpSortDrawTransparent> = vec![];
            let mut draws: Vec<DrawObjTmp> = vec![];
            if let (Ok((list_model, viewposition, viewdirection, disposed, distancecomp)), Ok((batchopaque, batchtransparent))) = (viewers.get(id_viewer.0), scenes.get(idscene.0)) {
                if disposed.0 { return; }

                    // log::warn!("renderer_draws : ModelListAfterCulling: {:?}, ", (list_model.0.len()));
                    renderer.draws.viewport = param.viewport.val();
                    let mut countmesh = 0;
                    list_model.0.iter().for_each(|id_obj| {
                        if let Ok((globalenable, disposed, nodeposition, rendersort, instancessortinfo, passids)) = models.get(id_obj.clone()) {
                            // log::warn!("Renderer: A {:?}", (disposed.0, globalenable.0));
                            if disposed.0 == true || globalenable.0 == false { return; }
                            let passids = passids.0;

                            let index = 0;
                            let is_transparent = param.blend.0;
                            // log::error!("is_transparent {:?}", (is_transparent));
                            if passtag.index() < passids.len() {
                                let passid = passids[passtag.index()];
                                if let Ok((PassDraw(Some(draw)), passrendererid)) = passes.get(passid) {
                                    if passrendererid.0 == _id_renderer {
                                        let distance = 0.;
                                        if is_transparent {
                                            if batchtransparent.0.distance {
                                                distancecomp.distance(&viewposition.0, &viewdirection.0, &nodeposition.position());
                                            }
                                        } else {
                                            if batchopaque.0.distance {
                                                distancecomp.distance(&viewposition.0, &viewdirection.0, &nodeposition.position());
                                            }
                                        }

                                        collect_draw(
                                            is_transparent, index, draw.pipeline, distance, draw, rendersort,
                                            &mut opaque_list, &mut transparent_list, &instancessortinfo, &mut draws
                                        );
                                    } else {
                                        // log::error!("PassDraw Renderer Error {:?}", (passtag));
                                    }
                                } else {
                                    // log::error!("PassDraw Error {:?}", (passtag, passid));
                                }
                            } else {
                                // log::error!("passtag.index() < passids.len() fail .");
                            }
                            countmesh += 1;
                        } else {
                            // log::warn!("models.get Fail");
                        }
                    });

                    opaque_list.sort();
                    transparent_list.sort();

                    // log::warn!("Mesh: {:?}", countmesh);
                    // log::warn!("Opaque: {:?}", opaque_list.len());
                    // log::warn!("Transparent: {:?}", transparent_list.len());

                    let mut lastdraw: Option<DrawObjTmp> = None;
                    opaque_list.iter().for_each(|tmp| {
                        // log::warn!("{:?}", tmp);
                        if let Some(draw) = draws.get(tmp.idx as usize) {

                            if let Some(tempdraw) = &mut lastdraw {
                                if tempdraw.can_batch_instance_memory(draw, true, instancedcache.one_mesh_max_instance_bytes()) {
                                    let last = tempdraw.instance_memory.as_mut().unwrap();
                                    let curr = draw.instance_memory.as_ref().unwrap();

                                    curr.data.iter().for_each(|v| {
                                        last.data.push(*v);
                                    });
                                    last.itemcount += curr.itemcount;
                                } else {
                                    // lastdraw 转 DrawObj
                                    if let (Ok((pipeline, bindgroups)), Ok(geo)) = (passbindgroups.get(tempdraw.passentity), geometrys.get(tempdraw.vertexentity)) {
                                        if let (Some(bindgroups), Some(geo)) = (&bindgroups.0, &geo.0) {
                                            collect_draw_batch(tempdraw, &pipeline.0, &bindgroups.0, geo, &mut renderer, &mut instancedcache, &mut allocator, &device, &queue, &mut count_vertex);
                                        } else {
                                            // log::error!("DrawObj data fail 2.");
                                        }
                                    } else {
                                        // log::error!("DrawObj data fail.");
                                    }

                                    lastdraw = Some(draw.clone());
                                }
                            } else {
                                lastdraw = Some(draw.clone());
                            }

                        }
                    });
                    
                    // lastdraw 转 DrawObj
                    if let Some(tempdraw) = &mut lastdraw {
                        if let (Ok((pipeline, bindgroups)), Ok(geo)) = (passbindgroups.get(tempdraw.passentity), geometrys.get(tempdraw.vertexentity)) {
                            if let (Some(bindgroups), Some(geo)) = (&bindgroups.0, &geo.0) {
                                collect_draw_batch(tempdraw, &pipeline.0, &bindgroups.0, geo, &mut renderer, &mut instancedcache, &mut allocator, &device, &queue, &mut count_vertex);
                            } else {
                                // log::error!("DrawObj data fail 2.");
                            }
                        } else {
                            // log::error!("DrawObj data fail.");
                        }
                    }
                    transparent_list.iter().for_each(|tmp| {
                        if let Some(draw) = draws.get(tmp.idx as usize) {

                            if let Some(tempdraw) = &mut lastdraw {
                                if tempdraw.can_batch_instance_memory(draw, true, instancedcache.one_mesh_max_instance_bytes()) {
                                    let last = tempdraw.instance_memory.as_mut().unwrap();
                                    let curr = draw.instance_memory.as_ref().unwrap();

                                    curr.data.iter().for_each(|v| {
                                        last.data.push(*v);
                                    });
                                    last.itemcount += curr.itemcount;
                                } else {
                                    // lastdraw 转 DrawObj
                                    if let (Ok((pipeline, bindgroups)), Ok(geo)) = (passbindgroups.get(tempdraw.passentity), geometrys.get(tempdraw.vertexentity)) {
                                        if let (Some(bindgroups), Some(geo)) = (&bindgroups.0, &geo.0) {
                                            collect_draw_batch(tempdraw, &pipeline.0, &bindgroups.0, geo, &mut renderer, &mut instancedcache, &mut allocator, &device, &queue, &mut count_vertex);
                                        } else {
                                            // log::error!("DrawObj data fail 2.");
                                        }
                                    } else {
                                        // log::error!("DrawObj data fail.");
                                    }

                                    lastdraw = Some(draw.clone());
                                }
                            } else {
                                lastdraw = Some(draw.clone());
                            }

                        }
                    });
                    // lastdraw 转 DrawObj
                    if let Some(tempdraw) = &mut lastdraw {
                        if let (Ok((pipeline, bindgroups)), Ok(geo)) = (passbindgroups.get(tempdraw.passentity), geometrys.get(tempdraw.vertexentity)) {
                            if let (Some(bindgroups), Some(geo)) = (&bindgroups.0, &geo.0) {
                                collect_draw_batch(tempdraw, &pipeline.0, &bindgroups.0, geo, &mut renderer, &mut instancedcache, &mut allocator, &device, &queue, &mut count_vertex);
                            } else {
                                // log::error!("DrawObj data fail 2.");
                            }
                        } else {
                            // log::error!("DrawObj data fail.");
                        }
                    }

                    // log::warn!("Renderer Draw {:?} {:?}", list_model.0.len(), renderer.draws.list.len());
            }

            renderer.vertexs = count_vertex;
        });

        record.drawobjs = (pi_time::Instant::now() - time1).as_micros() as u32;
        // // log::trace!("SysRendererDraws: {:?}", pi_time::Instant::now() - time1);
    }


#[inline(never)]
fn shader(
    _id_pass: Entity,
    meta: &Handle<ShaderEffectMeta>,
    key_meta: &Atom,
    // instance: &EVerticeExtendCode,
    vb: &VertexBufferLayoutsComp,
    bindgroups: &BindGroups3D,
    renderalignment: ERenderAlignmentForShader,
    assets: & ShareAssetMgr<Shader3D>,
    device: &RenderDevice,
) -> Result<Handle<Shader3D>, Shader3D> {
    
    // log::error!("Shader: {:?}", 2);
    let key_attributes = &vb.1;
    // let key_shader_defines = 0;

    // let key_set_blocks = bindgroups.key_set_blocks();

    // let mut lightingenable = false;

    let (set0, set1, set2, set3) = (&bindgroups.scene, &bindgroups.model, bindgroups.textures.as_ref(), bindgroups.lightingshadow.as_ref());
    let mut setidx = 0;
    let mut vs_defined_snippets = vec![];
    let mut fs_defined_snippets = vec![];
    let mut vs_running_model_snippets = vec![];
    let vs_running_after_effect_snippets = vec![];
    let vs_running_before_effect_snippets = vec![];
    let fs_running_before_effect_snippets = vec![];
    let fs_running_after_effect_snippets = vec![];

    // log::error!("Shader: {:?}", key_meta);
    // log::error!("{:?}", key_attributes);
    // log::error!("{:?}", key_attributes.vs_define_code());

    vs_defined_snippets.push(key_attributes.vs_define_code());

    if let Some(set) = set0 {
        vs_defined_snippets.push(set.vs_define_code(setidx));
        fs_defined_snippets.push(set.fs_define_code(setidx));
        setidx += 1;
    }

    if let Some(set) = set1 {
        let skin = set.key().key.skin;
        vs_defined_snippets.push(set.vs_define_code(setidx));
        fs_defined_snippets.push(set.fs_define_code(setidx));

        vs_running_model_snippets.push(set.vs_running_model_snippet(meta));
        // vs_running_model_snippets.push(instance.vs_running_code());

        vs_running_model_snippets.push(key_attributes.vs_running_code(meta));
        vs_running_model_snippets.push(skin.running_code());
        vs_running_model_snippets.push(renderalignment.running_code());

        vs_defined_snippets.push(renderalignment.define_code());

        setidx += 1;
    } else {
        vs_running_model_snippets.push(key_attributes.vs_running_code(meta));
    }

    // let set2 = 
    if let Some(set) = set2 {
        vs_defined_snippets.push(set.vs_define_code(setidx));
        fs_defined_snippets.push(set.fs_define_code(setidx));
        setidx += 1;
        // Some(set2.as_ref())
    }
    //  else { None };
    
    if let Some(set) = set3 {
        // lightingenable = true;
        vs_defined_snippets.push(set.vs_define_code(setidx));
        fs_defined_snippets.push(set.fs_define_code(setidx));
        // setidx += 1;
        // Some(set2.as_ref())
    }

    // if meta.check_instance.0 & instance.0 == meta.check_instance.0 {
    //     vs_running_after_effect_snippets.push(meta.effect_varying_while_instance.clone());
    // }

    let key_shader = KeyShader3D {
        key_meta: key_meta.clone(),
        // lighting: lightingenable,
        bind_defines: meta.binddefines,
        key_attributes: key_attributes.clone(),
        renderalignment: renderalignment,
    };

    if let Some(shader) = assets.get(&key_shader) {
        // log::debug!("SysPassShaderRequestByModel: 4");
        Ok(shader)
    } else {
        
        let shader = meta.build_2(
            &device,
            &key_meta,
            // &key_shader.key_attributes,
            // &instance,
            // &renderalignment,
            // &skin,
            &vs_defined_snippets,
            &vs_running_model_snippets,
            &vs_running_before_effect_snippets, &vs_running_after_effect_snippets,
            &fs_defined_snippets,
            &fs_running_before_effect_snippets, &fs_running_after_effect_snippets,
        );

        assets.insert(key_shader, shader)
    }
}

#[inline(never)]
fn pipeline(
    shader: &Handle<Shader3D>,
    bindgroups: &BindGroups3D,
    vb: &VertexBufferLayoutsComp,
    colorformat: &ColorFormat,
    depthstencilformat: &DepthStencilFormat,
    blend: ModelBlend,
    depth_state: &DepthState, stencil_state: &StencilState,
    cull: &PrimitiveState,
    _id_pass: Entity,
    assets: &ShareAssetMgr<Pipeline3D>,
    device: &RenderDevice,
) -> Result<Handle<Pipeline3D>, Pipeline3D> {
    // log::error!("Cull {:?}", cull);
    
    let key_shader = shader.key().clone();
    let bind_group_layouts = bindgroups.bind_group_layouts();
    let key_bindgroup_layouts = KeyPipelineFromBindGroup(bindgroups.key_bindgroup_layouts());

    let key_vertex_layouts = vb.0.as_key_pipeline_from_vertex_layout();

    let pass_color_format = colorformat.val();
    let pass_depth_format = depthstencilformat.val();
    let depth_stencil = if let Some(pass_depth_format) = pass_depth_format {
        Some(
            depth_stencil_state(
                pass_depth_format,
                depth_state, stencil_state
            )
        )
    } else { None };

    let targets = RenderTargetState::color_target(pass_color_format, &blend);
    let key_state = KeyRenderPipelineState {
        primitive: cull.state(),
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

    if let Some(pipeline) = assets.get(&key_u64) {
        // log::debug!("SysPipeline: 3 Pass");
        // *oldpipeline = PassPipeline::new(Some(pipeline));
        Ok(pipeline)
    } else {
        // log::debug!("SysPipeline: 4 Pass");
        let pipeline = KeyPipeline3D::create(key_pipeline, shader.clone(), bind_group_layouts, &device);
        assets.insert(key_u64, pipeline)
    }
}

#[inline(never)]
fn collect_draw(
    is_transparent: bool,
    pass: u8,
    pipeline: u64,
    distance: f32,
    draw: &DrawObjTmp,
    sort_param: &TransparentSortParam,
    opaque_list: &mut Vec<TmpSortDrawOpaque>,
    transparent_list: &mut Vec<TmpSortDrawTransparent>,
    instancessortinfo: &InstancedMeshTransparentSortCollection,
    draws: &mut Vec<DrawObjTmp>,
) {
    if is_transparent == false {
        let index = draws.len();
        draws.push(draw.clone());
        opaque_list.push(TmpSortDrawOpaque { idx: index as u16, pass, distance, pipeline, resourcehash: (draw.vertexhash.0, draw.bindgroupshash.0) });
    } else {
        // log::warn!("instancessortinfo.0.len() {}", instancessortinfo.0.len());
        if instancessortinfo.0.len() <= 1  {
            let index = draws.len();
            draws.push(draw.clone());
            transparent_list.push(TmpSortDrawTransparent { idx: index as u16, pass, distance, pipeline, queue: sort_param.clone(), resourcehash: (draw.vertexhash.0, draw.bindgroupshash.0) });
        } else {
            if let Some(instance_memory) = &draw.instance_memory {
                let itemsize = instance_memory.data.len() / instance_memory.itemcount as usize;
                instancessortinfo.0.iter().for_each(|(alphaindex, range)| {
                    let start = itemsize * range.start as usize;
                    let end = itemsize * range.end as usize;
                    if range.start < range.end && range.end <= instance_memory.itemcount {
                        let tmpdraw = DrawObjTmp {
                            pipeline: draw.pipeline.clone(),
                            bindgroupshash: draw.bindgroupshash.clone(),
                            passentity: draw.passentity.clone(),
                            vertexhash: draw.vertexhash.clone(),
                            vertexentity: draw.vertexentity.clone(),
                            instance_memory: Some(EVerteicesMemory {
                                data: instance_memory.data.as_slice()[start..end].to_vec(),
                                slot: instance_memory.slot,
                                itemcount: range.end as u32 - range.start as u32
                            }),
                            vertex_range: draw.vertex_range.clone(),
                            indice_range: draw.indice_range.clone(),
                        };
                        let index = draws.len();
                        draws.push(tmpdraw);
                        let mut queue = sort_param.clone();
                        queue.index += *alphaindex;
                        transparent_list.push(TmpSortDrawTransparent { idx: index as u16, pass, distance, pipeline, queue, resourcehash: (draw.vertexhash.0, draw.bindgroupshash.0) });
                    }
                });
            } else {
                let index = draws.len();
                draws.push(draw.clone());
                transparent_list.push(TmpSortDrawTransparent { idx: index as u16, pass, distance, pipeline, queue: sort_param.clone(), resourcehash: (draw.vertexhash.0, draw.bindgroupshash.0) });
            }
        }
    }
}

#[inline(never)]
fn collect_draw_batch(
    tempdraw: &mut DrawObjTmp,
    pipeline: &Option<Handle<Pipeline3D>>,
    bindgroups: &BindGroups3D,
    geo: &RenderGeometry,
    renderer: &mut Renderer,
    instancedcache: &mut InstanceBufferAllocator,
    allocator: &mut VertexBufferAllocator3D,
    device: &PiRenderDevice,
    queue: &PiRenderQueue,
    count_vertex: &mut usize,
) {
    
    if let Some(mem) = &tempdraw.instance_memory {
        let size_per_value = mem.data.len() as u32 / mem.itemcount;
        let mut bytelen = mem.data.len();
        let one_mesh_max_instance_bytes = instancedcache.one_mesh_max_instance_bytes();
        let mut instances = geo.instances();
        if bytelen > one_mesh_max_instance_bytes {
            let count = one_mesh_max_instance_bytes / size_per_value as usize;
            instances.end = count as u32 + instances.start;
            bytelen = count * size_per_value as usize;
        }
        let data = instancedcache.collect(&mem.data.as_slice()[0..bytelen], size_per_value, allocator, device, queue);
        let data = if let Some(data) = data {
            EVerticesBufferUsage::EVBRange(Arc::new(EVertexBufferRange::NotUpdatable(data.0, data.1, data.2)))
        } else {
            let data = instancedcache.instance_initial_buffer();
            EVerticesBufferUsage::EVBRange(Arc::new(EVertexBufferRange::NotUpdatable(data.0, data.1, data.2)))
        };

        let mut draw = DrawObj {
            pipeline: pipeline.clone(),
            bindgroups: bindgroups.groups(),
            vertices: geo.vertices(),
            instances,
            vertex: tempdraw.vertex_range.apply(geo),
            indices: tempdraw.indice_range.apply(geo),
        };
        draw.insert_vertices(RenderVertices { slot: mem.slot, buffer: data, buffer_range: None, size_per_value: size_per_value as u64 });
        draw.instances = Range { start: 0, end: mem.itemcount };
        let vertex = if let Some(indices) = &draw.indices {
            indices.value_range().end - indices.value_range().start
        } else { draw.vertex.end - draw.vertex.start };
        *count_vertex += (vertex * (draw.instances.end - draw.instances.start)) as usize;
        renderer.draws.list.push(Arc::new(draw));
    } else {
        let draw = DrawObj {
            pipeline: pipeline.clone(),
            bindgroups: bindgroups.groups(),
            vertices: geo.vertices(),
            instances: geo.instances(),
            vertex: tempdraw.vertex_range.apply(geo),
            indices: tempdraw.indice_range.apply(geo),
        };
        let vertex = if let Some(indices) = &draw.indices {
            indices.value_range().end - indices.value_range().start
        } else { draw.vertex.end - draw.vertex.start };
        *count_vertex += (vertex * (draw.instances.end - draw.instances.start)) as usize;
        renderer.draws.list.push(Arc::new(draw));
    }
}
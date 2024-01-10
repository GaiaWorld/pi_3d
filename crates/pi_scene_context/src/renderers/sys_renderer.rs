use std::sync::Arc;

use pi_assets::asset::Handle;
use pi_scene_shell::prelude::*;
use crate::{
    viewer::prelude::*,
    bindgroup::*,
    pass::*,
    geometry::prelude::*,
    transforms::prelude::*,
    meshes::prelude::*,
    flags::*
};

use super::{
    render_primitive::*,
    base::*,
    render_depth_and_stencil::*,
    render_sort::*,
    render_blend::*,
    render_target_state::*,
    renderer::*,
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
                    (true, Some(val)) => Some(val.clone()),
                    (false, _) => None,
                    _ => {
                        if bindgroups.val().is_some() { *bindgroups = PassBindGroups::new(None); }
                        // log::warn!("Bindgroups: lightshadow fail");
                        return;
                    }
                };

                // log::warn!("Bindgroups: {:?}", (true, bindgroups.0.is_some()));
                let data = BindGroups3D::create(set0, set1, textures, lightshadow);
                *bindgroups = PassBindGroups(Some(data));
            } else {
                if bindgroups.val().is_some() { *bindgroups = PassBindGroups::new(None); }
                // log::warn!("Bindgroups: Ready False");
            }
        });
    }

/// 渲染器搜集渲染
    pub fn sys_pass_shader_request_by_model<T: TPass + Component, I: TPassID + Component>(
        models: Query<
            (
                &GeometryID, &I
            ),
            Or<(Changed<GeometryID>, Changed<RenderAlignment>)>,
        >,
        mut passes: Query<&mut PassGeometryID, With<T>>,
    ) {
        // let time1 = pi_time::Instant::now();

        models.iter().for_each(
            |(id_geo, passid)| {
                if let Ok(mut idgeometry) = passes.get_mut(passid.id()) {
                    *idgeometry = PassGeometryID(id_geo.0);
                }
            }
        );

        // log::debug!("SysPassShaderRequestByModel: {:?}", pi_time::Instant::now() - time1);
    }
    pub fn sys_pass_shader_request_by_geometry<T: TPass + Component, I: TPassID + Component>(
        models: Query<
            (&GeometryID, &I),
        >,
        geometrys: Query<(Entity, &MeshID), Changed<VertexBufferLayoutsComp>>,
        mut passes: Query<&mut PassGeometryID, With<T>>,
    ) {
        // let time1 = pi_time::Instant::now();
        geometrys.iter().for_each(|(entity, idmesh)| {
            if let Ok((idgeo, idpass)) = models.get(idmesh.0) {
                if entity == idgeo.0 {
                    if let Ok(mut idgeometry) = passes.get_mut(idpass.id()) {
                        *idgeometry = PassGeometryID(entity);
                    }
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

    pub fn sys_pass_pipeline_request_by_model<T: TPass + Component, I: TPassID + Component>(
        models: Query<
            (&I, &DisposeReady),
            Or<(
                Changed<ModelBlend>, Changed<GeometryID>,
                Changed<CCullMode>, Changed<Topology>, Changed<CPolygonMode>, Changed<CFrontFace>, Changed<CUnClipDepth>,
                Changed<DepthWrite>, Changed<DepthCompare>, Changed<DepthBias>, Changed<StencilFront>, Changed<StencilBack>, Changed<StencilRead>, Changed<StencilWrite>
            )>
        >,
        mut passes: Query<&mut PassPipelineStateDirty, (With<T>, With<PassPipeline>)>,
    ) {
        // let time1 = pi_time::Instant::now();

        models.iter().for_each(|(passid, disposeenable)|{
            if disposeenable.0 == true { return; }
            if let Ok(mut flag ) = passes.get_mut(passid.id()) {
                // log::warn!("PassPipeline: By Model Modify");
                *flag = PassPipelineStateDirty;
            }
        });

        // log::trace!("SysPassPipelineRequest: {:?}", pi_time::Instant::now() - time1);
    }
    
    pub fn sys_pass_pipeline_request_by_renderer<T: TPass + Component, I: TPassID + Component>(
        renderers: Query<(&RendererEnable, &ViewerID, &PassTag), Or<(Changed<RenderColorFormat>, Changed<RenderDepthFormat>, Changed<RendererBlend>, Changed<RendererEnable>, Changed<PassTag>)>>,
        viewers: Query<(&ModelList, &ForceIncludeModelList)>,
        models: Query<&I>,
        mut passes: Query<&mut PassPipelineStateDirty, With<T>>,
    ) {
        // let time1 = pi_time::Instant::now();
        renderers.iter().for_each(|(enable, idviewer, passtag)| {
            if enable.0 && T::TAG == *passtag {
                if let Ok((modellist, forcemodels)) = viewers.get(idviewer.0) {
                    modellist.0.iter().for_each(|idmodel| {
                        if let Ok(passid) = models.get(*idmodel) {
                            if let Ok( mut flag ) = passes.get_mut(passid.id()) {
                                // log::warn!("PassPipeline: By Renderer Modify");
                                *flag = PassPipelineStateDirty;
                            }
                        }
                    });
                    forcemodels.0.iter().for_each(|idmodel| {
                        if let Ok(passid) = models.get(*idmodel) {
                            if let Ok( mut flag ) = passes.get_mut(passid.id()) {
                                // log::warn!("PassPipeline: By Renderer Modify");
                                *flag = PassPipelineStateDirty;
                            }
                        }
                    });
                }
            }
        });
    }

    pub fn sys_pass_pipeline(
        renderers: Query<(&RenderColorFormat, &RenderDepthFormat, &RendererBlend)>,
        models: Query<
            (
                &GeometryID,
                (&CCullMode, &Topology, &CPolygonMode, &CFrontFace, &CUnClipDepth),
                &ModelBlend,
                (&DepthWrite, &DepthCompare, &DepthBias, &StencilFront, &StencilBack, &StencilRead, &StencilWrite)
            ),
        >,
        geometrys: Query<&VertexBufferLayoutsComp>, 
        mut passes: Query<
            (ObjectID, &DisposeReady, &PassModelID, &PassBindGroups, &PassShader, &mut PassPipeline, &PassRendererID),
            Or<(Changed<PassShader>, Changed<PassPipelineStateDirty>, Changed<PassRendererID>)>
        >,
        assets: ResMut<ShareAssetMgr<Pipeline3D>>,
        device: Res<PiRenderDevice>,
        mut errors: ResMut<ErrorRecord>,
    ) {
        // let time1 = pi_time::Instant::now();

        passes.iter_mut().for_each(|(id_pass, disposeready, id_model, bindgroups, shader, mut oldpipeline, idrenderer)| {
            if disposeready.0 == true { return; }
            // log::warn!("SysPipeline: 0 Pass");
            if let (Some(shader), Some(bindgroups)) = (shader.val(), bindgroups.val()) {
                // log::warn!("SysPipeline: 1 Pass");
                if let Ok((
                    id_geo, 
                    (cull, topology, polygon, face, unclip_depth),
                    blend,
                    (depth_write, compare, bias, stencil_front, stencil_back, stencil_read, stencil_write)
                )) = models.get(id_model.0) {
                    // log::warn!("SysPipeline: 2 Pass {:?}", (geometrys.get(id_geo.0).is_ok(), renderers.get(idrenderer.0).is_ok()));
                    match (geometrys.get(id_geo.0), renderers.get(idrenderer.0)) {
                        (Ok(vb), Ok((colorformat, depthstencilformat, blendenable))) => {
                            let blend = if blendenable.0 { blend.clone() } else { ModelBlend::default() };
                            if let Ok(pipeline) = pipeline(
                                shader, bindgroups, vb, &colorformat.0, &depthstencilformat.0,
                                blend, depth_write, compare, bias, stencil_front, stencil_back, stencil_read, stencil_write,
                                cull, topology, polygon, face, unclip_depth,
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

        // log::trace!("SysPassPipelineRequest: {:?}", pi_time::Instant::now() - time1);
    }


    pub fn sys_pass_draw_modify_by_model<T: TPass + Component, I: TPassID + Component>(
        models: Query<&I, Or<(Changed<RenderGeometryEable>, Changed<IndiceRenderRange>, Changed<VertexRenderRange>, Changed<DisposeReady>)>>,
        mut passes: Query<&mut PassDrawDirty, With<T>>,
        // mut commands: Commands,
    ) {
        models.iter().for_each(|id_pass| {
            if let Ok(mut drawdirty) = passes.get_mut(id_pass.id()) {
                *drawdirty = PassDrawDirty;
            }
        });

        // log::trace!("SysPassDrawLoad: {:?}", pi_time::Instant::now() - time1);
    }
    
    pub fn sys_pass_draw_modify_by_pass(
        models: Query<(&GeometryID, &IndiceRenderRange, &VertexRenderRange, &RenderGeometryEable, &InstanceSourceRefs, &DisposeReady)>,
        geometrys: Query<&RenderGeometryComp>,
        mut passes: Query<(&PassModelID, &PassBindGroups, &PassPipeline, &mut PassDraw), Or<(Changed<PassPipeline>, Changed<PassDrawDirty>, Changed<PassBindGroups>, Changed<PassModelID>)>>,
        // mut commands: Commands,
    ) {
        passes.iter_mut().for_each(|(id_model, bindgroups, pipeline, mut old_draw)| {
            if let (Some(bindgroups), Some(pipeline)) = (bindgroups.val(), pipeline.val()) {
                if let Ok((id_geo, renderindices, rendervertex, geoenable, instances, disposed)) = models.get(id_model.0) {
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
                                    vertex: rendervertex.apply(rendergeo),
                                    indices: renderindices.apply(rendergeo),
                                }))
                            } else {
                                DrawObj3D::Tmp( DrawObjTmp {
                                    pipeline: Some(pipeline.clone()),
                                    bindgroups: bindgroups.clone(),
                                    vertices: rendergeo.vertices(),
                                    instances: rendergeo.instances(),
                                    vertex: rendervertex.apply(rendergeo),
                                    indices: renderindices.apply(rendergeo),
                                })
                            };

                            *old_draw = PassDraw(Some(draw));
                        } else {
                            *old_draw = PassDraw(None);
                        }
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

    pub fn sys_renderer_draws_modify(
        mut renderers: Query<
            ( ObjectID, &ViewerID, &mut Renderer, &PassTag, &RendererEnable, &RenderViewport, &RendererBlend )
        >,
        viewers: Query<
            (&ModelListAfterCulling, &ViewerGlobalPosition, &DisposeReady),
        >,
        models: Query<
            (
                &GlobalEnable, &DisposeReady, &GlobalMatrix, &TransparentSortParam, &InstancedMeshTransparentSortCollection,
                (&PassID01, &PassID02, &PassID03, &PassID04, &PassID05, &PassID06, &PassID07, &PassID08
                    // ,&PassID09, &PassID10, &PassID11, &PassID12
                )
            )
        >,
        passes: Query<
            (&PassDraw, &PassPipeline, &PassRendererID)
        >,
        mut record: ResMut<Performance>,
    ) {
        let time1 = pi_time::Instant::now();

        renderers.iter_mut().for_each(|(_id_renderer, id_viewer, mut renderer, passtag, enable, viewport, transparent)| {
            renderer.clear();
            // log::warn!("Renderer: {:?}, Camera {:?}, {:?}", _id, id_viewer.0, enable.0);
            if enable.0 == false {
                return;
            }
            let mut count_vertex = 0;
            // let mut list_sort_opaque: Vec<(Arc<DrawObj>, f32, TransparentSortParam, u8, u64)> = vec![];
            // let mut list_sort_blend: Vec<(Arc<DrawObj>, f32, TransparentSortParam, u8, u64)> = vec![];
            let mut opaque_list: Vec<TmpSortDrawOpaque> = vec![];
            let mut transparent_list: Vec<TmpSortDrawTransparent> = vec![];
            let mut draws: Vec<Arc<DrawObj>> = vec![];
            if let Ok((list_model, viewposition, disposed)) = viewers.get(id_viewer.0) {
                if disposed.0 { return; }

                    // log::warn!("ModelListAfterCulling: {:?}, ", (list_model.0.len()));
                    renderer.draws.viewport = viewport.val();
                    list_model.0.iter().for_each(|id_obj| {
                        if let Ok((globalenable, disposed, nodeposition, rendersort, instancessortinfo, passrecord)) = models.get(id_obj.clone()) {
                            // log::warn!("Renderer: A");
                            if disposed.0 == true || globalenable.0 == false { return; }
                            
                            let passids = [passrecord.0.0, passrecord.1.0, passrecord.2.0, passrecord.3.0, passrecord.4.0, passrecord.5.0, passrecord.6.0, passrecord.7.0
                            // , passrecord.8.0, passrecord.9.0, passrecord.10.0, passrecord.11.0
                            ];
    
                            let index = 0;
                            let is_transparent = transparent.0;
                            if passtag.index() < passids.len() {
                                let passid = passids[passtag.index()];
                                // log::warn!("Renderer: Query Draw");
                                if let Ok((PassDraw(Some(draw)), pipeline, passrendererid)) = passes.get(passid) {

                                    if passrendererid.0 == _id_renderer {
                                        let temp = nodeposition.position() - &viewposition.0;
                                        let distance = temp.x * temp.x + temp.y * temp.y + temp.z * temp.z;

                                        collect_draw(
                                            is_transparent, index, pipeline.key(), distance, draw, rendersort,
                                            &mut opaque_list, &mut transparent_list, &instancessortinfo, &mut draws
                                        );
                                    } else {
                                        // log::error!("PassDraw Error {:?}", (passtag));
                                    }
                                }
                            }
                        }
                    });

                    opaque_list.sort();
                    transparent_list.sort();

                    // log::warn!("Opaque: {:?}", opaque_list.len());
                    // log::warn!("Transparent: {:?}", transparent_list.len());

                    opaque_list.iter().for_each(|tmp| {
                        // log::warn!("{:?}", tmp);
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

            renderer.vertexs = count_vertex;
        });

        record.drawobjs = (pi_time::Instant::now() - time1).as_micros() as u32;
        // log::trace!("SysRendererDraws: {:?}", pi_time::Instant::now() - time1);
    }


fn shader(
    _id_pass: Entity,
    meta: &Handle<ShaderEffectMeta>,
    key_meta: &Atom,
    // instance: &EVerticeExtendCode,
    vb: &VertexBufferLayoutsComp,
    bindgroups: &BindGroups3D,
    renderalignment: &RenderAlignment,
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
        vs_running_model_snippets.push(skin.running_code());
        vs_running_model_snippets.push(renderalignment.running_code());

        vs_defined_snippets.push(renderalignment.define_code());

        setidx += 1;
    }
    vs_running_model_snippets.push(key_attributes.vs_running_code(meta));

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
        renderalignment: renderalignment.0,
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

fn pipeline(
    shader: &Handle<Shader3D>,
    bindgroups: &BindGroups3D,
    vb: &VertexBufferLayoutsComp,
    colorformat: &ColorFormat,
    depthstencilformat: &DepthStencilFormat,
    blend: ModelBlend,
    depth_write: &DepthWrite, compare: &DepthCompare, bias: &DepthBias, stencil_front: &StencilFront, stencil_back: &StencilBack, stencil_read: &StencilRead, stencil_write: &StencilWrite,
    cull: &CCullMode, topology: &Topology, polygon: &CPolygonMode, face: &CFrontFace, unclip_depth: &CUnClipDepth,
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

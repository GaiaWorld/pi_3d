use std::{sync::Arc};

use pi_engine_shell::prelude::*;
use crate::{
    viewer::prelude::*,
    pass::*,
    geometry::prelude::*,
    cameras::prelude::*,
    scene::prelude::*,
    flags::*,
};

use super::{
    render_primitive::PrimitiveState,
    base::*,
    pass::*,
    render_depth_and_stencil::*,
    render_blend::ModelBlend,
    render_target_state::RenderTargetState,
    renderer::{Renderer, RendererEnable, RenderSize},
};


/// 渲染器搜集渲染
    pub fn sys_pass_bind_groups<T: TPass + Component, I: TPassID + Component>(
        passes: Query<
            (ObjectID, &PassSource, &PassReady, &PassBindGroupScene, &PassBindGroupModel, &PassBindGroupTextureSamplers, &PassBindGroups, &T),
            Or<(Changed<PassReady>, Changed<PassBindGroupScene>, Changed<PassBindGroupModel>, Changed<PassBindGroupTextureSamplers>)>
        >,
        mut commands: Commands,
    ) {
        passes.iter().for_each(|(id_pass, id_model, ready, set0, set1, set2, old, _)| {
            if let Some((key_meta, meta)) = ready.val() {
                if let (Some(set0), Some(set1)) = (set0.val(), set1.val()) {
                    if meta.textures.len() > 0 && set2.val().is_none() {
                        if old.val().is_some() {
                            commands.entity(id_pass).insert(PassBindGroups::new(None));
                        }
                    } else {
                        commands.entity(id_pass).insert(PassBindGroups::new(Some(
                            BindGroups3D::create(set0.clone(), set1.clone(), set2.val().clone())
                        )));
                    }
                    return;
                }
            }
            
            if old.val().is_some() {
                commands.entity(id_pass).insert(PassBindGroups::new(None));
            }
        });
    }

/// 渲染器搜集渲染
    pub fn sys_pass_shader_request_by_model<T: TPass + Component, I: TPassID + Component>(
        models: Query<
            (
                ObjectID,
                &GeometryID, 
                &I,
            ),
            Changed<GeometryID>,
        >,
        geometrys: Query<(&EInstanceCodeComp, &VertexBufferLayoutsComp)>, 
        passes: Query<(&PassReady, &PassBindGroups, &PassShader), With<T>>,
        mut commands: Commands,
        mut shader_center: ResMut<AssetDataCenterShader3D>,
        mut shader_loader: ResMut<AssetLoaderShader3D>,
        device: Res<PiRenderDevice>,
    ) {
        let time1 = pi_time::Instant::now();

        models.iter().for_each(
            |(id_model, id_geo, passid)| {
                log::debug!("SysPassShaderRequestByModel: 0");
                let id_pass = passid.id();
                let (instance, vb) = if let Ok(val) = geometrys.get(id_geo.0) {
                    val
                } else {
                    log::debug!("SysPassShaderRequestByModel: 11");
                    commands.entity(id_pass).insert(PassShader(None));
                    return;
                };
                if let Ok((ready, bindgroups, old_shader)) = passes.get(id_pass.clone()) {
                    log::debug!("SysPassShaderRequestByModel: 2");
                    if let (Some((key_meta, meta)), Some(bindgroups)) = (ready.val(), bindgroups.val()) {
                        
                        log::debug!("SysPassShaderRequestByModel: 3");
                        let key_attributes = vb.as_key_shader_from_attributes();
                        let key_shader_defines = 0;

                        let key_set_blocks = bindgroups.key_set_blocks();

                        let key_shader = KeyShader3D {
                            key_meta: key_meta.clone(),
                            key_attributes,
                            key_set_blocks,
                            defines: 0,
                        };

                        let (set0, set1, set2) = (&bindgroups.scene, &bindgroups.model, bindgroups.textures.as_ref());
                        let set2 = if let Some(set2) = set2 {
                            Some(set2.as_ref())
                        } else { None };
                
                        if let Some(shader) = shader_center.get(&key_shader) {
                            log::debug!("SysPassShaderRequestByModel: 4");
                            commands.entity(id_pass).insert(PassShader::from((shader, None)));
                        } else {
                            log::debug!("SysPassShaderRequestByModel: 5");
                            if !shader_center.check(&key_shader) {
                                let shader = meta.build(&device, &key_shader.key_meta, &key_shader.key_attributes, &instance, set0.as_ref(), set1.as_ref(), set2, None);
                                shader_center.add(&key_shader, shader, None);
                            }
                            shader_loader.request(id_pass, &key_shader);
                        }
                    } else {
                        if old_shader.val().is_some() {
                            log::debug!("SysPassShaderRequestByModel: No Ready");
                            commands.entity(id_pass).insert(PassShader(None));
                        }
                    }
                }
            }
        );

        log::debug!("SysPassShaderRequestByModel: {:?}", pi_time::Instant::now() - time1);
    }

/// 渲染器搜集渲染
    pub fn sys_pass_shader_request_by_pass<T: TPass + Component, I: TPassID + Component>(
        models: Query<
            (
                &GeometryID, &I,
            ),
        >,
        geometrys: Query<(&EInstanceCodeComp, &VertexBufferLayoutsComp)>, 
        passes: Query<
            (ObjectID, &PassSource, &PassReady, &PassBindGroups, &PassShader, &T),
            Or<(Changed<PassReady>, Changed<PassBindGroups>)>
        >,
        mut commands: Commands,
        mut shader_center: ResMut<AssetDataCenterShader3D>,
        mut shader_loader: ResMut<AssetLoaderShader3D>,
        device: Res<PiRenderDevice>,
    ) {
        let time1 = pi_time::Instant::now();

        passes.iter().for_each(|(id_pass, id_model, ready, bindgroups, old_shader, _)| {
            log::debug!("SysPassShaderRequestByPass: 0");
            if let (Some((key_meta, meta)), Some(bindgroups)) = (ready.val(), bindgroups.val()) {
                log::debug!("SysPassShaderRequestByPass: 1");
                if let Ok((id_geometry, passid)) = models.get(id_model.0) {
                    log::debug!("SysPassShaderRequestByPass: 2");
                    let (instance, vb) = if let Ok(val) = geometrys.get(id_geometry.0) {
                        val
                    } else {
                        commands.entity(id_pass).insert(PassShader(None));
                        log::debug!("SysPassShaderRequestByPass: 11");
                        return;
                    };
                    let key_attributes = vb.as_key_shader_from_attributes();
                    let key_shader_defines = 0;
    
                    let key_set_blocks = bindgroups.key_set_blocks();
    
                    let key_shader = KeyShader3D {
                        key_meta: key_meta.clone(),
                        key_attributes,
                        key_set_blocks,
                        defines: 0,
                    };
    
                    let (set0, set1, set2) = (&bindgroups.scene, &bindgroups.model, bindgroups.textures.as_ref());
                    let set2 = if let Some(set2) = set2 {
                        Some(set2.as_ref())
                    } else { None };
            
                    if let Some(shader) = shader_center.get(&key_shader) {
                        log::debug!("SysPassShaderRequestByPass: 3");
                        commands.entity(id_pass).insert(PassShader::from((shader, None)));
                    } else {
                        log::debug!("SysPassShaderRequestByPass: 4");
                        if !shader_center.check(&key_shader) {
                            let shader = meta.build(&device, &key_shader.key_meta, &key_shader.key_attributes, &instance, set0.as_ref(), set1.as_ref(), set2, None);
                            shader_center.add(&key_shader, shader, None);
                        }
                        shader_loader.request(id_pass, &key_shader);
                    }
                } else {
                    if old_shader.val().is_some() {
                        log::debug!("SysPassShaderRequestByPass: No Geo");
                        commands.entity(id_pass).insert(PassShader(None));
                    }
                }
            } else {
                if old_shader.val().is_some() {
                    log::debug!("SysPassShaderRequestByPass: No Ready");
                    commands.entity(id_pass).insert(PassShader(None));
                }
            }
        });

        log::debug!("SysPassShaderRequestByPass: {:?}", pi_time::Instant::now() - time1);
    }

    pub fn sys_pass_shader_loaded(
        mut commands: Commands,
        mut shader_center: ResMut<AssetDataCenterShader3D>,
        mut shader_loader: ResMut<AssetLoaderShader3D>,
    ) {
        let time1 = pi_time::Instant::now();
        shader_center.single_create().iter().for_each(|(key, value)| {
            log::debug!("PassShaderLoaded: 0");
            shader_loader.loaded(key, value).drain(..).for_each(|(entity, component)| {
                log::debug!("PassShaderLoaded: 1");
                commands.entity(entity).insert(PassShader::from(component));
            })
        });

        log::trace!("SysPassShaderLoad: {:?}", pi_time::Instant::now() - time1);
    }

    pub fn sys_pass_pipeline_request_by_model<T: TPass + Component, I: TPassID + Component>(
        scenes: Query<&ScenePassRenderCfg>,
        models: Query<
            (
                &SceneID,
                &PrimitiveState, &ModelBlend,
                &GeometryID, &I,
                (&DepthWrite, &DepthCompare, &DepthBias, &StencilFront, &StencilBack, &StencilRead, &StencilWrite)
            ),
            Or<(
                Changed<PrimitiveState>, Changed<ModelBlend>, Changed<GeometryID>,
                Changed<DepthWrite>, Changed<DepthCompare>, Changed<DepthBias>, Changed<StencilFront>, Changed<StencilBack>, Changed<StencilRead>, Changed<StencilWrite>
            )>
        >,
        geometrys: Query<&VertexBufferLayoutsComp>, 
        passes: Query<
            (&PassShader, &PassBindGroups, &PassPipeline, &T),
            With<T>
        >,
        mut commands: Commands,
        mut pipeline_center: ResMut<AssetDataCenterPipeline3D>,
        mut pipeline_loader: ResMut<AssetLoaderPipeline3D>,
        device: Res<PiRenderDevice>,
    ) {
        let time1 = pi_time::Instant::now();

        models.iter().for_each(| (
            idscene,
            primitive, blend, id_geo, passid, 
            (depth_write, compare, bias, stencil_front, stencil_back, stencil_read, stencil_write)
        ) |{
            let passcfgs = if let Ok(cfg) = scenes.get(idscene.0) {
                cfg.query(T::TAG)
            } else {
                return;
            };

            log::debug!("SysPipeline: 0 Model");
            let id_pass = passid.id();
            let vb = if let Ok(vb) = geometrys.get(id_geo.0.clone()) {
                vb
            } else {
                log::debug!("SysPipeline: 11 Model");
                commands.entity(id_pass).insert(PassPipeline::new(None));
                return;
            };

            if let Ok((shader, bindgroups, old_draw, _)) = passes.get(id_pass) {
                log::debug!("SysPipeline: 1 Model");
                if let (Some(shader), Some(bindgroups)) = (shader.val(), bindgroups.val()) {
                    let key_shader = shader.key().clone();
                    let bind_group_layouts = bindgroups.bind_group_layouts();
                    let key_bindgroup_layouts = bindgroups.key_bindgroup_layouts();

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
                        primitive: primitive.state,
                        target_state: vec![targets[0].clone()],
                        depth_stencil: depth_stencil,
                        multisample: wgpu::MultisampleState { count: 1, mask: !0, alpha_to_coverage_enabled: false }
                    };

                    let key_pipeline = KeyPipeline3D {
                        key_state,
                        key_shader,
                        key_bindgroup_layouts,
                        key_vertex_layouts,
                    };

                    let key_u64 = key_pipeline.to_u64();

                    if let Some(pipeline) = pipeline_center.get(&key_u64) {
                        log::debug!("SysPipeline: 3 Model");
                        commands.entity(id_pass).insert(PassPipeline::new(Some(pipeline)));
                    } else {
                        log::debug!("SysPipeline: 4 Model");
                        if !pipeline_center.check(&key_u64) {
                            let pipeline = KeyPipeline3D::create(key_pipeline, shader.clone(), bind_group_layouts, &device);
                            pipeline_center.add(&key_u64, pipeline, None);
                        }
                        pipeline_loader.request(id_pass, &key_u64);
                    }
                } else {
                    if old_draw.val().is_some() {
                        log::trace!("SysPassPipelineRequest: No Shader");
                        commands.entity(id_pass).insert(PassPipeline::new(None));
                    }
                }
            }
        });

        log::trace!("SysPassPipelineRequest: {:?}", pi_time::Instant::now() - time1);
    }

    pub fn sys_pass_pipeline_request_by_pass<T: TPass + Component, I: TPassID + Component>(
        scenes: Query<&ScenePassRenderCfg>,
        models: Query<
            (
                &SceneID,
                &GeometryID, &PrimitiveState, &ModelBlend,
                (&DepthWrite, &DepthCompare, &DepthBias, &StencilFront, &StencilBack, &StencilRead, &StencilWrite)
            ),
        >,
        geometrys: Query<&VertexBufferLayoutsComp>, 
        passes: Query<
            (ObjectID, &PassSource, &PassBindGroups, &PassShader, &T),
            Changed<PassShader>
        >,
        mut commands: Commands,
        mut pipeline_center: ResMut<AssetDataCenterPipeline3D>,
        mut pipeline_loader: ResMut<AssetLoaderPipeline3D>,
        device: Res<PiRenderDevice>,
    ) {
        let time1 = pi_time::Instant::now();

        passes.iter().for_each(|(id_pass, id_model, bindgroups, shader, _)| {
            log::debug!("SysPipeline: 0 Pass");
            if let (Some(shader), Some(bindgroups)) = (shader.val(), bindgroups.val()) {
                log::debug!("SysPipeline: 1 Pass");
                if let Ok((
                    idscene, id_geo, primitive, blend,
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
                        log::debug!("SysPipeline: 11 Pass");
                        commands.entity(id_pass).insert(PassPipeline::new(None));
                        return;
                    };
                    let key_shader = shader.key().clone();
                    let bind_group_layouts = bindgroups.bind_group_layouts();
                    let key_bindgroup_layouts = bindgroups.key_bindgroup_layouts();

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
                        primitive: primitive.state,
                        target_state: vec![targets[0].clone()],
                        depth_stencil: depth_stencil,
                        multisample: wgpu::MultisampleState { count: 1, mask: !0, alpha_to_coverage_enabled: false }
                    };

                    let key_pipeline = KeyPipeline3D {
                        key_state,
                        key_shader,
                        key_bindgroup_layouts,
                        key_vertex_layouts,
                    };

                    let key_u64 = key_pipeline.to_u64();

                    if let Some(pipeline) = pipeline_center.get(&key_u64) {
                        log::debug!("SysPipeline: 3 Pass");
                        commands.entity(id_pass).insert(PassPipeline::new(Some(pipeline)));
                    } else {
                        log::debug!("SysPipeline: 4 Pass");
                        if !pipeline_center.check(&key_u64) {
                            let pipeline = KeyPipeline3D::create(key_pipeline, shader.clone(), bind_group_layouts, &device);
                            pipeline_center.add(&key_u64, pipeline, None);
                        }
                        pipeline_loader.request(id_pass, &key_u64);
                    }
                } else {
                    log::trace!("SysPassPipelineRequest: No Geo");
                    commands.entity(id_pass).insert(PassPipeline::new(None));
                }
            } else {
                log::trace!("SysPassPipelineRequest: No Shader");
                commands.entity(id_pass).insert(PassPipeline::new(None));
            }
        });

        log::trace!("SysPassPipelineRequest: {:?}", pi_time::Instant::now() - time1);
    }

    pub fn sys_pass_pipeline_loaded(
        mut commands: Commands,
        mut pipeline_center: ResMut<AssetDataCenterPipeline3D>,
        mut pipeline_loader: ResMut<AssetLoaderPipeline3D>,
        device: Res<PiRenderDevice>,
    ) {
        let time1 = pi_time::Instant::now();

        pipeline_center.single_create().iter().for_each(|(key, value)| {
            log::debug!("SysPassPipeline: 0");
            pipeline_loader.loaded(key, value).drain(..).for_each(|(entity, component)| {
                log::debug!("SysPassPipeline: 1");
                commands.entity(entity).insert(PassPipeline::from(component));
            })
        });

        log::trace!("SysPassPipeline: {:?}", pi_time::Instant::now() - time1);
    }

    pub fn sys_pass_draw_modify_by_pass<T: TPass + Component, I: TPassID + Component>(
        models: Query<&GeometryID>,
        geometrys: Query<&RenderGeometry>,
        passes: Query<(ObjectID, &PassSource, &PassBindGroups, &PassPipeline, &PassDraw, &T), Changed<PassPipeline>>,
        mut commands: Commands,
    ) {
        let time1 = pi_time::Instant::now();

        passes.iter().for_each(|(id_pass, id_model, bindgroups, pipeline, old_draw, _)| {
            if let (Some(bindgroups), Some(pipeline)) = (bindgroups.val(), pipeline.val()) {
                if let Ok(id_geo) = models.get(id_model.0) {
                    if let Ok(rendergeo) = geometrys.get(id_geo.0.clone()) {
                        let draw = DrawObj3D {
                            pipeline: Some(pipeline.clone()),
                            bindgroups: bindgroups.groups(),
                            vertices: rendergeo.vertices(),
                            instances: rendergeo.instances(),
                            vertex: rendergeo.vertex_range(),
                            indices: rendergeo.indices.clone(),
                        };
                        log::debug!("PassDrawLoaded: 1 Pass");
                        commands.entity(id_pass).insert(PassDraw(Some(Arc::new(draw))));
                    } else {
                        if old_draw.0.is_some() { commands.entity(id_pass).insert(PassDraw(None)); }
                    }
                }
            } else {
                if old_draw.0.is_some() { commands.entity(id_pass).insert(PassDraw(None)); }
            }
        });

        log::trace!("SysPassDrawLoad: {:?}", pi_time::Instant::now() - time1);
    }

    pub fn sys_pass_draw_modify_by_model<T: TPass + Component, I: TPassID + Component>(
        models: Query<(&GeometryID, &I), Changed<RenderGeometryEable>>,
        geometrys: Query<&RenderGeometry>,
        passes: Query<(&PassSource, &PassBindGroups, &PassPipeline, &PassDraw, &T)>,
        mut commands: Commands,
    ) {
        let time1 = pi_time::Instant::now();

        models.iter().for_each(|(id_geo, id_pass)| {
            if let Ok((id_model, bindgroups, pipeline, old_draw, _)) = passes.get(id_pass.id()) {
                if let (Some(bindgroups), Some(pipeline)) = (bindgroups.val(), pipeline.val()) {
                    if let Ok(rendergeo) = geometrys.get(id_geo.0.clone()) {
                            let draw = DrawObj3D {
                                pipeline: Some(pipeline.clone()),
                                bindgroups: bindgroups.groups(),
                                vertices: rendergeo.vertices(),
                                instances: rendergeo.instances(),
                                vertex: rendergeo.vertex_range(),
                                indices: rendergeo.indices.clone(),
                            };
                            log::debug!("PassDrawLoaded: 1 Model");
                            commands.entity(id_pass.id()).insert(PassDraw(Some(Arc::new(draw))));
                    } else {
                        if old_draw.0.is_some() { commands.entity(id_pass.id()).insert(PassDraw(None)); }
                    }
                }
            }
        });

        log::trace!("SysPassDrawLoad: {:?}", pi_time::Instant::now() - time1);
    }

    pub fn sys_renderer_draws_modify(
        mut renderers: Query<
            (
                ObjectID, &ViewerID, &mut Renderer, &PassTagOrders, &RendererEnable, &mut RenderSize
            )
        >,
        viewers: Query<
            (&ModelListAfterCulling, &ViewerSize, Option<&CameraViewport>),
        >,
        models: Query<
            (&PassID01, &PassID02, &PassID03, &PassID04, &PassID05, &PassID06, &PassID07, &PassID08)
        >,
        passes: Query<
            &PassDraw
        >
    ) {
        let time1 = pi_time::Instant::now();

        renderers.iter_mut().for_each(|(id, id_viewer, mut renderer, passtag_orders, enable, mut rendersize)| {
            renderer.clear();
            log::warn!("Renderer: {:?}, Camera {:?}, {:?}", id, id_viewer.0, enable.0);
            if enable.0 == false {
                return;
            }
            if let Ok((list_model, viewersize, viewport)) = viewers.get(id_viewer.0) {
                *rendersize = RenderSize(viewersize.0, viewersize.1);

                if let Some(viewport) = viewport {
                    renderer.draws.viewport = (viewport.x, viewport.y, viewport.w, viewport.h, viewport.mindepth, viewport.maxdepth);
                } else {
                    renderer.draws.viewport = (0., 0., 1., 1., -1., 1.);
                }
                list_model.0.iter().for_each(|id_obj| {
                    if let Ok(passrecord) = models.get(id_obj.clone()) {
                        passtag_orders.0.iter().for_each(|tag| {
                            let pass = tag.as_pass();
                            if pass == EPassTag::PASS_TAG_01 {
                                if let Ok(PassDraw(Some(draw))) = passes.get(passrecord.0.0) { renderer.draws.list.push(draw.clone()) }
                            }
                            else if pass == EPassTag::PASS_TAG_02 {
                                if let Ok(PassDraw(Some(draw))) = passes.get(passrecord.1.0) { renderer.draws.list.push(draw.clone()) }
                            }
                            else if pass == EPassTag::PASS_TAG_03 {
                                if let Ok(PassDraw(Some(draw))) = passes.get(passrecord.2.0) { renderer.draws.list.push(draw.clone()) }
                            }
                            else if pass == EPassTag::PASS_TAG_04 {
                                if let Ok(PassDraw(Some(draw))) = passes.get(passrecord.3.0) { renderer.draws.list.push(draw.clone()) }
                            }
                            else if pass == EPassTag::PASS_TAG_05 {
                                if let Ok(PassDraw(Some(draw))) = passes.get(passrecord.4.0) { renderer.draws.list.push(draw.clone()) }
                            }
                            else if pass == EPassTag::PASS_TAG_06 {
                                if let Ok(PassDraw(Some(draw))) = passes.get(passrecord.5.0) { renderer.draws.list.push(draw.clone()) }
                            }
                            else if pass == EPassTag::PASS_TAG_07 {
                                if let Ok(PassDraw(Some(draw))) = passes.get(passrecord.6.0) { renderer.draws.list.push(draw.clone()) }
                            }
                            else if pass == EPassTag::PASS_TAG_08 {
                                if let Ok(PassDraw(Some(draw))) = passes.get(passrecord.7.0) { renderer.draws.list.push(draw.clone()) }
                            }
                        });
                    }
                });
                log::warn!("Renderer Draw {:?} {:?}", list_model.0.len(), renderer.draws.list.len());
            }
        });

        log::trace!("SysRendererDraws: {:?}", pi_time::Instant::now() - time1);
    }


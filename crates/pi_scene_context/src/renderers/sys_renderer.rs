use std::{sync::Arc};

use pi_engine_shell::prelude::*;
use crate::{
    viewer::prelude::*,
    pass::*,
    geometry::prelude::*,
    cameras::prelude::*,
    scene::prelude::*,
    transforms::prelude::*, prelude::{RenderAlignment, RendererDrawCallRecord, IndiceRenderRange, DisposeReady},
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
            (ObjectID, &ModelPass, &PassReady, &PassBindGroupScene, &PassBindGroupModel, &PassBindGroupTextureSamplers, &mut PassBindGroups),
            Or<(Changed<PassReady>, Changed<PassBindGroupScene>, Changed<PassBindGroupModel>, Changed<PassBindGroupTextureSamplers>)>
        >,
        mut commands: Commands,
    ) {
        passes.iter_mut().for_each(|(id_pass, id_model, ready, set0, set1, set2, mut bindgroups)| {
            if let Some((key_meta, meta)) = ready.val() {
                if let (Some(set0), Some(set1)) = (set0.val(), set1.val()) {
                    if meta.textures.len() > 0 && set2.val().is_none() {
                        if bindgroups.val().is_some() {
                            *bindgroups = PassBindGroups::new(None);
                        }
                    } else {
                        *bindgroups = PassBindGroups::new(Some(
                            BindGroups3D::create(set0.clone(), set1.clone(), set2.val().clone())
                        ));
                    }
                    return;
                }
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
        passes: Query<(&PassReady, &PassBindGroups, &PassShader), With<T>>,
        mut commands: Commands,
        mut shader_center: ResMut<AssetDataCenterShader3D>,
        mut shader_loader: ResMut<AssetLoaderShader3D>,
        device: Res<PiRenderDevice>,
    ) {
        let time1 = pi_time::Instant::now();

        models.iter().for_each(
            |(id_model, id_geo, passid, renderalignment)| {
                // log::debug!("SysPassShaderRequestByModel: 0");
                let id_pass = passid.id();
                let (instance, vb) = if let Ok(val) = geometrys.get(id_geo.0) {
                    val
                } else {
                    // log::debug!("SysPassShaderRequestByModel: 11");
                            if let Some(mut cmd) = commands.get_entity(id_pass) {
                                cmd.insert(PassShader(None));
                            }
                    return;
                };
                if let Ok((ready, bindgroups, old_shader)) = passes.get(id_pass.clone()) {
                    // log::debug!("SysPassShaderRequestByModel: 2");
                    if let (Some((key_meta, meta)), Some(bindgroups)) = (ready.val(), bindgroups.val()) {
                        
                        // log::debug!("SysPassShaderRequestByModel: 3");
                        let key_attributes = vb.as_key_shader_from_attributes();
                        let key_shader_defines = 0;

                        let key_set_blocks = bindgroups.key_set_blocks();

                        let key_shader = KeyShader3D {
                            key_meta: key_meta.clone(),
                            key_attributes,
                            key_set_blocks,
                            defines: key_shader_defines,
                            renderalignment: renderalignment.0
                        };

                        let (set0, set1, set2) = (&bindgroups.scene, &bindgroups.model, bindgroups.textures.as_ref());
                        let mut vs_defines = vec![];
                        vs_defines.push(set0.vs_define_code());
                        vs_defines.push(set1.vs_define_code());
                        let mut fs_defines = vec![];
                        fs_defines.push(set0.fs_define_code());
                        fs_defines.push(set1.fs_define_code());
                        let set2 = if let Some(set2) = set2 {
                            vs_defines.push(set2.vs_define_code());
                            fs_defines.push(set2.fs_define_code());
                            Some(set2.as_ref())
                        } else { None };
                
                        if shader_center.request(&key_shader, None) {
                            // log::debug!("SysPassShaderRequestByModel: 4");
                            shader_loader.request(id_pass, &key_shader);
                        } else {
                            // log::debug!("SysPassShaderRequestByModel: 5");
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
                    } else {
                        if old_shader.val().is_some() {
                            // log::debug!("SysPassShaderRequestByModel: No Ready");
                            if let Some(mut cmd) = commands.get_entity(id_pass) {
                                cmd.insert(PassShader(None));
                            }
                        }
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
            (ObjectID, &ModelPass, &PassReady, &PassBindGroups, &PassShader, &T),
            Or<(Changed<PassReady>, Changed<PassBindGroups>)>
        >,
        mut commands: Commands,
        mut shader_center: ResMut<AssetDataCenterShader3D>,
        mut shader_loader: ResMut<AssetLoaderShader3D>,
        device: Res<PiRenderDevice>,
    ) {
        let time1 = pi_time::Instant::now();

        passes.iter().for_each(|(id_pass, id_model, ready, bindgroups, old_shader, _)| {
            // log::debug!("SysPassShaderRequestByPass: 0");
            if let (Some((key_meta, meta)), Some(bindgroups)) = (ready.val(), bindgroups.val()) {
                // log::debug!("SysPassShaderRequestByPass: 1");
                if let Ok((id_geometry, passid, renderalignment)) = models.get(id_model.0) {
                    // log::debug!("SysPassShaderRequestByPass: 2");
                    let (instance, vb) = if let Ok(val) = geometrys.get(id_geometry.0) {
                        val
                    } else {
                        if let Some(mut cmd) = commands.get_entity(id_pass) {
                            cmd.insert(PassShader(None));
                        }
                        // log::debug!("SysPassShaderRequestByPass: 11");
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
                        renderalignment: renderalignment.0
                    };
    
                    let (set0, set1, set2) = (&bindgroups.scene, &bindgroups.model, bindgroups.textures.as_ref());
                    let mut vs_defines = vec![];
                    vs_defines.push(set0.vs_define_code());
                    vs_defines.push(set1.vs_define_code());
                    let mut fs_defines = vec![];
                    fs_defines.push(set0.fs_define_code());
                    fs_defines.push(set1.fs_define_code());
                    let set2 = if let Some(set2) = set2 {
                        vs_defines.push(set2.vs_define_code());
                        fs_defines.push(set2.fs_define_code());
                        Some(set2.as_ref())
                    } else { None };
            
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
                } else {
                    if old_shader.val().is_some() {
                        // log::debug!("SysPassShaderRequestByPass: No Geo");
                            if let Some(mut cmd) = commands.get_entity(id_pass) {
                                cmd.insert(PassShader(None));
                            }
                    }
                }
            } else {
                if old_shader.val().is_some() {
                    // log::debug!("SysPassShaderRequestByPass: No Ready");
                            if let Some(mut cmd) = commands.get_entity(id_pass) {
                                cmd.insert(PassShader(None));
                            }
                }
            }
        });

        // log::debug!("SysPassShaderRequestByPass: {:?}", pi_time::Instant::now() - time1);
    }

    pub fn sys_pass_shader_loaded(
        mut commands: Commands,
        mut shader_center: ResMut<AssetDataCenterShader3D>,
        mut shader_loader: ResMut<AssetLoaderShader3D>,
    ) {
        let time1 = pi_time::Instant::now();
        shader_center.single_create().iter().for_each(|(key, value)| {
            // log::debug!("PassShaderLoaded: 0");
            shader_loader.loaded(key, value).drain(..).for_each(|(entity, component)| {
                // log::debug!("PassShaderLoaded: 1");
                            if let Some(mut cmd) = commands.get_entity(entity) {
                                cmd.insert(PassShader::from(component));
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
        let time1 = pi_time::Instant::now();

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
            if let Ok((shader, bindgroups, old_draw, _, oldpipeline)) = passes.get(id_pass) {
                // log::debug!("SysPipeline: 0 Model");
                let vb = if let Ok(vb) = geometrys.get(id_geo.0.clone()) {
                    vb
                } else {
                    // log::debug!("SysPipeline: 11 Model");
                    // *oldpipeline = PassPipeline::new(None);
                    return;
                };

                // log::debug!("SysPipeline: 1 Model");
                if let (Some(shader), Some(bindgroups)) = (shader.val(), bindgroups.val()) {
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

                    let key_pipeline = KeyPipeline3D {
                        key_state,
                        key_shader,
                        key_bindgroup_layouts,
                        key_vertex_layouts,
                    };

                    let key_u64 = key_pipeline.to_u64();

                    if pipeline_center.request(&key_u64, None) {
                        // log::debug!("SysPipeline: 3 Model");
                        // *oldpipeline = PassPipeline::new(Some(pipeline));
                        pipeline_loader.request(id_pass, &key_u64);
                        // commands.entity(id_pass).insert(PassPipeline::new(Some(pipeline)));
                    } else {
                        // log::debug!("SysPipeline: 4 Model");
                        if !pipeline_center.check(&key_u64) {
                            // log::warn!("SysPassPipeline: {:?}", key_pipeline);
                            let pipeline = KeyPipeline3D::create(key_pipeline, shader.clone(), bind_group_layouts, &device);
                            pipeline_center.add(&key_u64, pipeline, None);
                        }
                        pipeline_loader.request(id_pass, &key_u64);
                    }
                } else {
                    if old_draw.val().is_some() {
                        // *oldpipeline = PassPipeline::new(None);
                        // log::trace!("SysPassPipelineRequest: No Shader");
                        // commands.entity(id_pass).insert(PassPipeline::new(None));
                    }
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
        let time1 = pi_time::Instant::now();

        passes.iter().for_each(|(id_pass, id_model, bindgroups, shader, _, oldpipeline)| {
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
                        // *oldpipeline = PassPipeline::new(None);
                        // log::debug!("SysPipeline: 11 Pass");
                        // commands.entity(id_pass).insert(PassPipeline::new(None));
                        return;
                    };
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
                } else {
                    // log::trace!("SysPassPipelineRequest: No Geo");
                    // *oldpipeline = PassPipeline::new(None);
                    // commands.entity(id_pass).insert(PassPipeline::new(None));
                }
            } else {
                // log::trace!("SysPassPipelineRequest: No Shader");
                // *oldpipeline = PassPipeline::new(None);
                // commands.entity(id_pass).insert(PassPipeline::new(None));
            }
        });

        // log::trace!("SysPassPipelineRequest: {:?}", pi_time::Instant::now() - time1);
    }

    pub fn sys_pass_pipeline_loaded(
        // mut commands: Commands,
        mut passes: Query<&mut PassPipeline>,
        mut pipeline_center: ResMut<AssetDataCenterPipeline3D>,
        mut pipeline_loader: ResMut<AssetLoaderPipeline3D>,
        device: Res<PiRenderDevice>,
    ) {
        let time1 = pi_time::Instant::now();

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
        models: Query<(&GeometryID, &IndiceRenderRange, &RenderGeometryEable, &DisposeReady)>,
        geometrys: Query<&RenderGeometryComp>,
        mut passes: Query<(ObjectID, &ModelPass, &PassBindGroups, &PassPipeline, &mut PassDraw, &T), Changed<PassPipeline>>,
        // mut commands: Commands,
    ) {
        let time1 = pi_time::Instant::now();

        passes.iter_mut().for_each(|(id_pass, id_model, bindgroups, pipeline, mut old_draw, _)| {
            if let (Some(bindgroups), Some(pipeline)) = (bindgroups.val(), pipeline.val()) {
                if let Ok((id_geo, renderindices, geoenable, disposed)) = models.get(id_model.0) {
                    if geoenable.0 == false || disposed.0 == true { return; }
        
                    if let Ok(RenderGeometryComp(Some(rendergeo))) = geometrys.get(id_geo.0.clone()) {
                        if rendergeo.isok() {
                            let draw = DrawObj3D {
                                pipeline: Some(pipeline.clone()),
                                bindgroups: bindgroups.groups(),
                                vertices: rendergeo.vertices(),
                                instances: rendergeo.instances(),
                                vertex: rendergeo.vertex_range(),
                                indices: renderindices.apply(rendergeo),
                            };
                            
                            *old_draw = PassDraw(Some(Arc::new(draw)));
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
        models: Query<(&GeometryID, &I, &IndiceRenderRange, &RenderGeometryEable, &DisposeReady), Or<(Changed<RenderGeometryEable>, Changed<IndiceRenderRange>, Changed<DisposeReady>)>>,
        geometrys: Query<&RenderGeometryComp>,
        mut passes: Query<(&ModelPass, &PassBindGroups, &PassPipeline, &mut PassDraw, &T)>,
        // mut commands: Commands,
    ) {
        let time1 = pi_time::Instant::now();

        models.iter().for_each(|(id_geo, id_pass, renderindices, geoenable, disposed)| {
            if geoenable.0 == false || disposed.0 == true { return; }

            if let Ok((id_model, bindgroups, pipeline, mut old_draw, _)) = passes.get_mut(id_pass.id()) {
                if let (Some(bindgroups), Some(pipeline)) = (bindgroups.val(), pipeline.val()) {
                    if let Ok(RenderGeometryComp(Some(rendergeo))) = geometrys.get(id_geo.0.clone()) {
                        if rendergeo.isok() {
                            let draw = DrawObj3D {
                                pipeline: Some(pipeline.clone()),
                                bindgroups: bindgroups.groups(),
                                vertices: rendergeo.vertices(),
                                instances: rendergeo.instances(),
                                vertex: rendergeo.vertex_range(),
                                indices: renderindices.apply(rendergeo),
                            };
                            *old_draw = PassDraw(Some(Arc::new(draw)));
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
        mut scenes: Query<&ScenePassRenderCfg>,
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
                &DisposeReady, &GlobalTransform, &TransparentSortParam,
                (&PassID01, &PassID02, &PassID03, &PassID04, &PassID05, &PassID06, &PassID07, &PassID08)
            )
        >,
        passes: Query<
            (&PassDraw, &PassPipeline)
        >,
        mut record: ResMut<RendererDrawCallRecord>,
    ) {
        let time1 = pi_time::Instant::now();

        renderers.iter_mut().for_each(|(id, id_viewer, mut renderer, passtag_orders, enable, mut rendersize)| {
            renderer.clear();
            // log::warn!("Renderer: {:?}, Camera {:?}, {:?}", id, id_viewer.0, enable.0);
            if enable.0 == false {
                return;
            }
            let mut list_sort_opaque: Vec<(Arc<DrawObj>, f32, TransparentSortParam, u8, u64)> = vec![];
            let mut list_sort_blend: Vec<(Arc<DrawObj>, f32, TransparentSortParam, u8, u64)> = vec![];
            if let Ok((idscene, list_model, viewersize, viewport, viewposition, disposed)) = viewers.get(id_viewer.0) {
                if disposed.0 { return; }

                if let Ok(passcfg) = scenes.get(idscene.0) {
                    *rendersize = RenderSize(viewersize.0, viewersize.1);
                    
    
                    if let Some(viewport) = viewport {
                        renderer.draws.viewport = (viewport.x, viewport.y, viewport.w, viewport.h, viewport.mindepth, viewport.maxdepth);
                    } else {
                        renderer.draws.viewport = (0., 0., 1., 1., -1., 1.);
                    }
                    list_model.0.iter().for_each(|id_obj| {
    
                        if let Ok((disposed, nodeposition, rendersort, passrecord)) = models.get(id_obj.clone()) {
                            // log::warn!("Renderer: A");
                            if disposed.0 == true { return; }
                            
                            let temp = nodeposition.position() - &viewposition.0;
                            let distance = temp.x * temp.x + temp.y * temp.y + temp.z * temp.z;
    
                            let mut index = 0;
                            for tag in passtag_orders.0.iter() {
                                let pass = tag.as_pass();


                                let list = if passcfg.query(pass).blend() {
                                    &mut list_sort_blend
                                } else {
                                    &mut list_sort_opaque
                                };

                                if pass == EPassTag::PASS_TAG_01 {
                                    if let Ok((PassDraw(Some(draw)), pipeline)) = passes.get(passrecord.0.0) {
                                        // log::warn!("Renderer: B {:?}, {:?}", tag, passrecord.0.0);
                                        list.push((draw.clone(), distance, rendersort.clone(), index, pipeline.key()));
                                    }
                                }
                                else if pass == EPassTag::PASS_TAG_02 {
                                    if let Ok((PassDraw(Some(draw)), pipeline)) = passes.get(passrecord.1.0) {
                                        // log::warn!("Renderer: B {:?}, {:?}", tag, passrecord.1.0);
                                        list.push((draw.clone(), distance, rendersort.clone(), index, pipeline.key()));
                                    }
                                }
                                else if pass == EPassTag::PASS_TAG_03 {
                                    if let Ok((PassDraw(Some(draw)), pipeline)) = passes.get(passrecord.2.0) {
                                        // log::warn!("Renderer: B {:?}, {:?}", tag, passrecord.2.0);
                                        list.push((draw.clone(), distance, rendersort.clone(), index, pipeline.key()));
                                    }
                                }
                                else if pass == EPassTag::PASS_TAG_04 {
                                    if let Ok((PassDraw(Some(draw)), pipeline)) = passes.get(passrecord.3.0) {
                                        // log::warn!("Renderer: B {:?}, {:?}", tag, passrecord.3.0);
                                        list.push((draw.clone(), distance, rendersort.clone(), index, pipeline.key()));
                                    }
                                }
                                else if pass == EPassTag::PASS_TAG_05 {
                                    if let Ok((PassDraw(Some(draw)), pipeline)) = passes.get(passrecord.4.0) {
                                        // log::warn!("Renderer: B {:?}, {:?}", tag, passrecord.4.0);
                                        list.push((draw.clone(), distance, rendersort.clone(), index, pipeline.key()));
                                    }
                                }
                                else if pass == EPassTag::PASS_TAG_06 {
                                    if let Ok((PassDraw(Some(draw)), pipeline)) = passes.get(passrecord.5.0) {
                                        // log::warn!("Renderer: B {:?}, {:?}", tag, passrecord.5.0);
                                        list.push((draw.clone(), distance, rendersort.clone(), index, pipeline.key()));
                                    }
                                }
                                else if pass == EPassTag::PASS_TAG_07 {
                                    if let Ok((PassDraw(Some(draw)), pipeline)) = passes.get(passrecord.6.0) {
                                        // log::warn!("Renderer: B {:?}, {:?}", tag, passrecord.6.0);
                                        list.push((draw.clone(), distance, rendersort.clone(), index, pipeline.key()));
                                    }
                                }
                                else if pass == EPassTag::PASS_TAG_08 {
                                    if let Ok((PassDraw(Some(draw)), pipeline)) = passes.get(passrecord.7.0) {
                                        // log::warn!("Renderer: B {:?}, {:?}", tag, passrecord.7.0);
                                        list.push((draw.clone(), distance, rendersort.clone(), index, pipeline.key()));
                                    }
                                }
    
                                index += 1;
                            }
                        }
                    });
    
                    list_sort_opaque.sort_by(|a, b| {
                        match a.3.cmp(&b.3) {
                            std::cmp::Ordering::Less => { std::cmp::Ordering::Less },
                            std::cmp::Ordering::Greater => { std::cmp::Ordering::Greater },
                            std::cmp::Ordering::Equal => {
                                match a.1.partial_cmp(&b.1) {
                                    Some(ord) => ord,
                                    None => {
                                        b.4.cmp(&a.4)
                                    },
                                }
                            }
                        }
                    });
                    list_sort_blend.sort_by(|a, b| {
                        match a.3.cmp(&b.3) {
                            std::cmp::Ordering::Less => { std::cmp::Ordering::Less },
                            std::cmp::Ordering::Greater => { std::cmp::Ordering::Greater },
                            std::cmp::Ordering::Equal => {
                                match a.2.cmp(&b.2) {
                                    std::cmp::Ordering::Less => { std::cmp::Ordering::Less },
                                    std::cmp::Ordering::Greater => { std::cmp::Ordering::Greater },
                                    std::cmp::Ordering::Equal => {
                                        match b.1.partial_cmp(&a.1) {
                                            Some(ord) => ord,
                                            None => {
                                                b.4.cmp(&a.4)
                                            },
                                        }
                                    }
                                }
                            }
                        }
                    });
    
                    list_sort_opaque.iter().for_each(|(entity, _ , _ , _, _)| {
                        renderer.draws.list.push(entity.clone());
                    });
                    list_sort_blend.iter().for_each(|(entity, _ , _ , _, _)| {
                        renderer.draws.list.push(entity.clone());
                    });

                    record.0.insert(id, renderer.draws.list.len() as u32);
                    // log::warn!("Renderer Draw {:?} {:?}", list_model.0.len(), renderer.draws.list.len());
                }
            }
        });

        // log::trace!("SysRendererDraws: {:?}", pi_time::Instant::now() - time1);
    }


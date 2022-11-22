use std::time::Instant;

use pi_ecs::{prelude::{Query, ResMut, Res}, query::{With, Write, Changed, Or}};
use pi_ecs_macros::setup;
use pi_render::{rhi::{device::RenderDevice, RenderQueue}};
use pi_scene_math::{Vector3};
use render_geometry::geometry::VertexAttributeMeta;

use pi_scene_context::{
    object::{GameObject, ObjectID},
    transforms::{transform_node::{GlobalTransform}, dirty::DirtyGlobalTransform},
    renderers::{
        render_object::{RenderObjectID, RenderObjectMetaOpaque, RenderObjectVertice, RenderObjectIndices, RenderObjectBindGroup, RenderObjectMetaTransparent},
        pipeline::PipelineKey,
        render_blend::RenderBlend,
        render_depth_and_stencil::RenderDepthAndStencil,
        render_primitive::PrimitiveState,
        render_sort::RenderSortParam,
        render_target_state::RenderTargetState, render_mode::{RenderMode, ERenderMode},
    },
    flags::{SceneID},
    resources::{SingleRenderObjectPipelinePool, RenderDynUniformBuffer},
    cameras::camera::{CameraGlobalPosition},
    materials::{material::MaterialID, bind_group::{RenderBindGroupPool}},
    meshes::model::BuildinModelBind,
    vertex_data::{indices::{IDAttributeIndices, AttributeIndices}, position::{IDAttributePosition, AttributePosition}, normal::{IDAttributeNormal, AttributeNormal}, uv::{IDAttributeUV, AttributeUV}},
    main_camera_render::{MainCameraRenderer},
    layer_mask::LayerMask
};

use crate::{unlit_material::{UnlitMaterialPropertype}, bind_group::{UnlitMaterialTextureBindGroup, UnlitMaterialBindGroup}, shader::UnlitShaderPool, define::UnlitMaterialDefines};

use super::{pipeline::UnlitMaterialPipeline};

// pub struct SysUnlitShaderEffectUpdate;
// impl SysUnlitShaderEffectUpdate {
//     pub fn sys(
//         shaders: Res<_>,
//     ) {

//     }
// }

pub struct UnlitModelUniformUpdate;
#[setup]
impl UnlitModelUniformUpdate {
    #[system]
    pub fn tick(
        meshes: Query<GameObject, (&GlobalTransform, &MaterialID, &BuildinModelBind), With<DirtyGlobalTransform>>,
        materials: Query<GameObject, &UnlitMaterialPropertype>,
        mut dynbuffer: ResMut<RenderDynUniformBuffer>,
    ) {
        //  println!("DefaultMaterial Uniform TickUpdate");
        meshes.iter().for_each(|(transform, mat_id, model)| {
            match materials.get(mat_id.0) {
                Some(_) => {
                    //  println!("DefaultMaterial >>>>>>>>>>>> ");
                    dynbuffer.as_mut().set_uniform::<GlobalTransform>(&model.bind_offset, transform);
                },
                None => {
                    
                }
            }
        });
    }
}

pub struct UnlitMaterialUniformUpdate;
#[setup]
impl UnlitMaterialUniformUpdate {
    #[system]
    pub fn tick(
        materials: Query<GameObject, &UnlitMaterialPropertype, Changed<UnlitMaterialPropertype>>,
        mut dynbuffer: ResMut<RenderDynUniformBuffer>,
    ) {
        //  println!("DefaultMaterial Uniform TickUpdate");
        materials.iter().for_each(|(material)| {
            dynbuffer.as_mut().set_uniform::<UnlitMaterialPropertype>(&material.bind_offset, &material);
        });
    }
}

pub struct SysUnlitMaterialPipelineKey;
#[setup]
impl SysUnlitMaterialPipelineKey {
    #[system]
    pub fn tick(
        mut items: Query<GameObject, (&MaterialID, &RenderBlend, &RenderDepthAndStencil, &PrimitiveState, Write<PipelineKey>), Or<(Changed<MaterialID>, Changed<RenderBlend>, Changed<RenderDepthAndStencil>, Changed<PrimitiveState>)>>,
        materials: Query<GameObject, (&UnlitMaterialDefines), With<UnlitMaterialPropertype>>,
        device: Res<RenderDevice>,
        mut shaderpool: ResMut<UnlitShaderPool>,
        mut pipelines: ResMut<UnlitMaterialPipeline>,
        mut pipeline_pool: ResMut<SingleRenderObjectPipelinePool>,
    ) {
        items.iter_mut().for_each(|(matid, blend, depth_stencil, primitive, mut pipelinekey )| {
            if let Some((defines)) = materials.get(matid.0) {
                let shader = shaderpool.get(defines, &device);
                let definemode = defines.mode();

                let key = pipelines.build(
                    definemode,
                    &device,
                    &shader,
                    RenderTargetState::color_target(blend).as_slice(),
                    depth_stencil.state(),
                    primitive.state,
                    &mut pipeline_pool,
                );
                
                let key = PipelineKey { id: key };
    
                pipelinekey.write(key);
            }
        });
    }
}

pub struct UnlitMaterialFilter;
#[setup]
impl UnlitMaterialFilter {
    #[system]
    pub fn tick(
        query_camera: Query<GameObject, (&RenderObjectID, &SceneID, &LayerMask, &CameraGlobalPosition)>,
        mut query_renderers: Query<GameObject, &mut MainCameraRenderer>,
        meshes: Query<GameObject, (&MaterialID, &SceneID, &LayerMask, &RenderSortParam, &PipelineKey, &GlobalTransform, &IDAttributePosition, &IDAttributeNormal, &IDAttributeIndices, &IDAttributeUV, &BuildinModelBind, &RenderMode)>,
        materials: Query<GameObject, (&UnlitMaterialPropertype, &UnlitMaterialBindGroup, &UnlitMaterialTextureBindGroup)>,
        bindgroups: Res<RenderBindGroupPool>,
        positions: Query<GameObject, &AttributePosition>,
        normals: Query<GameObject, &AttributeNormal>,
        indices: Query<GameObject, &AttributeIndices>,
        uvs: Query<GameObject, &AttributeUV>,
    ) {
        
        let time = Instant::now();

        query_camera.iter().for_each(|(renderid, sceneid, layermask, camerapos)| {
            match query_renderers.get_mut(renderid.0) {
                Some(mut renderer) => {
                    if renderer.ready {
                        println!("opaque List >>>>>>>>>>>>>>> {:?}", renderid.0);
                        collect_opaque_normal_depth(
                            sceneid.0,
                            layermask,
                            &materials,
                            &meshes,
                            &camerapos.0,
                            &positions, &normals, &indices, &uvs,
                            &mut renderer,
                            &bindgroups,
                        );
                    }
                },
                None => todo!(),
            }
        });
        // let _use_time = Instant::now() - pre_frame_time;
        let time1 = Instant::now();
        println!("UnlitMaterialFilter: {:?}", time1 - time);
    }
}

fn collect_opaque_normal_depth(
    camera_sceneid: ObjectID,
    layermask: &LayerMask,
    materials: &Query<GameObject, (&UnlitMaterialPropertype, &UnlitMaterialBindGroup, &UnlitMaterialTextureBindGroup)>,
    query: &Query<GameObject, (&MaterialID, &SceneID, &LayerMask, &RenderSortParam, &PipelineKey, &GlobalTransform, &IDAttributePosition, &IDAttributeNormal, &IDAttributeIndices, &IDAttributeUV, &BuildinModelBind, &RenderMode)>,
    camerapos: &Vector3,
    positions: &Query<GameObject, &AttributePosition>,
    normals: &Query<GameObject, &AttributeNormal>,
    indices: &Query<GameObject, &AttributeIndices>,
    uvs: &Query<GameObject, &AttributeUV>,
    renderer: &mut MainCameraRenderer,
    bindgroups: &Res<RenderBindGroupPool>,
) {
    query.iter().for_each(|(matid, sceneid, layer, rendersort, pipeline, globaltransform, position, normal, indice, uv, model, rendermode)| {
        if camera_sceneid == sceneid.0 && layermask.include(&layer) {
            // println!("camera_sceneid & layermask ok >>>>>>>>>>>>>>>");

            if let Some((mat, valuebindgroup, texbindgroup )) = materials.get(matid.0) {

                match (positions.get(position.0), normals.get(normal.0), indices.get(indice.0), uvs.get(uv.0)) {
                    (Some(position), Some(normal), Some(indices), Some(uv)) => {
                        // if list.len() >= 1 { return; }
                        
                        let view_distance = camerapos.metric_distance(&globaltransform.position);

                        let value_bindgroup = bindgroups.get(valuebindgroup.0);
                        let tex_bindgroup = bindgroups.get(texbindgroup.0);

                        if value_bindgroup.is_some() && tex_bindgroup.is_some() {
                            let mut bind_groups = vec![];
                            bind_groups.push(RenderObjectBindGroup {
                                bind_group: valuebindgroup.0,
                                offsets: vec![
                                    *model.bind_offset,
                                    *mat.bind_offset
                                ],
                            });
                            bind_groups.push(RenderObjectBindGroup {
                                bind_group: texbindgroup.0,
                                offsets: vec![],
                            });
    
                            let positions = RenderObjectVertice {
                                slot: AttributePosition::SLOT,
                                gbid: position.meta.buffer_id,
                                start: position.meta.start,
                                end: position.meta.end,
                                count: (position.meta.end - position.meta.start) / position.meta.data_bytes_size
                            };
                            let indices = Some(
                                RenderObjectIndices {
                                    slot: AttributePosition::SLOT,
                                    gbid: indices.meta.buffer_id,
                                    start: indices.meta.start,
                                    end: indices.meta.end,
                                    count: (indices.meta.end - indices.meta.start) / indices.meta.data_bytes_size,
                                    format: indices.format,
                                }
                            );
                            let mut vertices = vec![];
                            let normal = RenderObjectVertice {
                                slot: AttributeNormal::SLOT,
                                gbid: normal.meta.buffer_id,
                                start: normal.meta.start,
                                end: normal.meta.end,
                                count: (normal.meta.end - normal.meta.start) / normal.meta.data_bytes_size
                            };
                            vertices.push(normal);
                            let uv = RenderObjectVertice {
                                slot: AttributeUV::SLOT,
                                gbid: uv.meta.buffer_id,
                                start: uv.meta.start,
                                end: uv.meta.end,
                                count: (uv.meta.end - uv.meta.start) / uv.meta.data_bytes_size
                            };
                            vertices.push(uv);

                            let mut instances = vec![];
                            match rendermode.0 {
                                ERenderMode::AlphaTest => {
    
                                },
                                ERenderMode::Opaque => {
                                    // println!("opaque draw obj >>>>>>>>>>>>>>>");
                                    let mut meta = RenderObjectMetaOpaque {
                                        bind_groups,
                                        pipeline: *pipeline,
                                        positions,
                                        indices,
                                        vertices,
                                        instances,
                                        render_sort: *rendersort,
                                        view_distance,
                                    };
                                    renderer.opaque_draws.draws.push(meta);
                                },
                                ERenderMode::Skybox => todo!(),
                                ERenderMode::Transparent => {
                                    let mut meta = RenderObjectMetaTransparent {
                                        bind_groups,
                                        pipeline: *pipeline,
                                        positions,
                                        indices,
                                        vertices,
                                        instances,
                                        render_sort: *rendersort,
                                    };
                                    renderer.transparent_draws.draws.push(meta);
                                },
                            };
                        }
                    },
                    (_, _, _, _) => {

                    }
                }
            }
        }
    });
}
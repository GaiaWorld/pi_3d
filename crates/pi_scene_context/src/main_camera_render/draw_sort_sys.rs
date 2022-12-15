use std::time::Instant;

use pi_ecs::prelude::{Query, Res};
use pi_ecs_macros::setup;
use pi_engine_shell::object::ObjectID;
use pi_scene_math::Vector3;
use render_data_container::{RenderIndices, TRenderGeometry};
use render_geometry::indices::{IndicesBufferDesc, AssetResBufferIndices};

use crate::{object::GameObject, renderers::{render_object::{RenderObjectID, RenderObjectBindGroup, RenderObjectMetaOpaque, RenderObjectMetaTransparent}, render_sort::RenderSortParam, pipeline::AssetResRenderPipeline, render_mode::{RenderMode, ERenderMode}}, flags::SceneID, layer_mask::LayerMask, cameras::camera::CameraGlobalPosition, materials::{material::MaterialID, bind_group::RenderBindGroupPool}, transforms::transform_node::GlobalTransform, meshes::model::BuildinModelBind, geometry::geometry::{RenderGeometry, RenderIndicesFrom}, };

use super::{MainCameraRenderer, pipeline::AssetResShaderMainCamera};

pub struct DrawSortTick;
#[setup]
impl DrawSortTick {
    #[system]
    pub fn tick(
        mut main_camera_renderers: Query<GameObject, &mut MainCameraRenderer>,
    ) {
        // println!("Draw Sort Tick");
        // main_camera_renderers.iter_mut().for_each(|mut item| {
        //     item.opaque_draws.draws.sort();
        //     item.skybox_draws.draws.sort();
        //     item.transparent_draws.draws.sort();
        // });
    }
}

pub struct SysMainCameraFilter;
#[setup]
impl SysMainCameraFilter {
    #[system]
    pub fn tick(
        query_camera: Query<GameObject, (&RenderObjectID, &SceneID, &LayerMask, &CameraGlobalPosition)>,
        mut query_renderers: Query<GameObject, &mut MainCameraRenderer>,
        bindgrouppool: Res<RenderBindGroupPool>,
        meshes: Query<GameObject, (&AssetResShaderMainCamera, &SceneID, &LayerMask, &RenderSortParam, &AssetResRenderPipeline, &GlobalTransform, &BuildinModelBind, &RenderMode, &RenderGeometry, Option<&IndicesBufferDesc>, Option<&AssetResBufferIndices>)>,
    ) {
        
        let time = Instant::now();

        //  println!("DefaultMaterial Filter");
        query_camera.iter().for_each(|(renderid, sceneid, layermask, camerapos)| {
            //  println!("Camera >>>>>>>>>>>>>>>");
    
                match query_renderers.get_mut(renderid.0) {
                    Some(mut renderer) => {
                        // println!("opaque List >>>>>>>>>>>>>>> {:?}", renderid.0);
                        if renderer.ready {
                            collect_opaque_normal_depth(
                                sceneid.0,
                                layermask,
                                &meshes,
                                &camerapos.0,
                                &mut renderer,
                                &bindgrouppool
                            );
                        }
                    },
                    None => todo!(),
                }
        });
        // let _use_time = Instant::now() - pre_frame_time;
        let time1 = Instant::now();
        println!("SysMainCameraFilter: {:?}", time1 - time);
    }
}

fn collect_opaque_normal_depth(
    camera_sceneid: ObjectID,
    layermask: &LayerMask,
    query: &Query<GameObject, (&AssetResShaderMainCamera, &SceneID, &LayerMask, &RenderSortParam, &AssetResRenderPipeline, &GlobalTransform, &BuildinModelBind, &RenderMode, &RenderGeometry, Option<&IndicesBufferDesc>, Option<&AssetResBufferIndices>)>,
    camerapos: &Vector3,
    renderer: &mut MainCameraRenderer,
    bindgrouppool: &RenderBindGroupPool,
) {
    query.iter().for_each(|(
        matid, sceneid
        , layer, rendersort, pipeline
        , globaltransform, model, rendermode
        , geometry, indesc, inres
    )| {
        // println!("collect >>>>>>>>>>>>>>> ");

        let mut indices = None;
        if let Some(indesc) = indesc {
            if let Some(inres) = inres {
                indices = Some(RenderIndices::create((indesc, inres)));
            } else {
                return;
            }
        }

        // println!("opaque draw obj >>>>>>>>>>>>>>> ");
        if camera_sceneid == sceneid.0 && layermask.include(&layer) {
            let mut bind_groups = vec![];

            if !matid.renderobj_bind_group(model, &mut bind_groups, bindgrouppool) {
                return;
            }

            let view_distance = camerapos.metric_distance(&globaltransform.position);

            match rendermode.0 {
                ERenderMode::AlphaTest => {

                },
                ERenderMode::Opaque => {
                    // println!(">>>>>>>>>>>>>>>>> 1");
                    let meta = RenderObjectMetaOpaque {
                        bind_groups,
                        pipeline: pipeline.pipeline(),
                        indices,
                        vertices: geometry.vertices(),
                        instances: geometry.instances(),
                        render_sort: *rendersort,
                        view_distance,
                    };
                    renderer.opaque_draws.draws.push(meta);
                },
                ERenderMode::Skybox => todo!(),
                ERenderMode::Transparent => {
                    // println!(">>>>>>>>>>>>>>>>> 2");
                    let meta = RenderObjectMetaTransparent {
                        bind_groups,
                        pipeline: pipeline.pipeline(),
                        indices,
                        vertices: geometry.vertices(),
                        instances: geometry.instances(),
                        render_sort: *rendersort,
                    };
                    renderer.transparent_draws.draws.push(meta);
                },
            };
        }
    });
}

use std::sync::Arc;

use pi_engine_shell::prelude::*;
use pi_scene_context::prelude::*;
use rand::Rng;

use crate::{base::*, ResTrailBuffer, StateTrail};


pub fn sys_trail_update(
    transforms: Query<(&GlobalMatrix, &LocalMatrix)>,
    scenes: Query<&SceneTime>,
    mut geometries: Query<&mut RenderGeometryComp>,
    mut items: Query<
        (
            &SceneID, &TrailLinkedTransform, &TrailGeometry,
            &mut TrailBase, &mut TrailPoints,
            &TrailWorldPlace, &TrailMinimunVertexDistance,
            &TrailColor, &ColorOverTrail, &TrailSize, &WidthOverTrail, &TrailAgeControl,
            &mut TrailRandom
        )
    >,
    mut buffer: ResMut<ResTrailBuffer>,
    queue: Res<PiRenderQueue>,
    mut state: ResMut<StateTrail>,
) {
    let time1 = pi_time::Instant::now();
    if let Some(trailbuffer) = &mut buffer.0 {
        items.iter_mut().for_each(|(
            idscene, idlinked, idgeo, mut base, mut points,
            worldspace, minimumdistance, colorcontrol, colorinterpolator, sizecontrol, widthinterpolator, agecontrol, mut random
        )| {
            // log::warn!("Trail Update");
            if let (Ok(scenetime), Ok((worldmatrix, localmatrix))) = (scenes.get(idscene.0), transforms.get(idlinked.0)) {
                base.update(scenetime.delta_ms() as u32);

                let parentmatrix = if let Some(local) = localmatrix.0.try_inverse() {
                    worldmatrix.matrix * local
                } else { worldmatrix.matrix.clone() };
                let worldmatrix = &worldmatrix.matrix;

                let randoms = BaseRandom { seed: random.0.gen_range(0..u64::MAX), base: random.0.gen_range(0.0..1.0), x: random.0.gen_range(0.0..1.0), y: random.0.gen_range(0.0..1.0), z: random.0.gen_range(0.0..1.0), w: random.0.gen_range(0.0..1.0) };
                let flag = points.run(
                    worldmatrix, &localmatrix.0,
                    &colorcontrol.0, &colorinterpolator.0, &colorinterpolator.0,
                    sizecontrol.0, &widthinterpolator.0,
                    agecontrol.0, &base,
                    &randoms, 1000., minimumdistance.0,
                    worldspace.0
                );
    
                // let time2 = pi_time::Instant::now();
                // log::warn!("Trail Update: {:?}", time2 - time1);
                // time1 = time2;

                // log::warn!("Trail Update Geometry: ");
                if flag {
                    if let Ok(mut geometry) = geometries.get_mut(idgeo.0) {
                        if let Some(geometry) = &mut geometry.0 {
                            let (start, end) = trailbuffer.collect(&points, worldspace.0, &parentmatrix);
                            // *geometry = AssetResVBSlot01::from(EVerticesBufferUsage::EVBRange(Arc::new(EVertexBufferRange::NotUpdatable(trailbuffer.buffer(), start, end))));
                            if let Some(vertices) = geometry.vertices.get_mut(0) {
                                // log::warn!("Trail Update Geometry: {:?}", (start, end));
                                vertices.buffer = EVerticesBufferUsage::EVBRange(Arc::new(EVertexBufferRange::NotUpdatable(trailbuffer.buffer(), start, end)));
                            }
                        }
                    }
                }
                
                // let time2 = pi_time::Instant::now();
                // log::warn!("Trail Update 2: {:?}", time2 - time1);
            }
        });
        trailbuffer.after_collect(&queue);
    }
    
    let time2 = pi_time::Instant::now();
    state.calc_time = (time2 - time1).as_millis() as u32;
    // log::warn!("Trail Update: {:?}", time2 - time1);
}

pub fn sys_dispose_about_trail_linked(
    transforms: Query<&DisposeReady, Changed<DisposeReady>>,
    trails: Query<(Entity, &TrailLinkedTransform, &TrailGeometry)>,
    mut disposereadylist: ResMut<ActionListDisposeReadyForRef>,
    mut _disposecanlist: ResMut<ActionListDisposeCan>,
) {
    trails.iter().for_each(|(entity, idlinked, _)| {
        if let Ok(state) = transforms.get(idlinked.0) {
            if state.0 == false { return; }

            disposereadylist.push(OpsDisposeReadyForRef::ops(entity));
        }
    });
}

pub fn sys_dispose_about_trail(
    trails: Query<(Entity, &DisposeReady, &TrailLinkedTransform), Changed<DisposeReady>>,
    mut disposecanlist: ResMut<ActionListDisposeCan>,
) {

    trails.iter().for_each(|(entity, state, _)| {
        if state.0 == false { return; }

        disposecanlist.push(OpsDisposeCan::ops(entity));
    });
}
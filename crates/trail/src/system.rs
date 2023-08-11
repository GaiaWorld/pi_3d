
use std::sync::Arc;

use pi_engine_shell::prelude::*;
use pi_scene_context::prelude::*;
use pi_scene_math::{*, coordiante_system::CoordinateSytem3, vector::TToolVector3};
use rand::Rng;

use crate::{base::*, ResTrailBuffer};


pub fn sys_trail_update(
    transforms: Query<&WorldMatrix>,
    scenes: Query<&SceneTime>,
    mut geometries: Query<&mut AssetResVBSlot01>,
    mut items: Query<(&SceneID, &TrailLinkedTransform, &TrailMesh, &TrailGeometry, &mut TrailBase, &mut TrailPoints, &TrailWorldPlace, &TrailMinimunVertexDistance, &TrailColor, &ColorOverTrail, &TrailSize, &WidthOverTrail, &TrailAgeControl, &mut TrailRandom)>,
    mut buffer: ResMut<ResTrailBuffer>
) {
    if let Some(trailbuffer) = &mut buffer.0 {
        items.iter_mut().for_each(|(
            idscene, idlinked, idmesh, idgeo, mut base, mut points,
            worldspace, minimumdistance, colorcontrol, colorinterpolator, sizecontrol, widthinterpolator, agecontrol, mut random
        )| {
            if let (Ok(scenetime), Ok(worldmatrix)) = (scenes.get(idscene.0), transforms.get(idlinked.0)) {
                base.update(scenetime.delta_ms() as u32);
                let mut newpos = Vector3::zeros();
                let mut newaxis = Vector3::zeros();
    
                CoordinateSytem3::transform_coordinates(&Vector3::zeros(), &worldmatrix.0, &mut newpos);
                CoordinateSytem3::transform_normal(&Vector3::new(0., 0., 1.), &worldmatrix.0, &mut newaxis);
    
                let randoms = BaseRandom { seed: random.0.gen_range(0..u64::MAX), base: random.0.gen_range(0.0..1.0), x: random.0.gen_range(0.0..1.0), y: random.0.gen_range(0.0..1.0), z: random.0.gen_range(0.0..1.0), w: random.0.gen_range(0.0..1.0) };
                points.run(
                    &newpos, &newaxis,
                    &colorcontrol.0, &colorinterpolator.0, &colorinterpolator.0,
                    sizecontrol.0, &widthinterpolator.0,
                    agecontrol.0, &base,
                    &randoms, minimumdistance.0,
                    &worldmatrix.0, worldspace.0
                );
    
                if let Ok(mut geometry) = geometries.get_mut(idgeo.0) {
                    let (start, end) = trailbuffer.collect(&points, worldspace.0, &worldmatrix.0);
                    *geometry = AssetResVBSlot01::from(EVerticesBufferUsage::EVBRange(Arc::new(EVertexBufferRange::NotUpdatable(trailbuffer.buffer(), start, end))));
                }
            }
        });
    }
}

pub fn sys_dispose_about_trail_linked(
    transforms: Query<&DisposeReady, Changed<DisposeReady>>,
    trails: Query<(Entity, &TrailLinkedTransform, &TrailMesh, &TrailGeometry)>,
    mut disposereadylist: ResMut<ActionListDisposeReady>,
    mut disposecanlist: ResMut<ActionListDisposeCan>,
) {
    trails.iter().for_each(|(entity, idlinked, idmesh, idgeo)| {
        if let Ok(state) = transforms.get(idlinked.0) {
            disposereadylist.push(OpsDisposeReady::ops(entity));
            disposereadylist.push(OpsDisposeReady::ops(idmesh.0));
        }
    });
}

pub fn sys_dispose_about_trail(
    trails: Query<(Entity, &DisposeReady, &TrailLinkedTransform), Changed<DisposeReady>>,
    mut disposereadylist: ResMut<ActionListDisposeReady>,
    mut disposecanlist: ResMut<ActionListDisposeCan>,
) {

    trails.iter().for_each(|(entity, state, idlinked)| {
        if state.0 == false { return; }

        disposecanlist.push(OpsDisposeCan::ops(entity));
    });
}

use pi_engine_shell::prelude::*;
use pi_hash::{XHashMap, XHashSet};
use pi_scene_math::{frustum::FrustumPlanes, Matrix, Vector3, Number, coordiante_system::CoordinateSytem3, vector::TToolVector3};

use super::base::{TBoundingInfoCalc, TFilter};

#[derive(Default)]
pub struct VecBoundingInfoCalc {
    pool: XHashMap<Entity, ((Number, Number, Number), (Number, Number, Number))>,
    fast: XHashSet<Entity>,
}

impl TBoundingInfoCalc for VecBoundingInfoCalc {
    fn add_fast(&mut self, key: Entity) {
        self.fast.insert(key);
        self.pool.remove(&key);
    }
    fn add(&mut self, key: Entity, min: (Number, Number, Number), max: (Number, Number, Number)) {
        self.fast.remove(&key);
        self.pool.insert(key, (min, max));
    }

    fn remove(&mut self, key: Entity) {
        self.fast.remove(&key);
        self.pool.remove(&key);
    }

    fn culling<F: TFilter>(&self, transform: &Matrix, filter: F, result: &mut Vec<Entity>) {
        let mut frustum_planes = FrustumPlanes::default();
        frustum_planes.from_transform_matrix(transform);
        self.pool.iter().for_each(|(entity, item)| {
            if filter.filter(*entity) && is_in_frustum(item.0, item.1, &frustum_planes) {
                result.push(*entity);
            }
        });

        self.fast.iter().for_each(|item| {
            result.push(*item);
        });
    }

    fn ray_test(&self, org: Vector3, dir: Vector3, result: &mut Option<Entity>) {
        todo!()
    }
    
}

pub fn is_in_frustum(min: (Number, Number, Number), max: (Number, Number, Number), frustum_planes: &FrustumPlanes) -> bool {
    let center = Vector3::new((min.0 + max.0) * 0.5, (min.1 + max.1) * 0.5,(min.2 + max.2) * 0.5);
    let radius = Vector3::new((min.0 - max.0) * 0.5, (min.1 - max.1) * 0.5,(min.2 - max.2) * 0.5);
    let radius = CoordinateSytem3::length(&radius);

    if frustum_planes.near.dot_coordinate(&center) <= -radius {
        return false;
    }
    if frustum_planes.far.dot_coordinate(&center) <= -radius {
        return false;
    }
    if frustum_planes.left.dot_coordinate(&center) <= -radius {
        return false;
    }
    if frustum_planes.right.dot_coordinate(&center) <= -radius {
        return false;
    }
    if frustum_planes.top.dot_coordinate(&center) <= -radius {
        return false;
    }
    if frustum_planes.bottom.dot_coordinate(&center) <= -radius {
        return false;
    }

    return true;
}

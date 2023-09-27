use parry3d::{bounding_volume::Aabb, math::Isometry, query::RayCast};
use pi_engine_shell::prelude::*;
use pi_hash::{XHashMap, XHashSet};
use pi_scene_math::{
    coordiante_system::CoordinateSytem3, frustum::FrustumPlanes, vector::TToolVector3, Matrix,
    Number, Point3, Vector3,
};

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
        // log::warn!("{:?}, {:?}", frustum_planes.top, frustum_planes.bottom);
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
        let origin = Point3::new(org.x, org.y, org.z);
        let ray = parry3d::query::Ray::new(origin.clone(), dir);
        let mut dest = f32::MAX;
        // println!("========= ray: {:?}", ray);
        self.pool.iter().for_each(|(entity, item)| {
            let aabb = Aabb::new(
                Point3::new(item.0 .0, item.0 .1, item.0 .2),
                Point3::new(item.1 .0, item.1 .1, item.1 .2),
            );
            
            
            if let Some(d) = aabb.cast_ray(&Isometry::identity(), &ray, f32::MAX, false) {
                // println!("========= id: {:?}, aabb: {:?}, dest: {}",  entity, aabb, d);
                // println!("========= dest： {}", dest);
                if dest > d  {
                    dest = d;
                    result.replace(*entity);
                }
            }
        });
    }
}

pub fn is_in_frustum(
    min: (Number, Number, Number),
    max: (Number, Number, Number),
    frustum_planes: &FrustumPlanes,
) -> bool {
    let center = Vector3::new(
        (min.0 + max.0) * 0.5,
        (min.1 + max.1) * 0.5,
        (min.2 + max.2) * 0.5,
    );
    let radius = Vector3::new(
        (min.0 - max.0) * 0.5,
        (min.1 - max.1) * 0.5,
        (min.2 - max.2) * 0.5,
    );
    let radius = CoordinateSytem3::length(&radius);
    // log::warn!("Radius: {}, {:?}", radius, (min, max));

    // let dotnear = frustum_planes.near.dot_coordinate(&center);
    // let dotfar = frustum_planes.far.dot_coordinate(&center);
    // let dotleft = frustum_planes.left.dot_coordinate(&center);
    // let dotright = frustum_planes.right.dot_coordinate(&center);
    // let dottop = frustum_planes.top.dot_coordinate(&center);
    // let dotbottom = frustum_planes.bottom.dot_coordinate(&center);
    // log::warn!("Dots: {:?}", (dotnear, dotfar, dotleft, dotright, dottop, dotbottom));

    // {
    let dotnear = frustum_planes.near.dot_coordinate(&center);
    if dotnear < 0. {
        return false;
    }
    let dotfar = frustum_planes.far.dot_coordinate(&center);
    if dotfar < 0. {
        return false;
    }
    let dotleft = frustum_planes.left.dot_coordinate(&center);
    if dotleft < 0. {
        return false;
    }
    let dotright = frustum_planes.right.dot_coordinate(&center);
    if dotright < 0. {
        return false;
    }
    let dottop = frustum_planes.top.dot_coordinate(&center);
    if dottop < 0. {
        return false;
    }
    let dotbottom = frustum_planes.bottom.dot_coordinate(&center);
    if dotbottom < 0. {
        return false;
    }
    // }

    if dotnear <= -radius {
        return false;
    }
    if dotfar <= -radius {
        return false;
    }
    if dotleft <= -radius {
        return false;
    }
    if dotright <= -radius {
        return false;
    }
    if dottop <= -radius {
        return false;
    }
    if dotbottom <= -radius {
        return false;
    }

    return true;
}

use pi_scene_shell::prelude::*;
use pi_scene_math::{
    coordiante_system::CoordinateSytem3, frustum::FrustumPlanes, vector::TToolVector3, Matrix,
    Number, Point3, Vector3,
};

use crate::flags::GlobalEnable;

use super::{base::{PiRay, PickResult, TBoundingInfoCalc, TFilter}, bounding_sphere::intersects_sphere};

#[derive(Default, Clone)]
pub struct VecBoundingInfoCalc {
    pool: XHashMap<Entity, ((Number, Number, Number), (Number, Number, Number), Number, i32)>,
    fast: XHashSet<Entity>,
    temp: XHashSet<Entity>,
}

impl TBoundingInfoCalc for VecBoundingInfoCalc {
    fn add_fast(&mut self, key: Entity) {
        self.fast.insert(key);
        self.pool.remove(&key);
    }
    fn add(&mut self, key: Entity, min: (Number, Number, Number), max: (Number, Number, Number), intersection_treshold: Number, alphaindex: i32) {
        self.fast.remove(&key);
        self.pool.insert(key, (min, max, intersection_treshold, alphaindex));
    }

    fn remove(&mut self, key: Entity) {
        self.fast.remove(&key);
        self.pool.remove(&key);
    }

    fn culling<F: TFilter>(&mut self, transform: &Matrix, filter: F, result: &mut Vec<Entity>) {
        let mut frustum_planes = FrustumPlanes::default();
        frustum_planes.from_transform_matrix(transform);

        filter.iter().for_each(|entity| {
            if self.fast.contains(entity) {
                result.push(*entity);
                // self.temp.insert(*entity);
            // } else if self.temp.contains(entity) {
            //     // 
            } else if let Some(item) = self.pool.get(entity) {
                let queryed = filter.query(*entity);
                if queryed && is_in_frustum(item.0, item.1, &frustum_planes) {
                    result.push(*entity);
                    // self.temp.insert(*entity);
                } 
            }
        });
    }

    fn ray_test(
        &self, piray: &PiRay, result: &mut Option<PickResult>,
        sortparams: &Query<&GlobalEnable>,
    ) {
        let origin = Point3::new(piray.origin.0, piray.origin.1, piray.origin.2);
        let ray = parry3d::query::Ray::new(origin, Vector3::new(piray.direction.0, piray.direction.1, piray.direction.2));
        let mut dest = f32::MAX;
        // println!("========= ray: {:?}", ray);
        let mut aabb = Aabb::new(
            Point3::new(0., 0., 0.01),
            Point3::new(0., 0., 0.01),
        );
        let mut temp = vec![];
        self.pool.iter().for_each(|(entity, item)| {
            if let Ok(genable) = sortparams.get(*entity) {
                if genable.0 {
                    temp.push((*entity, item));
                }
            }
        });
        temp.sort_by(|a, b| b.1.3.cmp(&a.1.3));

        // log::error!("Ray Test List: {:?}", (temp.len()));
        let mut lastalphaindex = i32::MAX;
        let mut isok = false;
        for (entity, item) in temp.iter() {
            aabb.mins.x = item.0 .0;
            aabb.mins.y = item.0 .1;
            aabb.mins.z = item.0 .2;
            aabb.maxs.x = item.1 .0;
            aabb.maxs.y = item.1 .1;
            aabb.maxs.z = item.1 .2;

            let centerx = (item.0 .0 + item.1 .0) * 0.5;
            let centery = (item.0 .1 + item.1 .1) * 0.5;
            let centerz = (item.0 .2 + item.1 .2) * 0.5;

            if isok && lastalphaindex != item.3 {
                break;
            }
            if !intersects_sphere((centerx, centery, centerz), item.2, 0., &piray.origin, &piray.direction) {
                continue;
            }
            lastalphaindex = item.3;
            if let Some(d) = aabb.cast_local_ray(&ray, f32::MAX, false) {
                // println!("========= id: {:?}, aabb: {:?}, dest: {}",  entity, aabb, d);
                // println!("========= dest： {}", dest);
                if d < dest  {
                    dest = d;
                    isok = true;
                    result.replace(PickResult {
                        target: *entity,
                        min: item.0,
                        max: item.1,
                        sortindex: item.3,
                        pickdetail: None,
                        bybounding: false
                    });
                }
            }
        }
    }
    fn entities(&self) -> Vec<Entity> {
        let count = self.fast.len() + self.pool.len();
        let mut result = Vec::with_capacity(count);
        self.fast.iter().for_each(|v| { result.push(*v); });
        self.pool.keys().for_each(|v| { result.push(*v); });
        result
    }
    fn size(&self) -> usize {
        self.fast.capacity() + self.pool.capacity()
    }
    fn reset_temp(&mut self) {
        self.temp.clear();
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
        (min.0 - max.0).abs() * 0.5,
        (min.1 - max.1).abs() * 0.5,
        (min.2 - max.2).abs() * 0.5,
    );
    let radius = CoordinateSytem3::length(&radius);
    // log::warn!("Radius: {}, {:?}", radius, (min, max));

    // let dotnear = frustum_planes.near.dot_coordinate(center.x, center.y, center.z);
    // let dotfar = frustum_planes.far.dot_coordinate(center.x, center.y, center.z);
    // let dotleft = frustum_planes.left.dot_coordinate(center.x, center.y, center.z);
    // let dotright = frustum_planes.right.dot_coordinate(center.x, center.y, center.z);
    // let dottop = frustum_planes.top.dot_coordinate(center.x, center.y, center.z);
    // let dotbottom = frustum_planes.bottom.dot_coordinate(center.x, center.y, center.z);
    // log::warn!("Dots: {:?}", (dotnear, dotfar, dotleft, dotright, dottop, dotbottom));

    // {
    let mut flag: bool = true;
    let dotnear = frustum_planes.near.dot_coordinate(center.x, center.y, center.z);
    flag = flag && dotnear > 0.;
    let dotfar = frustum_planes.far.dot_coordinate(center.x, center.y, center.z);
    flag = flag && dotfar > 0.;
    let dotleft = frustum_planes.left.dot_coordinate(center.x, center.y, center.z);
    flag = flag && dotleft > 0.;
    let dotright = frustum_planes.right.dot_coordinate(center.x, center.y, center.z);
    flag = flag && dotright > 0.;
    let dottop = frustum_planes.top.dot_coordinate(center.x, center.y, center.z);
    flag = flag && dottop > 0.;
    let dotbottom = frustum_planes.bottom.dot_coordinate(center.x, center.y, center.z);
    flag = flag && dotbottom > 0.;
    // }
    
    // log::warn!("dots: {:?}", (dotnear, dotfar, dotleft, dotright, dottop, dotbottom));

    if flag {
        return true;
    }

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

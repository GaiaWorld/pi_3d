// use ncollide3d::{
//     bounding_volume::AABB,
//     na::{Point3 as TreePoint3, Vector3},
// };
use parry3d::{
    bounding_volume::Aabb,
    na::{Isometry3, Point3},
    query::{Ray, RayCast},
    shape::{ConvexPolyhedron, Cuboid},
};
use pi_engine_shell::prelude::*;
use pi_hash::XHashSet;
use pi_scene_math::{Matrix, Number, Vector3, Vector4};
use pi_spatial::oct_helper::OctTree;

use super::base::{BoundingKey, TBoundingInfoCalc, TFilter};

pub struct BoundingOctTree {
    pub fast: XHashSet<Entity>,
    pub tree: OctTree<BoundingKey, (Isometry3<f32>, Cuboid)>,
}

impl TBoundingInfoCalc for BoundingOctTree {
    fn add_fast(&mut self, key: Entity) {
        self.fast.insert(key);
        self.tree.remove(BoundingKey(key));
    }
    fn add(&mut self, key: Entity, min: (Number, Number, Number), max: (Number, Number, Number)) {
        self.fast.remove(&key);
        self.tree.remove(BoundingKey(key));
        // let box_point = info.bounding_box.vectors_world;
        let points = vec![
            Point3::new(min.0, min.1, min.2),
            Point3::new(max.0, min.1, min.2),
            Point3::new(min.0, max.1, min.2),
            Point3::new(max.0, max.1, min.2),
            Point3::new(min.0, min.1, max.2),
            Point3::new(max.0, min.1, max.2),
            Point3::new(min.0, max.1, max.2),
            Point3::new(max.0, max.1, max.2),
        ];

        let obb = parry3d::utils::obb(&points);
        // let aadd_maxs = obb.0 * obb.1.local_aabb().maxs;
        // let aadd_mins = obb.0 * obb.1.local_aabb().mins;

        self.tree.add(
            BoundingKey(key),
            Aabb::new(
                Point3::new(min.0, min.1, min.2),
                Point3::new(max.0, max.1, max.2),
            ),
            obb,
        );
    }

    fn remove(&mut self, key: Entity) {
        self.fast.remove(&key);
        self.tree.remove(BoundingKey(key));
    }

    fn culling<F: TFilter>(&self, transform: &Matrix, filter: F, result: &mut Vec<Entity>) {
        if let Some(frustum) = compute_frustum(transform) {
            let aabb = frustum.local_aabb();

            let aabb = Aabb::new(
                Point3::new(aabb.mins.x, aabb.mins.y, aabb.mins.z),
                Point3::new(aabb.maxs.x, aabb.maxs.y, aabb.maxs.z),
            );

            let mut args: (ConvexPolyhedron, &mut Vec<Entity>, F) = (frustum, result, filter);

            self.tree.query(&aabb, intersects, &mut args, ab_query_func);
        }

        self.fast.iter().for_each(|item| {
            result.push(*item);
        });
    }

    fn ray_test(
        &self,
        origin: Vector3,
        dir: Vector3,
        result: &mut Option<Entity>,
    ) {
        let origin = Point3::new(origin.x, origin.y, origin.z);
        let ray = Ray::new(origin.clone(), dir);

        let temp = dir.normalize() * 10000000.;
        let max = Point3::new(origin.x + temp.x, origin.y + temp.y, origin.z + temp.z);
        let aabb = Aabb::new(origin, max);

        let mut args: (Ray, f32, &mut Option<Entity>) = (ray, 0., result);

        self.tree.query(&aabb, intersects, &mut args, ray_test_func);
    }
}

pub fn ab_query_func<F: TFilter>(
    arg: &mut (ConvexPolyhedron, &mut Vec<Entity>, F),
    id: BoundingKey,
    _aabb: &Aabb,
    bind: &(Isometry3<f32>, Cuboid),
) {
    if arg.2.filter(id.0) {
        // 优化:是否需要先判断frustum与aabb
        if parry3d::query::intersection_test(&Isometry3::identity(), &arg.0, &bind.0, &bind.1)
            .unwrap()
        {
            arg.1.push(id.0);
        }
    }
}

pub fn ray_test_func(
    arg: &mut (Ray, f32, &mut Option<Entity>),
    id: BoundingKey,
    _aabb: &Aabb,
    bind: &(Isometry3<f32>, Cuboid),
) {
    if let Some(distance) = bind.1.cast_ray(&bind.0, &arg.0, f32::MAX, false) {
        if distance < arg.1 {
            arg.1 = distance;
            arg.2.replace(id.0);
        }
    }
}

#[inline]
fn intersects(a: &Aabb, b: &Aabb) -> bool {
    a.mins.x <= b.maxs.x
        && a.maxs.x > b.mins.x
        && a.mins.y <= b.maxs.y
        && a.maxs.y > b.mins.y
        && a.mins.z <= b.maxs.z
        && a.maxs.z > b.mins.z
}

pub fn compute_frustum(view_projection: &Matrix) -> Option<ConvexPolyhedron> {
    let t = view_projection.try_inverse().unwrap();

    let p0 = t * Vector4::new(1., 1., 1., 1.);
    let p1 = t * Vector4::new(1., 1., -1., 1.);
    let p2 = t * Vector4::new(-1., 1., -1., 1.);
    let p3 = t * Vector4::new(-1., 1., 1., 1.);

    let p4 = t * Vector4::new(1., -1., 1., 1.);
    let p5 = t * Vector4::new(1., -1., -1., 1.);
    let p6 = t * Vector4::new(-1., -1., -1., 1.);
    let p7 = t * Vector4::new(-1., -1., 1., 1.);

    let points = vec![
        Point3::new(p0[0], p0[1], p0[2]),
        Point3::new(p1[0], p1[1], p1[2]),
        Point3::new(p2[0], p2[1], p2[2]),
        Point3::new(p3[0], p3[1], p3[2]),
        Point3::new(p4[0], p4[1], p4[2]),
        Point3::new(p5[0], p5[1], p5[2]),
        Point3::new(p6[0], p6[1], p6[2]),
        Point3::new(p7[0], p7[1], p7[2]),
    ];

    let indices = vec![
        [0, 1, 2],
        [2, 3, 0],
        [4, 5, 6],
        [6, 7, 4],
        [0, 1, 4],
        [4, 5, 1],
        [1, 2, 5],
        [5, 6, 2],
        [2, 3, 6],
        [6, 7, 3],
        [3, 0, 7],
        [7, 4, 0],
    ];

    ConvexPolyhedron::from_convex_mesh(points, &indices)
}

// pub type ActionListAddBindingInfo = ActionList<(BoundingKey, BoundingInfo)>;
pub type ActionListRemoveBindingInfo = ActionList<BoundingKey>;
pub type ActionListCheckBindingInfo = ActionList<BoundingKey>;

pub struct PluginBoundingOctTree;
impl Plugin for PluginBoundingOctTree {
    fn build(&self, _app: &mut App) {
        // let max = Vector3::new(100f32, 100f32, 100f32);
        // let min = max / 100f32;

        // let tree = OctTree::new(
        //     Aabb::new(
        //         Point3::new(-1024f32, -1024f32, -4194304f32),
        //         Point3::new(3072f32, 3072f32, 4194304f32),
        //     ),
        //     max,
        //     min,
        //     0,
        //     0,
        //     0,
        // );
        // app.insert_resource(BoundingOctTree(tree));
        // app.insert_resource(ActionListAddBindingInfo::default());
        // app.insert_resource(ActionListRemoveBindingInfo::default());

        // app.add_systems(
        // 	Update,
        // 	(
        //     sys_add_binding_info,
        //     sys_remove_binding_info,
        //     sys_check_binding_info,
        // ));
    }
}

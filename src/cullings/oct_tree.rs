use ncollide3d::{bounding_volume::AABB, na::Point3 as TreePoint3};
use parry3d::{
    na::{Isometry3, Point3},
    shape::{ConvexPolyhedron, Cuboid},
};
use pi_scene_math::{frustum::FrustumPlanes, Perspective3, Vector4};
use pi_spatialtree::OctTree;

use crate::cameras::camera::Camera;

use super::{BoundingInfo, BoundingKey, TBoundingInfoCalc};

/// aabb的查询函数的参数
pub struct AbQueryArgs {
    pub frustum: ConvexPolyhedron,
    pub result: Vec<BoundingKey>,
}
impl AbQueryArgs {
    pub fn new(frustum: ConvexPolyhedron) -> AbQueryArgs {
        AbQueryArgs {
            frustum,
            result: Vec::new(),
        }
    }
}

pub struct BoundingTree(OctTree<BoundingKey, f32, (Isometry3<f32>, Cuboid)>);

impl TBoundingInfoCalc for BoundingTree {
    fn add(&mut self, key: BoundingKey, info: BoundingInfo) {
        let box_point = info.bounding_box.vectors_world;
        let points = vec![
            Point3::new(box_point[0][0], box_point[0][1], box_point[0][2]),
            Point3::new(box_point[1][0], box_point[1][1], box_point[1][2]),
            Point3::new(box_point[2][0], box_point[2][1], box_point[2][2]),
            Point3::new(box_point[3][0], box_point[3][1], box_point[3][2]),
            Point3::new(box_point[4][0], box_point[4][1], box_point[4][2]),
            Point3::new(box_point[5][0], box_point[5][1], box_point[5][2]),
            Point3::new(box_point[6][0], box_point[6][1], box_point[6][2]),
            Point3::new(box_point[7][0], box_point[7][1], box_point[7][2]),
        ];

        let obb = parry3d::utils::obb(&points);
        let aadd_maxs = obb.0 * obb.1.local_aabb().maxs;
        let aadd_mins = obb.0 * obb.1.local_aabb().mins;

        self.0.add(
            key,
            AABB::new(
                TreePoint3::new(aadd_mins.x, aadd_mins.y, aadd_mins.z),
                TreePoint3::new(aadd_maxs.x, aadd_maxs.y, aadd_maxs.z),
            ),
            obb,
        );
    }

    fn remove(&mut self, key: BoundingKey) {
        let _ = self.0.remove(key);
    }

    fn check_boundings(&self, _: &FrustumPlanes, _: &mut Vec<BoundingKey>) {
        todo!()
    }

    fn check_boundings_of_tree(&self, frustum: &ConvexPolyhedron, result: &mut Vec<BoundingKey>) {
        let aabb = frustum.local_aabb();

        let aabb = AABB::new(
            TreePoint3::new(aabb.mins.x, aabb.mins.y, aabb.mins.z),
            TreePoint3::new(aabb.maxs.x, aabb.maxs.y, aabb.maxs.z),
        );

        let mut args = AbQueryArgs::new(frustum.clone());

        self.0.query(&aabb, intersects, &mut args, ab_query_func);
        *result = args.result
    }
}

pub fn ab_query_func(
    arg: &mut AbQueryArgs,
    id: BoundingKey,
    _aabb: &AABB<f32>,
    bind: &(Isometry3<f32>, Cuboid),
) {
    // 优化:是否需要先判断frustum与aabb
    if parry3d::query::intersection_test(&Isometry3::identity(), &arg.frustum, &bind.0, &bind.1)
        .unwrap()
    {
        arg.result.push(id);
    }
}

#[inline]
fn intersects(a: &AABB<f32>, b: &AABB<f32>) -> bool {
    a.mins.x <= b.maxs.x
        && a.maxs.x > b.mins.x
        && a.mins.y <= b.maxs.y
        && a.maxs.y > b.mins.y
        && a.mins.z <= b.maxs.z
        && a.maxs.z > b.mins.z
}

pub fn compute_frustum(camera: &Camera) -> Option<ConvexPolyhedron> {
    let aspect =
        (camera.viewport[2] - camera.viewport[0]) / (camera.viewport[3] - camera.viewport[1]);
    let projection = Perspective3::new(aspect, camera.fov * 2.0, camera.minz, camera.maxz);

    let view_projection = projection.as_matrix() * camera.project_matrix;
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

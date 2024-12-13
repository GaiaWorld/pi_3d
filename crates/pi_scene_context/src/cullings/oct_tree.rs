use pi_scene_math::{frustum::FrustumPlanes, Matrix, Number, Vector3};
use pi_scene_shell::prelude::*;

use crate::{flags::GlobalEnable, prelude::RenderQueueSortParam};

use super::{
    base::{BoundingKey, PiRay, PickResult, TBoundingInfoCalc, TFilter},
    bounding::is_in_frustum, bounding_sphere::intersects_sphere,
};

pub type TOctTreeBind = Number;

pub struct BoundingOctTree {
    fast: XHashSet<Entity>,
    tree: OctTree<BoundingKey, TOctTreeBind>,
    temp: XHashSet<Entity>,
}
impl BoundingOctTree {
    pub fn new(tree: OctTree<BoundingKey, TOctTreeBind>) -> Self {
        Self {
            fast: XHashSet::default(),
            tree,
            temp: XHashSet::default(),
        }
    }
}

impl TBoundingInfoCalc for BoundingOctTree {
    fn add_fast(&mut self, key: Entity) {
        self.fast.insert(key);
        self.tree.remove(BoundingKey(key));
    }
    fn add(&mut self, key: Entity, min: (Number, Number, Number), max: (Number, Number, Number), intersection_treshold: TOctTreeBind) {
        // println!("add: {:?}", (key, min, max));
        self.fast.remove(&key);
        self.tree.remove(BoundingKey(key));
        // let box_point = info.bounding_box.vectors_world;
        // let points = vec![
        //     Point3::new(min.0, min.1, min.2),
        //     Point3::new(max.0, min.1, min.2),
        //     Point3::new(min.0, max.1, min.2),
        //     Point3::new(max.0, max.1, min.2),
        //     Point3::new(min.0, min.1, max.2),
        //     Point3::new(max.0, min.1, max.2),
        //     Point3::new(min.0, max.1, max.2),
        //     Point3::new(max.0, max.1, max.2),
        // ];

        // let obb = parry3d::utils::obb(&points);
        // let aadd_maxs = obb.0 * obb.1.local_aabb().maxs;
        // let aadd_mins = obb.0 * obb.1.local_aabb().mins;

        self.tree.add(
            BoundingKey(key),
            Aabb::new(
                Point3::new(min.0, min.1, min.2),
                Point3::new(max.0, max.1, max.2),
            ),
            intersection_treshold,
        );
    }

    fn remove(&mut self, key: Entity) {
        self.fast.remove(&key);
        self.tree.remove(BoundingKey(key));
    }

    fn culling<F: TFilter>(&mut self, transform: &Matrix, filter: F, result: &mut Vec<Entity>) {
        let iter =  filter.iter();
        let len = iter.len();
        if len == 0 {
            return;
        }

        self.fast.iter().for_each(|item| {
            if filter.filter(*item) {
                result.push(*item);
            }
        });

        if len > result.len() {
            if let Some(frustum) = compute_frustum(transform) {
                let mut frustum_planes = FrustumPlanes::default();
                frustum_planes.from_transform_matrix(transform);
                let aabb = frustum.local_aabb();

                let aabb = Aabb::new(
                    Point3::new(aabb.mins.x, aabb.mins.y, aabb.mins.z),
                    Point3::new(aabb.maxs.x, aabb.maxs.y, aabb.maxs.z),
                );
                let mut args: (
                    ConvexPolyhedron,
                    FrustumPlanes,
                    &mut Vec<Entity>,
                    F,
                ) = (frustum, frustum_planes, result, filter);

                self.tree.query(&aabb, intersects, &mut args, ab_query_func);
            }
        }

    }

    fn ray_test(
        &self, piray: &PiRay, result: &mut Option<PickResult>,
        sortparams: &Query<(&RenderQueueSortParam, &GlobalEnable)>,
    ) {
        let ray = Ray::new(
            Point3::new(piray.origin.0, piray.origin.1, piray.origin.2),
            Vector3::new(piray.direction.0, piray.direction.1, piray.direction.2),
        );

        let minx = piray.origin.0.min(piray.far.0);
        let miny = piray.origin.1.min(piray.far.1);
        let minz = piray.origin.2.min(piray.far.2);
        let maxx = piray.origin.0.max(piray.far.0);
        let maxy = piray.origin.1.max(piray.far.1);
        let maxz = piray.origin.2.max(piray.far.2);

        let aabb = Aabb::new(
            Point3::new(minx, miny, minz),
            Point3::new(maxx, maxy, maxz),
        );

        let mut args: (Ray, f32, &mut Option<PickResult>, &Query<'_, (&RenderQueueSortParam, &GlobalEnable)>) = (ray, f32::MAX, result, sortparams);

        self.tree.query(&aabb, intersects, &mut args, ray_test_func);
    }
    fn entities(&self) -> Vec<Entity> {
        let count = self.fast.len() + self.tree.len();
        let mut result = Vec::with_capacity(count);
        self.fast.iter().for_each(|v| {
            result.push(*v);
        });
        self.tree.ab_map.keys().for_each(|v| {
            result.push(v.0);
        });
        result
    }
    fn size(&self) -> usize {
        self.fast.capacity() + self.tree.ab_map.capacity()
    }
    fn reset_temp(&mut self) {
        self.temp.clear();
    }
}

pub fn ab_query_func<F: TFilter>(
    arg: &mut (
        ConvexPolyhedron,
        FrustumPlanes,
        &mut Vec<Entity>,
        F,
        // &mut u128,
    ),
    id: BoundingKey,
    aabb: &Aabb,
    _bind: &TOctTreeBind,
) {
    if arg.3.filter(id.0) {
        if is_in_frustum(
            (aabb.mins.x, aabb.mins.y, aabb.mins.z),
            (aabb.maxs.x, aabb.maxs.y, aabb.maxs.z),
            &arg.1,
        )
        {
            arg.2.push(id.0);
        }
    }
}

pub fn ray_test_func(
    arg: &mut (Ray, f32, &mut Option<PickResult>, &Query<(&RenderQueueSortParam, &GlobalEnable)>),
    id: BoundingKey,
    aabb: &Aabb,
    _bind: &Number,
) {
    if let Ok((sortparam, genble)) = arg.3.get(id.0) {
        if genble.0 == false {
            return;
        }

        let centerx = (aabb.mins .x + aabb.maxs .x) * 0.5;
        let centery = (aabb.mins .y + aabb.maxs .y) * 0.5;
        let centerz = (aabb.mins .z + aabb.maxs .z) * 0.5;

        if !intersects_sphere((centerx, centery, centerz), *_bind, 0., &(arg.0.origin.x, arg.0.origin.y, arg.0.origin.z), &(arg.0.dir.x, arg.0.dir.y, arg.0.dir.z)) {
            return;
        }
        if let Some(distance) = aabb.cast_ray(&Isometry3::identity(), &arg.0, f32::MAX, false) {
            if distance < arg.1 {
                arg.1 = distance;
                // let min = bind.0.transform_point(&Point3::new(-1., -1., -1.));
                // let max = bind.0.transform_point(&Point3::new(1., 1., 1.));
                arg.2.replace(PickResult {
                    target: id.0,
                    min: (aabb.mins.x, aabb.mins.y, aabb.mins.z),
                    max: (aabb.maxs.x, aabb.maxs.y, aabb.maxs.z),
                    pickdetail: None,
                    bybounding: false,
                });
            }
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
    // let t = view_projection.try_inverse().unwrap();
    let mut t = view_projection.clone();
    CoordinateSytem3::try_inverse_mut(&mut t);

    let mut p0 = Vector3::zeros();
    CoordinateSytem3::transform_coordinates_floats(1., 1., 1., &t, &mut p0);

    let mut p1 = Vector3::zeros();
    CoordinateSytem3::transform_coordinates_floats(1., 1., 0., &t, &mut p1);

    let mut p2 = Vector3::zeros();
    CoordinateSytem3::transform_coordinates_floats(-1., 1., -0., &t, &mut p2);

    let mut p3 = Vector3::zeros();
    CoordinateSytem3::transform_coordinates_floats(-1., 1., 1., &t, &mut p3);

    let mut p4 = Vector3::zeros();
    CoordinateSytem3::transform_coordinates_floats(1., -1., 1., &t, &mut p4);

    let mut p5 = Vector3::zeros();
    CoordinateSytem3::transform_coordinates_floats(1., -1., -0., &t, &mut p5);

    let mut p6 = Vector3::zeros();
    CoordinateSytem3::transform_coordinates_floats(-1., -1., -0., &t, &mut p6);

    let mut p7 = Vector3::zeros();
    CoordinateSytem3::transform_coordinates_floats(-1., -1., 1., &t, &mut p7);

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
// pub type ActionListRemoveBindingInfo = ActionList<BoundingKey>;
// pub type ActionListCheckBindingInfo = ActionList<BoundingKey>;

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

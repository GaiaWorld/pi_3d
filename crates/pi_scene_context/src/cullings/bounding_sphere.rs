
use pi_scene_math::{frustum::FrustumPlanes, Matrix, Number, Vector3};

pub struct BoundingSphere {
    radius: f32,
    center: Vector3,
    radius_world: f32,
    center_world: Vector3,
}

impl Default for BoundingSphere {
    fn default() -> Self {
        Self {
            radius: 1.,
            center: Vector3::zeros(),
            radius_world: 1.,
            center_world: Vector3::zeros(),
        }
    }
}

impl BoundingSphere {
    pub fn new(min: &Vector3, max: &Vector3, world: &Matrix) -> Self {
        let mut result = BoundingSphere::default();

        result.reset(min, max, world);

        result
    }

    pub fn reset(&mut self, min: &Vector3, max: &Vector3, world: &Matrix) {
        let _distance = min.metric_distance(max);

        max.add_to(min, &mut self.center);
        self.center.scale_mut(0.5);
        self.radius *= 0.5;

        self.update(world);
    }

    pub fn update(&mut self, world: &Matrix) {
        if world.is_identity(0.00001) {
            self.radius = self.radius;
            self.center_world.copy_from(&self.center);
        } else {
            // 计算 center_world
            // 计算 radius_world
            // world.
        }
    }

    pub fn is_center_in_frustum(&self, frustum_planes: &FrustumPlanes) -> bool {
        let center = &self.center_world;

        if frustum_planes.near.dot_coordinate(center.x, center.y, center.z) < 0.0 {
            return false;
        }
        if frustum_planes.far.dot_coordinate(center.x, center.y, center.z) < 0.0 {
            return false;
        }
        if frustum_planes.left.dot_coordinate(center.x, center.y, center.z) < 0.0 {
            return false;
        }
        if frustum_planes.right.dot_coordinate(center.x, center.y, center.z) < 0.0 {
            return false;
        }
        if frustum_planes.top.dot_coordinate(center.x, center.y, center.z) < 0.0 {
            return false;
        }
        if frustum_planes.bottom.dot_coordinate(center.x, center.y, center.z) < 0.0 {
            return false;
        }

        return true;
    }

    pub fn is_in_frustum(&self, frustum_planes: &FrustumPlanes) -> bool {
        let center = &self.center_world;
        let radius = self.radius_world;

        if frustum_planes.near.dot_coordinate(center.x, center.y, center.z) <= -radius {
            return false;
        }
        if frustum_planes.far.dot_coordinate(center.x, center.y, center.z) <= -radius {
            return false;
        }
        if frustum_planes.left.dot_coordinate(center.x, center.y, center.z) <= -radius {
            return false;
        }
        if frustum_planes.right.dot_coordinate(center.x, center.y, center.z) <= -radius {
            return false;
        }
        if frustum_planes.top.dot_coordinate(center.x, center.y, center.z) <= -radius {
            return false;
        }
        if frustum_planes.bottom.dot_coordinate(center.x, center.y, center.z) <= -radius {
            return false;
        }

        return true;
    }
}

pub fn intersects_sphere(center: (Number, Number, Number), radius: Number, intersection_treshold: Number, origin: &(Number, Number, Number), direction: &(Number, Number, Number)) -> bool {
    let x = center.0 - origin.0;
    let y = center.1 - origin.1;
    let z = center.2 - origin.2;
    let pyth = x * x + y * y + z * z;
    let radius = radius + intersection_treshold;
    let rr = radius * radius;
    
    if radius <= 0. {
        return false;
    }

    if pyth <= rr {
        return true;
    }

    let dot = x * direction.0 + y * direction.1 + z * direction.2;
    if dot < 0.0 {
        return false;
    }

    let temp = pyth - dot * dot;

    return temp <= rr;
}
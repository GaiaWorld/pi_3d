use pi_scene_math::{frustum::FrustumPlanes, vector::TMinimizeMaximize, Matrix, Vector3};

pub struct BoundingBox {
    center: Vector3,
    extend: Vector3,
    center_world: Vector3,
    extend_world: Vector3,
    pub vectors_world: [Vector3; 8],
}

impl Default for BoundingBox {
    fn default() -> Self {
        Self {
            center: Vector3::zeros(),
            extend: Vector3::new(0.5, 0.5, 0.5),
            center_world: Vector3::zeros(),
            extend_world: Vector3::new(0.5, 0.5, 0.5),
            vectors_world: [Vector3::zeros(); 8],
        }
    }
}

impl BoundingBox {
    pub fn new(min: &Vector3, max: &Vector3, world: &Matrix) -> Self {
        let mut result = BoundingBox::default();

        result.reset(min, max, world);

        result
    }

    pub fn reset(&mut self, min: &Vector3, max: &Vector3, world: &Matrix) {
        max.add_to(min, &mut self.center);
        self.center.scale_mut(0.5);

        max.sub_to(min, &mut self.extend);
        self.extend.scale_mut(0.5);

        if world.is_identity(0.00001) {
            self.center_world.copy_from(&self.center);
            self.extend_world.copy_from(&self.extend);

            self.vectors_world[0].copy_from_slice(&[min.x, min.y, min.z]);
            self.vectors_world[1].copy_from_slice(&[max.x, max.y, max.z]);
            self.vectors_world[2].copy_from_slice(&[max.x, min.y, min.z]);
            self.vectors_world[3].copy_from_slice(&[min.x, max.y, min.z]);
            self.vectors_world[4].copy_from_slice(&[min.x, min.y, max.z]);
            self.vectors_world[5].copy_from_slice(&[max.x, max.y, min.z]);
            self.vectors_world[6].copy_from_slice(&[min.x, max.y, max.z]);
            self.vectors_world[7].copy_from_slice(&[max.x, min.y, max.z]);
        } else {
            self.center_world
                .copy_from_slice(&[f32::MAX, f32::MAX, f32::MAX]);
            self.extend_world
                .copy_from_slice(&[f32::MIN, f32::MIN, f32::MIN]);

            let mut min_world: Vector3 = Vector3::new(f32::MAX, f32::MAX, f32::MAX);
            let mut max_world: Vector3 = Vector3::new(f32::MIN, f32::MIN, f32::MIN);

            let v1: Vector3 = world.transform_vector(&Vector3::new(min.x, min.y, min.z));
            min_world.minimize_mut(&v1);
            max_world.maximize_mut(&v1);
            let v1: Vector3 = world.transform_vector(&Vector3::new(max.x, max.y, max.z));
            min_world.minimize_mut(&v1);
            max_world.maximize_mut(&v1);
            let v1: Vector3 = world.transform_vector(&Vector3::new(max.x, min.y, min.z));
            min_world.minimize_mut(&v1);
            max_world.maximize_mut(&v1);
            let v1: Vector3 = world.transform_vector(&Vector3::new(min.x, max.y, min.z));
            min_world.minimize_mut(&v1);
            max_world.maximize_mut(&v1);
            let v1: Vector3 = world.transform_vector(&Vector3::new(min.x, min.y, max.z));
            min_world.minimize_mut(&v1);
            max_world.maximize_mut(&v1);
            let v1: Vector3 = world.transform_vector(&Vector3::new(max.x, max.y, min.z));
            min_world.minimize_mut(&v1);
            max_world.maximize_mut(&v1);
            let v1: Vector3 = world.transform_vector(&Vector3::new(min.x, max.y, max.z));
            min_world.minimize_mut(&v1);
            max_world.maximize_mut(&v1);
            let v1: Vector3 = world.transform_vector(&Vector3::new(max.x, min.y, max.z));
            min_world.minimize_mut(&v1);
            max_world.maximize_mut(&v1);

            max_world.add_to(&min_world, &mut self.center_world);
            self.center_world.scale_mut(0.5);
            max_world.sub_to(&min_world, &mut self.extend_world);
            self.extend_world.scale_mut(0.5);
        }
    }

    pub fn is_in_frustum(&self, frustum_planes: &FrustumPlanes) -> bool {
        let bounding_vectors = &self.vectors_world;
        let mut can_return_false = true;

        for i in 0..8 {
            if frustum_planes.near.dot_coordinate(&bounding_vectors[i]) >= 0.0 {
                can_return_false = false;
                break;
            }
            if frustum_planes.far.dot_coordinate(&bounding_vectors[i]) >= 0.0 {
                can_return_false = false;
                break;
            }
            if frustum_planes.left.dot_coordinate(&bounding_vectors[i]) >= 0.0 {
                can_return_false = false;
                break;
            }
            if frustum_planes.right.dot_coordinate(&bounding_vectors[i]) >= 0.0 {
                can_return_false = false;
                break;
            }
            if frustum_planes.top.dot_coordinate(&bounding_vectors[i]) >= 0.0 {
                can_return_false = false;
                break;
            }
            if frustum_planes.bottom.dot_coordinate(&bounding_vectors[i]) >= 0.0 {
                can_return_false = false;
                break;
            }
        }
        return can_return_false;
    }
}

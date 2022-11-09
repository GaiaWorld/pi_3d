
use pi_render::rhi::dyn_uniform_buffer::Uniform;
use pi_scene_math::{Matrix, Vector3, Rotation3, coordiante_system::CoordinateSytem3, Quaternion, vector::TToolMatrix, Translation3, Isometry3, Vector4};

use crate::{bytes_write_to_memory, meshes::model::BuildinModelBind};


#[derive(Debug, Clone, Copy)]
pub struct TransformNode;

#[derive(Debug, Clone)]
pub struct LocalTransform {
    pub position: Vector3,
    pub scaling: Vector3,
    pub euler: Vector3,
    pub quaternion: Quaternion,
    pub rotation: Rotation3,
    pub matrix: Matrix,
    pub use_quaternion: bool,
}
impl Default for LocalTransform {
    fn default() -> Self {
        Self {
            position: Vector3::new(0., 0., 0.),
            scaling: Vector3::new(1., 1., 1.),
            euler: Vector3::new(0., 0., 0.),
            quaternion: Quaternion::identity(),
            rotation: Rotation3::identity(),
            matrix: Matrix::identity(),
            use_quaternion: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GlobalTransform {
    pub position: Vector3,
    pub scaling: Vector3,
    pub euler: Vector3,
    pub quaternion: Quaternion,
    pub rotation: Rotation3,
    pub matrix: Matrix,
    pub matrix_inv: Matrix,
    pub iso: Isometry3,
}
impl Default for GlobalTransform {
    fn default() -> Self {
        Self {
            position: Vector3::new(0., 0., 0.),
            scaling: Vector3::new(1., 1., 1.),
            euler: Vector3::new(0., 0., 0.),
            quaternion: Quaternion::identity(),
            rotation: Rotation3::identity(),
            matrix: Matrix::identity(),
            matrix_inv: Matrix::identity(),
            iso: Isometry3::identity(),
        }
    }
}
impl GlobalTransform {
    pub fn calc(&mut self, p_m: Option<Matrix>, l_transform: &LocalTransform) {
        calc_world_matrix(
            p_m,
            &l_transform.matrix,
            &mut self.matrix,
            &mut self.position,
            &mut self.rotation,
            &mut self.scaling,
            &mut self.iso,
        );

        match self.matrix.try_inverse() {
            Some(inv) => self.matrix_inv = inv,
            None => self.matrix_inv = Matrix::identity(),
        }
    }
}
impl Uniform for GlobalTransform {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        bytes_write_to_memory(bytemuck::cast_slice(self.matrix.as_slice()), index as usize + BuildinModelBind::OBJECT_TO_WORLD_OFFSIZE, buffer);
        bytes_write_to_memory(bytemuck::cast_slice(self.matrix_inv.as_slice()), index as usize + BuildinModelBind::WORLD_TO_OBJECT_OFFSIZE, buffer);
    }
}

pub fn calc_world_matrix(
    p_m: Option<Matrix>,
    l_m: &Matrix,
    w_m: &mut Matrix,
    g_p: &mut Vector3,
    g_r: &mut Rotation3,
    g_s: &mut Vector3,
    g_i: &mut Isometry3,
) {
    match p_m {
        Some(p_m) => {
            // p_m.mul_to(&l_m, w_m);
            // l_m.mul_to(&p_m, w_m);
            w_m.copy_from(&(p_m * l_m));
        },
        None => {
            w_m.copy_from(&l_m);
        },
    }

    CoordinateSytem3::matrix4_decompose_rotation(&w_m, Some(g_s), Some(g_r), Some(g_p));

    // println!("calc_world_matrix:");
    // println!("{}", w_m);
    // println!("absolute_scaling:");
    // println!("{}", g_s);
    // println!("absolute_rotation:");
    // println!("{}", g_r);
    // println!("absolute_position:");
    // println!("{}", g_p);

    g_i.clone_from(&Isometry3::from_parts(Translation3::new(g_p.x, g_p.y, g_p.z), Quaternion::from_matrix(&g_r.matrix())));
}

use pi_scene_math::{Matrix, Vector3, Rotation3, coordiante_system::CoordinateSytem3, Quaternion, vector::TToolMatrix, Translation3, Isometry3, Number};


#[derive(Debug, Clone, Copy)]
pub struct TransformNode;

#[derive(Debug, Clone)]
pub struct LocalPosition(pub Vector3);

#[derive(Debug, Clone)]
pub struct LocalEulerAngles(pub Vector3);

#[derive(Debug, Clone)]
pub struct LocalRotationQuaternion(pub Quaternion);

#[derive(Debug, Clone)]
pub struct LocalRoationWithQuaternion(pub bool);

#[derive(Debug, Clone)]
pub struct LocalRotation(pub Rotation3);

#[derive(Debug, Clone)]
pub struct LocalScaling(pub Vector3);


#[derive(Debug, Clone)]
pub struct LocalMatrix(pub Matrix, pub bool);
impl LocalMatrix {
    pub fn new(m: Matrix) -> Self {
        Self(m, true)
    }
}

#[derive(Debug, Clone)]
pub struct WorldMatrix(pub Matrix, pub bool);
impl WorldMatrix {
    pub fn new(m: Matrix) -> Self {
        Self(m, true)
    }
}

#[derive(Debug, Clone)]
pub struct WorldMatrixInv(pub Matrix, pub bool);
impl WorldMatrixInv {
    pub fn new(m: Matrix) -> Self {
        Self(m, true)
    }
}

#[derive(Debug, Clone)]
pub struct GlobalTransform {
    pub position: Vector3,
    pub scaling: Vector3,
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
            rotation: Rotation3::identity(),
            matrix: Matrix::identity(),
            matrix_inv: Matrix::identity(),
            iso: Isometry3::identity(),
        }
    }
}
impl GlobalTransform {
    pub fn euler_angles(&self) -> (Number, Number, Number) {
        self.rotation.euler_angles()
    }
    pub fn rotation_quaternion(&self) -> Quaternion {
        Quaternion::from_rotation_matrix(&self.rotation)
    }
    pub fn calc(p_m: &Matrix, l_matrix: &LocalMatrix) -> Self {
        let mut result = Self::default();
        calc_world_matrix(
            p_m,
            &l_matrix.0,
            &mut result.matrix,
            &mut result.position,
            &mut result.rotation,
            &mut result.scaling,
            &mut result.iso,
        );

        match result.matrix.try_inverse() {
            Some(inv) => result.matrix_inv = inv,
            None => result.matrix_inv = Matrix::identity(),
        };

        result
    }
}

pub fn calc_world_matrix(
    p_m: &Matrix,
    l_m: &Matrix,
    w_m: &mut Matrix,
    g_p: &mut Vector3,
    g_r: &mut Rotation3,
    g_s: &mut Vector3,
    g_i: &mut Isometry3,
) {
    w_m.copy_from(&(p_m * l_m));

    CoordinateSytem3::matrix4_decompose_rotation(&w_m, Some(g_s), Some(g_r), Some(g_p));

    // log::debug!("calc_world_matrix:");
    // log::debug!("{}", w_m);
    // log::debug!("absolute_scaling:");
    // log::debug!("{}", g_s);
    // log::debug!("absolute_rotation:");
    // log::debug!("{}", g_r);
    // log::debug!("absolute_position:");
    // log::debug!("{}", g_p);

    g_i.clone_from(&Isometry3::from_parts(Translation3::new(g_p.x, g_p.y, g_p.z), Quaternion::from_matrix(&g_r.matrix())));
}

use ncollide3d::utils::IsometryOps;
use pi_scene_math::{Matrix, Vector3, Rotation3, coordiante_system::CoordinateSytem3, Quaternion, vector::TToolMatrix, Translation3, Isometry3, Number};


#[derive(Debug, Clone, Copy)]
pub struct TransformNode;

#[derive(Debug, Clone)]
pub struct LocalPosition(pub Vector3);
impl pi_curves::curve::frame::FrameDataValue for LocalPosition {
    fn interpolate(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0.lerp(&rhs.0, amount))
    }

    fn append(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0 + rhs.0.scale(amount))
    }
    fn size() -> usize {
        3 * 4
    }
}

#[derive(Debug, Clone)]
pub struct LocalEulerAngles(pub Vector3);
impl pi_curves::curve::frame::FrameDataValue for LocalEulerAngles {
    fn interpolate(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0.lerp(&rhs.0, amount))
    }

    fn append(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0 + rhs.0.scale(amount))
    }
    fn size() -> usize {
        3 * 4
    }
}

#[derive(Debug, Clone)]
pub struct LocalRotationQuaternion(pub Quaternion);
impl pi_curves::curve::frame::FrameDataValue for LocalRotationQuaternion {
    fn interpolate(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        let temp = self.0.lerp(&rhs.0, amount);
        Self(Quaternion::from_quaternion(temp))
    }

    fn append(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        log::warn!("LocalRotationQuaternion has not 'append' operation!");
        self.clone()
    }
    fn size() -> usize {
        4 * 4
    }
}

#[derive(Debug, Clone)]
pub struct LocalRoationWithQuaternion(pub bool);

#[derive(Debug, Clone)]
pub struct LocalRotation(pub Rotation3);

#[derive(Debug, Clone)]
pub struct LocalScaling(pub Vector3);
impl pi_curves::curve::frame::FrameDataValue for LocalScaling {
    fn interpolate(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0.lerp(&rhs.0, amount))
    }

    fn append(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0 + rhs.0.scale(amount))
    }
    fn size() -> usize {
        3 * 4
    }
}


#[derive(Debug, Clone)]
pub struct LocalMatrix(pub Matrix, pub bool);
impl LocalMatrix {
    pub fn new(m: Matrix) -> Self {
        Self(m, true)
    }
}
impl pi_curves::curve::frame::FrameDataValue for LocalMatrix {
    fn interpolate(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0.scale(1.0 - amount) + rhs.0.scale(amount), true)
    }

    fn append(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0 + rhs.0.scale(amount), true)
    }
    fn size() -> usize {
        16 * 4
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
    pub position: Option<Vector3>,
    pub scaling: Option<Vector3>,
    pub rotation: Option<Rotation3>,
    pub matrix: Matrix,
    pub matrix_inv: Matrix,
    pub iso: Option<Isometry3>,
}
impl Default for GlobalTransform {
    fn default() -> Self {
        Self {
            position: None,
            scaling: None,
            rotation: None,
            matrix: Matrix::identity(),
            matrix_inv: Matrix::identity(),
            iso: None,
        }
    }
}
impl GlobalTransform {
    pub fn euler_angles(&mut self) -> (Number, Number, Number) {
        self.decompose();
        self.rotation.unwrap().euler_angles()
    }
    pub fn rotation_quaternion(&mut self) -> Quaternion {
        self.decompose();
        Quaternion::from_rotation_matrix(&self.rotation.unwrap())
    }
    pub fn position(&mut self) -> &Vector3 {
        self.decompose();
        self.position.as_ref().unwrap()
    }
    pub fn scaling(&mut self) -> &Vector3 {
        self.decompose();
        self.scaling.as_ref().unwrap()
    }
    pub fn iso(&mut self) -> &Isometry3 {
        self.decompose();
        self.iso.as_ref().unwrap()
    }
    pub fn calc(p_m: &Matrix, l_matrix: &LocalMatrix) -> Self {
        let mut result = Self::default();
        result.matrix.copy_from(&(p_m * l_matrix.0));

        match result.matrix.try_inverse() {
            Some(inv) => result.matrix_inv = inv,
            None => result.matrix_inv = Matrix::identity(),
        };

        result
    }
    fn decompose(&mut self) {
        if self.rotation.is_none() {
            let mut g_r = Rotation3::identity();
            let mut g_p = Vector3::new(0., 0., 0.);
            let mut g_s = Vector3::new(1., 1., 1.);
            CoordinateSytem3::matrix4_decompose_rotation(&self.matrix, Some(&mut g_s), Some(&mut g_r), Some(&mut g_p));

            // log::debug!("calc_world_matrix:");
            // log::debug!("{}", w_m);
            // log::debug!("absolute_scaling:");
            // log::debug!("{}", g_s);
            // log::debug!("absolute_rotation:");
            // log::debug!("{}", g_r);
            // log::debug!("absolute_position:");
            // log::debug!("{}", g_p);
        
            let iso = Isometry3::from_parts(Translation3::new(g_p.x, g_p.y, g_p.z), Quaternion::from_matrix(&g_r.matrix()));
            self.iso = Some(iso);
            self.rotation = Some(g_r);
            self.scaling = Some(g_s);
            self.position = Some(g_p);
        }
    }
}
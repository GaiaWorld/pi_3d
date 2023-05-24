use pi_engine_shell::prelude::*;
use pi_scene_math::{Matrix, Vector3, Rotation3, coordiante_system::CoordinateSytem3, Quaternion, vector::TToolMatrix, Translation3, Isometry3, Number};


#[derive(Debug, Clone, Copy, Component)]
pub struct TransformNode;

#[derive(Debug, Clone, Component)]
pub struct LocalPosition(pub Vector3);
impl pi_curves::curve::frame::FrameDataValue for LocalPosition {
    fn interpolate(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0.lerp(&rhs.0, amount))
    }

    fn hermite(value1: &Self, tangent1: &Self, value2: &Self, tangent2: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        let _1 = 1.;
        let _2 = 2.;
        let _3 = 3.;

        let squared = amount * amount;
        let cubed = amount * squared;
        let part1 = ((_2 * cubed) - (_3 * squared)) + _1;
        let part2 = (-_2 * cubed) + (_3 * squared);
        let part3 = (cubed - (_2 * squared)) + amount;
        let part4 = cubed - squared;

        let result = (((value1.0 * part1) + (value2.0 * part2)) + (tangent1.0 * part3)) + (tangent2.0 * part4);
        return Self(result);
    }

    fn append(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0 + rhs.0.scale(amount))
    }
    fn size() -> usize {
        3 * 4
    }
}

#[derive(Debug, Clone, Component)]
pub struct LocalEulerAngles(pub Vector3);
impl pi_curves::curve::frame::FrameDataValue for LocalEulerAngles {
    fn interpolate(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0.lerp(&rhs.0, amount))
    }

    fn hermite(value1: &Self, tangent1: &Self, value2: &Self, tangent2: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        let _1 = 1.;
        let _2 = 2.;
        let _3 = 3.;

        let squared = amount * amount;
        let cubed = amount * squared;
        let part1 = ((_2 * cubed) - (_3 * squared)) + _1;
        let part2 = (-_2 * cubed) + (_3 * squared);
        let part3 = (cubed - (_2 * squared)) + amount;
        let part4 = cubed - squared;

        let result = (((value1.0 * part1) + (value2.0 * part2)) + (tangent1.0 * part3)) + (tangent2.0 * part4);
        return Self(result);
    }

    fn append(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0 + rhs.0.scale(amount))
    }
    fn size() -> usize {
        3 * 4
    }
}

#[derive(Debug, Clone, Component)]
pub struct LocalRotationQuaternion(pub Quaternion);
impl pi_curves::curve::frame::FrameDataValue for LocalRotationQuaternion {
    fn interpolate(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        let temp = self.0.slerp(&rhs.0, amount);
        Self(temp)
    }

    fn hermite(value1: &Self, tangent1: &Self, value2: &Self, tangent2: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        let _1 = 1.;
        let _2 = 2.;
        let _3 = 3.;

        let squared = amount * amount;
        let cubed = amount * squared;
        let part1 = ((_2 * cubed) - (_3 * squared)) + _1;
        let part2 = (-_2 * cubed) + (_3 * squared);
        let part3 = (cubed - (_2 * squared)) + amount;
        let part4 = cubed - squared;

        let result = (((value1.0.quaternion() * part1) + (value2.0.quaternion() * part2)) + (tangent1.0.quaternion() * part3)) + (tangent2.0.quaternion() * part4);
        return Self(Quaternion::from_quaternion(result));
    }

    fn append(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        log::warn!("LocalRotationQuaternion has not 'append' operation!");
        self.clone()
    }
    fn size() -> usize {
        4 * 4
    }
}

#[derive(Debug, Clone, Component)]
pub struct LocalRoationWithQuaternion(pub bool);

#[derive(Debug, Clone, Component)]
pub struct LocalRotation(pub Rotation3);

#[derive(Debug, Clone, Component)]
pub struct LocalScaling(pub Vector3);
impl pi_curves::curve::frame::FrameDataValue for LocalScaling {
    fn interpolate(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0.lerp(&rhs.0, amount))
    }

    fn hermite(value1: &Self, tangent1: &Self, value2: &Self, tangent2: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        let _1 = 1.;
        let _2 = 2.;
        let _3 = 3.;

        let squared = amount * amount;
        let cubed = amount * squared;
        let part1 = ((_2 * cubed) - (_3 * squared)) + _1;
        let part2 = (-_2 * cubed) + (_3 * squared);
        let part3 = (cubed - (_2 * squared)) + amount;
        let part4 = cubed - squared;

        let result = (((value1.0 * part1) + (value2.0 * part2)) + (tangent1.0 * part3)) + (tangent2.0 * part4);
        return Self(result);
    }

    fn append(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0 + rhs.0.scale(amount))
    }
    fn size() -> usize {
        3 * 4
    }
}


#[derive(Debug, Clone, Component)]
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

    fn hermite(value1: &Self, tangent1: &Self, value2: &Self, tangent2: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        let _1 = 1.;
        let _2 = 2.;
        let _3 = 3.;

        let squared = amount * amount;
        let cubed = amount * squared;
        let part1 = ((_2 * cubed) - (_3 * squared)) + _1;
        let part2 = (-_2 * cubed) + (_3 * squared);
        let part3 = (cubed - (_2 * squared)) + amount;
        let part4 = cubed - squared;

        let result = (((value1.0 * part1) + (value2.0 * part2)) + (tangent1.0 * part3)) + (tangent2.0 * part4);
        return Self(result, true);
    }


    fn append(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0 + rhs.0.scale(amount), true)
    }
    fn size() -> usize {
        16 * 4
    }
}

#[derive(Debug, Clone, Component)]
pub struct WorldMatrix(pub Matrix, pub bool);
impl WorldMatrix {
    pub fn new(m: Matrix) -> Self {
        Self(m, true)
    }
}

#[derive(Debug, Clone, Component)]
pub struct WorldMatrixInv(pub Matrix, pub bool);
impl WorldMatrixInv {
    pub fn new(m: Matrix) -> Self {
        Self(m, true)
    }
}

#[derive(Debug, Clone, Component)]
pub struct GlobalTransform {
    pub position: Vector3,
    pub scaling: Option<Vector3>,
    pub rotation: Option<Rotation3>,
    pub matrix: Matrix,
    pub matrix_inv: Matrix,
    pub iso: Option<Isometry3>,
}
impl Default for GlobalTransform {
    fn default() -> Self {
        Self {
            position: Vector3::new(0., 0., 0.),
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
    pub fn position(&self) -> &Vector3 {
        &self.position
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
        
        result.position = Vector3::from(result.matrix.fixed_view::<3, 1>(0, 3));

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
        
            let temp = g_p.as_slice();
            let iso = Isometry3::from_parts(Translation3::new(temp[0], temp[1], temp[2]), Quaternion::from_matrix(&g_r.matrix()));
            self.iso = Some(iso);
            self.rotation = Some(g_r);
            self.scaling = Some(g_s);
        }
    }
}
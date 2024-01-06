use pi_engine_shell::prelude::*;
use pi_scene_math::{Matrix, Vector3, Rotation3, coordiante_system::CoordinateSytem3, Quaternion, vector::TToolMatrix, Translation3, Isometry3, Number, SQuaternion};

#[derive(Debug, Clone, Copy, Component)]
pub struct NodeDown(pub Option<Entity>);

#[derive(Debug, Clone, Copy, Component)]
pub struct NodeUp(pub Entity);

#[derive(Debug, Clone, Copy, Component)]
pub struct NodeBrothers {
    pub idx: usize,
    pub pre: Option<Entity>,
    pub next: Option<Entity>,
}

#[derive(Debug, Clone, Copy, Component)]
pub struct TransformNode;

#[derive(Clone, Copy, Component)]
pub struct TransformNodeDirty;

#[derive(Debug, Clone, Component)]
pub struct LocalDirtyRotation;

#[derive(Debug, Clone, Component)]
pub struct LocalDirtyScaling;

#[derive(Debug, Clone, Component, Default)]
pub struct RecordLocalPosition(pub LocalPosition);
impl TAnimatableCompRecord<LocalPosition> for RecordLocalPosition {
    fn comp(&self) -> LocalPosition {
        self.0.clone()
    }
}

#[derive(Debug, Clone, Component)]
pub struct LocalPosition(pub Vector3);
impl pi_curves::curve::frame::FrameDataValue for LocalPosition {
    fn interpolate(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0.lerp(&rhs.0, amount))
    }

    fn hermite(value1: &Self, tangent1: &Self, value2: &Self, tangent2: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue, frame_delta: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        let _1 = 1.;
        let _2 = 2.;
        let _3 = 3.;

        let squared = amount * amount;
        let cubed = amount * squared;
        let part1 = ((_2 * cubed) - (_3 * squared)) + _1;
        let part2 = (-_2 * cubed) + (_3 * squared);
        let part3 = ((cubed - (_2 * squared)) + amount) * frame_delta;
        let part4 = (cubed - squared) * frame_delta;

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
impl Default for LocalPosition {
    fn default() -> Self {
        Self(Vector3::new(0., 0., 0.))
    }
}
impl TAssetCapacity for LocalPosition {
    const ASSET_TYPE: &'static str = "AnimeCurveLocalPosition";
    fn capacity() -> AssetCapacity {
        AssetCapacity { flag: false, min: 5 * 1024 * 1024, max: 10 * 1024 * 1024, timeout: 1 * 60 * 1000 }
    }
}
impl TAnimatableComp for LocalPosition {

}

#[derive(Debug, Clone, Component, Default)]
pub struct RecordLocalEulerAngles(pub LocalEulerAngles);
impl TAnimatableCompRecord<LocalEulerAngles> for RecordLocalEulerAngles {
    fn comp(&self) -> LocalEulerAngles {
        self.0.clone()
    }
}

#[derive(Debug, Clone, Component)]
pub struct LocalEulerAngles(pub Vector3);
impl pi_curves::curve::frame::FrameDataValue for LocalEulerAngles {
    fn interpolate(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0.lerp(&rhs.0, amount))
    }

    fn hermite(value1: &Self, tangent1: &Self, value2: &Self, tangent2: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue, frame_delta: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        let _1 = 1.;
        let _2 = 2.;
        let _3 = 3.;

        let squared = amount * amount;
        let cubed = amount * squared;
        let part1 = ((_2 * cubed) - (_3 * squared)) + _1;
        let part2 = (-_2 * cubed) + (_3 * squared);
        let part3 = ((cubed - (_2 * squared)) + amount) * frame_delta;
        let part4 = (cubed - squared) * frame_delta;

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
impl Default for LocalEulerAngles {
    fn default() -> Self {
        Self(Vector3::new(0., 0., 0.))
    }
}
impl TAssetCapacity for LocalEulerAngles {
    const ASSET_TYPE: &'static str = "AnimeCurveLocalEulerAngles";
    fn capacity() -> AssetCapacity {
        AssetCapacity { flag: false, min: 5 * 1024 * 1024, max: 10 * 1024 * 1024, timeout: 1 * 60 * 1000 }
    }
}
impl TAnimatableComp for LocalEulerAngles {

}

#[derive(Debug, Clone, Component, Default)]
pub struct RecordLocalRotationQuaternion(pub LocalRotationQuaternion);
impl TAnimatableCompRecord<LocalRotationQuaternion> for RecordLocalRotationQuaternion {
    fn comp(&self) -> LocalRotationQuaternion {
        self.0.clone()
    }
}

#[derive(Debug, Clone, Component)]
pub struct LocalRotationQuaternion(pub SQuaternion<Number>);
impl LocalRotationQuaternion {
    pub fn create(x: Number, y: Number, z: Number, w: Number) -> Self {
        Self(SQuaternion::new(w, x, y, z))
    }
}
impl pi_curves::curve::frame::FrameDataValue for LocalRotationQuaternion {
    fn interpolate(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        let li = self.0.i;
        let lj = self.0.j;
        let lk = self.0.k;
        let lw = self.0.w;
        let ri = rhs.0.i;
        let rj = rhs.0.j;
        let rk = rhs.0.k;
        let rw = rhs.0.w;
        let mut num4 = li * ri + lj * rj + lk * rk + lw * rw;
        let flag = num4 < 0.;
        if flag {
            num4 = -1.0 * num4;
        }
        let (num2, num3) = if 0.9999999 < num4 {
            (
                if flag { -1. * amount } else { amount },
                1.0 - amount
            )
        } else {
            let num5 = Number::acos(num4);
            let num6 = 1.0 / Number::sin(num5);
            (
                if flag { -1.0 * Number::sin(amount * num5) * num6 } else { Number::sin(num5) * num6 },
                Number::sin((1.0 - amount) * num5) * num6
            )
        };

        Self::create(num3 * li + num2 * ri, num3 * lj + num2 * rj, num3 * lk + num2 * rk, num3 * lw + num2 * rw)
    }

    fn hermite(value1: &Self, tangent1: &Self, value2: &Self, tangent2: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue, frame_delta: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        let _1 = 1.;
        let _2 = 2.;
        let _3 = 3.;

        let squared = amount * amount;
        let cubed = amount * squared;
        let part1 = ((_2 * cubed) - (_3 * squared)) + _1;
        let part2 = (-_2 * cubed) + (_3 * squared);
        let part3 = ((cubed - (_2 * squared)) + amount) * frame_delta;
        let part4 = (cubed - squared) * frame_delta;

        let result = (((value1.0 * part1) + (value2.0 * part2)) + (tangent1.0 * part3)) + (tangent2.0 * part4);

        // log::warn!("{}, {:?}, {:?}, {:?}, {:?},", amount, frame_delta, (value1.0, tangent1.0), (value2.0, tangent2.0), result);

        // let i = (((value1.0.i * part1) + (value2.0.i * part2)) + (tangent1.0.i * part3)) + (tangent2.0.i * part4);
        // let j = (((value1.0.j * part1) + (value2.0.j * part2)) + (tangent1.0.j * part3)) + (tangent2.0.j * part4);
        // let k = (((value1.0.k * part1) + (value2.0.k * part2)) + (tangent1.0.k * part3)) + (tangent2.0.k * part4);
        // let w = (((value1.0.w * part1) + (value2.0.w * part2)) + (tangent1.0.w * part3)) + (tangent2.0.w * part4);
        let result = Self(result);

        // log::warn!("Value1: {:?} Value2: {:?} Result: {:?}", value1, value2, result);

        return result;
    }

    fn append(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        // log::warn!("LocalRotationQuaternion has not 'append' operation!");
        Self(self.0 + rhs.0 * amount)
    }
    fn size() -> usize {
        4 * 4
    }
}
impl Default for LocalRotationQuaternion {
    fn default() -> Self {
        let result = Self::create(0., 0., 0., 1.);
        result
    }
}
impl TAssetCapacity for LocalRotationQuaternion {
    const ASSET_TYPE: &'static str = "AnimeCurveLocalRotationQuaternion";
    fn capacity() -> AssetCapacity {
        AssetCapacity { flag: false, min: 5 * 1024 * 1024, max: 10 * 1024 * 1024, timeout: 1 * 60 * 1000 }
    }
}
impl TAnimatableComp for LocalRotationQuaternion {

}

#[derive(Debug, Clone, Component)]
pub struct LocalRoationWithQuaternion(pub bool);

#[derive(Debug, Clone, Component)]
pub struct LocalRotation(pub Rotation3);

#[derive(Debug, Clone, Component, Default)]
pub struct RecordLocalScaling(pub LocalScaling);
impl TAnimatableCompRecord<LocalScaling> for RecordLocalScaling {
    fn comp(&self) -> LocalScaling {
        self.0.clone()
    }
}

#[derive(Debug, Clone, Component)]
pub struct LocalScaling(pub Vector3);
impl pi_curves::curve::frame::FrameDataValue for LocalScaling {
    fn interpolate(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0.lerp(&rhs.0, amount))
    }

    fn hermite(value1: &Self, tangent1: &Self, value2: &Self, tangent2: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue, frame_delta: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        let _1 = 1.;
        let _2 = 2.;
        let _3 = 3.;

        let squared = amount * amount;
        let cubed = amount * squared;
        let part1 = ((_2 * cubed) - (_3 * squared)) + _1;
        let part2 = (-_2 * cubed) + (_3 * squared);
        let part3 = ((cubed - (_2 * squared)) + amount) * frame_delta;
        let part4 = (cubed - squared) * frame_delta;

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
impl Default for LocalScaling {
    fn default() -> Self {
        Self(Vector3::new(1., 1., 1.))
    }
}
impl TAssetCapacity for LocalScaling {
    const ASSET_TYPE: &'static str = "AnimeCurveLocalScaling";
    fn capacity() -> AssetCapacity {
        AssetCapacity { flag: false, min: 5 * 1024 * 1024, max: 10 * 1024 * 1024, timeout: 1 * 60 * 1000 }
    }
}
impl TAnimatableComp for LocalScaling {

}


#[derive(Debug, Clone, Component)]
pub struct LocalMatrix(pub Matrix);
impl LocalMatrix {
    pub fn new(m: Matrix) -> Self {
        Self(m)
    }
}
impl pi_curves::curve::frame::FrameDataValue for LocalMatrix {
    fn interpolate(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0.scale(1.0 - amount) + rhs.0.scale(amount))
    }

    fn hermite(value1: &Self, tangent1: &Self, value2: &Self, tangent2: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue, frame_delta: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        let _1 = 1.;
        let _2 = 2.;
        let _3 = 3.;

        let squared = amount * amount;
        let cubed = amount * squared;
        let part1 = ((_2 * cubed) - (_3 * squared)) + _1;
        let part2 = (-_2 * cubed) + (_3 * squared);
        let part3 = ((cubed - (_2 * squared)) + amount) * frame_delta;
        let part4 = (cubed - squared) * frame_delta;

        let result = (((value1.0 * part1) + (value2.0 * part2)) + (tangent1.0 * part3)) + (tangent2.0 * part4);
        return Self(result);
    }


    fn append(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0 + rhs.0.scale(amount))
    }
    fn size() -> usize {
        16 * 4
    }
}

// #[derive(Debug, Clone, Component)]
// pub struct WorldMatrix(pub Matrix, pub bool);
// impl WorldMatrix {
//     pub fn new(m: Matrix) -> Self {
//         Self(m, true)
//     }
// }

// #[derive(Debug, Clone, Component)]
// pub struct WorldMatrixInv(pub Matrix, pub bool);
// impl WorldMatrixInv {
//     pub fn new(m: Matrix) -> Self {
//         Self(m, true)
//     }
// }

#[derive(Debug, Clone, Component)]
pub struct GlobalTransform {
    // pub position: Vector3,
    pub scaling: Option<Vector3>,
    pub rotation: Option<Rotation3>,
    pub matrix: Matrix,
    pub matrix_inv: Matrix,
    pub iso: Option<Isometry3>,
}
impl Default for GlobalTransform {
    fn default() -> Self {
        Self {
            // position: Vector3::new(0., 0., 0.),
            scaling: None,
            rotation: None,
            matrix: Matrix::identity(),
            matrix_inv: Matrix::identity(),
            iso: None,
        }
    }
}
impl GlobalTransform {
    pub fn euler_angles(& self) -> (Number, Number, Number) {
        // self.decompose();
        self.rotation.unwrap().euler_angles()
    }
    pub fn rotation_quaternion(& self) -> Quaternion {
        // self.decompose();
        Quaternion::from_rotation_matrix(&self.rotation.unwrap())
    }
    pub fn rotation(& self) -> &Rotation3 {
        // self.decompose();
        self.rotation.as_ref().unwrap()
    }
    pub fn position(&self) -> Vector3 {
        Vector3::from(self.matrix.fixed_view::<3, 1>(0, 3))
    }
    pub fn scaling(& self) -> &Vector3 {
        // self.decompose();
        self.scaling.as_ref().unwrap()
    }
    pub fn iso(& self) -> &Isometry3 {
        // self.decompose();
        self.iso.as_ref().unwrap()
    }
    pub fn calc(p_m: &Matrix, l_matrix: &LocalMatrix) -> (Self, bool) {
        let mut flag = true;
        let mut result = Self::default();
        result.matrix.copy_from(&(p_m * l_matrix.0));

        if result.matrix.as_slice()[0].is_finite() {
            match result.matrix.try_inverse() {
                Some(val) => {
                    result.matrix_inv = val;
                }
                None => {
                    flag = false;
                    result.matrix = Matrix::identity();
                    result.matrix_inv = Matrix::identity(); 
                }
            }
        } else {
            flag = false;
            result.matrix = Matrix::identity();
            result.matrix_inv = Matrix::identity();
        }

        result.decompose();

        (result, flag)
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

#[derive(Default, Resource)]
pub struct TransformDirtyRoots(pub Vec<Entity>);
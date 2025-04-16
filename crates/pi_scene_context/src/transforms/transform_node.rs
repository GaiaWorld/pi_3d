use pi_scene_shell::prelude::*;
use pi_scene_math::{Matrix, Vector3, Rotation3, coordiante_system::CoordinateSytem3, Quaternion, vector::TToolMatrix, Translation3, Isometry3, Number, SQuaternion};

/// 标识实体类型 TransformNode
#[derive(Clone, Copy, Component, Default)]
pub struct TransformNode;

/// 标识实体 TransformNode 节点树信息是否脏
#[derive(Clone, Copy, Component, Default)]
pub struct TransformNodeDirty(pub bool);

/// 标识实体 TransformNode 节点父节点是否更新
#[derive(Clone, Copy, Component, Default)]
pub struct TransformNodeParent;

/// 记录 TransformNode 的局部坐标
#[derive(Clone, Component)]
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
        AssetCapacity { flag: false, min: 5 * 1024 * 1024, max: 1, timeout: 1 * 60 * 1000 }
    }
}
impl TAnimatableComp for LocalPosition {

}

/// 记录 TransformNode 的局部欧拉角
#[derive(Clone, Component)]
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
        AssetCapacity { flag: false, min: 5 * 1024 * 1024, max: 1, timeout: 1 * 60 * 1000 }
    }
}
impl TAnimatableComp for LocalEulerAngles {

}

/// 记录 TransformNode 的局部四元数
#[derive(Clone, Component)]
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
        AssetCapacity { flag: false, min: 5 * 1024 * 1024, max: 1, timeout: 1 * 60 * 1000 }
    }
}
impl TAnimatableComp for LocalRotationQuaternion {

}

/// 标识 TransformNode 的局部旋转是否由四元数影响
#[derive(Clone, Component, Default)]
pub struct LocalRoationWithQuaternion(pub bool);

/// 记录 TransformNode 的局部旋转
#[derive(Clone, Component, Default)]
pub struct LocalRotation(pub Rotation3);

/// 记录 TransformNode 的局部缩放
#[derive(Clone, Component)]
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
        AssetCapacity { flag: false, min: 5 * 1024 * 1024, max: 1, timeout: 1 * 60 * 1000 }
    }
}
impl TAnimatableComp for LocalScaling {

}

/// 标识 TransformNode 的局部矩阵是否脏
#[derive(Component, Default)]
pub struct FlagLocalMatrix;

/// 记录 TransformNode 的局部矩阵
#[derive(Clone, Component)]
pub struct LocalMatrix(pub Matrix);
impl LocalMatrix {
    pub fn new(m: Matrix) -> Self {
        Self(m)
    }
}
impl Default for LocalMatrix {
    fn default() -> Self {
        Self::new(Matrix::identity())
    }
}
impl pi_curves::curve::frame::FrameDataValue for LocalMatrix {
    fn interpolate(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self::new(self.0.scale(1.0 - amount) + rhs.0.scale(amount))
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
        return Self::new(result);
    }


    fn append(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self::new(self.0 + rhs.0.scale(amount))
    }
    fn size() -> usize {
        16 * 4
    }
}

/// 记录 TransformNode 的全局矩阵
#[derive(Clone, Component)]
pub struct GlobalMatrix {
    pub matrix: Matrix,
    pub matrix_inv: Matrix,
}
impl Default for GlobalMatrix {
    fn default() -> Self {
        Self {
            matrix: Matrix::identity(),
            matrix_inv: Matrix::identity(),
        }
    }
}
impl GlobalMatrix {
    pub fn matrix(&self) -> &Matrix {
        &self.matrix
    }
    pub fn position(&self) -> Vector3 {
        Vector3::from(self.matrix.fixed_view::<3, 1>(0, 3))
    }
    pub fn to_position(&self, vec: &mut Vector3) {
        vec.copy_from_slice(&self.matrix.as_slice()[12..15]);
    }
    pub fn xyz(&self) -> (Number, Number, Number) {
        let m = self.matrix.as_slice();
        (m[12], m[13], m[14])
    }
    pub fn calc(&mut self, p_m: &Matrix, l_matrix: &LocalMatrix) -> bool {
        let mut flag: bool = true;
        // result.matrix.copy_from(&(p_m * l_matrix.0));
        CoordinateSytem3::mul_to(p_m, &l_matrix.0, &mut self.matrix);
        // p_m.mul_to(&l_matrix.0, &mut result.matrix);

        if self.matrix.as_slice()[0].is_finite() {
            self.matrix_inv.clone_from(&self.matrix);
            flag = CoordinateSytem3::try_inverse_mut(&mut self.matrix_inv);
            // match result.matrix.try_inverse() {
            //     Some(val) => {
            //         result.matrix_inv = val;
            //     }
            //     None => {
            //         flag = false;
            //         result.matrix = Matrix::identity();
            //         result.matrix_inv = Matrix::identity(); 
            //     }
            // }
        } else {
            flag = false;
            self.matrix = Matrix::identity();
            self.matrix_inv = Matrix::identity();
        }

        flag
    }
}

/// 记录 TransformNode 的全局节点信息
#[derive(Component)]
pub struct AbsoluteTransform {
    scaling: Vector3,
    // rotation: Rotation3,
    quaternion: Quaternion,
    _needupdate: bool,
}
impl Default for AbsoluteTransform {
    fn default() -> Self {
        Self {
            scaling: Vector3::new(1., 1., 1.),
            // rotation: Rotation3::identity(),
            quaternion: Quaternion::identity(),
            _needupdate: true,
        }
    }
}
impl AbsoluteTransform {
    pub fn reset_while_world_matrix_update(&mut self) {
        // self.scaling = None;
        // self.rotation = None;
        // self.quaternion = None;
        self._needupdate = true;
    }
    // pub fn euler_angles(&mut self, gm: &Matrix) -> (Number, Number, Number) {
    //     self.decompose(gm);
    //     self.rotation.euler_angles()
    // }
    pub fn rotation_quaternion(&mut self, gm: &Matrix, tmpscl: &mut Vector3, tmprot: &mut Rotation3) -> &Quaternion {
        self.decompose(gm, tmpscl, tmprot);
        &self.quaternion
    }
    // pub fn rotation(&mut self, gm: &Matrix) -> &Rotation3 {
    //     self.decompose(gm);
    //     &self.rotation
    // }
    pub fn scaling(&mut self, gm: &Matrix, tmpscl: &mut Vector3, tmprot: &mut Rotation3) -> &Vector3 {
        self.decompose(gm,  tmpscl, tmprot);
        &self.scaling
    }
    pub fn iso(&mut self, gm: &Matrix, tmpscl: &mut Vector3, tmprot: &mut Rotation3) -> Isometry3 {
        self.decompose(gm, tmpscl, tmprot);
        let temp = Vector3::from(gm.fixed_view::<3, 1>(0, 3));
        Isometry3::from_parts(Translation3::new(temp[0], temp[1], temp[2]), self.quaternion.clone())
    }
    fn decompose(&mut self, gm: &Matrix, tmpscl: &mut Vector3, tmprot: &mut Rotation3) {
        if self._needupdate {
            // let mut g_p = Vector3::new(0., 0., 0.);
            // let mut g_s = Vector3::new(1., 1., 1.);
            CoordinateSytem3::matrix4_decompose_rotation(&gm, Some(tmpscl), Some(tmprot), None);
            self.quaternion = Quaternion::from_rotation_matrix(&tmprot);
            // self.rotation = g_r;
            self.scaling.copy_from(&tmpscl);
            self._needupdate = true;
        }
    }
}

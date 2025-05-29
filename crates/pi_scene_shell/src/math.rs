use crossbeam::queue::ArrayQueue;
use pi_scene_math::{Vector3, Matrix, Rotation3, coordiante_system::CoordinateSytem3, vector::{TToolMatrix, TToolRotation, TToolVector3}, Number, Isometry3, Quaternion, SQuaternion};
use pi_share::Share;
use pi_world_macros::Resource;
use simba::simd::{SimdBool, SimdComplexField, SimdPartialOrd, SimdRealField};

use crate::shader::ERenderAlignment;

#[derive(Resource)]
pub struct ResMatrixPool(Share<ArrayQueue<Matrix>>);
impl ResMatrixPool {
    pub fn create(count: usize) -> Self {
        Self(Share::new(ArrayQueue::new(count)))
    }
    pub fn identity(&self) -> RecyclableMatrix {
        if let Some(item) = self.0.pop() {
            RecyclableMatrix(self.0.clone(), item)
        } else {
            RecyclableMatrix(self.0.clone(), Matrix::identity())
        }
    }
}
pub struct RecyclableMatrix(Share<ArrayQueue<Matrix>>, Matrix);
impl AsRef<Matrix> for RecyclableMatrix {
    fn as_ref(&self) -> &Matrix {
        &self.1
    }
}
impl AsMut<Matrix> for RecyclableMatrix {
    fn as_mut(&mut self) -> &mut Matrix {
        &mut self.1
    }
}
impl Drop for RecyclableMatrix {
    fn drop(&mut self) {
        self.1.fill_with_identity();
        let _ = self.0.push(self.1);
    }
}

#[derive(Resource)]
pub struct ResVector3Pool(Share<ArrayQueue<Vector3>>);
impl ResVector3Pool {
    pub fn create(count: usize) -> Self {
        Self(Share::new(ArrayQueue::new(count)))
    }
    pub fn zeros(&self) -> RecyclableVector3 {
        if let Some(mut item) = self.0.pop() {
            item.fill(0.);
            RecyclableVector3(self.0.clone(), item)
        } else {
            RecyclableVector3(self.0.clone(), Vector3::zeros())
        }
    }
    pub fn ones(&self) -> RecyclableVector3 {
        if let Some(mut item) = self.0.pop() {
            item.fill(1.);
            RecyclableVector3(self.0.clone(), item)
        } else {
            RecyclableVector3(self.0.clone(), Vector3::zeros())
        }
    }
}
pub struct RecyclableVector3(Share<ArrayQueue<Vector3>>, Vector3);
impl AsRef<Vector3> for RecyclableVector3 {
    fn as_ref(&self) -> &Vector3 {
        &self.1
    }
}
impl AsMut<Vector3> for RecyclableVector3 {
    fn as_mut(&mut self) -> &mut Vector3 {
        &mut self.1
    }
}
impl Drop for RecyclableVector3 {
    fn drop(&mut self) {
        let _ = self.0.push(self.1);
    }
}

pub trait TRenderAlignmentCalc {
    fn calc_rotation(&self, g_rotation: &SQuaternion<Number>, g_velocity: &Vector3, result: &mut SQuaternion<Number>) -> bool;
    fn calc_local(&self, g_velocity: &Vector3, length_scale: Number, length_modify: Number, temp: &mut Matrix, temp2: &mut Matrix, result: &mut Matrix) -> bool;
    fn calc_matrix(&self, g_positon: &Vector3, g_scale: &Vector3, g_rotation: &Quaternion, g_velocity: &Vector3, l_positon: &Vector3, l_scale: &Vector3, l_rotation: &Quaternion, l_euler: &Vector3, refwmatrix: & mut Matrix, reflmatrix: & mut Matrix, result: & mut Matrix);
}
impl TRenderAlignmentCalc for ERenderAlignment {
    #[inline(always)]
    fn calc_rotation(&self, g_rotation: &SQuaternion<Number>, g_velocity: &Vector3, result: &mut SQuaternion<Number>) -> bool {
        // let mut m = Rotation3::identity();
        match self {
            ERenderAlignment::View => {
                // let (_, _, z) =  g_rotation_euler;
                // m = CoordinateSytem3::rotation_matrix_from_euler_angles(0., 0., z);
                false
            },
            ERenderAlignment::World => {
                // m = Rotation3::identity();
                false
            },
            ERenderAlignment::Local => {
                // m = g_rotation.clone();
                result.clone_from(g_rotation);
                true
            },
            ERenderAlignment::Facing => {
                // let (_, _, z) =  g_rotation_euler;
                // m = CoordinateSytem3::rotation_matrix_from_euler_angles(0., 0., z);
                false
            },
            ERenderAlignment::Velocity => {
                let vlen = CoordinateSytem3::length(g_velocity);
                let z_axis = if vlen > f32::EPSILON {
                    // log::warn!("Vel A");
                    g_velocity.scale(1.0 / vlen)
                } else {
                    // log::warn!("Vel B");
                    Vector3::new(0., 0., 1.)
                };
                quaternion_from_unit_vector(&Vector3::z_axis(), &z_axis, result);

                // let mut y_axis = Vector3::new(0., 1., 0.);
                // let mut x_axis = y_axis.cross(&z_axis);
                // if CoordinateSytem3::length(&x_axis) > f32::EPSILON {
                //     x_axis.normalize_mut();
                //     y_axis = z_axis.cross(&x_axis);
                // } else {
                //     y_axis = Vector3::new(1., 0., 0.);
                //     x_axis = y_axis.cross(&z_axis);
                // }
                // m = CoordinateSytem3::rotation_matrix_from_axises(&x_axis, &y_axis, &z_axis);
                true
            },
            ERenderAlignment::StretchedBillboard => {
                let vlen = CoordinateSytem3::length(g_velocity);
                let x_axis = if vlen > f32::EPSILON {
                    g_velocity.scale(-1.0 / vlen)
                } else {
                    Vector3::new(1., 0., 0.)
                };
                quaternion_from_unit_vector(&Vector3::x_axis(), &x_axis, result);

                // let mut y_axis = Vector3::new(0., 1., 0.);
                // let mut z_axis = x_axis.cross(&y_axis);
                // if CoordinateSytem3::length(&z_axis) > f32::EPSILON {
                //     z_axis.normalize_mut();
                //     y_axis = z_axis.cross(&x_axis);
                // } else {
                //     y_axis = Vector3::new(0., 0., 1.);
                //     z_axis = x_axis.cross(&y_axis);
                // }
                // m = CoordinateSytem3::rotation_matrix_from_axises(&x_axis, &y_axis, &z_axis);
                true
            },
            ERenderAlignment::HorizontalBillboard => {
                // let (_, _, z) =  g_rotation_euler;
                quaternion_from_euler_angles((-90_f32).to_radians(), 0., 0., result);
                true
            },
            ERenderAlignment::VerticalBillboard => {
                // let (_, _, z) =  g_rotation_euler;
                // m = CoordinateSytem3::rotation_matrix_from_euler_angles(0., 0., z);
                false
            },
        }
    }
    #[inline(always)]
    fn calc_local(&self, _g_velocity: &Vector3, length_scale: Number, length_modify: Number, temp: &mut Matrix, temp2: &mut Matrix, result: &mut Matrix) -> bool {
        match self {
            ERenderAlignment::View => false,
            ERenderAlignment::World => false,
            ERenderAlignment::Local => false,
            ERenderAlignment::Facing => false,
            ERenderAlignment::Velocity => false,
            ERenderAlignment::StretchedBillboard => {
                // let mut result = Matrix::identity();
                temp2.fill_with_identity();
                // log::warn!("Velocity: {:?}", _g_velocity);

                // let v = Vector3::new(0., 1., 0.);
                // let _g_velocity = &v;
                let vlen = CoordinateSytem3::length(_g_velocity);
                let x_axis = if vlen > f32::EPSILON {
                    _g_velocity.scale(-1.0 / vlen)
                } else {
                    Vector3::new(1., 0., 0.)
                };
                let d_rotation = CoordinateSytem3::quaternion_from_unit_vector(&Vector3::x_axis(), &x_axis).to_rotation_matrix();
                // result = result * &d_rotation.to_homogeneous();
                temp2.fixed_view_mut::<3, 3>(0, 0).copy_from(d_rotation.matrix());

                let vlen = length_scale + length_modify;
                let scaling = Vector3::new(vlen, 1., 1.);
                let translation = Vector3::new(0.5, 0., 0.);
                matrix4_compose_no_rotation(&scaling, &translation, temp);
                CoordinateSytem3::mul_to(&temp2, &temp, result);
                // temp2.mul_to(&temp, result);
                true
            },
            ERenderAlignment::HorizontalBillboard => false,
            ERenderAlignment::VerticalBillboard => false,
        }
    }

    #[inline(always)]
    fn calc_matrix(&self, g_positon: &Vector3, g_scale: &Vector3, g_rotation: &Quaternion, g_velocity: &Vector3, l_positon: &Vector3, l_scale: &Vector3, l_rotation: &Quaternion, l_euler: &Vector3, refwmatrix: & mut Matrix, reflmatrix: & mut Matrix, result: & mut Matrix) {

        match self {
            ERenderAlignment::View => {
                calc_matrix_view(g_positon, g_scale, g_rotation, g_velocity, l_positon, l_scale, l_rotation, l_euler, refwmatrix, reflmatrix, result);
            },
            ERenderAlignment::World => {
                calc_matrix_world(g_positon, g_scale, g_rotation, g_velocity, l_positon, l_scale, l_rotation, l_euler, refwmatrix, reflmatrix, result);
            },
            ERenderAlignment::Local => {
                calc_matrix_local(g_positon, g_scale, g_rotation, g_velocity, l_positon, l_scale, l_rotation, l_euler, refwmatrix, reflmatrix, result);
            },
            ERenderAlignment::Facing => {
                calc_matrix_facing(g_positon, g_scale, g_rotation, g_velocity, l_positon, l_scale, l_rotation, l_euler, refwmatrix, reflmatrix, result);
            },
            ERenderAlignment::Velocity => {
                calc_matrix_velocity(g_positon, g_scale, g_rotation, g_velocity, l_positon, l_scale, l_rotation, l_euler, refwmatrix, reflmatrix, result);
            },
            ERenderAlignment::StretchedBillboard => {
                calc_matrix_strentched(g_positon, g_scale, g_rotation, g_velocity, l_positon, l_scale, l_rotation, l_euler, refwmatrix, reflmatrix, result);
            },
            ERenderAlignment::HorizontalBillboard => {
                calc_matrix_horizontal(g_positon, g_scale, g_rotation, g_velocity, l_positon, l_scale, l_rotation, l_euler, refwmatrix, reflmatrix, result);
            },
            ERenderAlignment::VerticalBillboard => {
                calc_matrix_vertical(g_positon, g_scale, g_rotation, g_velocity, l_positon, l_scale, l_rotation, l_euler, refwmatrix, reflmatrix, result);
            },
        }
    }
}

#[inline(always)]
pub fn calc_matrix_view<'a>(g_positon: &'a Vector3, g_scale: &'a Vector3, _g_rotation: &'a SQuaternion<Number>, _g_velocity: &'a Vector3, l_positon: &'a Vector3, l_scale: &'a Vector3, l_rotation: &'a SQuaternion<Number>, _l_euler: &'a Vector3, refwmatrix: &'a mut Matrix, reflmatrix: &'a mut Matrix, result: &'a mut Matrix) {

    matrix4_compose_no_rotation(g_scale, g_positon, refwmatrix);

    matrix4_compose_quaternion(l_scale, &l_rotation, l_positon, reflmatrix);

    CoordinateSytem3::mul_to(&refwmatrix, &reflmatrix, result);
}
#[inline(always)]
pub fn calc_matrix_world<'a>(g_positon: &'a Vector3, g_scale: &'a Vector3, _g_rotation: &'a SQuaternion<Number>, _g_velocity: &'a Vector3, l_positon: &'a Vector3, l_scale: &'a Vector3, l_rotation: &'a SQuaternion<Number>, _l_euler: &'a Vector3, refwmatrix: &'a mut Matrix, reflmatrix: &'a mut Matrix, result: &'a mut Matrix) {

    matrix4_compose_no_rotation(g_scale, g_positon, refwmatrix);

    matrix4_compose_quaternion(l_scale, &l_rotation, l_positon, reflmatrix);

    CoordinateSytem3::mul_to(&refwmatrix, &reflmatrix, result);
}
#[inline(always)]
pub fn calc_matrix_local<'a>(g_positon: &'a Vector3, g_scale: &'a Vector3, g_rotation: &'a SQuaternion<Number>, _g_velocity: &'a Vector3, l_positon: &'a Vector3, l_scale: &'a Vector3, l_rotation: &'a SQuaternion<Number>, _l_euler: &'a Vector3, refwmatrix: &'a mut Matrix, reflmatrix: &'a mut Matrix, result: &'a mut Matrix) {

    matrix4_compose_quaternion(g_scale, g_rotation, g_positon, refwmatrix);

    matrix4_compose_quaternion(l_scale, l_rotation, l_positon, reflmatrix);

    CoordinateSytem3::mul_to(&refwmatrix, &reflmatrix, result);
}
#[inline(always)]
pub fn calc_matrix_facing<'a>(g_positon: &'a Vector3, g_scale: &'a Vector3, _g_rotation: &'a SQuaternion<Number>, _g_velocity: &'a Vector3, l_positon: &'a Vector3, l_scale: &'a Vector3, l_rotation: &'a SQuaternion<Number>, _l_euler: &'a Vector3, refwmatrix: &'a mut Matrix, reflmatrix: &'a mut Matrix, result: &'a mut Matrix) {

    matrix4_compose_no_rotation(g_scale, g_positon, refwmatrix);

    matrix4_compose_quaternion(l_scale, &l_rotation, l_positon, reflmatrix);

    CoordinateSytem3::mul_to(&refwmatrix, &reflmatrix, result);
}
#[inline(always)]
pub fn calc_matrix_velocity<'a>(g_positon: &'a Vector3, g_scale: &'a Vector3, _g_rotation: &'a SQuaternion<Number>, g_velocity: &'a Vector3, l_positon: &'a Vector3, l_scale: &'a Vector3, l_rotation: &'a SQuaternion<Number>, _l_euler: &'a Vector3, refwmatrix: &'a mut Matrix, reflmatrix: &'a mut Matrix, result: &'a mut Matrix) {

    matrix4_compose_no_rotation(g_scale, g_positon, refwmatrix);

    matrix4_compose_quaternion(l_scale, &l_rotation, l_positon, reflmatrix);

    CoordinateSytem3::mul_to(&refwmatrix, &reflmatrix, result);

    let mut lookat = Isometry3::identity();
    let mut look_target = g_velocity.clone();
    if look_target.magnitude_squared() < 0.000001 {
        // matrix = matrix * &lookat.to_matrix();
    } else {
        let cood = CoordinateSytem3::left();
        CoordinateSytem3::transform_normal_floats(look_target.x, look_target.y, look_target.z, &result, &mut look_target);
        CoordinateSytem3::lookat(&cood, &Vector3::zeros(), g_velocity, &Vector3::new(0., 1., 0.), &mut lookat);
        CoordinateSytem3::mul_to(&result, &lookat.to_matrix(), refwmatrix);
        result.copy_from(&refwmatrix);
    }
}
#[inline(always)]
pub fn calc_matrix_strentched<'a>(g_positon: &'a Vector3, g_scale: &'a Vector3, _g_rotation: &'a SQuaternion<Number>, _g_velocity: &'a Vector3, l_positon: &'a Vector3, l_scale: &'a Vector3, _l_rotation: &'a SQuaternion<Number>, _l_euler: &'a Vector3, refwmatrix: &'a mut Matrix, reflmatrix: &'a mut Matrix, result: &'a mut Matrix) {

    // matrix4_compose_no_rotation(g_scale, g_positon, refwmatrix);
    
    // let vlen = CoordinateSytem3::length(_g_velocity);
    // let mut temp = if vlen > f32::EPSILON {
    //     _g_velocity.scale(-1.0 / vlen)
    // } else {
    //     Vector3::new(-1., 0., 0.)
    // };
    // let mut quat = SQuaternion::<Number>::identity();
    // // quaternion_from_unit_vector(&Vector3::x_axis(), &temp, &mut quat);
    // temp.x = 1.;
    // temp.y = 1.;
    // temp.z = 1.;
    // log::error!("{:?}", (g_positon, l_positon));

    matrix4_compose_quaternion(&g_scale, &_g_rotation, g_positon, result);

    // CoordinateSytem3::mul_to(&refwmatrix, &reflmatrix, result);

    // result.copy_from(&refwmatrix);
}
#[inline(always)]
pub fn calc_matrix_horizontal<'a>(g_positon: &'a Vector3, g_scale: &'a Vector3, _g_rotation: &'a SQuaternion<Number>, _g_velocity: &'a Vector3, l_positon: &'a Vector3, l_scale: &'a Vector3, _l_rotation: &'a SQuaternion<Number>, l_euler: &'a Vector3, refwmatrix: &'a mut Matrix, reflmatrix: &'a mut Matrix, result: &'a mut Matrix) {

    matrix4_compose_no_rotation(g_scale, g_positon, refwmatrix);

    let mut l_rotation = SQuaternion::identity();
    quaternion_from_euler_angles((-90_f32).to_radians(), 0., l_euler.z, &mut l_rotation);
    matrix4_compose_quaternion(l_scale, &l_rotation, l_positon, reflmatrix);

    CoordinateSytem3::mul_to(&refwmatrix, &reflmatrix, result);
}
#[inline(always)]
pub fn calc_matrix_vertical<'a>(g_positon: &'a Vector3, g_scale: &'a Vector3, _g_rotation: &'a SQuaternion<Number>, _g_velocity: &'a Vector3, l_positon: &'a Vector3, l_scale: &'a Vector3, _l_rotation: &'a SQuaternion<Number>, l_euler: &'a Vector3, refwmatrix: &'a mut Matrix, reflmatrix: &'a mut Matrix, result: &'a mut Matrix) {

    matrix4_compose_no_rotation(g_scale, g_positon, refwmatrix);

    let mut l_rotation = SQuaternion::identity();
    quaternion_from_euler_angles(0., l_euler.y, l_euler.z, &mut l_rotation);
    matrix4_compose_quaternion(l_scale, &l_rotation, l_positon, reflmatrix);

    CoordinateSytem3::mul_to(&refwmatrix, &reflmatrix, result);
}
#[inline(always)]
pub fn calc_local_strentched<'a>(_g_velocity: &'a Vector3, length_scale: Number, length_modify: Number) -> Option<Matrix> {
    let mut result = Matrix::identity();
    // log::warn!("Velocity: {:?}", _g_velocity);

    // let v = Vector3::new(0., 1., 0.);
    // let _g_velocity = &v;
    let vlen = CoordinateSytem3::length(_g_velocity);
    let x_axis = if vlen > f32::EPSILON {
        _g_velocity.scale(-1.0 / vlen)
    } else {
        Vector3::new(1., 0., 0.)
    };
    let d_rotation = CoordinateSytem3::quaternion_from_unit_vector(&Vector3::x_axis(), &x_axis).to_rotation_matrix();
    result = result * &d_rotation.to_homogeneous();

    let mut temp = Matrix::identity();
    let vlen = length_scale + length_modify;
    let scaling = Vector3::new(vlen, 1., 1.);
    let translation = Vector3::new(0.5, 0., 0.);
    matrix4_compose_no_rotation(&scaling, &translation, &mut temp);
    // let mut temp = Matrix::identity();
    // temp.append_translation_mut(&translation);
    // Some(result * temp)
    Some(result * temp)
}

/// 
/// strentched 对齐模式的局部矩阵计算
/// 使用全局速度数据,
/// 速度方向为x轴正方向,
/// 速度向量长度为
#[inline(always)]
pub fn calc_local_strentched_call<'a>(_scale: &'a Vector3, _l_velocity: &'a Vector3, length_scale: Number, length_modify: Number, refwmatrix: &'a mut Matrix, reflmatrix: &'a mut Matrix, result: &'a mut Matrix) {
    result.fill_with_identity();
    refwmatrix.fill_with_identity();
    reflmatrix.fill_with_identity();

    let vlen = CoordinateSytem3::length(_l_velocity);
    let dlen = length_scale + length_modify * vlen;

    let mut temp = if vlen > f32::EPSILON {
        _l_velocity.scale(-1.0 / vlen)
    } else {
        Vector3::new(-1., 0., 0.)
    };
    let mut quat = SQuaternion::<Number>::identity();
    quaternion_from_unit_vector(&Vector3::x_axis(), &temp, &mut quat);
    temp.x = 1.; temp.y = 1.; temp.z = 1.;
    matrix4_compose_quaternion(&temp, &quat, &Vector3::zeros(), result);
    // log::error!("{:?}", (dlen, &_scale));

    // // 通过 Speed Scale 与 Length Scale 计算沿X轴的缩放
    let scaling = Vector3::new(dlen * _scale.y, 1. * _scale.x, 1. * _scale.z);
    // // 局部坐标系中向x正方向移动半个单位,使面片左侧对齐坐标系原点
    let translation = Vector3::new(0.5, 0., 0.);
    // // 计算缩放位移操作矩阵
    reflmatrix.append_nonuniform_scaling_mut(&scaling);
    CoordinateSytem3::mul_to(&result, &reflmatrix, refwmatrix);
    
    reflmatrix.fill_with_identity();
    reflmatrix.append_translation_mut(&translation);
    CoordinateSytem3::mul_to(&refwmatrix, &reflmatrix, result);
}

#[inline(always)]
pub fn matrix4_compose_rotation(scaling: &Vector3, rotmat: &Rotation3, translation: &Vector3, result: &mut Matrix) {
    result.fill_with_identity();

    result.fixed_view_mut::<3, 3>(0, 0).copy_from(rotmat.matrix());
    // result.append_nonuniform_scaling_mut(scaling);
    result.prepend_nonuniform_scaling_mut(scaling);

    result.append_translation_mut(translation);
    // CoordinateSytem3::matrix4_compose_rotation(scaling, rotmat, translation, result)
}

#[inline(always)]
pub fn quaternion_from_euler_angles(x: Number, y: Number, z: Number, result: &mut SQuaternion<Number>) {
    quaternion_from_yaw_pitch_roll(y, x, z, result)
}
#[inline(always)]
pub fn quaternion_from_yaw_pitch_roll(yaw: Number, pitch: Number, roll: Number, result: &mut SQuaternion<Number>) {
    // Quaternion::from_rotation_matrix(&Self::rotation_matrix_from_euler_angles(x, y, z))
    let half_roll  = roll * 0.5;
    let half_pitch = pitch * 0.5;
    let half_yaw   = yaw * 0.5;

    let (sin_roll, cos_roll) = half_roll.sin_cos();
    let (sin_pitch, cos_pitch) = half_pitch.sin_cos();
    let (sin_yaw, cos_yaw) = half_yaw.sin_cos();

    result.i = cos_yaw * sin_pitch * cos_roll + sin_yaw * cos_pitch * sin_roll;
    result.j = sin_yaw * cos_pitch * cos_roll - cos_yaw * sin_pitch * sin_roll;
    result.k = cos_yaw * cos_pitch * sin_roll - sin_yaw * sin_pitch * cos_roll;
    result.w = cos_yaw * cos_pitch * cos_roll + sin_yaw * sin_pitch * sin_roll;
}
#[inline(always)]
pub fn matrix4_compose_no_rotation(scaling: &Vector3, translation: &Vector3, result: &mut Matrix) {
    result.fill_with_identity();
    result.append_nonuniform_scaling_mut(scaling);
    result.append_translation_mut(translation);
    // CoordinateSytem3::matrix4_compose_rotation(scaling, rotmat, translation, result)
}

#[inline(always)]
pub fn matrix4_compose_quaternion(scale: &Vector3, rotation: &SQuaternion<Number>, translation: &Vector3, result: &mut Matrix) {
    let x = rotation.i; let y = rotation.j; let z = rotation.k; let w = rotation.w;
    let x2 = x + x; let y2 = y + y; let z2 = z + z;
    let xx = x * x2; let xy = x * y2; let xz = x * z2;
    let yy = y * y2; let yz = y * z2; let zz = z * z2;
    let wx = w * x2; let wy = w * y2; let wz = w * z2;
    let sx = scale.x; let sy = scale.y; let sz = scale.z;
    result[0] = (1. - (yy + zz)) * sx;
    result[1] = (xy + wz) * sx;
    result[2] = (xz - wy) * sx;
    result[3] = 0.;
    result[4] = (xy - wz) * sy;
    result[5] = (1. - (xx + zz)) * sy;
    result[6] = (yz + wx) * sy;
    result[7] = 0.;
    result[8] = (xz + wy) * sz;
    result[9] = (yz - wx) * sz;
    result[10] = (1. - (xx + yy)) * sz;
    result[11] = 0.;
    result[12] = translation.x;
    result[13] = translation.y;
    result[14] = translation.z;
    result[15] = 1.;
}

#[inline(always)]
pub fn quaternion_from_unit_vector(axis: &nalgebra::Unit<Vector3>, vec_to: &Vector3, quat: &mut SQuaternion<Number>) {
    let r = Vector3::dot(axis, vec_to) + 1.0;
    // let quat = 
    if r < f32::EPSILON {
        if f32::abs(axis.x) > f32::abs(axis.z) {
            // nalgebra::Quaternion::new(0., -1.0 * axis.y, axis.x, 0.)
            quat.w = 0.;
            quat.i = -1.0 * axis.y;
            quat.j = axis.x;
            quat.k = 0.;
        } else {
            quat.w = 0.;
            quat.i = 0.;
            quat.j = -1.0 * axis.z;
            quat.k = axis.y;
            // nalgebra::Quaternion::new(0., 0.0, -1.0 * axis.z, axis.y)
        }
    } else {
        let temp = Vector3::cross(axis, vec_to);
        // nalgebra::Quaternion::new(r, temp.x, temp.y, temp.z)
        quat.w = r;
        quat.i = temp.x;
        quat.j = temp.y;
        quat.k = temp.z;
    };
    // quat
}
#[inline(always)]
pub fn calc_local_other<'a>(_g_velocity: &'a Vector3, _length_scale: Number, _length_modify: Number) -> Option<Matrix> {
    None
}
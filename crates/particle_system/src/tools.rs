use pi_scene_shell::prelude::*;
use pi_scene_context::prelude::EScalingMode;
use pi_scene_math::*;
use pi_wy_rng::WyRng;
use rand::{Rng, SeedableRng};

// const RADIANS_ITER_COUNT: usize = 360;
// const RADIANS_ITER_E: f32 = std::f32::consts::PI * 2. / 360.;

pub type TBurstData = [f32; 4];

#[derive(Clone, Copy, PartialEq, Default)]
pub enum EMeshParticleSpaceMode {
    #[default]
    Local = 0,
    /**
     * 发射在世界空间时, 父级尽量不要有旋转动画, 因为 动画 与 粒子动画的衔接有误差，无法完美适配
     */
    World = 1,
}
impl EMeshParticleSpaceMode {
    pub fn mode(&self) -> EParticleSimulationSpace {
        match self {
            EMeshParticleSpaceMode::Local => EParticleSimulationSpace::Local,
            EMeshParticleSpaceMode::World => EParticleSimulationSpace::World,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Default)]
pub enum EMeshParticleScaleMode {
    #[default]
    Hierarchy = 0,
    Local = 1,
    Shape = 2,
}
impl EMeshParticleScaleMode {
    pub fn mode(&self) -> EScalingMode {
        match self {
            EMeshParticleScaleMode::Hierarchy => EScalingMode::Hierarchy,
            EMeshParticleScaleMode::Local => EScalingMode::Local,
            EMeshParticleScaleMode::Shape => EScalingMode::Shape,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Default)]
pub enum EParticleRenderAlignment {
    /**
     * 粒子面向相机平面。
     */
    #[default]
    View = 0,
    /**
     * 粒子与世界轴对齐
     */
    World = 1,
    /**
     * Local 无需额外适配
     */
    Local = 2,
    /**
     * 粒子面向相机的位置点
     */
    Facing = 3,
    /**
     * 粒子朝向它们的速度方向。
     */
    Velocity = 4,
}

#[derive(Clone, Copy, Default, PartialEq)]
pub enum EParticleRenderMode {
    #[default]
    Billboard = 0,
    StretchedBillboard = 1,
    HorizontalBillboard = 2,
    VerticalBillboard = 3,
    Mesh = 4,
    None = 5,
}

pub struct AgeLifeTime {
    pub(crate) age: u32,
    pub(crate) lifetime: u32,
    pub(crate) progress: f32,
}
impl Default for AgeLifeTime {
    fn default() -> Self {
        Self { age: u32::MAX, lifetime: 0, progress: 0. }
    }
}

/// 发射器的矩阵信息
pub struct EmitMatrix {
    pub(crate) matrix_invert: Matrix,
    pub(crate) matrix: Matrix,
    pub(crate) scaling: Vector3,
    // pub(crate) position: Vector3,
    pub(crate) rotation: Rotation3,
}
impl Default for EmitMatrix {
    fn default() -> Self {
        Self {
            matrix_invert: Matrix::identity(),
            matrix: Matrix::identity(),
            scaling: Vector3::new(1., 1., 1.),
            // position: Vector3::new(0., 0., 0.),
            rotation: Rotation3::identity(),
        }
    }
}

pub struct Velocity {
    pub(crate) value: Vector3,
    pub(crate) delta: Vector3,
}
impl Default for Velocity {
    fn default() -> Self {
        Self {
            value: Vector3::zeros(),
            delta: Vector3::zeros(),
        }
    }
}

pub struct OrbitVelocity {
    /// 角速度
    pub(crate) orbit: Vector3,
    /// 角速度
    pub(crate) orbit_len: f32,
    /// 中心偏移
    pub(crate) offset: Vector3,
    /// 径向速度量
    pub(crate) radial: Number,
}
impl Default for OrbitVelocity {
    fn default() -> Self {
        Self {
            orbit: Vector3::zeros(),
            offset: Vector3::zeros(),
            radial: 0.,
            orbit_len: 0.,
        }
    }
}

pub struct SpeedFactor {
    pub(crate) value: Number,
}
impl Default for SpeedFactor {
    fn default() -> Self {
        Self {
            value: 1.,
        }
    }
}

pub struct Direction {
    pub(crate) value: Vector3,
    pub(crate) length: Number,
    pub(crate) velocity_force: Vector3,
    pub(crate) velocity_start: Vector3,
}
impl Default for Direction {
    fn default() -> Self {
        Self {
            value: Vector3::zeros(),
            length: 0.,
            velocity_force: Vector3::zeros(),
            velocity_start: Vector3::zeros(),
        }
    }
}

pub struct Force {
    pub(crate) value: Vector3,
}
impl Default for Force {
    fn default() -> Self {
        Self {
            value: Vector3::zeros(),
        }
    }
}

pub struct GravityFactor {
    pub(crate) value: Vector3,
}
impl Default for GravityFactor {
    fn default() -> Self {
        Self {
            value: Vector3::zeros(),
        }
    }
}

/// 在局部空间影响
pub struct LimitVelocityScalar {
    pub(crate) value: Number,
    pub(crate) dampen: Number,
}
impl Default for LimitVelocityScalar {
    fn default() -> Self {
        Self {
            value: f32::MAX,
            dampen: 0.
        }
    }
}

pub struct TextureUV {
    pub(crate) start_frame: f32,
    pub(crate) row: f32,
    pub(crate) uscale: f32,
    pub(crate) vscale: f32,
    pub(crate) uoffset: f32,
    pub(crate) voffset: f32,
}
impl Default for TextureUV {
    fn default() -> Self {
        Self {
            start_frame: 0.,
            row: 0.,
            uscale: 1.,
            vscale: 1.,
            uoffset: 0.,
            voffset: 0.,
        }
    }
}

pub fn rotation_yaw_pitch_roll_to_ref(yaw: f32, pitch: f32, roll: f32, result: &mut Vector4) {
    // Produces a quaternion from Euler angles in the z-y-x orientation (Tait-Bryan angles)
    let half_roll = roll * 0.5;
    let half_pitch = pitch * 0.5;
    let half_yaw = yaw * 0.5;

    let sin_roll = f32::sin(half_roll);
    let cos_roll = f32::cos(half_roll);
    let sin_pitch = f32::sin(half_pitch);
    let cos_pitch = f32::cos(half_pitch);
    let sin_yaw = f32::sin(half_yaw);
    let cos_yaw = f32::cos(half_yaw);

    result[0] = (cos_yaw * sin_pitch * cos_roll) + (sin_yaw * cos_pitch * sin_roll);
    result[1] = (sin_yaw * cos_pitch * cos_roll) - (cos_yaw * sin_pitch * sin_roll);
    result[2] = (cos_yaw * cos_pitch * sin_roll) - (sin_yaw * sin_pitch * cos_roll);
    result[3] = (cos_yaw * cos_pitch * cos_roll) + (sin_yaw * sin_pitch * sin_roll);
}

pub fn direction_to_quaternion(direction: Vector3, result: &mut Vector4) {
    let x_axis = direction[0];
    let y_axis = direction[1];
    let z_axis = direction[2];

    let yaw = -f32::atan2(z_axis, x_axis) + std::f32::consts::PI / 2.;
    let len = f32::sqrt(x_axis * x_axis + z_axis * z_axis);
    let pitch = -f32::atan2(y_axis, len);

    rotation_yaw_pitch_roll_to_ref(yaw, pitch, 0., result);
}


// 适配xyz都为0的情况
pub fn normalize(v: &Vector3) -> Vector3 {
    if v[0] != 0.0 || v[1] != 0.0 || v[2] != 0.0 {
        return v.normalize();
    }
    Vector3::new(0.0, 0.0, 0.0)
}

pub fn multiply(v1: &Vector3, v2: &Vector3) -> Vector3 {
    Vector3::new(v1.x * v2.x, v1.y * v2.y, v1.z * v2.z)
}

#[derive(Component, Default)]
pub struct Random(pub(crate) WyRng);
impl Random {
    pub fn new(seed: u64) -> Self {
        Self(WyRng::seed_from_u64(seed))
    }
    pub fn random(&mut self) -> f32 {
        self.0.gen_range(0.0..1.0)
    }
    pub fn random_range(&mut self, start: f32, end: f32) -> f32 {
        self.0.gen_range(start..end)
    }
}
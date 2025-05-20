use std::ops::Sub;

use crossbeam::queue::SegQueue;
use pi_scene_shell::prelude::*;
use pi_scene_context::prelude::*;
use pi_scene_math::{*, coordiante_system::CoordinateSytem3, vector::{TToolVector3, TToolRotation, TToolMatrix}};
use pi_trail_renderer::{TrailPoints, TrailBase, TrailBuffer};
use rand::Rng;

use crate::{
    modifier::*, prelude::TypeShapeEmitter, tools::*
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet, PartialOrd, Ord)]
pub enum StageParticleSystem {
    ParticleSysCreate,
    _ParticleSysCreate,
    ParticleSysCommand,
    ParticleSysParamStart,
    ParticleSysEmission,
    ParticleSysCalc,
    ParticleSysMatrix,
    ParticleSysUpdate,
    ParticleSysDispose,
}

pub type IdxParticle = usize;

// pub type TCurveTime = Number;
pub type TCurveValue = Number;
pub type TCurveInTangent = Number;
pub type TCurveOutTangent = Number;
pub type TCurveScalar = Number;

pub const PARTICLE_MIN_VALUE: Number = 0.00000001;

pub enum TCurveMode {
    /**
     * 静态数值
     */
    Constant,
    /**
     * 静态数值随机 - XYZ 随机值相同
     */
    TwoConstants,
    /**
     * 曲线插值
     */
    Curve,
    /**
     * 曲线插值
     */
    TwoCurves,
    Random,
}

pub enum TGradienMode {
    /**
     * 静态数值
     */
    Color,
    /**
     * 静态数值随机 - XYZ 随机值相同
     */
    TwoColors,
    /**
     * 曲线插值
     */
    Gradient,
    /**
     * 曲线插值
     */
    TwoGradients,
    Random,
}

fn _idx_constant(_idx: usize) -> usize {
    0
}
fn _idx(idx: usize) -> usize {
    idx
}

#[derive(Resource)]
pub struct ParticleSystemInstant(pub pi_time::Instant);

#[derive(Resource)]
pub struct ParticleSystemPerformance {
    pub debug: bool,
    pub time: pi_time::Instant,
    pub sys_ids: u32,
    pub sys_prewarm: u32,
    pub sys_emission: u32,
    pub sys_start: u32,
    pub sys_over_life_time: u32,
    pub sys_direction: u32,
    pub sys_by_speed: u32,
    pub sys_emitmatrix: u32,
    pub sys_update_buffer: u32,
    pub sys_update_buffer_trail: u32,
    pub particles: u32,
    pub maxparticles: u32,
    /// 运行的帧间隔控制
    pub frame_time_ms: u32,
    /// 更新的帧间隔控制
    pub update_frame_time_ms: u32,
    /// 上一次运行的帧时间
    pub last_running_time: u64,
    /// 是否更新数据
    pub update_buffer: bool,
}
impl Default for ParticleSystemPerformance {
    fn default() -> Self {
        Self {
            debug: false,
            time: pi_time::Instant::now(),
            sys_ids: 0,
            sys_prewarm: 0,
            sys_emission: 0,
            sys_start: 0,
            sys_over_life_time: 0,
            sys_direction: 0,
            sys_by_speed: 0,
            sys_emitmatrix: 0,
            sys_update_buffer: 0,
            sys_update_buffer_trail: 0,
            particles: 0,
            maxparticles: 0,
            frame_time_ms: 0,
            update_frame_time_ms: 0,
            last_running_time: 0,
            update_buffer: false,
        }
    }
}
impl ParticleSystemPerformance {
    pub fn total(&self) -> u32 {
        self.sys_ids
        + self.sys_emission
        + self.sys_start
        + self.sys_over_life_time
        + self.sys_direction
        + self.sys_by_speed
        + self.sys_emitmatrix
        + self.sys_update_buffer
        + self.sys_update_buffer_trail
    }
}

// pub struct ICurveKey(TCurveTime, TCurveValue, TCurveInTangent, TCurveOutTangent, TCurveMode);

// pub struct ICurve(Vec<ICurveKey>, TCurveScalar);

#[derive(Resource, Default)]
pub struct ResParticleCalculatorUninstallQueue(pub(crate) Share<SegQueue<Entity>>);
impl ResParticleCalculatorUninstallQueue {
    pub fn queue(&self) -> Share<SegQueue<Entity>> {
        self.0.clone()
    }
}

pub type KeyParticleSystemCalculator = u64;

pub struct ParticleSystemCalculatorID(pub Entity, pub usize, pub Share<SegQueue<Entity>>);
impl std::fmt::Debug for ParticleSystemCalculatorID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl Drop for ParticleSystemCalculatorID {
    fn drop(&mut self) {
        self.2.push(self.0)
    }
}
impl Asset for ParticleSystemCalculatorID {
    type Key = KeyParticleSystemCalculator;
    // const TYPE: &'static str = "ParticleSystemCalculatorID";
}
impl Size for ParticleSystemCalculatorID {
    fn size(&self) -> usize {
        self.1
    }
}
impl TAssetCapacity for ParticleSystemCalculatorID {
    fn capacity() -> AssetCapacity {
        AssetCapacity { flag: false, min: 100 * 1024, max: 1, timeout: 10 * 1000 }
    }

    const ASSET_TYPE: &'static str = "ParticleSystemCalculator";
}

#[derive(Clone, Copy)]
pub enum EParticleAttributeType {
    Matrix,
    Color,
    Tilloff,
    Extend4A,
    Extend4B,
}

pub struct ParticleAttribute {
    pub vtype: EParticleAttributeType,
    pub attr: Atom,
}

#[derive(Component, Default)]
pub struct ParticleAttributes(pub Vec<ParticleAttribute>);

#[derive(Component, Default)]
pub struct ParticleCalculatorStartModifiers {
    pub(crate) emission: ParticleCalculatorEmission,
    pub(crate) shapeemitter: ParticleCalculatorShapeEmitter,
    pub(crate) startlifetime: ParticleCalculatorStartLifetime,
    pub(crate) startcolor: ParticleCalculatorStartColor,
    pub(crate) startspeed: ParticleCalculatorStartSpeed,
    pub(crate) startsize: ParticleCalculatorStartSize,
    pub(crate) gravity: ParticleCalculatorGravity,
    pub(crate) startrotation: ParticleCalculatorStartRotation,
}

#[derive(Default)]
pub struct ParticleCalculatorEmission {
    pub(crate) bursts: Vec<TBurstData>,
    pub(crate) rateovertime: FloatInterpolation,
}
#[derive(Default)]
pub struct ParticleCalculatorShapeEmitter(pub(crate) TypeShapeEmitter);
#[derive(Default)]
pub struct ParticleCalculatorStartLifetime(pub(crate) FloatInterpolation);
#[derive(Default)]
pub struct ParticleCalculatorStartColor(pub(crate) StartColor);
#[derive(Default)]
pub struct ParticleCalculatorStartSpeed(pub(crate) FloatInterpolation);
#[derive(Default)]
pub struct ParticleCalculatorStartSize(pub(crate) StartSize);
#[derive(Default)]
pub struct ParticleCalculatorGravity(pub(crate) Gravity, pub Vector3);
#[derive(Default)]
pub struct ParticleCalculatorStartRotation(pub(crate) StartRotation);
#[derive(Default)]

pub struct ParticleCalculatorForceOverLifetime(pub(crate) ForceOverLifetime);
#[derive(Default)]
pub struct ParticleCalculatorOrbitRadial(pub(crate) FloatInterpolation);
#[derive(Default)]
pub struct ParticleCalculatorOrbitOffset(pub(crate) TranslationInterpolate);
#[derive(Default)]
pub struct ParticleCalculatorOrbitVelocity(pub(crate) TranslationInterpolate);

#[derive(Component, Default)]
pub struct ParticleCalculatorOverLifetime {
    pub(crate) orbitoffset: ParticleCalculatorOrbitOffset,
    pub(crate) orbitvelocity: ParticleCalculatorOrbitVelocity,
    pub(crate) orbitradial: ParticleCalculatorOrbitRadial,
    pub(crate) force: ParticleCalculatorForceOverLifetime,
    pub(crate) size: Option<ParticleCalculatorSizeOverLifetime>,
    pub(crate) velocity: Option<ParticleCalculatorVelocityOverLifetime>,
    pub(crate) speed: Option<ParticleCalculatorSpeedModifier>,
    pub(crate) rotation: Option<ParticleCalculatorRotationOverLifetime>,
    pub(crate) color: Option<ParticleCalculatorColorOverLifetime>,
    pub(crate) sizebyspeed: Option<ParticleCalculatorSizeBySpeed>,
    pub(crate) colorbyspeed: Option<ParticleCalculatorColorBySpeed>,
    pub(crate) rotationbyspeed: Option<ParticleCalculatorRotationBySpeed>,
    pub(crate) texturesheet: Option<ParticleCalculatorTextureSheet>,
    pub(crate) limitvelocity: Option<ParticleCalculatorLimitVelocityOverLifetime>
}

pub struct ParticleCalculatorVelocityOverLifetime(pub(crate) VelocityOverLifetime);
pub struct ParticleCalculatorLimitVelocityOverLifetime(pub(crate) LimitVelocityOverLifetime);
pub struct ParticleCalculatorSpeedModifier(pub(crate) SpeedModifier);
pub struct ParticleCalculatorSizeOverLifetime(pub(crate) SizeOverLifetime);
pub struct ParticleCalculatorSizeBySpeed(pub(crate) SizeBySpeed);
pub struct ParticleCalculatorRotationOverLifetime(pub(crate) RotationOverLifetime);
pub struct ParticleCalculatorRotationBySpeed(pub(crate) RotationBySpeed);

#[derive(Component, Default)]
pub struct ParticleCalculatorCustomV4(pub Option<_ParticleCalculatorCustomV4>);
pub struct _ParticleCalculatorCustomV4 {
    pub(crate) x: FloatInterpolation,
    pub(crate) y: FloatInterpolation,
    pub(crate) z: FloatInterpolation,
    pub(crate) w: FloatInterpolation,
}

pub struct ParticleCalculatorColorOverLifetime(pub(crate) ColorOverLifetime);
pub struct ParticleCalculatorColorBySpeed(pub(crate) ColorBySpeed);
pub struct ParticleCalculatorTextureSheet(pub(crate) TextureSheet);

#[derive(Component, Default)]
pub struct ParticleCalculatorTrail(pub(crate) Option<TrailModifier>);


#[derive(Component, Default)]
pub struct ParticleCalculatorBase {
    pub(crate) looping: bool,
    pub(crate) prewarm: bool,
    pub(crate) delay: u32,
    pub(crate) scaling_space: EScalingMode,
    pub(crate) simulation_space: EParticleSimulationSpace,
    pub(crate) render_alignment: EParticleRenderAlignment,
    pub(crate) render_mode: EParticleRenderMode,
    pub(crate) duration: u32,
    pub(crate) maxcount: usize,
    pub(crate) stretched_length_scale: f32,
    pub(crate) stretched_velocity_scale: f32,
    pub(crate) pivot: Vector3,
}
impl ParticleCalculatorBase {
    pub fn render_align(&self) -> Option<ERenderAlignment> {
        match self.render_mode {
            EParticleRenderMode::StretchedBillboard => {
                Some(ERenderAlignment::StretchedBillboard)
            },
            EParticleRenderMode::HorizontalBillboard => {
                Some(ERenderAlignment::HorizontalBillboard)
            },
            EParticleRenderMode::VerticalBillboard => {
                Some(ERenderAlignment::VerticalBillboard)
            },
            EParticleRenderMode::None => None,
            _ => {
                match self.render_alignment {
                    EParticleRenderAlignment::View => {
                        Some(ERenderAlignment::View)
                    },
                    EParticleRenderAlignment::World => {
                        Some(ERenderAlignment::World)
                    },
                    EParticleRenderAlignment::Local => {
                        Some(ERenderAlignment::Local)
                    },
                    EParticleRenderAlignment::Facing => {
                        Some(ERenderAlignment::Facing)
                    },
                    EParticleRenderAlignment::Velocity => {
                        Some(ERenderAlignment::Velocity)
                    },
                }
            },
        }
    }
}

#[derive(Resource)]
pub struct ArgParticleCommonBufferSize(pub u32);

#[derive(Resource)]
pub struct ResParticleTrailBuffer(pub Option<TrailBuffer>);
impl TAssetCapacity for ResParticleTrailBuffer {
    const ASSET_TYPE: &'static str = "PARTICLE_TRAIL_BUFFER";
    fn capacity() -> AssetCapacity {
        AssetCapacity { flag: false, min: 1024 * 1024, max: 2, timeout: 1000 }
    }
}

#[derive(Resource)]
pub struct ArgParticleTrailBufferSize(pub u32);

#[derive(Component, Default)]
pub struct ParticleTrailMesh {
    pub mesh: Entity,
    pub geo: Entity,
}
impl ParticleTrailMesh {
    pub fn new(
        trailmesh: Entity,
        trailgeo: Entity
    ) -> Self {
        Self { mesh: trailmesh, geo: trailgeo }
    }
}

#[derive(Component, Default)]
pub struct ParticleTrail {
    pub pathlist: Vec<TrailPoints>,
    pub timelist: Vec<TrailBase>,
    pub lifetime: Vec<u32>,
}
impl ParticleTrail {
    pub fn new(
        maxcout: usize
    ) -> Self {
        let mut pathlist = Vec::with_capacity(maxcout);
        let mut timelist = Vec::with_capacity(maxcout);
        let mut lifetime = Vec::with_capacity(maxcout);
        for _ in 0..maxcout {
            pathlist.push(TrailPoints::default());
            timelist.push(TrailBase::new(0));
            lifetime.push(0);
        }
        Self { pathlist, timelist, lifetime }
    }
    pub fn start(
        &mut self,
        newids: &Vec<IdxParticle>,
        ages: &Vec<AgeLifeTime>,
        diewaittimes: &mut Vec<u32>,
        randomlist: &Vec<BaseRandom>,
        time: &ParticleSystemTime,
        trailmodifier: &TrailModifier,
    ) {
        newids.iter().for_each(|idx| {
            let randoms = randomlist.get(*idx).unwrap();
            let age = ages.get(*idx).unwrap();
            let diewaittime = diewaittimes.get_mut(*idx).unwrap();

            let lifetime = (trailmodifier.lifetime.interpolate(time.loop_progress, randoms.base) * age.lifetime as f32) as u32;

            if trailmodifier.die_with_particle == false {
                *diewaittime = lifetime;
            } else {
                *diewaittime = 0;
            }

            let item = self.lifetime.get_mut(*idx).unwrap();
            *item = lifetime;

            let item = self.timelist.get_mut(*idx).unwrap();
            *item = TrailBase::new(u32::MAX);

            let item = self.pathlist.get_mut(*idx).unwrap();
            item.reset();
        });
    }
    pub fn run_new(
        &mut self,
        newids: &Vec<IdxParticle>,
        randomlist: &Vec<BaseRandom>,
        colors: &Vec<Vector4>,
        localpositions: &Vec<Vector3>,
        localscalings: &Vec<Vector3>,
        localrotations: &Vec<Vector3>,
        worldmatrixs: &ParticleEmitMatrix,
        directions: &Vec<Direction>,
        trailmodifier: &TrailModifier,
    ) {
        let mut color = Vector4::new(1., 1., 1., 1.);
        let mut localscaling = Vector3::new(1., 0., 0.);
        let mut localmatrix = Matrix::identity();
        let trailworldspace = trailmodifier.use_world_space;
        newids.iter().for_each(|idx| {
            let randoms = randomlist.get(*idx).unwrap();
            let particlecolor = colors.get(*idx).unwrap();
            let agecontrol = self.lifetime.get(*idx).unwrap();

            let direction = directions.get(*idx).unwrap();
            let translation = localpositions.get(*idx).unwrap() + direction.value.scale(-1. * PARTICLE_MIN_VALUE / f32::max(direction.length, 1.));
            let scaling = localscalings.get(*idx).unwrap();
            let eulers = localrotations.get(*idx).unwrap();
            CoordinateSytem3::matrix4_compose_euler_angle(scaling, eulers, &translation, &mut localmatrix);

            let parentmatrix = &worldmatrixs.get(*idx).unwrap().matrix;
            // let worldmatrix = &worldmatrixs.get(*idx).unwrap().pose;

            let age = self.timelist.get_mut(*idx).unwrap();

            let item = self.pathlist.get_mut(*idx).unwrap();

            if trailmodifier.inherit_particle_color {
                color.copy_from(particlecolor);
            } else {
                color.copy_from_slice(&[1., 1., 1., 1.]);
            }

            let width: f32 = if trailmodifier.size_affects_width {
                1.
            } else {
                CoordinateSytem3::transform_normal_floats(1., 0., 0., &localmatrix, &mut localscaling);
                let len = CoordinateSytem3::length(&localscaling);
                if len < PARTICLE_MIN_VALUE { 0. } else { 1. / len }
            };

            // log::warn!("Trail: {:?}, {:?}", age, trailworldspace);
            item.run(
                parentmatrix, &localmatrix, 
                &color, &trailmodifier.color_over_lifetime.color4_interpolate.gradient, &trailmodifier.color_over_trail.color4_interpolate.gradient,
                width, &trailmodifier.width_over_trail, *agecontrol, &age, randoms, 9999999., trailmodifier.minimun_vertex_distance,
                trailworldspace
            );
        });
    }
    pub fn run(
        &mut self,
        activeids: &Vec<IdxParticle>,
        randomlist: &Vec<BaseRandom>,
        colors: &Vec<Vector4>,
        localpositions: &Vec<Vector3>,
        localscalings: &Vec<Vector3>,
        localrotations: &Vec<Vector3>,
        worldmatrixs: &ParticleEmitMatrix,
        time: &ParticleSystemTime,
        trailmodifier: &TrailModifier,
    ) {
        let mut color = Vector4::new(1., 1., 1., 1.);
        let basesize = Vector3::new(0.5773502691896257 as f32, 0.5773502691896257 as f32, 0.5773502691896257 as f32);
        let mut localscaling = Vector3::new(1., 0., 0.);
        let mut localmatrix = Matrix::identity();
        activeids.iter().for_each(|idx| {
            let randoms = randomlist.get(*idx).unwrap();
            let particlecolor = colors.get(*idx).unwrap();
            let agecontrol = self.lifetime.get(*idx).unwrap();

            let translation = localpositions.get(*idx).unwrap();
            let scaling = localscalings.get(*idx).unwrap();
            let eulers = localrotations.get(*idx).unwrap();
            CoordinateSytem3::matrix4_compose_euler_angle(scaling, eulers, &translation, &mut localmatrix);

            let parentmatrix = &worldmatrixs.get(*idx).unwrap().matrix;
            let worldmatrix = parentmatrix * localmatrix;

            let age = self.timelist.get_mut(*idx).unwrap();
            age.update(time.running_delta_ms);

            let item = self.pathlist.get_mut(*idx).unwrap();

            if trailmodifier.inherit_particle_color {
                color.copy_from(particlecolor);
            } else {
                color.copy_from_slice(&[1., 1., 1., 1.]);
            }

            let width: f32 = if trailmodifier.size_affects_width {
                1.
            } else {
                CoordinateSytem3::transform_normal_floats(basesize.x, basesize.y, basesize.z, &localmatrix, &mut localscaling);
                let len = CoordinateSytem3::length(&localscaling);
                if len < PARTICLE_MIN_VALUE { 0. } else { 1. / len }
            };

            // log::warn!("Trail: {:?}, {:?}", age, trailworldspace);
            item.run(
                &worldmatrix, &localmatrix, 
                &color, &trailmodifier.color_over_lifetime.color4_interpolate.gradient, &trailmodifier.color_over_trail.color4_interpolate.gradient,
                width, &trailmodifier.width_over_trail, *agecontrol, &age, randoms, 9999999., trailmodifier.minimun_vertex_distance,
                trailmodifier.use_world_space
            );
        });
    }
}

#[derive(Component, Default)]
pub struct ParticleDieWaitTime(pub Vec<u32>);
impl ParticleDieWaitTime {
    pub fn new(maxcount: usize) -> Self {
        let mut list = Vec::with_capacity(maxcount);
        for _ in 0..maxcount {
            list.push(0);
        }
        Self (list)
    }
    pub fn start(
        &mut self,
        newids: &Vec<IdxParticle>,
        ages: &Vec<AgeLifeTime>,
        randoms: &Vec<BaseRandom>,
        time: &ParticleSystemTime,
        interpolation: Option<&FloatInterpolation>,
    ) {
        newids.iter().for_each(|idx| {
            let item = self.0.get_mut(*idx).unwrap();
    
            if let Some(interpolation) = interpolation {
                let random = randoms.get(*idx).unwrap();
                let age = ages.get(*idx).unwrap();
                *item = (interpolation.interpolate(time.loop_progress, random.base) * age.lifetime as f32) as u32;
            } else {
                *item = 0;
            }
        });
    }
}

#[derive(Component, Default)]
pub struct ParticleSystemActive(pub bool);

#[derive(Component, Default)]
pub struct ParticleSystemRunningState {
    pub isrunning: bool,
    pub deltatime: u64,
    pub update_buffer_interval_frame: u8,
    pub waitframe: u8,
    pub updatebuffer: bool,
}

#[derive(Component, Default)]
pub struct ParticleSystemModifyState;

/// 存活的粒子ID列表
#[derive(Component, Default)]
pub struct ParticleIDs {
    pub(crate) calculator: Option<Handle<ParticleSystemCalculatorID>>,
    /// 存活的粒子ID列表
    pub(crate) actives: Vec<IdxParticle>,
    /// 存活的粒子ID列表
    pub(crate) dies: Vec<IdxParticle>,
    /// 非存活的粒子ID列表
    pub(crate) unactives: Vec<IdxParticle>,
    /// 新创建的粒子ID列表
    pub(crate) newids: Vec<IdxParticle>,
    pub(crate) maxcount: usize,
}
impl ParticleIDs {
    pub fn new(calculator: Handle<ParticleSystemCalculatorID>, maxcount: usize) -> Self {
        let mut unactives = Vec::with_capacity(maxcount);
        for i in 0..maxcount {
            unactives.push(maxcount - 1 - i);
        }
        Self {
            calculator: Some(calculator),
            actives: vec![],
            unactives: unactives,
            newids: vec![],
            dies: vec![],
            maxcount,
        }
    }
    pub fn create_new(&mut self, newcount: usize) {
        let num = newcount.min(self.unactives.len());

        // log::warn!("New: {:?}, {:?}", newcount, num);

        for _ in 0..num {
            let idx = self.unactives.pop().unwrap();
            self.newids.push(idx);
            self.actives.push(idx);
        }
    }
    pub fn clear_new(&mut self) {
        self.newids.clear();
    }
    pub fn reset(&mut self) {
        let maxcount = self.maxcount;
        let mut unactives = Vec::with_capacity(maxcount);
        for i in 0..maxcount {
            unactives.push(maxcount - 1 - i);
        }

        self.actives.clear();
        self.newids.clear();
        self.unactives = unactives;
    }
    pub fn count(&self) -> usize {
        self.actives.len()
    }
}

/// 粒子系统
#[derive(Component, Default)]
pub struct ParticleSystemTime {
    /// 运行速度
    pub(crate) time_scale: f32,
    /// 上一次运行时间点
    pub(crate) last_running_timems: u32,
    /// 运行时的有效间隔时间
    pub(crate) running_delta_ms: u32,
    /// 总时间
    pub(crate) total_ms: u32,
    /// 总运行时间
    pub(crate) total_running_ms: u32,
    /// 计算间隔时间的一半
    pub(crate) half_frame_time_ms: u32,
    /// 延迟时间
    pub(crate) delay_ms: u32,
    /// 发射器循环次数
    pub(crate) emission_loop: u32,
    /// 发射器循环进度
    pub(crate) emission_progress: Number,
    /// 粒子系统循环次数
    pub(crate) loop_count: u32,
    /// 粒子系统当次循环的时间
    pub(crate) loop_progress: Number,
    /// 粒子系统是否进入新一轮循环
    pub(crate) loop_new: bool,
}
impl ParticleSystemTime {
    pub fn new(frame_time_ms: u32) -> Self {
        Self {
            time_scale: 1.,
            last_running_timems: 0,
            running_delta_ms: 0,
            total_ms: 0,
            total_running_ms: 0,
            half_frame_time_ms: frame_time_ms / 2,
            delay_ms: 0,
            emission_loop: 0,
            emission_progress: 0.,
            loop_count: 0,
            loop_progress: 0.,
            loop_new: false,
        }
    }
    ///
    /// * delta_ms 间隔时间
    /// * emission_time 一次发射器循环的时间 固定的 1000 ms
    /// * duration 粒子系统发射持续时间
    pub fn run(&mut self, delta_ms: u32, emission_time: u32, duration: u32) {
        self.total_ms += (delta_ms as f32 * self.time_scale) as u32;

        if self.total_ms < self.delay_ms  {
            self.running_delta_ms = 0;
        } else {
            self.total_running_ms = self.total_ms - self.delay_ms;

            // 初次启动
            if self.last_running_timems == 0 && self.delay_ms <= self.total_ms {
                self.running_delta_ms = self.half_frame_time_ms;
            } else {
                // 间隔时间到达帧运行间隔
                if self.delay_ms + self.last_running_timems + self.half_frame_time_ms < self.total_ms {
                    self.running_delta_ms = self.total_ms - (self.delay_ms + self.last_running_timems);
                } else {
                    self.running_delta_ms = 0;
                    return;
                }
            }
    
            self.last_running_timems = self.total_ms;
            self.emission_loop = self.total_running_ms / emission_time;
            self.emission_progress = (self.total_running_ms - self.emission_loop * emission_time) as Number / (emission_time as Number);
    
            let loop_count = self.total_running_ms / duration;
            if loop_count != self.loop_count {
                self.loop_new = true;
            }
            self.loop_count = loop_count;
            self.loop_progress = (self.total_running_ms - loop_count * duration) as Number / duration as Number;
        }
    }
}

#[derive(Component, Default, Deref, DerefMut)]
pub struct ParticleRandom(pub(crate) Random);
impl ParticleRandom {
    pub fn new(seed: u64) -> Self {
        Self(Random::new(seed))
    }
}

#[derive(Component, Default, Deref)]
pub struct ParticleBaseRandom(pub(crate) Vec<BaseRandom>);
impl ParticleBaseRandom {
    pub fn new(maxcount: usize) -> Self {
        let mut vec = Vec::with_capacity(maxcount);
        for _ in 0..maxcount {
            vec.push(BaseRandom::default())
        }
        Self(vec)
    }
    pub fn run(
        &mut self,
        newids: &Vec<IdxParticle>,
        activeids: &Vec<IdxParticle>,
        random: &mut Random,
    ) {
        newids.iter().for_each(|idx| {
            let item = self.0.get_mut(*idx).unwrap();
            item.base = random.random();
            item.seed = random.0.gen_range(0..u64::MAX);
        });
        activeids.iter().for_each(|idx| {
            let item = self.0.get_mut(*idx).unwrap();
            item.x = random.random();
            item.y = random.random();
            item.z = random.random();
            item.w = random.random();
        });
    }
}

/// 新生粒子ID列表
#[derive(Component, Default)]
pub struct ParticleSystemEmission {
    /// 距离上次发射的时间
    pub(crate) last_rate_time: u32,
    pub(crate) burst_loop_count_record: Vec<usize>,
}
impl ParticleSystemEmission {
    pub fn new() -> Self {
        Self { last_rate_time: 0, burst_loop_count_record: vec![] }
    }
    /// rate_over_time 每秒发射粒子数目
    pub fn start(
        &mut self,
        isloop: bool,
        duration: u32,
        time: &ParticleSystemTime,
        rate_over_time: usize,
        bursts: &Vec<TBurstData>,
        ids: &mut ParticleIDs,
    ) {
        let burstcount = bursts.len();
        let mut newcount = 0;

        let oldcount = self.burst_loop_count_record.len();
        if oldcount < burstcount {
            for _ in oldcount..burstcount {
                self.burst_loop_count_record.push(0);
            }
        }

        if isloop == true || time.total_running_ms <= duration {
            // 新的一轮循环
            if time.loop_new {
                // 剩余 Bursts 全部激活
                let mut idx = 0;
                bursts.iter().for_each(|burst| {
                    let busrt_time = (burst[0] * 1000.) as usize;
                    let busrt_count = burst[1] as usize;
                    let _busrt_loop = burst[2] as usize;
                    let busrt_interval = (burst[3] * 1000.) as usize;
    
                    if busrt_time < duration as usize {
                        let needloopcount = (duration as usize - busrt_time + busrt_interval - 1) / busrt_interval;
                        if needloopcount > self.burst_loop_count_record[idx] {
                            newcount += (needloopcount - self.burst_loop_count_record[idx]) * busrt_count;
                        }
                    }
    
                    self.burst_loop_count_record[idx] = 0;
                    idx += 1;
                });
            }
    
            let loop_progress_time = (time.loop_progress * duration as Number) as usize;
            let mut idx = 0;
            bursts.iter().for_each(|burst| {
                let busrt_time = (burst[0] * 1000.) as usize;
                let busrt_count = burst[1] as usize;
                let _busrt_loop = burst[2] as usize;
                let busrt_interval = (burst[3] * 1000.) as usize;
    
                if busrt_time < loop_progress_time as usize {
                    let needloopcount = (loop_progress_time as usize - busrt_time + busrt_interval - 1) / busrt_interval;
                    if needloopcount > self.burst_loop_count_record[idx] {
                        newcount += (needloopcount - self.burst_loop_count_record[idx]) * busrt_count;
                    }
                    self.burst_loop_count_record[idx] = needloopcount;
                }
    
                idx += 1;
            });

            if rate_over_time > 0 {
                self.last_rate_time += time.running_delta_ms;
                let per_rate_time = 1000. / rate_over_time as Number;
                let count = ((self.last_rate_time as Number) / per_rate_time).floor() as usize;
                if count > 0 {
                    self.last_rate_time = self.last_rate_time - (per_rate_time * count as Number) as u32;
                    newcount += count;
                } else if time.loop_count == 0 && ids.actives.len() == 0 {
                    self.last_rate_time += (per_rate_time) as u32;
                }
            }
        }
        
        // log::error!("{:?}", (isloop, time.total_running_ms, duration, newcount));
        ids.create_new(newcount);
    }
}

#[derive(Default, Deref)]
pub struct ParticleAgeLifetime(pub(crate) Vec<AgeLifeTime>);
impl ParticleAgeLifetime {
    pub fn new(maxcount: usize) -> Self {
        let mut vec = Vec::with_capacity(maxcount);
        for _ in 0..maxcount {
            vec.push(AgeLifeTime::default())
        }
        Self(vec)
    }
    pub fn start(
        &mut self,
        time: &ParticleSystemTime,
        newids: &Vec<IdxParticle>,
        calculator: &FloatInterpolation,
        randomlist: &Vec<BaseRandom>,
    ) {
        self.0.iter_mut().for_each(|item| {
            if item.age < u32::MAX - time.running_delta_ms {
                item.age += time.running_delta_ms;
                if item.lifetime == 0 {
                    item.progress = 1.;
                } else {
                    item.progress = f32::max(0., f32::min(item.age as f32 / item.lifetime as f32, 1.))
                }
            }
        });
        newids.iter().for_each(|idx| {
            let randoms = randomlist.get(*idx).unwrap();
            self.0[*idx] = AgeLifeTime {
                age: 0,
                lifetime: (calculator.interpolate(time.loop_progress, randoms.base) * 1000.) as u32,
                progress: 0.,
            };
        });
    }
}

#[derive(Default, Deref)]
pub struct ParticleStartColor(pub(crate) Vec<Vector4>);
impl ParticleStartColor {
    pub fn new(maxcount: usize) -> Self {
        let mut vec = Vec::with_capacity(maxcount);
        for _ in 0..maxcount {
            vec.push(Vector4::new(1., 1., 1., 1.));
        }
        Self(vec)
    }
    pub fn start(
        &mut self,
        newids: &Vec<IdxParticle>,
        colors: &mut Vec<Vector4>,
        randomlist: &Vec<BaseRandom>,
        time: &ParticleSystemTime,
        start_color_interpolation: &StartColor,
    ) {
        newids.iter().for_each(|idx| {
            let item = self.0.get_mut(*idx).unwrap();
            let color = colors.get_mut(*idx).unwrap();
            let randoms = randomlist.get(*idx).unwrap();
            start_color_interpolation.modify(item, time.loop_progress, randoms);
            color.copy_from(&item);
            // log::warn!("Start Color: {:?}", item);
        });
    }
}

#[derive(Default)]
pub struct ParticleStartScaling(pub(crate) Vec<Vector3>);
impl ParticleStartScaling {
    pub fn new(maxcount: usize) -> Self {
        let mut vec = Vec::with_capacity(maxcount);
        for _ in 0..maxcount {
            vec.push(Vector3::new(1., 1., 1.));
        }
        Self(vec)
    }
    pub fn start(
        &mut self,
        newids: &Vec<IdxParticle>,
        localscalings: &mut Vec<Vector3>,
        randomlist: &Vec<BaseRandom>,
        time: &ParticleSystemTime,
        start_size_interpolation: &StartSize,
    ) {
        newids.iter().for_each(|idx| {
            let item = self.0.get_mut(*idx).unwrap();
            let localscaling = localscalings.get_mut(*idx).unwrap();
            let randoms = randomlist.get(*idx).unwrap();
            start_size_interpolation.modify(item, time.loop_progress, randoms);
            localscaling.copy_from(&item);
        });
    }
}

#[derive(Component, Default)]
pub struct ParticleStart {
    pub color: ParticleStartColor,
    pub scale: ParticleStartScaling,
    pub ages: ParticleAgeLifetime,
}
impl ParticleStart {
    pub fn new(maxcount: usize) -> Self {
        Self {
            color: ParticleStartColor::new(maxcount),
            scale: ParticleStartScaling::new(maxcount),
            ages: ParticleAgeLifetime::new(maxcount),
        }
    }
}

#[derive(Default, Deref, DerefMut)]
pub struct ParticleLocalPosition(pub(crate) Vec<Vector3>);
impl ParticleLocalPosition {
    pub fn new(maxcount: usize) -> Self {
        let mut vec = Vec::with_capacity(maxcount);
        for _ in 0..maxcount {
            vec.push(Vector3::new(0., 0., 0.));
        }
        Self(vec)
    }
    pub fn start(
        &mut self,
        newids: &Vec<IdxParticle>,
        directions: &mut Vec<Direction>,
        randomlist: &Vec<BaseRandom>,
        time: &ParticleSystemTime,
        emitter: &TypeShapeEmitter,
        startspeed: &ParticleCalculatorStartSpeed,
        tempvec3: &mut Vector3,
    ) {
        let emission_total = newids.len() as f32;
        let mut emission_index = 0;
        newids.iter().for_each(|idx| {
            let position_to_update = self.0.get_mut(*idx).unwrap();
            let direction_to_update = directions.get_mut(*idx).unwrap();
            let randoms = randomlist.get(*idx).unwrap();
            let mut random = Random::new(randoms.seed);

            emitter.start_position_function(position_to_update, time.emission_loop as f32, time.emission_progress, emission_index as f32, emission_total, &mut random, tempvec3);

            let local_position = &position_to_update;
            emitter.start_direction_function(&mut direction_to_update.velocity_start, local_position, &mut random, tempvec3);

            let startspeed = startspeed.0.interpolate(time.emission_progress, randoms.base);
            direction_to_update.velocity_start.scale_mut(startspeed);
            direction_to_update.value.copy_from(&direction_to_update.velocity_start);
            direction_to_update.velocity_force.x = 0.;direction_to_update.velocity_force.y = 0.;direction_to_update.velocity_force.z = 0.;

            // log::warn!("StartPosition: {:?}, Direction: {:?}", position_to_update, direction_to_update.velocity_start);

            emission_index += 1;
        });
    }
}

#[derive(Default, Deref)]
pub struct ParticleLocalRotation(pub(crate) Vec<Vector3>);
impl ParticleLocalRotation {
    pub fn new(maxcount: usize) -> Self {
        let mut vec = Vec::with_capacity(maxcount);
        for _ in 0..maxcount {
            vec.push(Vector3::new(0., 0., 0.));
        }
        Self(vec)
    }
    pub fn start(
        &mut self,
        newids: &Vec<IdxParticle>,
        randomlist: &Vec<BaseRandom>,
        time: &ParticleSystemTime,
        calculator: &StartRotation,
    ) {
        // let delta_seconds = time.running_delta_ms as f32 / 1000.0;
        newids.iter().for_each(|idx| {
            let randoms = randomlist.get(*idx).unwrap();
            let item = self.0.get_mut(*idx).unwrap();
            calculator.modify(item, time.emission_progress, randoms);
        });
    }
    pub fn run(
        &mut self,
        activeids: &Vec<IdxParticle>,
        ages: &Vec<AgeLifeTime>,
        randomlist: &Vec<BaseRandom>,
        time: &ParticleSystemTime,
        calculator: &RotationOverLifetime,
    ) {
        let delta_seconds = time.running_delta_ms as f32 / 1000.0;
        activeids.iter().for_each(|idx| {
            let age = ages.get(*idx).unwrap();
            let randoms = randomlist.get(*idx).unwrap();
            let item = self.0.get_mut(*idx).unwrap();
            calculator.modify(item, age.progress, delta_seconds, randoms);
        });
    }
    pub fn speed(
        &mut self,
        activeids: &Vec<IdxParticle>,
        directions: &Vec<Direction>,
        randomlist: &Vec<BaseRandom>,
        time: &ParticleSystemTime,
        calculator: &RotationBySpeed,
    ) {
        let delta_seconds = time.running_delta_ms as f32 / 1000.0;
        activeids.iter().for_each(|idx| {
            let direction = directions.get(*idx).unwrap();
            let randoms = randomlist.get(*idx).unwrap();
            let item = self.0.get_mut(*idx).unwrap();
            calculator.modify(item, direction.length, delta_seconds, randoms);
        });
    }
}

#[derive(Default, Deref, DerefMut)]
pub struct ParticleLocalScaling(pub(crate) Vec<Vector3>);
impl ParticleLocalScaling {
    pub fn new(maxcount: usize) -> Self {
        let mut vec = Vec::with_capacity(maxcount);
        for _ in 0..maxcount {
            vec.push(Vector3::new(1., 1., 1.));
        }
        Self(vec)
    }
    pub fn run(
        &mut self,
        activeids: &Vec<IdxParticle>,
        ages: &Vec<AgeLifeTime>,
        startsizes: &Vec<Vector3>,
        randomlist: &Vec<BaseRandom>,
        calculator: &SizeOverLifetime,
    ) {
        activeids.iter().for_each(|idx| {
            let age = ages.get(*idx).unwrap();
            let randoms = randomlist.get(*idx).unwrap();
            let startsize = startsizes.get(*idx).unwrap();
            let item = self.0.get_mut(*idx).unwrap();
            item.copy_from(startsize);
            calculator.modify(item, age.progress, randoms);
        });
    }
    pub fn speed(
        &mut self,
        activeids: &Vec<IdxParticle>,
        directions: &Vec<Direction>,
        randomlist: &Vec<BaseRandom>,
        calculator: &SizeBySpeed,
    ) {
        activeids.iter().for_each(|idx| {
            let direction = directions.get(*idx).unwrap();
            let randoms = randomlist.get(*idx).unwrap();
            let item = self.0.get_mut(*idx).unwrap();
            calculator.modify(item, direction.length, randoms);
        });
    }
}

#[derive(Component, Default)]
pub struct ParticleLocal {
    pub position: ParticleLocalPosition,
    pub rotation: ParticleLocalRotation,
    pub scalings: ParticleLocalScaling,
    pub speedfector: ParticleSpeedFactor,
    pub colorsanduvs: ParticleColorAndUV,
}
impl ParticleLocal {
    pub fn new(maxcount: usize) -> Self {
        Self {
            position: ParticleLocalPosition::new(maxcount),
            rotation: ParticleLocalRotation::new(maxcount),
            scalings: ParticleLocalScaling::new(maxcount),
            speedfector: ParticleSpeedFactor::new(maxcount),
            colorsanduvs: ParticleColorAndUV::new(maxcount),
        }
    }
}

#[derive(Component, Default)]
pub struct ParticleVelocityAndForce {
    pub velocity: ParticleVelocity,
    pub limitvelocityscalar: ParticleLimitVelocityScalar,
    pub forces: ParticleForce,
    pub speedfector: ParticleSpeedFactor,
    pub gravities: ParticleGravityFactor,
}
impl ParticleVelocityAndForce {
    pub fn new(maxcount: usize, force_is_local_space: bool, force_constant: bool, gravity_calculator: &ParticleCalculatorGravity, simulation_mode: &EParticleSimulationSpace) -> Self {
        Self {
            velocity: ParticleVelocity::new(maxcount),
            limitvelocityscalar: ParticleLimitVelocityScalar::new(maxcount),
            forces: ParticleForce::new(maxcount, force_is_local_space, force_constant),
            speedfector: ParticleSpeedFactor::new(maxcount),
            gravities: ParticleGravityFactor::new(maxcount, gravity_calculator, simulation_mode),
        }
    }
}

/// 发射时的全局矩阵 - 在 World 空间发射在发射时即固定, 在 Local 空间发射实时更新为发射器全局矩阵
#[derive(Component)]
pub struct ParticleEmitMatrix {
    pub(crate) emits: Vec<EmitMatrix>,
    scaling_mode: fn(& Isometry3, & Vector3, & Vector3, & Matrix, & Matrix, & mut Vector3, & mut Matrix, & mut Matrix),
    simulation: fn(& mut Vec<EmitMatrix>, & Vec<usize>, & Vec<usize>, & Vector3, & SQuaternion<Number>, & Matrix, & Matrix),
    idx: fn(usize) -> usize,
}
impl ParticleEmitMatrix {
    pub fn new(maxcount: usize, scalingmode: &EScalingMode, simulation_mode: &EParticleSimulationSpace) -> Self {
        let scaling_mode = match scalingmode {
            EScalingMode::Hierarchy     => ParticleEmitMatrix::scaling_mode_hierarchy,
            EScalingMode::Local         => ParticleEmitMatrix::scaling_mode_local,
            EScalingMode::Shape         => ParticleEmitMatrix::scaling_mode_shape,
        };
        let simulation = match simulation_mode {
            EParticleSimulationSpace::Local         => ParticleEmitMatrix::simulation_local,
            EParticleSimulationSpace::World         => ParticleEmitMatrix::simulation_world,
        };
        let idx = match simulation_mode {
            EParticleSimulationSpace::Local         => _idx_constant,
            EParticleSimulationSpace::World         => _idx,
        };
        let maxcount = match simulation_mode {
            EParticleSimulationSpace::Local         => 1 as usize,
            EParticleSimulationSpace::World         => maxcount,
        };

        let mut emits = Vec::with_capacity(maxcount);
        for _ in 0..maxcount {
            emits.push(EmitMatrix::default());
        }

        Self {
            emits,
            scaling_mode,
            simulation,
            idx,
        }
    }
    pub fn get(&self, idx: usize) -> Option<&EmitMatrix> {
        self.emits.get((self.idx)(idx))
    }
    pub fn emit(
        &mut self,
        newids: &Vec<IdxParticle>,
        ids: &Vec<IdxParticle>,
        _world_matrix: &Matrix,
        _world_matrix_inv: &Matrix,
        _iso: &Isometry3,
        _global_position: &Vector3,
        global_rotation: &SQuaternion<Number>,
        global_scaling: &Vector3,
        local_scaling: &Vector3,
    ) {
        // log::warn!("EmitMatrix:");
        let mut scaling = global_scaling.clone();
        let mut emittermatrix = Matrix::identity();
        let mut emittermatrix_invert = Matrix::identity();
        (self.scaling_mode)(_iso, global_scaling, local_scaling, _world_matrix, _world_matrix_inv, &mut scaling, &mut emittermatrix, &mut emittermatrix_invert);

        (self.simulation)(&mut self.emits, ids, newids, &scaling, global_rotation, &emittermatrix, &emittermatrix_invert);

        // log::warn!("EmitMatrix: End");
    }
    pub fn scaling_mode_hierarchy<'a>(_iso: &'a Isometry3, _global_scaling: &'a Vector3, _local_scaling: &'a Vector3, _world_matrix: &'a Matrix, _world_matrix_inv: &'a Matrix, _resultscale: &'a mut Vector3, result_world_matrix: &'a mut Matrix, result_world_matrix_inv: &'a mut Matrix) {
        result_world_matrix.clone_from(_world_matrix);
        result_world_matrix_inv.clone_from(_world_matrix_inv);
    }
    pub fn scaling_mode_local<'a>(_iso: &'a Isometry3, _global_scaling: &'a Vector3, local_scaling: &'a Vector3, _world_matrix: &'a Matrix, _world_matrix_inv: &'a Matrix, resultscale: &'a mut Vector3, result_world_matrix: &'a mut Matrix, result_world_matrix_inv: &'a mut Matrix) {
        
        resultscale.copy_from(local_scaling);


        result_world_matrix.clone_from(&_iso.to_matrix()); // Matrix::identity();
        result_world_matrix.append_nonuniform_scaling_mut(local_scaling);

        result_world_matrix_inv.clone_from(&result_world_matrix);
        CoordinateSytem3::try_inverse_mut(result_world_matrix_inv);
    }
    pub fn scaling_mode_shape<'a>(_iso: &'a Isometry3, global_scaling: &'a Vector3, _local_scaling: &'a Vector3, _world_matrix: &'a Matrix, _world_matrix_inv: &'a Matrix, resultscale: &'a mut Vector3, result_world_matrix: &'a mut Matrix, result_world_matrix_inv: &'a mut Matrix) {
        
        resultscale.copy_from_slice(&[1., 1., 1.]);

        result_world_matrix.clone_from(&_iso.to_matrix()); // Matrix::identity();
        result_world_matrix.append_nonuniform_scaling_mut(global_scaling);
        
        result_world_matrix_inv.clone_from(&result_world_matrix);
        CoordinateSytem3::try_inverse_mut(result_world_matrix_inv);
    }
    pub fn simulation_local<'a>(emits: &'a mut Vec<EmitMatrix>, _ids: &'a Vec<usize>, _newids: &'a Vec<usize>, scaling: &'a Vector3, global_rotation: &'a SQuaternion<Number>, emittermatrix: &'a Matrix, emittermatrix_invert: &'a Matrix) {

        let item = emits.get_mut(0).unwrap();
        item.scaling.clone_from(scaling);
        item.rotation.i = global_rotation.i; item.rotation.j = global_rotation.j; item.rotation.k = global_rotation.k; item.rotation.w = global_rotation.w;
        // item.rotation.clone_from(global_rotation);
        item.matrix.clone_from(emittermatrix);
        item.matrix_invert.clone_from(emittermatrix_invert);
    }
    pub fn simulation_world<'a>(emits: &'a mut Vec<EmitMatrix>, _ids: &'a Vec<usize>, _newids: &'a Vec<usize>, scaling: &'a Vector3, global_rotation: &'a SQuaternion<Number>, emittermatrix: &'a Matrix, emittermatrix_invert: &'a Matrix) {
        _newids.iter().for_each(|idx| {
            let item = emits.get_mut(*idx).unwrap();
            item.scaling.clone_from(scaling);
            item.rotation.i = global_rotation.i; item.rotation.j = global_rotation.j; item.rotation.k = global_rotation.k; item.rotation.w = global_rotation.w;
            // item.rotation.clone_from(global_rotation);
            item.matrix.clone_from(emittermatrix);
            item.matrix_invert.clone_from(emittermatrix_invert);
        });
    }
}
impl Default for ParticleEmitMatrix  {
    fn default() -> Self {
        Self::new(1, &EScalingMode::default(), &EParticleSimulationSpace::default())
    }
}

/// 粒子局部重力影响
pub struct ParticleGravityFactor {
    pub(crate) values: Vec<GravityFactor>,
    pub(crate) _runcall: fn(&mut Vec<GravityFactor>, &Vec<IdxParticle>, &Vec<AgeLifeTime>, &ParticleEmitMatrix, &Vec<BaseRandom>, &ParticleSystemTime, &ParticleCalculatorGravity),
    pub(crate) _idxcall: fn(usize) -> usize,
}
impl ParticleGravityFactor {
    pub fn new(maxcount: usize, calculator: &ParticleCalculatorGravity, simulation_mode: &EParticleSimulationSpace) -> Self {
        let mut vec = Vec::with_capacity(maxcount);
        match (&calculator.0.interpolation.mode, simulation_mode) {
            (EInterpolationCurveMode::Constant, EParticleSimulationSpace::Local) => {
                let maxcount = 1;
                for _ in 0..maxcount {
                    vec.push(GravityFactor::default());
                }
                Self {
                    values: vec,
                    _runcall: Self::_run_constant,
                    _idxcall: _idx_constant,
                }
            },
            (_, _) => {
                for _ in 0..maxcount {
                    vec.push(GravityFactor::default());
                }
                Self {
                    values: vec,
                    _runcall: Self::_run,
                    _idxcall: _idx,
                }
            }
        }
    }
    pub fn run(
        &mut self,
        ids: &Vec<IdxParticle>,
        ages: &Vec<AgeLifeTime>,
        emitmatrixs: &ParticleEmitMatrix,
        randomlist: &Vec<BaseRandom>,
        time: &ParticleSystemTime,
        calculator: &ParticleCalculatorGravity,
    ) {
        (self._runcall)(&mut self.values, ids, ages, emitmatrixs, randomlist, time, calculator);
    }
    pub fn get(&self, idx: usize) -> Option<&GravityFactor> {
        self.values.get((self._idxcall)(idx))
    }
    fn _run_constant<'a>(
        items: &'a mut Vec<GravityFactor>,
        _ids: &'a Vec<IdxParticle>,
        ages: &'a Vec<AgeLifeTime>,
        emitmatrixs: &'a ParticleEmitMatrix,
        randomlist: &'a Vec<BaseRandom>,
        time: &'a ParticleSystemTime,
        calculator: &'a ParticleCalculatorGravity,
    ) {
        let delta_seconds = time.running_delta_ms as f32 / 1000.0;

        let item = items.get_mut(0).unwrap();
        let randoms = randomlist.get(0).unwrap();
        let age = ages.get(0).unwrap();
        let emitmatrix = emitmatrixs.get(0).unwrap();
        
        let mut factor = 0.;
        calculator.0.modify(&mut factor, age.progress, delta_seconds, randoms);
        // log::error!("Gravity: {}", factor);
        if factor.abs() < PARTICLE_MIN_VALUE {
            item.value.copy_from_slice(&[0., 0., 0.]);
        } else {
            // item.value.copy_from(&calculator.1.scale(factor));
            let x = calculator.1.x * factor;
            let y = calculator.1.y * factor;
            let z = calculator.1.z * factor;
            CoordinateSytem3::transform_normal_floats(x, y, z, &emitmatrix.matrix_invert, &mut item.value);
            // log::warn!("Gravity {:?}", (1, &item.value));
        }
    }
    fn _run<'a>(
        items: &'a mut Vec<GravityFactor>,
        ids: &'a Vec<IdxParticle>,
        ages: &'a Vec<AgeLifeTime>,
        emitmatrixs: &'a ParticleEmitMatrix,
        randomlist: &'a Vec<BaseRandom>,
        time: &'a ParticleSystemTime,
        calculator: &'a ParticleCalculatorGravity,
    ) {
        let delta_seconds = time.running_delta_ms as f32 / 1000.0;
        ids.iter().for_each(|idx| {
            let item = items.get_mut(*idx).unwrap();
            let randoms = randomlist.get(*idx).unwrap();
            let age = ages.get(*idx).unwrap();
            let emitmatrix = emitmatrixs.get(*idx).unwrap();
            
            let mut factor = 0.;
            calculator.0.modify(&mut factor, age.progress, delta_seconds, randoms);

            // item.value.copy_from(&calculator.1.scale(factor));
            let x = calculator.1.x * factor;
            let y = calculator.1.y * factor;
            let z = calculator.1.z * factor;
            CoordinateSytem3::transform_normal_floats(x, y, z, &emitmatrix.matrix_invert, &mut item.value);
            // log::warn!("Gravity {:?}", (0, &item.value));
        });
    }
}
impl Default for ParticleGravityFactor {
    fn default() -> Self {
        Self::new(1, &ParticleCalculatorGravity::default(), &EParticleSimulationSpace::default())
    }
}

/// 粒子局部外力影响
pub struct ParticleForce {
    pub(crate) values: Vec<Force>,
    pub(crate) _runcall: fn(&mut Vec<Force>, &Vec<IdxParticle>, &Vec<AgeLifeTime>, &ParticleEmitMatrix, &Vec<BaseRandom>, &ParticleSystemTime, &ForceOverLifetime),
    pub(crate) _idxcall: fn(usize) -> usize,
}
impl ParticleForce {
    pub fn new(maxcount: usize, is_local_space: bool, constant: bool) -> Self {
        match (is_local_space, constant) {
            (true, true) => {
                let maxcount = 1;
                let mut vec = Vec::with_capacity(maxcount);
                for _ in 0..maxcount {
                    vec.push(Force::default());
                }
                Self {
                    values: vec,
                    _runcall: Self::_run_local_constant,
                    _idxcall: _idx_constant,
                }
            },
            (_, _) => {
                let mut vec = Vec::with_capacity(maxcount);
                for _ in 0..maxcount {
                    vec.push(Force::default());
                }
                Self {
                    values: vec,
                    _runcall: Self::_run,
                    _idxcall: _idx_constant,
                }
            }
        }
    }
    pub fn get(&self, idx: usize) -> Option<&Force> {
        self.values.get((self._idxcall)(idx))
    }
    pub fn run(
        &mut self,
        ids: &Vec<IdxParticle>,
        ages: &Vec<AgeLifeTime>,
        emitmatrixs: &ParticleEmitMatrix,
        randomlist: &Vec<BaseRandom>,
        time: &ParticleSystemTime,
        calculator: &ForceOverLifetime,
    ) {
        (self._runcall)(&mut self.values, ids, ages, emitmatrixs, randomlist, time, calculator);
    }
    fn _run_local_constant<'a>(
        items: &mut Vec<Force>,
        _ids: &Vec<IdxParticle>,
        ages: &Vec<AgeLifeTime>,
        emitmatrixs: &ParticleEmitMatrix,
        randomlist: &Vec<BaseRandom>,
        time: &ParticleSystemTime,
        calculator: &ForceOverLifetime,
    ) {
        let delta_seconds = time.running_delta_ms as f32 / 1000.0;
        let item = items.get_mut(0).unwrap();
        let randoms = randomlist.get(0).unwrap();
        let age = ages.get(0).unwrap();
        let emitmatrix = emitmatrixs.get(0).unwrap();

        calculator.modify(item, age.progress, delta_seconds, randoms);

        if calculator.is_local_space == false {
            CoordinateSytem3::transform_normal_floats(item.value.x, item.value.y, item.value.z, &emitmatrix.matrix_invert, &mut item.value);
        }
    }
    fn _run<'a>(
        items: &mut Vec<Force>,
        ids: &Vec<IdxParticle>,
        ages: &Vec<AgeLifeTime>,
        emitmatrixs: &ParticleEmitMatrix,
        randomlist: &Vec<BaseRandom>,
        time: &ParticleSystemTime,
        calculator: &ForceOverLifetime,
    ) {
        let delta_seconds = time.running_delta_ms as f32 / 1000.0;
        ids.iter().for_each(|idx| {
            let item = items.get_mut(*idx).unwrap();
            let randoms = randomlist.get(*idx).unwrap();
            let age = ages.get(*idx).unwrap();
            let emitmatrix = emitmatrixs.get(*idx).unwrap();

            calculator.modify(item, age.progress, delta_seconds, randoms);

            if calculator.is_local_space == false {
                CoordinateSytem3::transform_normal_floats(item.value.x, item.value.y, item.value.z, &emitmatrix.matrix_invert, &mut item.value);
            }
        });
    }
}
impl Default for ParticleForce {
    fn default() -> Self {
        Self::new(1, true, true)
    }
}

/// 粒子局部速度向量
#[derive(Default, Deref)]
pub struct ParticleVelocity(pub(crate) Vec<Velocity>);
impl ParticleVelocity {
    pub fn new(maxcount: usize) -> Self {
        let mut vec = Vec::with_capacity(maxcount);
        for _ in 0..maxcount {
            vec.push(Velocity::default());
        }
        Self(vec)
    }
    pub fn run(
        &mut self,
        ids: &Vec<IdxParticle>,
        ages: &Vec<AgeLifeTime>,
        randomlist: &Vec<BaseRandom>,
        _: &ParticleSystemTime,
        calculator: &VelocityOverLifetime,
    ) {
        // let delta_seconds = time.running_delta_ms as f32 / 1000.0;
        ids.iter().for_each(|idx| {
            let item = self.0.get_mut(*idx).unwrap();
            let randoms = randomlist.get(*idx).unwrap();
            let age = ages.get(*idx).unwrap();
            
            calculator.modify(item, age.progress, randoms);
        });
    }
}

/// 粒子局部速度因子
#[derive(Default, Deref)]
pub struct ParticleSpeedFactor(pub(crate) Vec<SpeedFactor>);
impl ParticleSpeedFactor {
    pub fn new(maxcount: usize) -> Self {
        let mut vec = Vec::with_capacity(maxcount);
        for _ in 0..maxcount { vec.push(SpeedFactor::default()); }
        Self(vec)
    }
    pub fn run(
        &mut self,
        ids: &Vec<IdxParticle>,
        ages: &Vec<AgeLifeTime>,
        randomlist: &Vec<BaseRandom>,
        _: &ParticleSystemTime,
        calculator: &SpeedModifier,
    ) {
        // let delta_seconds = time.running_delta_ms as f32 / 1000.0;
        ids.iter().for_each(|idx| {
            let item = self.0.get_mut(*idx).unwrap();
            let randoms = randomlist.get(*idx).unwrap();
            let age = ages.get(*idx).unwrap();
            
            calculator.modify(item, age.progress, randoms);
        });
    }
}

/// 粒子局部轨道速度向量
#[derive(Component)]
pub struct ParticleOrbitVelocity {
    pub(crate) values: Vec<(Vector3, Number)>,
    pub(crate) _runcall: fn(&mut Vec<(Vector3, Number)>, &Vec<IdxParticle>, &Vec<AgeLifeTime>, &Vec<BaseRandom>, &ParticleCalculatorOrbitVelocity),
    pub(crate) _idxcall: fn(usize) -> usize,
}
impl ParticleOrbitVelocity {
    pub fn new(maxcount: usize, velocity: & ParticleCalculatorOrbitVelocity) -> Self {
        match velocity.0.constant() {
            true => {
                let maxcount = 1;
                let mut vec = Vec::with_capacity(maxcount);
                for _ in 0..maxcount { vec.push((Vector3::zeros(), 0.)); }
                Self { values: vec, _runcall: Self::_run_local_constant, _idxcall: _idx_constant }
            },
            _ => {
                let mut vec = Vec::with_capacity(maxcount);
                for _ in 0..maxcount { vec.push((Vector3::zeros(), 0.)); }
                Self { values: vec, _runcall: Self::_run, _idxcall: _idx }
            },
        }
    }
    pub fn get(&self, idx: usize) -> Option<&(Vector3, Number)> {
        self.values.get((self._idxcall)(idx))
    }
    pub fn run(
        &mut self,
        ids: &Vec<IdxParticle>,
        ages: &Vec<AgeLifeTime>,
        randomlist: &Vec<BaseRandom>,
        velocity: &ParticleCalculatorOrbitVelocity,
    ) {
        // // let delta_seconds = time.running_delta_ms as f32 / 1000.0;
        // ids.iter().for_each(|idx| {
        //     let item = self.0.get_mut(*idx).unwrap();
        //     let randoms = randomlist.get(*idx).unwrap();
        //     let age = ages.get(*idx).unwrap();

        //     if let Ok(calculator) = offset {
        //         calculator.0.compute(age.progress, randoms, &mut item.offset);
        //     }
        //     if let Ok(calculator) = velocity {
        //         calculator.0.compute(age.progress, randoms, &mut item.orbit);
        //         item.orbit_len = CoordinateSytem3::length_squared(&item.orbit);
        //     }
        //     if let Ok(calculator) = radial {
        //         item.radial = calculator.0.interpolate(age.progress, randoms.w);
        //     }
        // });
        (self._runcall)(&mut self.values, ids, ages, randomlist, velocity);
    }
    fn _run_local_constant<'a>(
        items: &'a mut Vec<(Vector3, Number)>,
        _ids: &'a Vec<IdxParticle>,
        ages: &'a Vec<AgeLifeTime>,
        randomlist: &'a Vec<BaseRandom>,
        calculator: &'a ParticleCalculatorOrbitVelocity,
    ) {
        let item = items.get_mut(0).unwrap();
        let randoms = randomlist.get(0).unwrap();
        let age = ages.get(0).unwrap();

        calculator.0.compute(age.progress, randoms, &mut item.0);
        item.1 = CoordinateSytem3::length_squared(&item.0);
    }
    fn _run<'a>(
        items: &'a mut Vec<(Vector3, Number)>,
        ids: &'a Vec<IdxParticle>,
        ages: &'a Vec<AgeLifeTime>,
        randomlist: &'a Vec<BaseRandom>,
        calculator: &'a ParticleCalculatorOrbitVelocity,
    ) {
        ids.iter().for_each(|idx| {
            let item = items.get_mut(*idx).unwrap();
            let randoms = randomlist.get(*idx).unwrap();
            let age = ages.get(*idx).unwrap();

            calculator.0.compute(age.progress, randoms, &mut item.0);
            item.1 = CoordinateSytem3::length_squared(&item.0);
        });
    }
}
impl Default for ParticleOrbitVelocity {
    fn default() -> Self {
        Self::new(1, &ParticleCalculatorOrbitVelocity::default())
    }
}

/// 粒子局部轨道速度向量
#[derive(Component)]
pub struct ParticleOrbitOffset {
    pub(crate) values: Vec<Vector3>,
    pub(crate) _runcall: fn(&mut Vec<Vector3>, &Vec<IdxParticle>, &Vec<AgeLifeTime>, &Vec<BaseRandom>, &ParticleCalculatorOrbitOffset),
    pub(crate) _idxcall: fn(usize) -> usize,
}
impl Default for ParticleOrbitOffset {
    fn default() -> Self {
        Self::new(1, &ParticleCalculatorOrbitOffset::default())
    }
}
impl ParticleOrbitOffset {
    pub fn new(maxcount: usize, offset: & ParticleCalculatorOrbitOffset) -> Self {
        match offset.0.constant() {
            true => {
                let maxcount = 1;
                let mut vec = Vec::with_capacity(maxcount);
                for _ in 0..maxcount { vec.push(Vector3::zeros()); }
                Self { values: vec, _runcall: Self::_run_local_constant, _idxcall: _idx_constant }
            },
            _ => {
                let mut vec = Vec::with_capacity(maxcount);
                for _ in 0..maxcount { vec.push(Vector3::zeros()); }
                Self { values: vec, _runcall: Self::_run, _idxcall: _idx }
            },
        }
    }
    pub fn get(&self, idx: usize) -> Option<&Vector3> {
        self.values.get((self._idxcall)(idx))
    }
    pub fn run(
        &mut self,
        ids: &Vec<IdxParticle>,
        ages: &Vec<AgeLifeTime>,
        randomlist: &Vec<BaseRandom>,
        offset: &ParticleCalculatorOrbitOffset,
    ) {
        (self._runcall)(&mut self.values, ids, ages, randomlist, offset);
    }
    fn _run_local_constant<'a>(
        items: &'a mut Vec<Vector3>,
        _ids: &'a Vec<IdxParticle>,
        ages: &'a Vec<AgeLifeTime>,
        randomlist: &'a Vec<BaseRandom>,
        calculator: &'a ParticleCalculatorOrbitOffset,
    ) {
        let item = items.get_mut(0).unwrap();
        let randoms = randomlist.get(0).unwrap();
        let age = ages.get(0).unwrap();

            calculator.0.compute(age.progress, randoms, item);
    }
    fn _run<'a>(
        items: &'a mut Vec<Vector3>,
        ids: &'a Vec<IdxParticle>,
        ages: &'a Vec<AgeLifeTime>,
        randomlist: &'a Vec<BaseRandom>,
        calculator: &'a ParticleCalculatorOrbitOffset,
    ) {
        ids.iter().for_each(|idx| {
            let item = items.get_mut(*idx).unwrap();
            let randoms = randomlist.get(*idx).unwrap();
            let age = ages.get(*idx).unwrap();

                calculator.0.compute(age.progress, randoms, item);
        });
    }
}

/// 粒子局部轨道速度向量
#[derive(Component)]
pub struct ParticleOrbitRadial {
    pub(crate) values: Vec<Number>,
    pub(crate) _runcall: fn(&mut Vec<Number>, &Vec<IdxParticle>, &Vec<AgeLifeTime>, &Vec<BaseRandom>, &ParticleCalculatorOrbitRadial),
    pub(crate) _idxcall: fn(usize) -> usize,
}
impl Default for ParticleOrbitRadial {
    fn default() -> Self {
        Self::new(1, &ParticleCalculatorOrbitRadial::default())
    }
}
impl ParticleOrbitRadial {
    pub fn new(maxcount: usize, offset: & ParticleCalculatorOrbitRadial) -> Self {
        match offset.0.mode {
            EInterpolationCurveMode::Constant => {
                let maxcount = 1;
                let mut vec = Vec::with_capacity(maxcount);
                for _ in 0..maxcount { vec.push(0.); }
                Self { values: vec, _runcall: Self::_run_local_constant, _idxcall: _idx_constant }
            },
            _ => {
                let mut vec = Vec::with_capacity(maxcount);
                for _ in 0..maxcount { vec.push(0.); }
                Self { values: vec, _runcall: Self::_run, _idxcall: _idx }
            },
        }
    }
    pub fn get(&self, idx: usize) -> Option<&Number> {
        self.values.get((self._idxcall)(idx))
    }
    pub fn run(
        &mut self,
        ids: &Vec<IdxParticle>,
        ages: &Vec<AgeLifeTime>,
        randomlist: &Vec<BaseRandom>,
        radial: &ParticleCalculatorOrbitRadial,
    ) {
        (self._runcall)(&mut self.values, ids, ages, randomlist, radial);
    }
    fn _run_local_constant<'a>(
        items: &'a mut Vec<Number>,
        _ids: &'a Vec<IdxParticle>,
        ages: &'a Vec<AgeLifeTime>,
        randomlist: &'a Vec<BaseRandom>,
        calculator: &'a ParticleCalculatorOrbitRadial,
    ) {
        let item = items.get_mut(0).unwrap();
        let randoms = randomlist.get(0).unwrap();
        let age = ages.get(0).unwrap();

            *item = calculator.0.interpolate(age.progress, randoms.w);
    }
    fn _run<'a>(
        items: &'a mut Vec<Number>,
        ids: &'a Vec<IdxParticle>,
        ages: &'a Vec<AgeLifeTime>,
        randomlist: &'a Vec<BaseRandom>,
        calculator: &'a ParticleCalculatorOrbitRadial,
    ) {
        ids.iter().for_each(|idx| {
            let item = items.get_mut(*idx).unwrap();
            let randoms = randomlist.get(*idx).unwrap();
            let age = ages.get(*idx).unwrap();

                *item = calculator.0.interpolate(age.progress, randoms.w);
        });
    }
}

#[derive(Default, Deref)]
pub struct ParticleLimitVelocityScalar(pub(crate) Vec<LimitVelocityScalar>);
impl ParticleLimitVelocityScalar {
    pub fn new(maxcount: usize) -> Self {
        let mut vec = Vec::with_capacity(maxcount);
        for _ in 0..maxcount {
            vec.push(LimitVelocityScalar::default());
        }
        Self(vec)
    }
    pub fn run(
        &mut self,
        ids: &Vec<IdxParticle>,
        ages: &Vec<AgeLifeTime>,
        randomlist: &Vec<BaseRandom>,
        _: &ParticleSystemTime,
        calculator: &LimitVelocityOverLifetime,
    ) {
        // let delta_seconds = time.running_delta_ms as f32 / 1000.0;
        ids.iter().for_each(|idx| {
            let item = self.0.get_mut(*idx).unwrap();
            let randoms = randomlist.get(*idx).unwrap();
            let age = ages.get(*idx).unwrap();
            
            calculator.modify(item, age.progress, randoms);
        });
    }
}

/// 粒子局部步进向量
#[derive(Component, Deref, DerefMut)]
pub struct ParticleDirection(pub(crate) Vec<Direction>);
impl Default for ParticleDirection {
    fn default() -> Self {
        Self::new(1)
    }
}
impl ParticleDirection {
    pub fn new(maxcount: usize) -> Self {
        let mut vec = Vec::with_capacity(maxcount);
        for _ in 0..maxcount {
            vec.push(Direction::default());
        }
        Self(vec)
    }
    pub fn run(
        &mut self,
        ids: &Vec<IdxParticle>,
        forces: &ParticleForce,
        gravities: &ParticleGravityFactor,
        velocities: &Vec<Velocity>,
        limitscalars: &Vec<LimitVelocityScalar>,
        orbitsvelocity: &ParticleOrbitVelocity,
        orbitsoffset: &ParticleOrbitOffset,
        orbitsradial: &ParticleOrbitRadial,
        speedfactors: &Vec<SpeedFactor>,
        positions: &mut Vec<Vector3>,
        emitter: &TypeShapeEmitter,
        time: &ParticleSystemTime,
        temp: &mut Vector3,
        orbit_center: &mut Vector3,
        orbit_direction: &mut Vector3,
    ) {
        // log::warn!("Direction: ");
        let delta_seconds = time.running_delta_ms as f32 / 1000.0;
        let inv_delta_seconds = 1. / delta_seconds;
        let half_delta_seconds = delta_seconds * 0.5;
        // let origin = Vector3::zeros();
        ids.iter().for_each(|idx| {
            let force: &Force = forces.get(*idx).unwrap();
            let gravity: &GravityFactor = gravities.get(*idx).unwrap();
            let velocity: &Velocity = velocities.get(*idx).unwrap();
            let limitscalar: &LimitVelocityScalar = limitscalars.get(*idx).unwrap();
            let orbit_offset: &Vector3 = orbitsoffset.get(*idx).unwrap();
            let orbit_velocity: &(Vector3, Number) = orbitsvelocity.get(*idx).unwrap();
            let orbit_radial: &Number = orbitsradial.get(*idx).unwrap();
            let speedfactor: &SpeedFactor = speedfactors.get(*idx).unwrap();
            let direction: &mut Direction = self.0.get_mut(*idx).unwrap();
            let position = positions.get_mut(*idx).unwrap();

            // 力 -> 加速度
            // let a = force.value + gravity.value; //  / 1.; // 质量为 1
            // direction.velocity_force += a.scale(half_delta_seconds);
            direction.velocity_force.x += (force.value.x + gravity.value.x) * half_delta_seconds;
            direction.velocity_force.y += (force.value.y + gravity.value.y) * half_delta_seconds;
            direction.velocity_force.z += (force.value.z + gravity.value.z) * half_delta_seconds;

            // let mut velocity = velocity.value + direction.velocity_force + direction.velocity_start;
            let temp = temp.as_mut_slice();
            temp[0] = velocity.value.x + direction.velocity_force.x + direction.velocity_start.x;
            temp[1] = velocity.value.y + direction.velocity_force.y + direction.velocity_start.y;
            temp[2] = velocity.value.z + direction.velocity_force.z + direction.velocity_start.z;

            let velocity = temp;
            // log::error!("Velocity: {:?}", (velocity, force.value, gravity.value, direction.velocity_start));

            // 轨道速度
            emitter.orbit_center(&position, orbit_offset, orbit_center);
            // let radial_vec: Vector3 = position.sub(&orbit_center);
            orbit_center.x = position.x - orbit_center.x;
            orbit_center.y = position.y - orbit_center.y;
            orbit_center.z = position.z - orbit_center.z;
            orbit_direction.x = 0.; orbit_direction.y = 0.; orbit_direction.z = 0.;
            let radial_vec = &orbit_center;
            if PARTICLE_MIN_VALUE < orbit_velocity.1 {
                let temp = delta_seconds * speedfactor.value;
                let orbit_rotation = CoordinateSytem3::rotation_matrix_from_euler_angles(orbit_velocity.0.x * temp, orbit_velocity.0.y * temp, orbit_velocity.0.z * temp);
                
                // orbit_direction = orbit_rotation.transform_vector(&radial_vec) - radial_vec;
                orbit_rotation.matrix().mul_to(&radial_vec, orbit_direction);
                orbit_direction.x -= radial_vec.x;
                orbit_direction.y -= radial_vec.y;
                orbit_direction.z -= radial_vec.z;
            };
            if PARTICLE_MIN_VALUE < orbit_radial.abs() {
                let radial_len = CoordinateSytem3::length(&radial_vec);
                if PARTICLE_MIN_VALUE < radial_len {
                    let scl = 1. / radial_len * orbit_radial;
                    // velocity += radial_vec.scale(1. / radial_len).scale(*orbit_radial);
                    velocity[0] += radial_vec.x * scl;
                    velocity[1] += radial_vec.y * scl;
                    velocity[2] += radial_vec.z * scl;
                };
            }

            // velocity.scale_mut(speedfactor.value);
            velocity[0] *= speedfactor.value;
            velocity[1] *= speedfactor.value;
            velocity[2] *= speedfactor.value;

            // let mut new_direction = velocity.scale( delta_seconds) + orbit_direction;
            velocity[0] *= delta_seconds;
            velocity[1] *= delta_seconds;
            velocity[2] *= delta_seconds;
            velocity[0] += orbit_direction.x;
            velocity[1] += orbit_direction.y;
            velocity[2] += orbit_direction.z;
            // log::warn!("velocity: {:?}, {:?}, {:?}", velocity, new_direction, delta_seconds);

            let mut directionscalar = (velocity[0] * velocity[0] + velocity[1] * velocity[1] + velocity[2] * velocity[2]).sqrt(); // new_direction.metric_distance(&origin);
            if limitscalar.value < Number::MAX {
                let limitscalarval = limitscalar.value * delta_seconds;
                let delta = directionscalar - limitscalarval;
                if PARTICLE_MIN_VALUE < delta {
                    let factor = limitscalarval + (delta) * Number::exp(Number::ln(delta + 1.0) * (0. - limitscalar.dampen));
                    // let factor = 1.0 - limitscalar.dampen * (directionscalar - limitscalar.value * delta_seconds) / directionscalar * (0.66);

                    // new_direction.scale_mut(factor / directionscalar);
                    let scl = factor / directionscalar;
                    velocity[0] *= scl;
                    velocity[1] *= scl;
                    velocity[2] *= scl;

                    directionscalar = factor;
                    // log::warn!("Limit: {:?}, {:?}, {:?}", limitscalarval, directionscalar, factor);
                }
            }

            // direction.value = new_direction.scale(inv_delta_seconds);
            direction.value.x = velocity[0] * inv_delta_seconds;
            direction.value.y = velocity[1] * inv_delta_seconds;
            direction.value.z = velocity[2] * inv_delta_seconds;

            direction.length = directionscalar / delta_seconds;

            // log::warn!("Direction: {:?}, {:?}, {:?}", direction.value, new_direction, delta_seconds);

            // *position += new_direction;
            position.x += velocity[0];
            position.y += velocity[1];
            position.z += velocity[2];
        });
        // log::warn!("Direction: End");
    }
}

/// 粒子实时颜色
pub struct ParticleColorAndUV {
    pub(crate) color: ParticleColor,
    pub(crate) uv: ParticleUV,
}
impl Default for ParticleColorAndUV {
    fn default() -> Self {
        Self::new(1)
    }
}
impl ParticleColorAndUV {
    pub fn new(maxcount: usize) -> Self {
        Self { color: ParticleColor::new(maxcount), uv: ParticleUV::new(maxcount) }
    }
}

/// 粒子实时颜色
pub struct ParticleColor(pub(crate) Vec<Vector4>);
impl ParticleColor {
    pub fn new(maxcount: usize) -> Self {
        let mut vec = Vec::with_capacity(maxcount);
        for _ in 0..maxcount {
            vec.push(Vector4::new(1., 1., 1., 1.));
        }
        Self(vec)
    }
    pub fn run(
        &mut self,
        activeids: &Vec<IdxParticle>,
        ages: &Vec<AgeLifeTime>,
        startcolors: &Vec<Vector4>,
        randomlist: &Vec<BaseRandom>,
        calculator: &ColorOverLifetime,
    ) {
        activeids.iter().for_each(|idx| {
            let age = ages.get(*idx).unwrap();
            let startcolor = startcolors.get(*idx).unwrap();
            let randoms = randomlist.get(*idx).unwrap();
            let item = self.0.get_mut(*idx).unwrap();
            calculator.modify(item, age.progress, randoms);
            *item = item.component_mul(startcolor);
            // log::warn!("Color: {:?}", item);
        });
    }
    pub fn speed(
        &mut self,
        activeids: &Vec<IdxParticle>,
        directions: &Vec<Direction>,
        randomlist: &Vec<BaseRandom>,
        calculator: &ColorBySpeed,
    ) {
        activeids.iter().for_each(|idx| {
            let direction = directions.get(*idx).unwrap();
            let randoms = randomlist.get(*idx).unwrap();
            let item = self.0.get_mut(*idx).unwrap();
            calculator.modify(item, direction.length, randoms);
        });
    }
}


pub struct ParticleUV(pub(crate) Vec<TextureUV>);
impl ParticleUV {
    pub fn new(maxcount: usize) -> Self {
        let mut vec = Vec::with_capacity(maxcount);
        for _ in 0..maxcount {
            vec.push(TextureUV::default());
        }
        Self(vec)
    }
    pub fn start(
        &mut self,
        newids: &Vec<IdxParticle>,
        randomlist: &Vec<BaseRandom>,
        calculator: &TextureSheet,
    ) {
        newids.iter().for_each(|idx| {
            let item = self.0.get_mut(*idx).unwrap();
            let randoms = randomlist.get(*idx).unwrap();
            calculator.modify_for_start(item, randoms);
        });
    }
    pub fn run(
        &mut self,
        ids: &Vec<IdxParticle>,
        ages: &Vec<AgeLifeTime>,
        randomlist: &Vec<BaseRandom>,
        calculator: &TextureSheet,
    ) {
        ids.iter().for_each(|idx| {
            let item = self.0.get_mut(*idx).unwrap();
            let age = ages.get(*idx).unwrap();
            let randoms = randomlist.get(*idx).unwrap();
            calculator.modify_for_over_lifetime(item, age.progress, randoms);
        });
    }
}

#[derive(Component, Default, Deref, DerefMut)]
pub struct ParticleCustomV4(pub(crate) Vec<Vector4>);
impl ParticleCustomV4 {
    pub fn new(maxcount: usize) -> Self {
        let mut vec = Vec::with_capacity(maxcount);
        for _ in 0..maxcount {
            vec.push(Vector4::zeros());
        }
        Self(vec)
    }
}

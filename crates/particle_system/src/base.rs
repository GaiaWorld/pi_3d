
use std::{ops::Sub, sync::Arc};

use bevy::prelude::Deref;
use crossbeam::queue::SegQueue;
use pi_assets::asset::{Asset, Size, Handle};
use pi_engine_shell::prelude::*;
use pi_scene_context::prelude::*;
use pi_scene_math::{*, coordiante_system::CoordinateSytem3, vector::{TToolVector3, TToolRotation, TToolMatrix}};
use pi_share::Share;
use pi_trail_renderer::{TrailPoints, TrailBase, TrailBuffer};
use rand::Rng;

use crate::{
    tools::*,
    modifier::*,
    emitter::ShapeEmitter,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet, PartialOrd, Ord)]
pub enum StageParticleSystem {
    ParticleSysCommand,
    ParticleSysEmission,
    ParticleSysParamStart,
    ParticleSysParamOverLifetime,
    ParticleSysDirection,
    ParticleSysParamBySpeed,
    ParticleSysMatrix,
    ParticleSysUpdate,
}

pub type IdxParticle = usize;

// pub type TCurveTime = Number;
pub type TCurveValue = Number;
pub type TCurveInTangent = Number;
pub type TCurveOutTangent = Number;
pub type TCurveScalar = Number;

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

#[derive(Resource, Default)]
pub struct ParticleSystemPerformance {
    pub sys_ids: u32,
    pub sys_emission: u32,
    pub sys_emitter: u32,
    pub sys_start_lifetime: u32,
    pub sys_start_size: u32,
    pub sys_start_rotation: u32,
    pub sys_start_color: u32,
    pub sys_start_texture_sheet: u32,
    pub sys_size_over_life_time: u32,
    pub sys_color_over_life_time: u32,
    pub sys_rotation_over_life_time: u32,
    pub sys_velocity_over_life_time: u32,
    pub sys_orbit_over_life_time: u32,
    pub sys_speed_modifier_over_life_time: u32,
    pub sys_limit_velocity_over_life_time: u32,
    pub sys_texturesheet: u32,
    pub sys_direction: u32,
    pub sys_color_by_speed: u32,
    pub sys_size_by_speed: u32,
    pub sys_rotation_by_speed: u32,
    pub sys_emitmatrix: u32,
    pub sys_update_buffer: u32,
    pub sys_update_buffer_trail: u32,
    pub particles: u32,
    pub maxparticles: u32,
}
impl ParticleSystemPerformance {
    pub fn total(&self) -> u32 {
        self.sys_ids
        + self.sys_emission
        + self.sys_emitter
        + self.sys_start_lifetime
        + self.sys_start_size
        + self.sys_start_rotation
        + self.sys_start_color
        + self.sys_start_texture_sheet
        + self.sys_size_over_life_time
        + self.sys_color_over_life_time
        + self.sys_rotation_over_life_time
        + self.sys_velocity_over_life_time
        + self.sys_orbit_over_life_time
        + self.sys_speed_modifier_over_life_time
        + self.sys_limit_velocity_over_life_time
        + self.sys_texturesheet
        + self.sys_direction
        + self.sys_color_by_speed
        + self.sys_size_by_speed
        + self.sys_rotation_by_speed
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
        AssetCapacity { flag: false, min: 100 * 1024, max: 200 * 1024, timeout: 10 * 1000 }
    }

    const ASSET_TYPE: &'static str = "ParticleSystemCalculator";
}

pub struct ParticleSystemCalculatorBufferMatrix {
    // pub 
}


#[derive(Component)]
pub struct ParticleCalculatorEmission {
    pub(crate) bursts: Vec<TBurstData>,
    pub(crate) rateovertime: FloatInterpolation,
}

#[derive(Component, Deref)]
pub struct ParticleCalculatorShapeEmitter(pub(crate) ShapeEmitter);
#[derive(Component)]
pub struct ParticleCalculatorStartLifetime(pub(crate) FloatInterpolation);
#[derive(Component)]
pub struct ParticleCalculatorStartColor(pub(crate) StartColor);
#[derive(Component)]
pub struct ParticleCalculatorStartSpeed(pub(crate) FloatInterpolation);
#[derive(Component)]
pub struct ParticleCalculatorStartSize(pub(crate) StartSize);

#[derive(Component)]
pub struct ParticleCalculatorGravity(pub(crate) FloatInterpolation);

#[derive(Component)]
pub struct ParticleCalculatorStartRotation(pub(crate) StartRotation);

#[derive(Component)]
pub struct ParticleCalculatorVelocityOverLifetime(pub(crate) VelocityOverLifetime);
#[derive(Component)]
pub struct ParticleCalculatorLimitVelocityOverLifetime(pub(crate) LimitVelocityOverLifetime);
#[derive(Component)]
pub struct ParticleCalculatorForceOverLifetime(pub(crate) ForceOverLifetime);
#[derive(Component)]
pub struct ParticleCalculatorSpeedModifier(pub(crate) SpeedModifier);
#[derive(Component)]
pub struct ParticleCalculatorOrbitRadial(pub(crate) FloatInterpolation);
#[derive(Component)]
pub struct ParticleCalculatorOrbitOffset(pub(crate) TranslationInterpolate);
#[derive(Component)]
pub struct ParticleCalculatorOrbitVelocity(pub(crate) TranslationInterpolate);

#[derive(Component)]
pub struct ParticleCalculatorSizeOverLifetime(pub(crate) SizeOverLifetime);
#[derive(Component)]
pub struct ParticleCalculatorSizeBySpeed(pub(crate) SizeBySpeed);

#[derive(Component)]
pub struct ParticleCalculatorRotationOverLifetime(pub(crate) RotationOverLifetime);
#[derive(Component)]
pub struct ParticleCalculatorRotationBySpeed(pub(crate) RotationBySpeed);

#[derive(Component)]
pub struct ParticleCalculatorCustomV4 {
    pub(crate) x: FloatInterpolation,
    pub(crate) y: FloatInterpolation,
    pub(crate) z: FloatInterpolation,
    pub(crate) w: FloatInterpolation,
}

#[derive(Component)]
pub struct ParticleCalculatorColorOverLifetime(pub(crate) ColorOverLifetime);
#[derive(Component)]
pub struct ParticleCalculatorColorBySpeed(pub(crate) ColorBySpeed);

#[derive(Component)]
pub struct ParticleCalculatorTextureSheet(pub(crate) TextureSheet);

#[derive(Component)]
pub struct ParticleCalculatorTrail(pub(crate) TrailModifier);


#[derive(Component)]
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

#[derive(Resource, Deref, DerefMut)]
pub struct ResParticleCommonBuffer(pub Option<Arc<NotUpdatableBufferRange>>);
impl TAssetCapacity for ResParticleCommonBuffer {
    const ASSET_TYPE: &'static str = "PARTICLE_COMMON_BUFFER";
    fn capacity() -> AssetCapacity {
        AssetCapacity { flag: false, min: 1024 * 1024, max: 1024 * 1024, timeout: 1000 }
    }
}
impl ResParticleCommonBuffer {
    pub fn new(
        maxbytes: u32, 
        allocator: &mut VertexBufferAllocator,
        device: &RenderDevice,
        queue: &RenderQueue,
    ) -> Self {
        let size = maxbytes;
        let mut data = Vec::with_capacity(size as usize);
        for _ in 0..size {
            data.push(0);
        }

        let buffer = allocator.create_not_updatable_buffer_pre(device, queue, &data, None);
        Self(buffer)
    }
    pub fn f32_count(&self) -> usize {
        if let Some(item) = &self.0 {
            item.size() as usize / 4 
        } else {
            0
        }
    }
    pub fn buffer(&self, start: u32, end: u32) -> EVerticesBufferUsage {
        EVerticesBufferUsage::EVBRange(Arc::new(EVertexBufferRange::NotUpdatable(self.0.as_ref().unwrap().clone(), start, end)))
    }
    pub fn update(&self, data: &[u8], queue: &RenderQueue) -> bool {
        if let Some(item) = &self.0 {
            if data.len() as u32 <= item.size()  {
                queue.write_buffer(item.buffer(), 0, data);
                return false;
            } else {
                return true;
            }
        } else {
            return false;
        }
    }
}

#[derive(Resource)]
pub struct ResParticleTrailBuffer(pub Option<TrailBuffer>);
impl TAssetCapacity for ResParticleTrailBuffer {
    const ASSET_TYPE: &'static str = "PARTICLE_TRAIL_BUFFER";
    fn capacity() -> AssetCapacity {
        AssetCapacity { flag: false, min: 1024 * 1024, max: 1024 * 1024, timeout: 1000 }
    }
}

#[derive(Component)]
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

#[derive(Component)]
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
        worldmatrixs: &Vec<EmitMatrix>,
        directions: &Vec<Direction>,
        trailmodifier: &TrailModifier,
    ) {
        let mut color = Vector4::new(1., 1., 1., 1.);
        let mut localscaling = Vector3::new(1., 0., 0.);
        let trailworldspace = trailmodifier.use_world_space;
        newids.iter().for_each(|idx| {
            let randoms = randomlist.get(*idx).unwrap();
            let particlecolor = colors.get(*idx).unwrap();
            let agecontrol = self.lifetime.get(*idx).unwrap();

            let direction = directions.get(*idx).unwrap();
            let translation = localpositions.get(*idx).unwrap() + direction.value.scale(-1. * 0.0000001 / f32::max(direction.length, 1.));
            let scaling = localscalings.get(*idx).unwrap();
            let eulers = localrotations.get(*idx).unwrap();
            let mut localmatrix = Matrix::identity();
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
                CoordinateSytem3::transform_normal(&Vector3::new(1., 0., 0.), &localmatrix, &mut localscaling);
                let len = CoordinateSytem3::length(&localscaling);
                if len < 0.00000001 { 0. } else { 1. / len }
            };

            // log::warn!("Trail: {:?}, {:?}", age, trailworldspace);
            let _ = item.run(
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
        worldmatrixs: &Vec<EmitMatrix>,
        time: &ParticleSystemTime,
        trailmodifier: &TrailModifier,
        trailbuffer: &mut TrailBuffer,
    ) -> (u32, u32) {
        let mut color = Vector4::new(1., 1., 1., 1.);
        let basesize = Vector3::new(0.5773502691896257 as f32, 0.5773502691896257 as f32, 0.5773502691896257 as f32);
        let mut localscaling = Vector3::new(1., 0., 0.);
        let trailworldspace = trailmodifier.use_world_space;
        let mut start = u32::MAX;
        let mut end = 0;
        activeids.iter().for_each(|idx| {
            let randoms = randomlist.get(*idx).unwrap();
            let particlecolor = colors.get(*idx).unwrap();
            let agecontrol = self.lifetime.get(*idx).unwrap();

            let translation = localpositions.get(*idx).unwrap();
            let scaling = localscalings.get(*idx).unwrap();
            let eulers = localrotations.get(*idx).unwrap();
            let mut localmatrix = Matrix::identity();
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
                CoordinateSytem3::transform_normal(&basesize, &localmatrix, &mut localscaling);
                let len = CoordinateSytem3::length(&localscaling);
                if len < 0.00000001 { 0. } else { 1. / len }
            };

            // log::warn!("Trail: {:?}, {:?}", age, trailworldspace);
            let flag = item.run(
                &worldmatrix, &localmatrix, 
                &color, &trailmodifier.color_over_lifetime.color4_interpolate.gradient, &trailmodifier.color_over_trail.color4_interpolate.gradient,
                width, &trailmodifier.width_over_trail, *agecontrol, &age, randoms, 9999999., trailmodifier.minimun_vertex_distance,
                trailworldspace
            );

            // log::warn!("Trail: {:?}, {:?}", age, flag);
            if flag {
                let (istart, iend) = trailbuffer.collect(&item, trailworldspace,  parentmatrix);
                start = istart.min(start);
                end = iend.max(end);
            }
        });

        (start, end)
    }
}

#[derive(Component)]
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

#[derive(Component)]
pub struct ParticleActive(pub bool);

#[derive(Component)]
pub struct ParticleState {
    pub(crate) start: bool,
    pub(crate) playing: bool,
}

/// 存活的粒子ID列表
#[derive(Component)]
pub struct ParticleIDs {
    pub(crate) calculator: Handle<ParticleSystemCalculatorID>,
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
            calculator,
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
}

/// 粒子系统
#[derive(Component)]
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
    pub fn new() -> Self {
        Self {
            time_scale: 1.,
            last_running_timems: 0,
            running_delta_ms: 0,
            total_ms: 0,
            total_running_ms: 0,
            half_frame_time_ms: 0,
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
                if self.delay_ms + self.last_running_timems + self.half_frame_time_ms <= self.total_ms {
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

#[derive(Component, Deref, DerefMut)]
pub struct ParticleRandom(pub(crate) Random);
impl ParticleRandom {
    pub fn new(seed: u64) -> Self {
        Self(Random::new(seed))
    }
}

#[derive(Component, Deref)]
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
#[derive(Component)]
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
                        let needloopcount = (duration as usize - busrt_time) / busrt_interval;
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
                    let needloopcount = (loop_progress_time as usize - busrt_time) / busrt_interval;
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
        ids.create_new(newcount);
    }
}

#[derive(Component, Deref)]
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

#[derive(Component, Deref)]
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

#[derive(Component)]
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

// #[derive(Component)]
// pub struct ParticleStartRotation(pub(crate) Vec<Vector3>);
// impl ParticleStartRotation {
//     pub fn new(maxcount: usize) -> Self {
//         let mut vec = Vec::with_capacity(maxcount);
//         for _ in 0..maxcount {
//             vec.push(Vector3::new(0., 0., 0.));
//         }
//         Self(vec)
//     }
//     pub fn start(
//         &mut self,
//         newids: &Vec<IdxParticle>,
//         randomlist: &Vec<BaseRandom>,
//         time: &ParticleSystemTime,
//         start_angle_interpolation: &StartRotation,
//     ) {
//         newids.iter().for_each(|idx| {
//             let item = self.0.get_mut(*idx).unwrap();
//             let randoms = randomlist.get(*idx).unwrap();
//             start_angle_interpolation.modify(item, time.loop_progress, randoms);
//         });
//     }
// }

#[derive(Component, Deref, DerefMut)]
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
        emitter: &ShapeEmitter,
        startspeed: &ParticleCalculatorStartSpeed,
    ) {
        let emission_total = newids.len() as f32;
        let mut emission_index = 0;
        newids.iter().for_each(|idx| {
            let position_to_update = self.0.get_mut(*idx).unwrap();
            let direction_to_update = directions.get_mut(*idx).unwrap();
            let randoms = randomlist.get(*idx).unwrap();
            let mut random = Random::new(randoms.seed);

            emitter.start_position_function(position_to_update, time.emission_loop as f32, time.emission_progress, emission_index as f32, emission_total, &mut random);

            let local_position = &position_to_update;
            emitter.start_direction_function(&mut direction_to_update.velocity_start, local_position, &mut random);

            let startspeed = startspeed.0.interpolate(time.emission_progress, randoms.base);
            direction_to_update.velocity_start.scale_mut(startspeed);
            direction_to_update.value.copy_from(&direction_to_update.velocity_start);

            // log::warn!("StartPosition: {:?}, Direction: {:?}", position_to_update, direction_to_update.velocity_start);

            emission_index += 1;
        });
    }
    // pub fn run(
    //     &mut self,
    //     ids: &Vec<IdxParticle>,
    // ) {

    // }
}

#[derive(Component, Deref)]
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

#[derive(Component, Deref, DerefMut)]
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
        randomlist: &Vec<BaseRandom>,
        calculator: &SizeOverLifetime,
    ) {
        activeids.iter().for_each(|idx| {
            let age = ages.get(*idx).unwrap();
            let randoms = randomlist.get(*idx).unwrap();
            let item = self.0.get_mut(*idx).unwrap();
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

/// 粒子实时颜色
#[derive(Component, Deref, DerefMut)]
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

/// 发射时的全局矩阵 - 在 World 空间发射在发射时即固定, 在 Local 空间发射实时更新为发射器全局矩阵
#[derive(Component, Deref)]
pub struct ParticleEmitMatrix(pub(crate) Vec<EmitMatrix>);
impl ParticleEmitMatrix {
    pub fn new(maxcount: usize) -> Self {
        let mut vec = Vec::with_capacity(maxcount);
        for _ in 0..maxcount {
            vec.push(EmitMatrix::default());
        }
        Self(vec)
    }
    pub fn emit(
        &mut self,
        newids: &Vec<IdxParticle>,
        ids: &Vec<IdxParticle>,
        simulation: &EParticleSimulationSpace,
        scalingmode: &EScalingMode,
        _world_matrix: &Matrix,
        _world_matrix_inv: &Matrix,
        _iso: &Isometry3,
        _global_position: &Vector3,
        global_rotation: &Rotation3,
        global_scaling: &Vector3,
        local_scaling: &Vector3,
    ) {
        // log::warn!("EmitMatrix:");
        let mut scaling = global_scaling.clone();
        let mut emitscaling = global_scaling.clone();
        // let mut pose_invert = if let Some(temp) = world_matrix.append_translation(&global_position.scale(-1.)).try_inverse() {
        //     temp
        // } else { Matrix::identity() };

        let (emittermatrix, emittermatrix_invert) = match scalingmode {
            EScalingMode::Hierarchy => {
                (_world_matrix.clone(), _world_matrix_inv.clone())
            },
            EScalingMode::Local => {
                scaling.copy_from(local_scaling);
                emitscaling.copy_from(local_scaling);

                let mut emittermatrix = _iso.to_matrix(); // Matrix::identity();
                emittermatrix.append_nonuniform_scaling_mut(&emitscaling);
                // CoordinateSytem3::matrix4_compose_rotation(&emitscaling, &global_rotation, &global_position, &mut emittermatrix);
                let emittermatrix_invert = if let Some(temp) = emittermatrix.try_inverse() {
                    temp
                } else { Matrix::identity() };
                (emittermatrix, emittermatrix_invert)
            },
            EScalingMode::Shape => {
                scaling.copy_from_slice(&[1., 1., 1.]);

                let mut emittermatrix = _iso.to_matrix(); // Matrix::identity();
                emittermatrix.append_nonuniform_scaling_mut(&emitscaling);
                // CoordinateSytem3::matrix4_compose_rotation(&emitscaling, &global_rotation, &global_position, &mut emittermatrix);
                let emittermatrix_invert = if let Some(temp) = emittermatrix.try_inverse() {
                    temp
                } else { Matrix::identity() };
                (emittermatrix, emittermatrix_invert)
            },
        };

        match simulation {
            EParticleSimulationSpace::Local => {
                ids.iter().for_each(|idx| {
                    let item = self.0.get_mut(*idx).unwrap();
                    // item.position.clone_from(&global_position);
                    item.scaling.clone_from(&scaling);
                    item.rotation.clone_from(&global_rotation);
                    item.matrix.clone_from(&emittermatrix);
                    item.matrix_invert.clone_from(&emittermatrix_invert);
                });
            },
            EParticleSimulationSpace::World => {
                newids.iter().for_each(|idx| {
                    let item = self.0.get_mut(*idx).unwrap();
                    // item.position.clone_from(&global_position);
                    item.scaling.clone_from(&scaling);
                    item.rotation.clone_from(&global_rotation);
                    item.matrix.clone_from(&emittermatrix);
                    item.matrix_invert.clone_from(&emittermatrix_invert);
                });
            },
        }
        // log::warn!("EmitMatrix: End");
    }
}

/// 粒子局部重力影响
#[derive(Component, Deref)]
pub struct ParticleGravityFactor(pub(crate) Vec<GravityFactor>);
impl ParticleGravityFactor {
    pub fn new(maxcount: usize) -> Self {
        let mut vec = Vec::with_capacity(maxcount);
        for _ in 0..maxcount {
            vec.push(GravityFactor::default());
        }
        Self(vec)
    }
    pub fn run(
        &mut self,
        ids: &Vec<IdxParticle>,
        ages: &Vec<AgeLifeTime>,
        emitmatrixs: &Vec<EmitMatrix>,
        randomlist: &Vec<BaseRandom>,
        time: &ParticleSystemTime,
        gravity: &Vector3,
        calculator: &Gravity,
    ) {
        let delta_seconds = time.running_delta_ms as f32 / 1000.0;
        ids.iter().for_each(|idx| {
            let item = self.0.get_mut(*idx).unwrap();
            let randoms = randomlist.get(*idx).unwrap();
            let age = ages.get(*idx).unwrap();
            let emitmatrix = emitmatrixs.get(*idx).unwrap();
            
            let mut factor = 0.;
            calculator.modify(&mut factor, age.progress, delta_seconds, randoms);

            CoordinateSytem3::transform_normal(&gravity.scale(factor), &emitmatrix.matrix_invert, &mut item.value);
        });
    }
}

/// 粒子局部外力影响
#[derive(Component, Deref)]
pub struct ParticleForce(pub(crate) Vec<Force>);
impl ParticleForce {
    pub fn new(maxcount: usize) -> Self {
        let mut vec = Vec::with_capacity(maxcount);
        for _ in 0..maxcount {
            vec.push(Force::default());
        }
        Self(vec)
    }
    pub fn run(
        &mut self,
        ids: &Vec<IdxParticle>,
        ages: &Vec<AgeLifeTime>,
        emitmatrixs: &Vec<EmitMatrix>,
        randomlist: &Vec<BaseRandom>,
        time: &ParticleSystemTime,
        calculator: &ForceOverLifetime,
    ) {
        let delta_seconds = time.running_delta_ms as f32 / 1000.0;
        ids.iter().for_each(|idx| {
            let item = self.0.get_mut(*idx).unwrap();
            let randoms = randomlist.get(*idx).unwrap();
            let age = ages.get(*idx).unwrap();
            let emitmatrix = emitmatrixs.get(*idx).unwrap();

            calculator.modify(item, age.progress, delta_seconds, randoms);

            if calculator.is_local_space == false {
                CoordinateSytem3::transform_normal(&item.value.clone(), &emitmatrix.matrix_invert, &mut item.value);
            }
        });
    }
}

/// 粒子局部速度向量
#[derive(Component, Deref)]
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
#[derive(Component, Deref)]
pub struct ParticleSpeedFactor(pub(crate) Vec<SpeedFactor>);
impl ParticleSpeedFactor {
    pub fn new(maxcount: usize) -> Self {
        let mut vec = Vec::with_capacity(maxcount);
        for _ in 0..maxcount {
            vec.push(SpeedFactor::default());
        }
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
#[derive(Component, Deref)]
pub struct ParticleOrbitVelocity(pub(crate) Vec<OrbitVelocity>);
impl ParticleOrbitVelocity {
    pub fn new(maxcount: usize) -> Self {
        let mut vec = Vec::with_capacity(maxcount);
        for _ in 0..maxcount {
            vec.push(OrbitVelocity::default());
        }
        Self(vec)
    }
    pub fn run<T>(
        &mut self,
        ids: &Vec<IdxParticle>,
        ages: &Vec<AgeLifeTime>,
        randomlist: &Vec<BaseRandom>,
        _: &ParticleSystemTime,
        offset: Result<&ParticleCalculatorOrbitOffset, T>,
        velocity: Result<&ParticleCalculatorOrbitVelocity, T>,
        radial: Result<&ParticleCalculatorOrbitRadial, T>,
    ) {
        // let delta_seconds = time.running_delta_ms as f32 / 1000.0;
        ids.iter().for_each(|idx| {
            let item = self.0.get_mut(*idx).unwrap();
            let randoms = randomlist.get(*idx).unwrap();
            let age = ages.get(*idx).unwrap();

            if let Ok(calculator) = offset {
                calculator.0.compute(age.progress, randoms, &mut item.offset);
            }
            if let Ok(calculator) = velocity {
                calculator.0.compute(age.progress, randoms, &mut item.orbit);
                item.orbit_len = CoordinateSytem3::length_squared(&item.orbit);
            }
            if let Ok(calculator) = radial {
                item.radial = calculator.0.interpolate(age.progress, randoms.w);
            }
        });
    }
}

#[derive(Component, Deref)]
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
        forces: &Vec<Force>,
        gravities: &Vec<GravityFactor>,
        velocities: &Vec<Velocity>,
        limitscalars: &Vec<LimitVelocityScalar>,
        orbits: &Vec<OrbitVelocity>,
        speedfactors: &Vec<SpeedFactor>,
        positions: &mut Vec<Vector3>,
        emitter: &ShapeEmitter,
        time: &ParticleSystemTime,
    ) {
        // log::warn!("Direction: ");
        let delta_seconds = time.running_delta_ms as f32 / 1000.0;
        let half_delta_seconds = delta_seconds * 0.5;
        let origin = Vector3::zeros();
        ids.iter().for_each(|idx| {
            let force: &Force = forces.get(*idx).unwrap();
            let gravity: &GravityFactor = gravities.get(*idx).unwrap();
            let velocity: &Velocity = velocities.get(*idx).unwrap();
            let limitscalar: &LimitVelocityScalar = limitscalars.get(*idx).unwrap();
            let orbit: &OrbitVelocity = orbits.get(*idx).unwrap();
            let speedfactor: &SpeedFactor = speedfactors.get(*idx).unwrap();
            let direction: &mut Direction = self.0.get_mut(*idx).unwrap();
            let position = positions.get_mut(*idx).unwrap();

            // 力 -> 加速度
            let a = force.value + gravity.value; //  / 1.; // 质量为 1
            direction.velocity_force += a.scale(half_delta_seconds);

            let mut velocity = velocity.value + direction.velocity_force + direction.velocity_start;

            // 轨道速度
            let mut orbit_center: Vector3 = Vector3::zeros();
            emitter.orbit_center(&position, &orbit.offset, &mut orbit_center);
            let radial_vec: Vector3 = position.sub(&orbit_center);
            let orbit_direction = if orbit.orbit_len < 0.00000001 {
                let temp = delta_seconds * speedfactor.value;
                let orbit_rotation = CoordinateSytem3::rotation_matrix_from_euler_angles(orbit.orbit.x * temp, orbit.orbit.y * temp, orbit.orbit.z * temp);
                orbit_rotation.transform_vector(&radial_vec) - radial_vec
            }
            else {
                Vector3::zeros()
            };

            if 0.00000001 < orbit.radial.abs() {
                let radial_len = CoordinateSytem3::length(&radial_vec);
                if 0.00000001 < radial_len {
                    velocity += radial_vec.scale(1. / radial_len).scale(orbit.radial);
                };
            }

            velocity.scale_mut(speedfactor.value);

            let mut new_direction = velocity.scale( delta_seconds) + orbit_direction;
            // log::warn!("velocity: {:?}, {:?}, {:?}", velocity, new_direction, delta_seconds);

            let mut directionscalar = new_direction.metric_distance(&origin);
            if limitscalar.value < Number::MAX {
                let limitscalarval = limitscalar.value * delta_seconds;
                let delta = directionscalar - limitscalarval;
                if 0.00000001 < delta {
                    let factor = limitscalarval + (delta) * Number::exp(Number::ln(delta + 1.0) * (0. - limitscalar.dampen));
                    // let factor = 1.0 - limitscalar.dampen * (directionscalar - limitscalar.value * delta_seconds) / directionscalar * (0.66);
                    new_direction.scale_mut(factor / directionscalar);
                    directionscalar = factor;
                    // log::warn!("Limit: {:?}, {:?}, {:?}", limitscalarval, directionscalar, factor);
                }
            }

            direction.value = new_direction.scale(1. / delta_seconds);
            direction.length = directionscalar / delta_seconds;

            // log::warn!("Direction: {:?}, {:?}, {:?}", direction.value, new_direction, delta_seconds);

            *position += new_direction;
        });
        // log::warn!("Direction: End");
    }
}


#[derive(Component, Deref)]
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

#[derive(Component, Deref, DerefMut)]
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

#[derive(Component)]
pub struct ParticleGlobalPosList(pub(crate) Vec<Vec<Vector3>>);

#[derive(Component)]
pub struct ParticleLocalPosList(pub(crate) Vec<Vec<Vector3>>);

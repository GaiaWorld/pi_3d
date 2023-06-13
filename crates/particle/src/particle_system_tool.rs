use std::time::{Duration, Instant, UNIX_EPOCH};

use nalgebra::AbstractRotation;
use pi_scene_math::{
    coordiante_system::CoordinateSytem3, vector::TToolMatrix, Matrix, Quaternion, Rotation3,
    Vector3,
};
use rand::Rng;

use crate::{
    emitter::ishape_emitter_type::IShapeEmitterType,
    interpolation::{Color4Gradient, FloatInterpolation, IInterpolation},
    iparticle_system_config::EInterpolationCurveMode,
    modifier::{
        base::{Color4Interpolate, IParticleModifier, ScalingInterpolate, TranslationInterpolate},
        color_by_speed::ColorBySpeed,
        color_over_lifetime::ColorOverLifetime,
        custom_data_for_uv::CustomDataForUV,
        force_over_lifetime::ForceOverLifetime,
        gravity::Gravity,
        limit_velocity_over_lifetime::LimitVelocityOverLifetime,
        local_position::LocalPosition,
        noise::Noise,
        render_world_matrix::{
            HorizontalBillboard, RenderAlignmentFacing, RenderAlignmentLocal,
            RenderAlignmentVelocity, RenderAlignmentView, RenderAlignmentWorld, StretchedBillboard,
            TRenderWorldMatrixCompute, VerticalBillboard,
        },
        rotation_by_speed::RotationBySpeed,
        rotation_over_lifetime::RotationOverLifetime,
        size_by_speed::SizeBySpeed,
        size_over_lifetime::SizeOverLifetime,
        start_color::StartColor,
        start_rotation::StartRotation,
        start_size::StartSize,
        texture_sheet::TextureSheet,
        trail::TrailModifier,
        velocity_over_lifetime::VelocityOverLifetime,
    },
    multiply, normalize,
    particle::Particle,
    pool::Pool,
};

#[derive(Clone, Copy, PartialEq)]
pub enum EMeshParticleSpaceMode {
    Local = 0,
    /**
     * 发射在世界空间时, 父级尽量不要有旋转动画, 因为 动画 与 粒子动画的衔接有误差，无法完美适配
     */
    World = 1,
}

#[derive(Clone, Copy, PartialEq)]
pub enum EMeshParticleScaleMode {
    Hierarchy = 0,
    Local = 1,
    Shape = 2,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ERenderAlignment {
    /**
     * 粒子面向相机平面。
     */
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

#[derive(Clone, Copy)]
pub enum ERenderMode {
    Billboard = 0,
    StretchedBillboard = 1,
    HorizontalBillboard = 2,
    VerticalBillboard = 3,
    Mesh = 4,
    None = 5,
}

/**
 * 对接 Unity ParticleSystem Mesh Mode
 */
pub struct ParticleSystemTool {
    pub sqrt3: f32,

    OneVector3: Vector3,
    /**
     * 计算的间隔时间
     */
    pub computeDeltaTime: u64,
    /**
     * 最大粒子数
     */
    pub maxParticles: usize,
    // private _maxParticles: number = 100;
    // public get maxParticles() {
    //     return self._maxParticles;
    // }
    // public set maxParticles(value: number) {
    //     self._maxParticles = value + 1;
    // }
    /**
     * 是否循环
     */
    pub looping: bool,
    /**
     * 是否预热
     */
    pub prewarm: bool,
    /**
     * 粒子系统发射持续时间
     */
    pub duration: u64,
    _loopCount: f32,
    /**
     * 延时时间 - 毫秒
     */
    pub startDelay: i32,
    pub emitterShape: Option<Box<dyn IShapeEmitterType>>,
    /**
     * 一次发射循环的时间 - 毫秒
     */
    pub emissionTime: f32,
    /**
     * 当前已循环次数
     */
    pub emissionLoop: f32,
    /**
     * 当前发射循环的进度
     */
    pub emissionProgress: f32,
    /**
     * 每秒发射的粒子数目
     */
    pub rateOverTime: f32,
    /**
     * 指定时间点 开始 持续 指定时间内 发射指定数目
     * [开始时间, 发射数目, 循环次数, 间隔]
     * @tip 需要按 开始时间从小到大排序
     */
    pub bursts: Vec<[f32; 4]>,
    _burstsLoopCount: Vec<f32>,

    /**
     * 发射空间
     * * Local - 本地: 发射开始时的方向受父级影响, 发射后的位置也受父级影响
     * * World - 世界空间: 发射开始时的方向受父级影响, 发射后的位置不再受父级影响
     */
    pub simulationSpace: EMeshParticleSpaceMode,
    pub scalingSpace: EMeshParticleScaleMode,
    _renderAlignment: ERenderAlignment,
    _renderMode: ERenderMode,

    pub renderPivot: Vector3,

    pub startLifetimeInterpolation: FloatInterpolation,
    pub startSpeedInterpolation: FloatInterpolation,
    pub startSizeInterpolation: StartSize,
    pub startRotationInterpolation: StartRotation,
    pub startColorInterpolation: StartColor,

    pub gravityInterpolation: Gravity,
    /**
     * 位移速度
     * @tip 默认模式 - ConstantsUnite - XYZ 共用一个随机数
     * @tip 为 Constants | ConstantsUnite 模式应用插值时, 仅创建时使用, 意义为: 创建时速度基础上附加速度
     * @tip 为 Curves 模式时, 运行过程中使用, 意义为: 创建时速度基础上附加速度 - 每次循环累加 此次循环 与 上次循环 速度值的差值
     */
    pub velocityOverLifetimeInterpolation: VelocityOverLifetime,
    pub enableVelocityOverLifeTime: bool,

    pub limitVelocityOverLifetimeInterpolation: LimitVelocityOverLifetime,
    pub enableLimitVelocityOverLifeTime: bool,

    pub forceOverLifetimeInterpolation: ForceOverLifetime,
    pub enableForceOverLifeTime: bool,

    pub colorOverLifetimeInterpolation: ColorOverLifetime,
    pub enableColorOverLifeTime: bool,
    pub colorBySpeedInterpolation: ColorBySpeed,
    pub enableColorBySpeed: bool,
    /**
     * 缩放
     * @tip 默认模式 - ConstantsUnite - XYZ 共用一个随机数
     * @tip 为 Constants | ConstantsUnite 模式应用插值时, 仅创建时使用, 意义为: 仅对创建时大小的影响倍数
     * @tip 为 Curves 模式时, 运行过程中使用, 意义为: 创建时大小的影响倍数
     */
    pub sizeOverLifetimeInterpolation: SizeOverLifetime,
    pub enableSizeOverLifeTime: bool,
    pub sizeBySpeedInterpolation: SizeBySpeed,
    pub enableSizeBySpeed: bool,

    pub localPositionModifier: LocalPosition,
    /**
     * 旋转
     * @tip 默认模式 - ConstantsUnite - XYZ 共用一个随机数
     * @tip Unity 中单一 Angle 对应 只处理 Y 轴
     * @tip 与 Unity 编辑时不同, 此处单位为 弧度
     * @tip 为 Constants | ConstantsUnite 模式时, 仅创建时使用, 意义为: 每秒旋转速度
     * @tip 为 Curves 模式时, 运行过程中使用, 意义为: 每秒旋转速度
     */
    pub rotationOverLifetimeInterpolation: RotationOverLifetime,
    pub enableRotationOverLifeTime: bool,
    pub rotationBySpeedInterpolation: RotationBySpeed,
    pub enableRotationBySpeed: bool,
    /**
     * Texture Sheet Animation
     */
    pub textureSheetInterpolation: TextureSheet,
    pub enableTextureSheet: bool,

    pub customDataForMainUV: CustomDataForUV,
    pub enableCustomDataForMainUV: bool,

    pub noiseModifier: Noise,
    pub enableNoise: bool,

    pub stretchedVelocityScale: f32,
    pub stretchedLengthScale: f32,

    // 局部空间数据 - 每粒子
    overLifetimeModifierList: Vec<Box<dyn IParticleModifier>>,

    particleList: Vec<Particle>,
    activeParticleList: Vec<*mut Particle>,

    _particleSystemAge: u64,

    _lastTime: u64,
    _deltaTime: f32,
    _lastCreateTime: u64,

    _isPlaying: bool,
    _isComputeable: bool,

    //  scene: pubScene;
    pub trail: Option<TrailModifier>,
    _enableTrail: bool,
    _trailLifetimeScale: f32,

    pub particleWorldMatrixCompute: Box<TRenderWorldMatrixCompute>,

    pub getCameraRotationMatrixInvert: Box<dyn Fn() -> Matrix>,
    pub getCameraPosition: Box<dyn Fn() -> Vector3>,
    pub getParentWorldMatrix: Box<dyn Fn() -> Matrix>,
    pub getWorldMatrix: Box<dyn Fn() -> Matrix>,
    pub getLocalMatrix: Box<dyn Fn() -> Matrix>,
    pub getCameraMatrix: Box<dyn Fn() -> Matrix>,

    _isDisposed: bool,

    _mpMatrixList: Option<Vec<f32>>,
    _mpColorData: Option<Vec<f32>>,
    _mpUVData: Option<Vec<f32>>,

    pub colorData: Option<Vec<f32>>,
    pub uvData: Option<Vec<f32>>,

    pool: Pool,
    _maxId: usize,
    pub TempVector3Scaling: Vector3,
    pub TempVector3Translation: Vector3,
    pub TempVector3Rotation: Vector3,
    pub TempQuaternion_0: Quaternion,
    pub TempMatrix_1: Matrix,

    _start_p_s: Vector3,
    _start_p_r: Quaternion,
    _start_p_p: Vector3,

    _start_l_s: Vector3,
    _start_l_r: Quaternion,
    _start_l_p: Vector3,

    _maxLifetimeOverage: u64,
    _stepTime: f32,
}

impl ParticleSystemTool {
    pub fn set_renderAlignment(&mut self, value: ERenderAlignment) {
        self._renderAlignment = value;
        self._updateParticleWorldMatrixCompute();
    }
    pub fn get_renderAlignment(&self) -> ERenderAlignment {
        return self._renderAlignment.clone();
    }

    pub fn set_renderMode(&mut self, value: ERenderMode) {
        self._renderMode = value;
        self._updateParticleWorldMatrixCompute();
    }
    pub fn get_renderMode(&mut self) -> ERenderMode {
        return self._renderMode;
    }

    pub fn set_enableTrail(&mut self, value: bool) {
        self._enableTrail = value;
        if let Some(v) = &mut self.trail {
            v._enabled = value;
        }
        // Trail
        self._trailLifetimeScale = 0.;
        if value {
            if let Some(v) = &mut self.trail {
                if !v.dieWithParticle {
                    self._trailLifetimeScale = 1.;
                }
            }
        }
    }
    pub fn get_enableTrail(&self) -> bool {
        return self._enableTrail;
    }

    pub fn new() -> Self {
        Self {
            sqrt3: 3.0f32.sqrt(),
            OneVector3: Vector3::new(1., 1., 1.),
            computeDeltaTime: 15,
            maxParticles: 100,
            looping: true,
            prewarm: true,
            duration: 5,
            _loopCount: 0.,
            startDelay: 0,
            emitterShape: None,
            emissionTime: 1000.,
            emissionLoop: 0.,
            emissionProgress: 0.,
            rateOverTime: 10.,
            bursts: vec![],
            _burstsLoopCount: vec![],
            simulationSpace: EMeshParticleSpaceMode::Local,
            scalingSpace: EMeshParticleScaleMode::Hierarchy,
            _renderAlignment: ERenderAlignment::Local,
            _renderMode: ERenderMode::Billboard,
            renderPivot: Vector3::zeros(),
            startLifetimeInterpolation: FloatInterpolation::default(),
            startSpeedInterpolation: FloatInterpolation::default(),
            startSizeInterpolation: StartSize::default(),
            startRotationInterpolation: StartRotation::new(
                FloatInterpolation::default(),
                FloatInterpolation::default(),
                FloatInterpolation::default(),
            ),
            startColorInterpolation: StartColor::new(Color4Interpolate::new(
                Color4Gradient::default(),
            )),
            gravityInterpolation: Gravity::new(FloatInterpolation::new()),
            velocityOverLifetimeInterpolation: VelocityOverLifetime::new(
                FloatInterpolation::default(),
                FloatInterpolation::default(),
                FloatInterpolation::default(),
            ),
            enableVelocityOverLifeTime: false,
            limitVelocityOverLifetimeInterpolation: LimitVelocityOverLifetime::new(
                FloatInterpolation::default(),
            ),
            enableLimitVelocityOverLifeTime: false,
            forceOverLifetimeInterpolation: ForceOverLifetime::new(TranslationInterpolate::new(
                FloatInterpolation::default(),
                FloatInterpolation::default(),
                FloatInterpolation::default(),
            )),
            enableForceOverLifeTime: false,
            colorOverLifetimeInterpolation: ColorOverLifetime::new(Color4Interpolate::new(
                Color4Gradient::default(),
            )),
            enableColorOverLifeTime: false,
            colorBySpeedInterpolation: ColorBySpeed::new(Color4Interpolate::new(
                Color4Gradient::default(),
            )),
            enableColorBySpeed: false,
            sizeOverLifetimeInterpolation: SizeOverLifetime::new(
                FloatInterpolation::default(),
                FloatInterpolation::default(),
                FloatInterpolation::default(),
            ),
            enableSizeOverLifeTime: false,
            sizeBySpeedInterpolation: SizeBySpeed::new(
                FloatInterpolation::default(),
                FloatInterpolation::default(),
                FloatInterpolation::default(),
            ),
            enableSizeBySpeed: false,
            localPositionModifier: LocalPosition::new(),
            rotationOverLifetimeInterpolation: RotationOverLifetime::new(
                FloatInterpolation::default(),
                FloatInterpolation::default(),
                FloatInterpolation::default(),
            ),
            enableRotationOverLifeTime: false,
            rotationBySpeedInterpolation: RotationBySpeed::new(
                FloatInterpolation::default(),
                FloatInterpolation::default(),
                FloatInterpolation::default(),
            ),
            enableRotationBySpeed: false,
            textureSheetInterpolation: TextureSheet::new(
                FloatInterpolation::default(),
                FloatInterpolation::default(),
            ),
            enableTextureSheet: false,
            customDataForMainUV: CustomDataForUV::new(),
            enableCustomDataForMainUV: false,
            noiseModifier: Noise::new(),
            enableNoise: false,
            stretchedVelocityScale: 0.,
            stretchedLengthScale: 1.,
            overLifetimeModifierList: vec![],
            particleList: vec![],
            activeParticleList: vec![],
            _particleSystemAge: 0,
            _lastTime: 0,
            _deltaTime: 0.,
            _lastCreateTime: 0,
            _isPlaying: false,
            _isComputeable: false,
            trail: Some(TrailModifier::new()),
            _enableTrail: false,
            _trailLifetimeScale: 0.,
            particleWorldMatrixCompute: Box::new(RenderAlignmentLocal::compute),
            getCameraRotationMatrixInvert: Box::new(|| Matrix::identity()),
            getCameraPosition: Box::new(|| Vector3::zeros()),
            getParentWorldMatrix: Box::new(|| Matrix::identity()),
            getWorldMatrix: Box::new(|| Matrix::identity()),
            getLocalMatrix: Box::new(|| Matrix::identity()),
            getCameraMatrix: Box::new(|| Matrix::identity()),
            _isDisposed: false,
            _mpMatrixList: None,
            _mpColorData: None,
            _mpUVData: None,
            pool: Pool::new(),
            _maxId: 0,
            TempVector3Scaling: Vector3::zeros(),
            TempVector3Translation: Vector3::zeros(),
            TempVector3Rotation: Vector3::zeros(),
            TempQuaternion_0: Quaternion::identity(),
            TempMatrix_1: Matrix::identity(),
            _start_p_s: Vector3::zeros(),
            _start_p_r: Quaternion::identity(),
            _start_p_p: Vector3::zeros(),
            _start_l_s: Vector3::zeros(),
            _start_l_r: Quaternion::identity(),
            _start_l_p: Vector3::zeros(),
            _maxLifetimeOverage: 0,
            _stepTime: 16.0,
            colorData: None,
            uvData: None,
        }
    }

    pub fn dispose(&mut self) {
        if (self._isDisposed) {
            return;
        }
        self.stop();

        if let Some(trail) = self.trail.take() {
            trail.dispose()
        }

        self._mpMatrixList = None;
        self._mpColorData = None;

        self.startLifetimeInterpolation.dispose();
        self.startSpeedInterpolation.dispose();

        self.bursts.clear(); // self.bursts                 = undefined;
        self._isDisposed = true;

        self.activeParticleList.clear();

        let len = self.particleList.len();
        for i in (0..=len - 1).rev() {
            self.pool.recycleParticle(self.particleList[i].clone());
        }
        self.particleList.clear();

        return;
    }

    pub fn get_mpMatrixList(&self) -> Option<Vec<f32>> {
        return self._mpMatrixList.clone();
    }
    pub fn get_mpColorData(&self) -> Option<Vec<f32>> {
        return self._mpColorData.clone();
    }
    pub fn get_mpUVData(&self) -> Option<Vec<f32>> {
        return self._mpUVData.clone();
    }

    pub fn build(&mut self) {
        self._mpMatrixList = Some(vec![0.; 16 * self.maxParticles]);
        self._mpColorData = Some(vec![0.; 4 * self.maxParticles]);
        self._mpUVData = Some(vec![0.; 4 * self.maxParticles]);
        self.colorData = Some(vec![0.; 4 * self.maxParticles]);
        self.uvData = Some(vec![0.; 4 * self.maxParticles]);
        // self._mpUVSheetData = new Float32Array(4 * self.maxParticles);

        for i in self._maxId..self.maxParticles {
            let r = self.pool.createParticle();
            // println!("r.pos: {:?}", r.position);
            self.particleList.push(self.pool.createParticle());
            // pru
        }
        self._maxId = self.maxParticles;
    }

    pub fn start(&mut self) {
        if !self._isPlaying {
            if (self.startDelay > 0 && !self.prewarm) {
                // TODO
                // setTimeout(self._startCall, self.startDelay);
                // self._startCall();
            } else {
                self._startCall();
            }
        }
    }

    fn _startCall(&mut self) {
        println!("=========== _startCall1");
        if !self._isDisposed {
            // println!("=========== _startCall2");
            let now = std::time::SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis();
            // println!("time nowu128: {}, u64: {}", now, now as u64);
            let now = now as u64;
            self._isPlaying = true;
            self._isComputeable = true;
            self._particleSystemAge = 0;
            self._lastTime = now;
            self._lastCreateTime = self._lastTime;
            // println!("=========== _startCall3! now: {}", now);
            let count = self.particleList.len();
            for i in 0..count {
                self.particleList[i].reset();
                // println!("r.pos{:p}: {:?}", &self.particleList[i], self.particleList[i].position)
            }
            // println!("=========== _startCall3, self.prewarm: {}", self.prewarm);
            if self.prewarm {
                // println!("=========== _startCall4");
                let deltaTime = self.computeDeltaTime;
                let counter = self.duration / deltaTime;
                // now.checked_sub(duration)
                self._lastTime = now - self.duration;
                self._lastCreateTime = self._lastTime;

                let mut tempCurr = now - self.duration;

                for i in 1..counter as usize {
                    tempCurr = tempCurr + deltaTime;
                    // println!("=========== _startCall5, tempCurr: {}", tempCurr);
                    for j in 0..count {
                        // self.particleList[j].reset();
                        // println!("count: {:?}, j: {}", count, j);
                        // println!("r.pos{}: {:?}",i, self.particleList[j].position);
                        // println!("r.pos{:p}: {:?}", &self.particleList[j], self.particleList[j].position)
                    }
                    self.varCompute(tempCurr, Matrix::identity(), Matrix::identity());
                }
            }

            if let Some(a) = &mut self.trail {
                a._enabled = self._enableTrail
            }
        }
    }

    pub fn stop(&mut self) {
        self._isComputeable = false;

        if (!self._isDisposed) {
            self._stop();
        }

        self._isPlaying = false;
    }

    fn _stop(&mut self) {
        if (self._isPlaying) {
            self._burstsLoopCount.clear();
        }

        self._isPlaying = false;
        if let Some(a) = &mut self.trail {
            a.set_enable(false);
        }
        // self.trail.enable = false;
    }

    pub fn mpUpdate(
        &mut self,
        world_matrix: Matrix,
        local_matrix: Matrix,
        camera_pos: Vector3,
        camera_rotation_matrix_invert: Matrix,
    ) -> f32 {
        return self._update(
            world_matrix,
            local_matrix,
            camera_pos,
            camera_rotation_matrix_invert,
        );
    }

    fn _updateParticleWorldMatrixCompute(&mut self) {
        match (self.get_renderMode()) {
            ERenderMode::StretchedBillboard => {
                self.particleWorldMatrixCompute = Box::new(StretchedBillboard::compute);
            }
            ERenderMode::HorizontalBillboard => {
                self.particleWorldMatrixCompute = Box::new(HorizontalBillboard::compute);
            }
            ERenderMode::VerticalBillboard => {
                self.particleWorldMatrixCompute = Box::new(VerticalBillboard::compute);
            }
            _ => match (self.get_renderAlignment()) {
                ERenderAlignment::View => {
                    self.particleWorldMatrixCompute = Box::new(RenderAlignmentView::compute);
                }
                ERenderAlignment::World => {
                    self.particleWorldMatrixCompute = Box::new(RenderAlignmentWorld::compute);
                }
                ERenderAlignment::Facing => {
                    self.particleWorldMatrixCompute = Box::new(RenderAlignmentFacing::compute);
                }
                ERenderAlignment::Velocity => {
                    self.particleWorldMatrixCompute = Box::new(RenderAlignmentVelocity::compute);
                }
                ERenderAlignment::Local => {
                    self.particleWorldMatrixCompute = Box::new(RenderAlignmentLocal::compute);
                }
            },
        }
    }

    pub fn get_maxLifetimeOverage(&self) -> u64 {
        return self._maxLifetimeOverage;
    }

    fn _formatStartInfo(
        &mut self,
        parentWM: Option<Matrix>,
        localWM: Matrix,
        resultStartMatrix: &mut Matrix,
    ) {
        // 分解出 TRS
        if let Some(matrix) = parentWM.as_ref() {
            // parentWM.decompose(self._start_p_s, self._start_p_r, self._start_p_p);
            // println!("matrix: {:?}", matrix);
            let mut rotation = Rotation3::identity();
            CoordinateSytem3::matrix4_decompose_rotation(
                &matrix,
                Some(&mut self._start_p_s),
                Some(&mut rotation),
                Some(&mut self._start_p_p),
            );
            let rotation = rotation.euler_angles();
            self._start_p_r = Quaternion::from_euler_angles(rotation.0, rotation.1, rotation.2);
        } else {
            self._start_p_p[0] = 0.;
            self._start_p_p[1] = 0.;
            self._start_p_p[2] = 0.;

            self._start_p_s[0] = 1.;
            self._start_p_s[1] = 1.;
            self._start_p_s[2] = 1.;
            use nalgebra::Quaternion as MQuaternion;
            self._start_p_r = Quaternion::from_quaternion(MQuaternion::new(0., 0., 0., 1.));
        }
        let mut rotation = Rotation3::identity();
        CoordinateSytem3::matrix4_decompose_rotation(
            &localWM,
            Some(&mut self._start_l_s),
            Some(&mut rotation),
            Some(&mut self._start_l_p),
        );
        let rotation = rotation.euler_angles();
        self._start_l_r = Quaternion::from_euler_angles(rotation.0, rotation.1, rotation.2);

        // localWM.decompose(self._start_l_s, self._start_l_r, self._start_l_p);

        // if (self.renderAlignment == ERenderAlignment.View) {
        //     self._start_p_r.copyFromFloats(0, 0, 0, 1);
        //     self._start_l_r.copyFromFloats(0, 0, 0, 1);
        // }
        // 忽略粒子节点上的本地旋转 - 仅发射后的位移受此影响
        if let ERenderAlignment::World = self._renderAlignment {
            self._start_l_r = Quaternion::default();
        } else {
            //
        }
        if let EMeshParticleScaleMode::Hierarchy = self.scalingSpace {
            //
        } else if (self.scalingSpace == EMeshParticleScaleMode::Local) {
            self._start_p_s = Vector3::new(1., 1., 1.);
        } else {
            self._start_p_s = Vector3::new(1., 1., 1.);
            self._start_l_s = Vector3::new(1., 1., 1.);
        }
        // pubMatrix.ComposeToRef(self._start_p_s, self._start_p_r, self._start_p_p, resultStartMatrix);
        // let a = Matrix::f
        // println!(
        //     "self._start_p_r: {:?}, self._start_p_s: {:?}, self._start_p_p: {:?}",
        //     self._start_p_r, self._start_p_s, self._start_p_p
        // );
        // println!(
        //     "self._start_l_r: {:?}, self._start_l_s: {:?}, self._start_l_p: {:?}",
        //     self._start_l_r, self._start_l_s, self._start_l_p
        // );
        // println!("resultStartMatrix: {:?}  ", resultStartMatrix);
        let euler_angles = self._start_p_r.euler_angles();
        *resultStartMatrix = Matrix::new_nonuniform_scaling(&self._start_p_s)
            * Matrix::from_euler_angles(euler_angles.0, euler_angles.1, euler_angles.2)
            * Matrix::new_translation(&self._start_p_p);
        // println!("resultStartMatrix1: {:?}  ", resultStartMatrix);
        // pubMatrix.ComposeToRef(self._start_l_s, self._start_l_r, self._start_l_p, ParticleSystemTool.TempMatrix_1);
        let euler_angles = self._start_l_r.euler_angles();
        let mat = Matrix::new_nonuniform_scaling(&self._start_l_s)
            * Matrix::from_euler_angles(euler_angles.0, euler_angles.1, euler_angles.2)
            * Matrix::new_translation(&self._start_l_p);

        *resultStartMatrix = mat * (*resultStartMatrix);
        // println!("resultStartMatrix2: {:?}  ", resultStartMatrix);
        // ParticleSystemTool.TempMatrix_1.multiplyToRef(resultStartMatrix, resultStartMatrix);
    }

    fn _update(
        &mut self,
        world_matrix: Matrix,
        local_matrix: Matrix,
        camera_pos: Vector3,
        camera_rotation_matrix_invert: Matrix,
    ) -> f32 {
        if self._mpColorData.is_none() || self._mpMatrixList.is_none() {
            return 0.0;
        }

        // const cameraWM = self.getCameraMatrix();
        let cameraRotationMatrix = camera_rotation_matrix_invert;
        let cameraGlobalPos = camera_pos;
        let _start_parent_m = world_matrix.clone();
        let _start_local_m = local_matrix;
        let mut _start_w_m = world_matrix;

        let mut temp_start_matrix = Matrix::identity();
        // println!("temp_start_matrix1: {:?}", temp_start_matrix);
        self._formatStartInfo(
            Some(_start_parent_m),
            _start_local_m,
            &mut temp_start_matrix,
        );
        let mut real_start_matrix = Matrix::identity();

        // println!("temp_start_matrix: {:?}", temp_start_matrix);
        let mut activeCount = 0;
        if let ERenderMode::None = self._renderMode {
        } else {
            let len = self.activeParticleList.len();
            // println!("len: {}", len);
            for i in 0..len {
                let particle = unsafe { &mut *self.activeParticleList[i] };
                // println!(
                //     "========== particle.age: {}, particle.lifetime: {}",
                //     particle.age, particle.lifetime
                // );
                if particle.age > particle.lifetime {
                    continue;
                }
                let ii = activeCount * 4;

                let _color = particle.color;
                // println!("particle.color: {:?}", _color);
                let _uv = particle.uv;
                // println!("particle.uv: {:?}", _uv);

                // /**
                //  * 仅 Pi/Simple 导出为 带纹理动画的粒子
                //  */
                if let Some(_mpColorData) = &mut self._mpColorData {
                    _mpColorData[ii + 0] = _color[0] + (_uv[0] * 1000.).floor() * 10.;
                    _mpColorData[ii + 1] = _color[1] + (_uv[1] * 1000.).floor() * 10.;
                    _mpColorData[ii + 2] = _color[2] + (_uv[2] * 1000.).floor() * 10.;
                    _mpColorData[ii + 3] = _color[3] + (_uv[3] * 1000.).floor() * 10.;
                }

                if let Some(colorData) = &mut self.colorData {
                    colorData[ii + 0] = _color[0];
                    colorData[ii + 1] = _color[1];
                    colorData[ii + 2] = _color[2];
                    colorData[ii + 3] = _color[3];
                }

                if let Some(uvData) = &mut self.uvData {
                    uvData[ii + 0] = _uv[0];
                    uvData[ii + 1] = _uv[1];
                    uvData[ii + 2] = _uv[2];
                    uvData[ii + 3] = _uv[3];
                }

                // if (self.simulationSpace == EMeshParticleSpaceMode.Local) {
                //     realStartMatrix.copyFrom(tempStartMatrix);
                // }
                // else {
                // realStartMatrix.copyFrom(tempStartMatrix);
                real_start_matrix = particle.emit_world_matrix;
                _start_w_m = particle.start_world_matrix;
                // }

                let mut TempVector3Rotation = (particle.rotation);
                if let ERenderMode::Billboard = self._renderMode {
                    TempVector3Rotation[2] *= -1.;
                }
                // if (self.emitterShape.alignDirection) {
                //     TempVector3Rotation =+ addInPlace(particle.emitRotation);
                // }

                // let mut pivot = Vector3::zeros();
                let mut pivot = multiply(&particle.scaling, &self.renderPivot);
                // println!("pivot: {:?}", pivot);
                pivot = pivot + particle.position;
                // println!("particle.position: {:?}", particle.position);
                // println!("pivot2: {:?}", pivot);
                // println!("tempStartMatrix0: {:?}", temp_start_matrix);
                (self.particleWorldMatrixCompute)(
                    _start_w_m,
                    &mut real_start_matrix,
                    &mut temp_start_matrix,
                    cameraRotationMatrix,
                    cameraGlobalPos,
                    &mut particle.readldirection,
                    particle.direction_length,
                    pivot,
                    particle.scaling,
                    self.TempVector3Rotation,
                    self.stretchedLengthScale,
                    self.stretchedVelocityScale,
                );
                // println!("tempStartMatrix: {:?}", temp_start_matrix);
                if let Some(matlist) = &mut self._mpMatrixList {
                    temp_start_matrix.transpose_mut();
                    // temp_start_matrix.as_slice().iter().enumerate().for_each(|(idx, val)| {
                    //     matlist[activeCount * 16 + idx] = *val;
                    // });
                    matlist[activeCount * 16] = temp_start_matrix[0];
                    matlist[activeCount * 16 + 1] = temp_start_matrix[1];
                    matlist[activeCount * 16 + 2] = temp_start_matrix[2];
                    matlist[activeCount * 16 + 3] = temp_start_matrix[3];
                    matlist[activeCount * 16 + 4] = temp_start_matrix[4];
                    matlist[activeCount * 16 + 5] = temp_start_matrix[5];
                    matlist[activeCount * 16 + 6] = temp_start_matrix[6];
                    matlist[activeCount * 16 + 7] = temp_start_matrix[7];
                    matlist[activeCount * 16 + 8] = temp_start_matrix[8];
                    matlist[activeCount * 16 + 9] = temp_start_matrix[9];
                    matlist[activeCount * 16 + 10] = temp_start_matrix[10];
                    matlist[activeCount * 16 + 11] = temp_start_matrix[11];
                    matlist[activeCount * 16 + 12] = temp_start_matrix[12];
                    matlist[activeCount * 16 + 13] = temp_start_matrix[13];
                    matlist[activeCount * 16 + 14] = temp_start_matrix[14];
                    matlist[activeCount * 16 + 15] = temp_start_matrix[15];
                }

                activeCount += 1;
            }
        }

        return activeCount as f32;
    }

    // 变量计算 - 异步计算
    pub fn varCompute(&mut self, currTime: u64, world_matrix: Matrix, local_matrix: Matrix) {
        // println!("============varCompute1");
        let now = currTime;
        let delta = now - self._lastTime;
        if (delta < self.computeDeltaTime) {
            return;
        }
        // println!("============varCompute2");
        self.activeParticleList.clear();

        if (self._isPlaying && self._isComputeable) {
            // println!("============varCompute3");
            self._lastTime = now;
            self._particleSystemAge += delta;
            self._deltaTime = delta as f32 / 1000.0;

            let scaleUpdateSpeed = (self._deltaTime).min(50.0 / 1000.0);
            self._stepTime = scaleUpdateSpeed * 1000.0;
            // println!(
            //     "============varCompute3.1:  now: {}, self._lastCreateTime: {}",
            //     now, self._lastCreateTime
            // );
            let createDelta = now as i64 - self._lastCreateTime as i64;

            // self.emitRate / self.duration;
            let tempLoop = self._particleSystemAge / self.duration;

            let mut canNew = true;
            // println!(
            //     "============varCompute4:  self._particleSystemAge: {}, self.duration: {}",
            //     self._particleSystemAge, self.duration
            // );
            let check = self._particleSystemAge as i64 - self.duration as i64;

            if check > 0 && !self.looping {
                // 等待已发射粒子的消失
                if check > self._maxLifetimeOverage as i64 {
                    self.stop();
                    return;
                }
                canNew = false;
            } else {
                self._maxLifetimeOverage = 0;
            }

            let localTimeDiff = self._particleSystemAge % self.duration;
            let mut burstCreateCount = 0;

            self.emissionLoop = (self._particleSystemAge as f32 / self.emissionTime as f32).floor();
            self.emissionProgress =
                self._particleSystemAge as f32 % self.emissionTime / self.emissionTime;

            let burstsCount = self.bursts.len();
            // println!("====== burstsCount: {}", burstsCount);
            self._burstsLoopCount = vec![0.; burstsCount];
            // 新的一轮循环
            if (self._loopCount < tempLoop as f32) {
                // 剩余 Bursts 全部激活
                for i in 0..burstsCount {
                    let [burstTime, burstCount, burstLoop, interval] = self.bursts[i];

                    if (self.duration > burstTime as u64) {
                        // if (!self._burstsLoopCount[i]) {
                        self._burstsLoopCount[i] = 0.;
                        // }

                        let tempBurstLoop = ((self.duration as f32 - burstTime) / interval).floor();

                        // 预估爆发次数 大于 已爆发次数
                        if (tempBurstLoop > self._burstsLoopCount[i]) {
                            // 可爆发次数 大于 已爆发次数
                            while (burstLoop > self._burstsLoopCount[i]) {
                                self._burstsLoopCount[i] += 1.;
                                burstCreateCount += burstCount as i32;
                            }
                        }
                    }

                    self._burstsLoopCount[i] = 0.;
                }

                self._loopCount = tempLoop as f32;
            }

            for i in 0..burstsCount {
                let [burstTime, burstCount, burstLoop, interval] = self.bursts[i];
                if (localTimeDiff > burstTime as u64) {
                    // if (!self._burstsLoopCount[i]) {
                    self._burstsLoopCount[i] = 0.;
                    // }

                    let tempBurstLoop = ((localTimeDiff as f32 - burstTime) / interval).floor();

                    // 预估爆发次数 大于 已爆发次数
                    if (tempBurstLoop > self._burstsLoopCount[i]) {
                        // 可爆发次数 大于 已爆发次数
                        if (burstLoop > self._burstsLoopCount[i]) {
                            self._burstsLoopCount[i] += 1.;
                            burstCreateCount += burstCount as i32;
                        }
                    }
                }
            }

            let mut newCount = 0.;
            if (self.rateOverTime > 0.) {
                let perRateNeedTime = 1000. / self.rateOverTime;
                newCount = ((createDelta as f32 + perRateNeedTime * 0.5) * self.rateOverTime
                    / 1000.)
                    .floor();
                if newCount > 0. {
                    self._lastCreateTime += newCount as u64 * perRateNeedTime as u64;
                }
            } else {
                self._lastCreateTime = now;
            }
            newCount += burstCreateCount as f32;

            if (!canNew) {
                newCount = 0.;
            }

            let mut newParticleList = vec![];
            let mut deltaAge = 0.;
            // println!("pos: {:?}", self.particleList);
            for i in 0..self.maxParticles {
                let particle = &mut self.particleList[i];

                if (self.trail.is_some() && self._enableTrail) {
                    deltaAge = particle.trial_lifetime * self._trailLifetimeScale;
                }

                if (particle.age - particle.lifetime >= deltaAge) {
                    if (newCount > 0.) {
                        newParticleList.push(particle as *mut Particle);

                        self.activeParticleList.push(particle as *mut Particle);
                        newCount -= 1.;
                    }
                } else {
                    self.activeParticleList.push(particle);
                }
            }
            self.activeParticleList.sort_by(|a, b| {
                let a = unsafe { &*(*a) };
                let b = unsafe { &*(*b) };
                // println!("pos: {:?}", a.position);
                a.age.partial_cmp(&b.age).unwrap()
            });

            let mut emitWorldMatrix = Matrix::identity();
            // let _startWorldRM = Matrix::identity();
            let parentWorldMatrix = world_matrix.clone();
            let worldMatrix = world_matrix;
            let localMatrix = local_matrix;

            // let mut postion = Vector3::new(0., 0., 0.);
            let mut _startWorldRM = Rotation3::identity();
            // let mut scaling = Vector3::new(1., 1., 1.);
            CoordinateSytem3::matrix4_decompose_rotation(
                &worldMatrix,
                None,
                Some(&mut _startWorldRM),
                None,
            );

            // worldMatrix.getRotationMatrixToRef(_startWorldRM);
            // let euler_angles = _startWorldRM.euler_angles()
            // Quaternion::from_euler_angles(euler_angles.0, euler_angles.1, euler_angles.2).mat
            // use ncollide::simba::scalar::SubsetOf;
            let _startWorldRM = _startWorldRM.matrix().try_inverse().unwrap();
            let _startWorldRM = Matrix::new(
                _startWorldRM[0],
                _startWorldRM[1],
                _startWorldRM[2],
                0.,
                _startWorldRM[3],
                _startWorldRM[4],
                _startWorldRM[6],
                0.,
                _startWorldRM[6],
                _startWorldRM[7],
                _startWorldRM[8],
                0.,
                0.,
                0.,
                0.,
                1.,
            );

            self._formatStartInfo(Some(parentWorldMatrix), localMatrix, &mut emitWorldMatrix);

            self.applyModifier();

            // for p in &self.activeParticleList{
            //     let r = unsafe { &mut *(*p) };
            //     println!("1activeParticleList pos{:?}: {:?}", *p, r.position);
            // }
            // for p in &newParticleList{
            //     let r = unsafe { &mut *(*p) };
            //     println!("1newParticleList pos{:?}: {:?}", p, r.position);
            // }

            self.varInit(
                newParticleList.clone(),
                worldMatrix,
                _startWorldRM,
                emitWorldMatrix,
            );
            // for p in &self.activeParticleList{
            //     let r = unsafe { &mut *(*p) };
            //     println!("2activeParticleList pos{:?}: {:?}", *p, r.position);
            // }
            // for p in &newParticleList{
            //     let r = unsafe { &mut *(*p) };
            //     println!("2newParticleList pos{:?}: {:?}", p, r.position);
            // }

            self.varPre(
                delta as f32,
                scaleUpdateSpeed as f32,
                worldMatrix,
                _startWorldRM,
                emitWorldMatrix,
            );

            if let Some(trail) = self.trail.as_mut() {
                if (self._enableTrail) {
                    let mat = (self.getCameraMatrix)();

                    // trail.modify(
                    //     Vector3::new(mat[3], mat[7], mat[11]),
                    //     &self.activeParticleList,
                    // );
                }
            }
        }
    }

    fn applyModifier(&mut self) {
        self.overLifetimeModifierList.clear();
        if (self.enableColorOverLifeTime) {
            self.overLifetimeModifierList
                .push(Box::new(self.colorOverLifetimeInterpolation.clone()));
        }
        if (self.enableForceOverLifeTime) {
            self.overLifetimeModifierList
                .push(Box::new(self.forceOverLifetimeInterpolation.clone()));
        }
        if (self.enableRotationOverLifeTime) {
            self.overLifetimeModifierList
                .push(Box::new(self.rotationOverLifetimeInterpolation.clone()));
        }
        if (self.enableSizeOverLifeTime) {
            self.overLifetimeModifierList
                .push(Box::new(self.sizeOverLifetimeInterpolation.clone()));
        }
        if (self.enableVelocityOverLifeTime) {
            self.overLifetimeModifierList
                .push(Box::new(self.velocityOverLifetimeInterpolation.clone()));
        }

        if (self.enableNoise) {
            self.overLifetimeModifierList
                .push(Box::new(self.noiseModifier.clone()));
        }

        self.overLifetimeModifierList
            .push(Box::new(self.localPositionModifier.clone()));

        if (self.enableColorBySpeed) {
            self.overLifetimeModifierList
                .push(Box::new(self.colorBySpeedInterpolation.clone()));
        }
        if (self.enableRotationBySpeed) {
            self.overLifetimeModifierList
                .push(Box::new(self.rotationBySpeedInterpolation.clone()));
        }
        if (self.enableSizeBySpeed) {
            self.overLifetimeModifierList
                .push(Box::new(self.sizeBySpeedInterpolation.clone()));
        }

        if (self.enableLimitVelocityOverLifeTime) {
            self.overLifetimeModifierList.push(Box::new(
                self.limitVelocityOverLifetimeInterpolation.clone(),
            ));
        }
        if (self.enableTextureSheet) {
            self.overLifetimeModifierList
                .push(Box::new(self.textureSheetInterpolation.clone()));
        }
        if (self.enableCustomDataForMainUV) {
            self.overLifetimeModifierList
                .push(Box::new(self.customDataForMainUV.clone()));
        }
    }

    /**
     * 初始化
     */
    pub fn varInit(
        &mut self,
        newParticleList: Vec<*mut Particle>,
        worldMatrix: Matrix,
        startWorldMatrixInvert: Matrix,
        emitWorldMatrix: Matrix,
    ) {
        let newCount = newParticleList.len();

        let progress = self._particleSystemAge % self.duration / self.duration;
        let mut i = 0;
        for particle in newParticleList {
            let particle = unsafe { &mut *particle };

            particle.reset();

            particle.global_record_list.clear();
            particle.local_record_list.clear();
            particle.start_world_matrix = worldMatrix;

            particle.start_world_matrix_invert = (startWorldMatrixInvert);
            particle.emit_world_matrix = (emitWorldMatrix);
            // println!(
            //     "particle.emit_world_matrix {:?}",
            //     particle.emit_world_matrix
            // );
            self.initNew(particle, progress, i, newCount);

            i += 1;
        }
    }

    pub fn initNew(
        &mut self,
        particle: &mut Particle,
        progress: u64,
        emissionIndex: usize,
        emissionTotal: usize,
    ) {
        let emissionLoop = self.emissionLoop;
        let emissionProgress = self.emissionProgress;
        let mut rng = rand::thread_rng();

        let startSpeed = self
            .startSpeedInterpolation
            .interpolate(progress as f32, rng.gen::<f32>());
        // println!("startSpeed: {:?}", startSpeed);
        let mut startLocalPosition = Vector3::zeros();
        if let Some(emitterShape) = &self.emitterShape {
            emitterShape.start_position_function(
                particle.start_world_matrix,
                &mut startLocalPosition,
                emissionLoop,
                emissionProgress,
                emissionIndex as f32,
                emissionTotal as f32,
                true,
            );
        }

        // 发射器局部矩阵影响
        startLocalPosition = self
            .emitterShape
            .as_mut()
            .unwrap()
            .get_localMatrix()
            .transform_vector(&startLocalPosition);

        let mut startDirection = Vector3::zeros();
        // println!(":startDirection0 {:?}, particle.start_world_matrix: {:?}", startDirection, particle.start_world_matrix);
        // println!("startLocalPosition: {:?}", startLocalPosition);
        if let Some(emitterShape) = &self.emitterShape {
            // println!("emitterShape: {:?}", emitterShape);
            emitterShape.start_direction_function(
                particle.start_world_matrix,
                &mut startDirection,
                Vector3::zeros(),
                startLocalPosition,
                true,
            );
        }
        // println!("startDirection0: {:?}", startDirection);
        startDirection *= (startSpeed);
        // println!("initNew {:?}", 1);
        particle.age = 0.;
        particle.lifetime = self
            .startLifetimeInterpolation
            .interpolate(progress as f32, rng.gen::<f32>());
        // println!("startLocalPosition: {:?}", startLocalPosition);
        particle.position = (startLocalPosition);
        // println!("startDirection: {:?}", startDirection);
        particle.direction = (startDirection);

        // emitRotation
        let xAxis = startDirection[0];
        let yAxis = startDirection[1];
        let zAxis = startDirection[2];
        let yaw = -zAxis.atan2(xAxis) + std::f32::consts::PI / 2.;
        let len = (xAxis * xAxis + zAxis * zAxis).sqrt();
        let pitch = -(yAxis).atan2(len);
        particle.emit_rotation = Vector3::new(pitch, yaw, 0.);
        let mut progress = progress as f32;
        self.startColorInterpolation.modify(particle, progress, 0.);
        self.startRotationInterpolation
            .modify(particle, progress, 0.);
        self.startSizeInterpolation.modify(particle, progress, 0.);
        if (self.enableTextureSheet) {
            self.textureSheetInterpolation.set_runAsStart(true);
            self.textureSheetInterpolation
                .modify(particle, &mut (progress), 0.);
        }

        particle.trial_lifetime = 0.;
        particle.trial_width = 1.;
        if let Some(trail) = self.trail.as_mut() {
            if (self._enableTrail) {
                if (rng.gen::<f32>() <= trail.ratio) {
                    particle.trial_lifetime =
                        trail.lifetime.interpolate(progress, rng.gen::<f32>()) * particle.lifetime;
                    if (trail.sizeAffectsLifetime) {
                        particle.trial_lifetime *= particle.start_scaling.magnitude();
                    }
                }
                if (trail.widthOverTrail.mode == EInterpolationCurveMode::Constant
                    || trail.widthOverTrail.mode == EInterpolationCurveMode::TwoConstants)
                {
                    particle.trial_width =
                        trail.widthOverTrail.interpolate(progress, rng.gen::<f32>());
                }
            }
        }

        startDirection = normalize(&startDirection);
        startDirection = startDirection * (0.01);
        particle.local_record_list.push([
            -0.002,
            startLocalPosition[0] - startDirection[0],
            startLocalPosition[1] - startDirection[1],
            startLocalPosition[2] - startDirection[2],
        ]);
        particle.local_record_list.push([
            -0.001,
            startLocalPosition[0],
            startLocalPosition[1],
            startLocalPosition[2],
        ]);
        startLocalPosition = particle
            .start_world_matrix
            .transform_vector(&startLocalPosition);
        startDirection = particle
            .start_world_matrix
            .transform_vector(&startDirection);

        particle.global_record_list.push([
            -0.002,
            startLocalPosition[0] - startDirection[0],
            startLocalPosition[1] - startDirection[1],
            startLocalPosition[2] - startDirection[2],
        ]);
        particle.global_record_list.push([
            -0.001,
            startLocalPosition[0],
            startLocalPosition[1],
            startLocalPosition[2],
        ]);
    }

    /**
     * 变量预处理
     * @param index
     */
    pub fn varPre(
        &mut self,
        deltaTimeMS: f32,
        scaleUpdateSpeed: f32,
        worldMatrix: Matrix,
        startWorldMatrixInvert: Matrix,
        emitWorldMatrix: Matrix,
    ) {
        let activeParticles = &mut self.activeParticleList;
        let count = activeParticles.len();
        let mut globalAmount = self._particleSystemAge % self.duration / self.duration;
        globalAmount = 0u64.max(1u64.min(globalAmount));

        self._maxLifetimeOverage = 0;
        for i in 0..count {
            let particle = unsafe { &mut *activeParticles[i] };
            // println!("varPre particle.pos:{}", particle.position);
            // if i == 0 {
            //     println!("{}particle.age:{}", i, particle.age);
            // }
            particle.age += deltaTimeMS;
            // if i == 0 {
            //     println!("{}particle.age:{}", i, particle.age);
            // }
            self._maxLifetimeOverage = self._maxLifetimeOverage.max(
                (particle.lifetime - particle.age
                    + particle.trial_lifetime * self._trailLifetimeScale) as u64,
            );

            if (particle.age <= particle.lifetime) {
                if (self.simulationSpace == EMeshParticleSpaceMode::Local) {
                    particle.start_world_matrix = (worldMatrix);
                    particle.start_world_matrix_invert = (startWorldMatrixInvert);
                    particle.emit_world_matrix = (emitWorldMatrix);
                }

                let mut particleAmount = particle.age / particle.lifetime;
                particleAmount = 1.0f32.min(particleAmount);

                self.gravityInterpolation
                    .modify(particle, globalAmount as f32, scaleUpdateSpeed);

                self.textureSheetInterpolation.set_runAsStart(false);

                // 重置scaling
                particle.color = (particle.start_color);
                particle.scaling = particle.start_scaling;

                let modifierCount = self.overLifetimeModifierList.len();
                // println!("=========== modifierCount:{}", modifierCount);

                // let r = unsafe { &mut *(*p) };
                // println!("varPre1 pos: {:?}", particle.position);

                for j in 0..modifierCount {
                    let modifier = &mut self.overLifetimeModifierList[j];
                    modifier.modify(particle, &mut particleAmount, scaleUpdateSpeed);
                }
                // println!("varPre2 pos: {:?}", particle.position);
            }
        }
    }
}

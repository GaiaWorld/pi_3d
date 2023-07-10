use std::time::UNIX_EPOCH;

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
        base::{Color4Interpolate, IParticleModifier, TranslationInterpolate},
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

#[derive(Clone, Copy, PartialEq, Default)]
pub enum EMeshParticleSpaceMode {
    #[default]
    Local = 0,
    /**
     * 发射在世界空间时, 父级尽量不要有旋转动画, 因为 动画 与 粒子动画的衔接有误差，无法完美适配
     */
    World = 1,
}

#[derive(Clone, Copy, PartialEq, Default)]
pub enum EMeshParticleScaleMode {
    #[default]
    Hierarchy = 0,
    Local = 1,
    Shape = 2,
}

#[derive(Clone, Copy, PartialEq, Default)]
pub enum ERenderAlignment {
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

#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub enum ERenderMode {
    #[default]
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
    pub name: String,
    pub sqrt3: f32,

    _one_vector3: Vector3,
    /**
     * 计算的间隔时间
     */
    pub compute_delta_time: u64,
    /**
     * 最大粒子数
     */
    pub max_particles: usize,
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
    _loop_count: f32,
    /**
     * 延时时间 - 毫秒
     */
    pub start_delay: i32,
    pub emitter_shape: Option<Box<dyn IShapeEmitterType>>,
    /**
     * 一次发射循环的时间 - 毫秒
     */
    pub emission_time: f32,
    /**
     * 当前已循环次数
     */
    pub emission_loop: f32,
    /**
     * 当前发射循环的进度
     */
    pub emission_progress: f32,
    /**
     * 每秒发射的粒子数目
     */
    pub rate_over_time: f32,
    /**
     * 指定时间点 开始 持续 指定时间内 发射指定数目
     * [开始时间, 发射数目, 循环次数, 间隔]
     * @tip 需要按 开始时间从小到大排序
     */
    pub bursts: Vec<[f32; 4]>,
    _bursts_loop_count: Vec<f32>,

    /**
     * 发射空间
     * * Local - 本地: 发射开始时的方向受父级影响, 发射后的位置也受父级影响
     * * World - 世界空间: 发射开始时的方向受父级影响, 发射后的位置不再受父级影响
     */
    pub simulation_space: EMeshParticleSpaceMode,
    pub scaling_space: EMeshParticleScaleMode,
    _render_alignment: ERenderAlignment,
    _render_mode: ERenderMode,

    pub render_pivot: Vector3,

    pub start_lifetime_interpolation: FloatInterpolation,
    pub start_speed_interpolation: FloatInterpolation,
    pub start_size_interpolation: StartSize,
    pub start_rotation_interpolation: StartRotation,
    pub start_color_interpolation: StartColor,

    pub gravity_interpolation: Gravity,
    /**
     * 位移速度
     * @tip 默认模式 - ConstantsUnite - XYZ 共用一个随机数
     * @tip 为 Constants | ConstantsUnite 模式应用插值时, 仅创建时使用, 意义为: 创建时速度基础上附加速度
     * @tip 为 Curves 模式时, 运行过程中使用, 意义为: 创建时速度基础上附加速度 - 每次循环累加 此次循环 与 上次循环 速度值的差值
     */
    pub velocity_over_lifetime_interpolation: VelocityOverLifetime,
    pub enable_velocity_over_life_time: bool,

    pub limit_velocity_over_lifetime_interpolation: LimitVelocityOverLifetime,
    pub enable_limit_velocity_over_life_time: bool,

    pub force_over_lifetime_interpolation: ForceOverLifetime,
    pub enable_force_over_life_time: bool,

    pub color_over_lifetime_interpolation: ColorOverLifetime,
    pub enable_color_over_life_time: bool,
    pub color_by_speed_interpolation: ColorBySpeed,
    pub enable_color_by_speed: bool,
    /**
     * 缩放
     * @tip 默认模式 - ConstantsUnite - XYZ 共用一个随机数
     * @tip 为 Constants | ConstantsUnite 模式应用插值时, 仅创建时使用, 意义为: 仅对创建时大小的影响倍数
     * @tip 为 Curves 模式时, 运行过程中使用, 意义为: 创建时大小的影响倍数
     */
    pub size_over_lifetime_interpolation: SizeOverLifetime,
    pub enable_size_over_life_time: bool,
    pub size_by_speed_interpolation: SizeBySpeed,
    pub enable_size_by_speed: bool,

    pub local_position_modifier: LocalPosition,
    /**
     * 旋转
     * @tip 默认模式 - ConstantsUnite - XYZ 共用一个随机数
     * @tip Unity 中单一 Angle 对应 只处理 Y 轴
     * @tip 与 Unity 编辑时不同, 此处单位为 弧度
     * @tip 为 Constants | ConstantsUnite 模式时, 仅创建时使用, 意义为: 每秒旋转速度
     * @tip 为 Curves 模式时, 运行过程中使用, 意义为: 每秒旋转速度
     */
    pub rotation_over_lifetime_interpolation: RotationOverLifetime,
    pub enable_rotation_over_life_time: bool,
    pub rotation_by_speed_interpolation: RotationBySpeed,
    pub enable_rotation_by_speed: bool,
    /**
     * Texture Sheet Animation
     */
    pub texture_sheet_interpolation: TextureSheet,
    pub enable_texture_sheet: bool,

    pub custom_data_for_main_uv: CustomDataForUV,
    pub enable_custom_data_for_main_uv: bool,

    pub noise_modifier: Noise,
    pub enable_noise: bool,

    pub stretched_velocity_scale: f32,
    pub stretched_length_scale: f32,

    // 局部空间数据 - 每粒子
    over_lifetime_modifier_list: Vec<Box<dyn IParticleModifier>>,

    particle_list: Vec<Particle>,
    pub active_particle_list: Vec<*mut Particle>,

    _particle_system_age: u64,

    _last_time: u64,
    _delta_time: f32,
    _last_create_time: u64,

    _is_playing: bool,
    _is_computeable: bool,

    //  scene: pubScene;
    pub trail: Option<TrailModifier>,
    _enable_trail: bool,
    _trail_lifetime_scale: f32,

    pub particle_world_matrix_compute: Box<TRenderWorldMatrixCompute>,

    pub get_camera_rotation_matrix_invert: Box<dyn Fn() -> Matrix>,
    pub get_camera_position: Box<dyn Fn() -> Vector3>,
    pub get_parent_world_matrix: Box<dyn Fn() -> Matrix>,
    pub get_world_matrix: Box<dyn Fn() -> Matrix>,
    pub get_local_matrix: Box<dyn Fn() -> Matrix>,
    pub get_camera_matrix: Box<dyn Fn() -> Matrix>,

    _is_disposed: bool,

    pub _mp_matrix_list: Option<Vec<f32>>,
    _mp_color_data: Option<Vec<f32>>,
    _mp_uvdata: Option<Vec<f32>>,

    pub color_data: Option<Vec<f32>>,
    pub uv_data: Option<Vec<f32>>,

    pool: Pool,
    _max_id: usize,
    pub temp_vector3_scaling: Vector3,
    pub temp_vector3_translation: Vector3,
    pub temp_vector3_rotation: Vector3,
    pub temp_quaternion_0: Quaternion,
    pub temp_matrix_1: Matrix,

    _start_p_s: Vector3,
    _start_p_r: Quaternion,
    _start_p_p: Vector3,

    _start_l_s: Vector3,
    _start_l_r: Quaternion,
    _start_l_p: Vector3,

    _max_lifetime_overage: u64,
    _step_time: f32,
}

impl ParticleSystemTool {
    pub fn set_render_alignment(&mut self, value: ERenderAlignment) {
        self._render_alignment = value;
        self._update_particle_world_matrix_compute();
    }
    pub fn get_render_alignment(&self) -> ERenderAlignment {
        return self._render_alignment.clone();
    }

    pub fn set_render_mode(&mut self, value: ERenderMode) {
        self._render_mode = value;
        self._update_particle_world_matrix_compute();
    }
    pub fn get_render_mode(&mut self) -> ERenderMode {
        return self._render_mode;
    }

    pub fn set_enable_trail(&mut self, value: bool) {
        self._enable_trail = value;
        if let Some(v) = &mut self.trail {
            v._enabled = value;
        }
        // Trail
        self._trail_lifetime_scale = 0.;
        if value {
            if let Some(v) = &mut self.trail {
                if !v.die_with_particle {
                    self._trail_lifetime_scale = 1.;
                }
            }
        }
    }
    pub fn get_enable_trail(&self) -> bool {
        return self._enable_trail;
    }

    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            sqrt3: 3.0f32.sqrt(),
            _one_vector3: Vector3::new(1., 1., 1.),
            compute_delta_time: 15,
            max_particles: 100,
            looping: true,
            prewarm: true,
            duration: 5,
            _loop_count: 0.,
            start_delay: 0,
            emitter_shape: None,
            emission_time: 1000.,
            emission_loop: 0.,
            emission_progress: 0.,
            rate_over_time: 10.,
            bursts: vec![],
            _bursts_loop_count: vec![],
            simulation_space: EMeshParticleSpaceMode::Local,
            scaling_space: EMeshParticleScaleMode::Hierarchy,
            _render_alignment: ERenderAlignment::Local,
            _render_mode: ERenderMode::Billboard,
            render_pivot: Vector3::zeros(),
            start_lifetime_interpolation: FloatInterpolation::default(),
            start_speed_interpolation: FloatInterpolation::default(),
            start_size_interpolation: StartSize::default(),
            start_rotation_interpolation: StartRotation::new(
                FloatInterpolation::default(),
                FloatInterpolation::default(),
                FloatInterpolation::default(),
            ),
            start_color_interpolation: StartColor::new(Color4Interpolate::new(
                Color4Gradient::default(),
            )),
            gravity_interpolation: Gravity::new(FloatInterpolation::new()),
            velocity_over_lifetime_interpolation: VelocityOverLifetime::new(
                FloatInterpolation::default(),
                FloatInterpolation::default(),
                FloatInterpolation::default(),
            ),
            enable_velocity_over_life_time: false,
            limit_velocity_over_lifetime_interpolation: LimitVelocityOverLifetime::new(
                FloatInterpolation::default(),
            ),
            enable_limit_velocity_over_life_time: false,
            force_over_lifetime_interpolation: ForceOverLifetime::new(TranslationInterpolate::new(
                FloatInterpolation::default(),
                FloatInterpolation::default(),
                FloatInterpolation::default(),
            )),
            enable_force_over_life_time: false,
            color_over_lifetime_interpolation: ColorOverLifetime::new(Color4Interpolate::new(
                Color4Gradient::default(),
            )),
            enable_color_over_life_time: false,
            color_by_speed_interpolation: ColorBySpeed::new(Color4Interpolate::new(
                Color4Gradient::default(),
            )),
            enable_color_by_speed: false,
            size_over_lifetime_interpolation: SizeOverLifetime::new(
                FloatInterpolation::default(),
                FloatInterpolation::default(),
                FloatInterpolation::default(),
            ),
            enable_size_over_life_time: false,
            size_by_speed_interpolation: SizeBySpeed::new(
                FloatInterpolation::default(),
                FloatInterpolation::default(),
                FloatInterpolation::default(),
            ),
            enable_size_by_speed: false,
            local_position_modifier: LocalPosition::new(),
            rotation_over_lifetime_interpolation: RotationOverLifetime::new(
                FloatInterpolation::default(),
                FloatInterpolation::default(),
                FloatInterpolation::default(),
            ),
            enable_rotation_over_life_time: false,
            rotation_by_speed_interpolation: RotationBySpeed::new(
                FloatInterpolation::default(),
                FloatInterpolation::default(),
                FloatInterpolation::default(),
            ),
            enable_rotation_by_speed: false,
            texture_sheet_interpolation: TextureSheet::new(
                FloatInterpolation::default(),
                FloatInterpolation::default(),
            ),
            enable_texture_sheet: false,
            custom_data_for_main_uv: CustomDataForUV::new(),
            enable_custom_data_for_main_uv: false,
            noise_modifier: Noise::new(),
            enable_noise: false,
            stretched_velocity_scale: 0.,
            stretched_length_scale: 1.,
            over_lifetime_modifier_list: vec![],
            particle_list: vec![],
            active_particle_list: vec![],
            _particle_system_age: 0,
            _last_time: 0,
            _delta_time: 0.,
            _last_create_time: 0,
            _is_playing: false,
            _is_computeable: false,
            trail: Some(TrailModifier::new()),
            _enable_trail: false,
            _trail_lifetime_scale: 0.,
            particle_world_matrix_compute: Box::new(RenderAlignmentLocal::compute),
            get_camera_rotation_matrix_invert: Box::new(|| Matrix::identity()),
            get_camera_position: Box::new(|| Vector3::zeros()),
            get_parent_world_matrix: Box::new(|| Matrix::identity()),
            get_world_matrix: Box::new(|| Matrix::identity()),
            get_local_matrix: Box::new(|| Matrix::identity()),
            get_camera_matrix: Box::new(|| Matrix::identity()),
            _is_disposed: false,
            _mp_matrix_list: None,
            _mp_color_data: None,
            _mp_uvdata: None,
            pool: Pool::new(),
            _max_id: 0,
            temp_vector3_scaling: Vector3::zeros(),
            temp_vector3_translation: Vector3::zeros(),
            temp_vector3_rotation: Vector3::zeros(),
            temp_quaternion_0: Quaternion::identity(),
            temp_matrix_1: Matrix::identity(),
            _start_p_s: Vector3::zeros(),
            _start_p_r: Quaternion::identity(),
            _start_p_p: Vector3::zeros(),
            _start_l_s: Vector3::zeros(),
            _start_l_r: Quaternion::identity(),
            _start_l_p: Vector3::zeros(),
            _max_lifetime_overage: 0,
            _step_time: 16.0,
            color_data: None,
            uv_data: None,
        }
    }

    pub fn dispose(&mut self) {
        if self._is_disposed {
            return;
        }
        self.stop();

        if let Some(trail) = self.trail.take() {
            trail.dispose()
        }

        self._mp_matrix_list = None;
        self._mp_color_data = None;

        self.start_lifetime_interpolation.dispose();
        self.start_speed_interpolation.dispose();

        self.bursts.clear(); // self.bursts                 = undefined;
        self._is_disposed = true;

        self.active_particle_list.clear();

        let len = self.particle_list.len();
        for i in (0..=len - 1).rev() {
            self.pool.recycle_particle(self.particle_list[i].clone());
        }
        self.particle_list.clear();

        return;
    }

    pub fn get_mp_matrix_list(&mut self) -> Option<Vec<f32>> {
        let r = self._mp_matrix_list.clone();
        if let Some(v) = self._mp_matrix_list.as_mut() {
            v.clear();
        }
        r
    }
    pub fn get_mp_color_data(&mut self) -> Option<Vec<f32>> {
        let r = self.color_data.clone();
        if let Some(v) = self.color_data.as_mut() {
            v.clear();
        }
        r
    }
    pub fn get_mp_uvdata(&mut self) -> Option<Vec<f32>> {
        let r = self.uv_data.clone();
        if let Some(v) = self.uv_data.as_mut() {
            v.clear();
        }
        r
    }

    pub fn build(&mut self) {
        self._mp_matrix_list = Some(vec![]);
        self._mp_color_data = Some(vec![]);
        self._mp_uvdata = Some(vec![]);
        self.color_data = Some(vec![]);
        self.uv_data = Some(vec![]);
        // self._mpUVSheetData = new Float32Array(4 * self.maxParticles);

        for _i in self._max_id..self.max_particles {
            // let _r = self.pool.createParticle();
            // println!("r.pos: {:?}", r.position);
            self.particle_list.push(self.pool.create_particle());
            // pru
        }
        self._max_id = self.max_particles;
    }

    pub fn start(&mut self) {
        if !self._is_playing {
            if self.start_delay > 0 && !self.prewarm {
                // TODO
                // setTimeout(self._startCall, self.startDelay);
                // self._startCall();
            } else {
                self._start_call();
            }
        }
    }

    fn _start_call(&mut self) {
        println!("=========== _startCall1");
        if !self._is_disposed {
            // println!("=========== _startCall2");
            let now = std::time::SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis();
            // println!("time nowu128: {}, u64: {}", now, now as u64);
            let now = now as u64;
            self._is_playing = true;
            self._is_computeable = true;
            self._particle_system_age = 0;
            self._last_time = now;
            self._last_create_time = self._last_time;
            // println!("=========== _startCall3! now: {}", now);
            let count = self.particle_list.len();
            for i in 0..count {
                self.particle_list[i].reset();
                // println!("r.pos{:p}: {:?}", &self.particleList[i], self.particleList[i].position)
            }
            // println!("=========== _startCall3, self.prewarm: {}", self.prewarm);
            if self.prewarm {
                // println!("=========== _startCall4");
                let delta_time = self.compute_delta_time;
                let counter = self.duration / delta_time;
                // now.checked_sub(duration)
                self._last_time = now - self.duration;
                self._last_create_time = self._last_time;

                let mut temp_curr = now - self.duration;

                for _i in 1..counter as usize {
                    temp_curr = temp_curr + delta_time;
                    // println!("=========== _startCall5, tempCurr: {}", tempCurr);
                    for _j in 0..count {
                        // self.particleList[j].reset();
                        // println!("count: {:?}, j: {}", count, j);
                        // println!("r.pos{}: {:?}",i, self.particleList[j].position);
                        // println!("r.pos{:p}: {:?}", &self.particleList[j], self.particleList[j].position)
                    }
                    self.var_compute(temp_curr, Matrix::identity(), Matrix::identity());
                }
            }

            if let Some(a) = &mut self.trail {
                a._enabled = self._enable_trail
            }
        }
    }

    pub fn stop(&mut self) {
        self._is_computeable = false;

        if !self._is_disposed {
            self._stop();
        }

        self._is_playing = false;
    }

    fn _stop(&mut self) {
        if self._is_playing {
            self._bursts_loop_count.clear();
        }

        self._is_playing = false;
        if let Some(a) = &mut self.trail {
            a.set_enable(false);
        }
        // self.trail.enable = false;
    }

    pub fn mp_update(
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

    fn _update_particle_world_matrix_compute(&mut self) {
        match self.get_render_mode() {
            ERenderMode::StretchedBillboard => {
                self.particle_world_matrix_compute = Box::new(StretchedBillboard::compute);
            }
            ERenderMode::HorizontalBillboard => {
                self.particle_world_matrix_compute = Box::new(HorizontalBillboard::compute);
            }
            ERenderMode::VerticalBillboard => {
                self.particle_world_matrix_compute = Box::new(VerticalBillboard::compute);
            }
            _ => match self.get_render_alignment() {
                ERenderAlignment::View => {
                    self.particle_world_matrix_compute = Box::new(RenderAlignmentView::compute);
                }
                ERenderAlignment::World => {
                    self.particle_world_matrix_compute = Box::new(RenderAlignmentWorld::compute);
                }
                ERenderAlignment::Facing => {
                    self.particle_world_matrix_compute = Box::new(RenderAlignmentFacing::compute);
                }
                ERenderAlignment::Velocity => {
                    self.particle_world_matrix_compute = Box::new(RenderAlignmentVelocity::compute);
                }
                ERenderAlignment::Local => {
                    self.particle_world_matrix_compute = Box::new(RenderAlignmentLocal::compute);
                }
            },
        }
    }

    pub fn get_max_lifetime_overage(&self) -> u64 {
        return self._max_lifetime_overage;
    }

    fn _format_start_info(
        &mut self,
        parent_wm: Option<Matrix>,
        local_wm: Matrix,
        result_start_matrix: &mut Matrix,
    ) {
        // 分解出 TRS
        if let Some(matrix) = parent_wm.as_ref() {
            // parentWM.decompose(self._start_p_s, self._start_p_r, self._start_p_p);
            // println!("matrix: {:?}", matrix);
            CoordinateSytem3::matrix4_decompose(
                &matrix,
                Some(&mut self._start_p_s),
                Some(&mut self._start_p_r),
                Some(&mut self._start_p_p),
            );
        } else {
            self._start_p_p.x = 0.;
            self._start_p_p.y = 0.;
            self._start_p_p.z = 0.;

            self._start_p_r = Quaternion::identity();

            self._start_p_s.x = 1.;
            self._start_p_s.y = 1.;
            self._start_p_s.z = 1.;
            // use nalgebra::Quaternion as MQuaternion;
        }
        // let mut rotation = Rotation3::identity();
        CoordinateSytem3::matrix4_decompose(
            &local_wm,
            Some(&mut self._start_l_s),
            Some(&mut self._start_l_r),
            Some(&mut self._start_l_p),
        );
        // let rotation = rotation.euler_angles();
        // self._start_l_r = Quaternion::from_euler_angles(rotation.0, rotation.1, rotation.2);

        // localWM.decompose(self._start_l_s, self._start_l_r, self._start_l_p);

        // if (self.renderAlignment == ERenderAlignment.View) {
        //     self._start_p_r.copyFromFloats(0, 0, 0, 1);
        //     self._start_l_r.copyFromFloats(0, 0, 0, 1);
        // }
        // 忽略粒子节点上的本地旋转 - 仅发射后的位移受此影响
        if let ERenderAlignment::World = self._render_alignment {
            self._start_l_r = Quaternion::identity();
            println!("self._start_l_r: {:?}", self._start_l_r);
        } else {
            //
        }
        if let EMeshParticleScaleMode::Hierarchy = self.scaling_space {
            //
        } else if self.scaling_space == EMeshParticleScaleMode::Local {
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

        // let euler_angles = self._start_p_r.euler_angles();
        // *result_start_matrix = Matrix::new_nonuniform_scaling(&self._start_p_s)
        //     * Matrix::from_euler_angles(euler_angles.0, euler_angles.1, euler_angles.2)
        //     * Matrix::new_translation(&self._start_p_p);

        CoordinateSytem3::matrix4_compose(
            &self._start_p_s,
            &self._start_p_r,
            &self._start_p_p,
            result_start_matrix,
        );
        // println!("resultStartMatrix1: {:?}  ", resultStartMatrix);
        // pubMatrix.ComposeToRef(self._start_l_s, self._start_l_r, self._start_l_p, ParticleSystemTool.TempMatrix_1);
        // let euler_angles = self._start_l_r.euler_angles();
        // let mat = Matrix::new_nonuniform_scaling(&self._start_l_s)
        //     * Matrix::from_euler_angles(euler_angles.0, euler_angles.1, euler_angles.2)
        //     * Matrix::new_translation(&self._start_l_p);

        let mut mat = Matrix::identity();
        CoordinateSytem3::matrix4_compose(
            &self._start_l_s,
            &self._start_l_r,
            &self._start_l_p,
            &mut mat,
        );

        *result_start_matrix = mat * (*result_start_matrix);
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
        if self._mp_color_data.is_none() || self._mp_matrix_list.is_none() {
            return 0.0;
        }

        // const cameraWM = self.getCameraMatrix();
        let camera_rotation_matrix = camera_rotation_matrix_invert;
        let camera_global_pos = camera_pos;
        let _start_parent_m = world_matrix.clone();
        let _start_local_m = local_matrix;
        let mut _start_w_m = world_matrix;

        let mut temp_start_matrix = Matrix::identity();
        // println!("temp_start_matrix1: {:?}", temp_start_matrix);
        self._format_start_info(
            Some(_start_parent_m),
            _start_local_m,
            &mut temp_start_matrix,
        );
        let mut _real_start_matrix = Matrix::identity();

        println!("self._render_mode: {:?}", self._render_mode);
        let mut active_count = 0;
        if let ERenderMode::None = self._render_mode {
        } else {
            let len = self.active_particle_list.len();
            println!("========== len: {}", len);
            for i in 0..len {
                let particle = unsafe { &mut *self.active_particle_list[i] };
                // println!(
                //     "========== particle.age: {}, particle.lifetime: {}",
                //     particle.age, particle.lifetime
                // );
                if particle.age > particle.lifetime {
                    println!("========== continue");
                    continue;
                }


                let _color = particle.color;
                let _uv = particle.uv;

                // /**
                //  * 仅 Pi/Simple 导出为 带纹理动画的粒子
                //  */
                if let Some(mp_color_data) = &mut self._mp_color_data {
                    mp_color_data.push(_color[0] + (_uv[0] * 1000.).floor() * 10.);
                    mp_color_data.push(_color[1] + (_uv[1] * 1000.).floor() * 10.);
                    mp_color_data.push(_color[2] + (_uv[2] * 1000.).floor() * 10.);
                    mp_color_data.push(_color[3] + (_uv[3] * 1000.).floor() * 10.);
                }

                if let Some(color_data) = &mut self.color_data {
                    color_data.push(_color[0]);
                    color_data.push(_color[1]);
                    color_data.push(_color[2]);
                    color_data.push(_color[3]);
                }

                if let Some(uv_data) = &mut self.uv_data {
                    uv_data.push(_uv[0]);
                    uv_data.push(_uv[1]);
                    uv_data.push(_uv[2]);
                    uv_data.push(_uv[3]);
                }

                // if (self.simulationSpace == EMeshParticleSpaceMode.Local) {
                //     realStartMatrix.copyFrom(tempStartMatrix);
                // }
                // else {
                // realStartMatrix.copyFrom(tempStartMatrix);
                _real_start_matrix = particle.emit_world_matrix;
                _start_w_m = particle.start_world_matrix;
                // }

                let mut temp_vector3_rotation = particle.rotation;
                if let ERenderMode::Billboard = self._render_mode {
                    temp_vector3_rotation[2] *= -1.;
                }
                // if (self.emitterShape.alignDirection) {
                //     TempVector3Rotation =+ addInPlace(particle.emitRotation);
                // }

                // let mut pivot = Vector3::zeros();
                let mut pivot = multiply(&particle.scaling, &self.render_pivot);
                // println!("pivot: {:?}", pivot);
                pivot = pivot + particle.position;
                // println!("particle.position: {:?}", particle.position);
                // println!("pivot2: {:?}", pivot);
                // println!("tempStartMatrix0: {:?}", temp_start_matrix);
                (self.particle_world_matrix_compute)(
                    _start_w_m,
                    &mut _real_start_matrix,
                    &mut temp_start_matrix, //&mut r,
                    camera_rotation_matrix,
                    camera_global_pos,
                    &mut particle.readldirection,
                    particle.direction_length,
                    pivot,
                    particle.scaling,
                    self.temp_vector3_rotation,
                    self.stretched_length_scale,
                    self.stretched_velocity_scale,
                );
                // println!("tempStartMatrix: {:?}", temp_start_matrix);

                if let Some(matlist) = &mut self._mp_matrix_list {
                    temp_start_matrix.transpose_mut();
                    // temp_start_matrix.as_slice().iter().enumerate().for_each(|(idx, val)| {
                    //     matlist[activeCount * 16 + idx] = *val;
                    // });
                    matlist.push(temp_start_matrix[0]);
                    matlist.push(temp_start_matrix[1]);
                    matlist.push(temp_start_matrix[2]);
                    matlist.push(temp_start_matrix[3]);
                    matlist.push(temp_start_matrix[4]);
                    matlist.push(temp_start_matrix[5]);
                    matlist.push(temp_start_matrix[6]);
                    matlist.push(temp_start_matrix[7]);
                    matlist.push(temp_start_matrix[8]);
                    matlist.push(temp_start_matrix[9]);
                    matlist.push(temp_start_matrix[10]);
                    matlist.push(temp_start_matrix[11]);
                    matlist.push(temp_start_matrix[12]);
                    matlist.push(temp_start_matrix[13]);
                    matlist.push(temp_start_matrix[14]);
                    matlist.push(temp_start_matrix[15]);
                }

                active_count += 1;
            }
        }

        return active_count as f32;
    }

    // 变量计算 - 异步计算
    pub fn var_compute(&mut self, curr_time: u64, world_matrix: Matrix, local_matrix: Matrix) {
        // println!("============varCompute1");
        let now = curr_time;
        let delta = now - self._last_time;
        if delta < self.compute_delta_time {
            return;
        }
        // println!("============varCompute2");
        self.active_particle_list.clear();

        if self._is_playing && self._is_computeable {
            // println!("============varCompute3");
            self._last_time = now;
            self._particle_system_age += delta;
            self._delta_time = delta as f32 / 1000.0;

            let scale_update_speed = (self._delta_time).min(50.0 / 1000.0);
            self._step_time = scale_update_speed * 1000.0;
            // println!(
            //     "============varCompute3.1:  now: {}, self._lastCreateTime: {}",
            //     now, self._lastCreateTime
            // );
            let create_delta = now as i64 - self._last_create_time as i64;
            // println!("create_delta: {}", create_delta);
            // self.emitRate / self.duration;
            let temp_loop = self._particle_system_age / self.duration;

            let mut can_new = true;
            // println!(
            //     "============varCompute4:  self._particleSystemAge: {}, self.duration: {}",
            //     self._particleSystemAge, self.duration
            // );
            let check = self._particle_system_age as i64 - self.duration as i64;

            if check > 0 && !self.looping {
                // 等待已发射粒子的消失
                if check > self._max_lifetime_overage as i64 {
                    self.stop();
                    return;
                }
                can_new = false;
            } else {
                self._max_lifetime_overage = 0;
            }

            let local_time_diff = self._particle_system_age % self.duration;
            let mut burst_create_count = 0;

            self.emission_loop =
                (self._particle_system_age as f32 / self.emission_time as f32).floor();
            self.emission_progress =
                self._particle_system_age as f32 % self.emission_time / self.emission_time;

            let bursts_count = self.bursts.len();
            // println!("====== burstsCount: {}", burstsCount);
            self._bursts_loop_count = vec![0.; bursts_count];
            // 新的一轮循环
            if self._loop_count < temp_loop as f32 {
                // 剩余 Bursts 全部激活
                for i in 0..bursts_count {
                    let [burst_time, burst_count, burst_loop, interval] = self.bursts[i];

                    if self.duration > burst_time as u64 {
                        // if (!self._burstsLoopCount[i]) {
                        self._bursts_loop_count[i] = 0.;
                        // }

                        let temp_burst_loop =
                            ((self.duration as f32 - burst_time) / interval).floor();

                        // 预估爆发次数 大于 已爆发次数
                        if temp_burst_loop > self._bursts_loop_count[i] {
                            // 可爆发次数 大于 已爆发次数
                            while burst_loop > self._bursts_loop_count[i] {
                                self._bursts_loop_count[i] += 1.;
                                burst_create_count += burst_count as i32;
                            }
                        }
                    }

                    self._bursts_loop_count[i] = 0.;
                }

                self._loop_count = temp_loop as f32;
            }

            for i in 0..bursts_count {
                let [burst_time, burst_count, burst_loop, interval] = self.bursts[i];
                if local_time_diff > burst_time as u64 {
                    // if (!self._burstsLoopCount[i]) {
                    self._bursts_loop_count[i] = 0.;
                    // }

                    let temp_burst_loop =
                        ((local_time_diff as f32 - burst_time) / interval).floor();

                    // 预估爆发次数 大于 已爆发次数
                    if temp_burst_loop > self._bursts_loop_count[i] {
                        // 可爆发次数 大于 已爆发次数
                        if burst_loop > self._bursts_loop_count[i] {
                            self._bursts_loop_count[i] += 1.;
                            burst_create_count += burst_count as i32;
                        }
                    }
                }
            }

            let mut new_count = 0.;
            if self.rate_over_time > 0. {
                let per_rate_need_time = 1000. / self.rate_over_time;
                new_count = ((create_delta as f32 + per_rate_need_time * 0.5)
                    * self.rate_over_time
                    / 1000.)
                    .floor();
                if new_count > 0. {
                    self._last_create_time += new_count as u64 * per_rate_need_time as u64;
                }
            } else {
                self._last_create_time = now;
            }
            new_count += burst_create_count as f32;

            if !can_new {
                new_count = 0.;
            }

            let mut new_particle_list = vec![];
            let mut delta_age = 0.;
            // println!("pos: {:?}", self.particleList);
            for i in 0..self.max_particles {
                let particle = &mut self.particle_list[i];

                if self.trail.is_some() && self._enable_trail {
                    delta_age = particle.trial_lifetime * self._trail_lifetime_scale;
                }

                if particle.age - particle.lifetime >= delta_age {
                    if new_count > 0. {
                        new_particle_list.push(particle as *mut Particle);

                        self.active_particle_list.push(particle as *mut Particle);
                        new_count -= 1.;
                    }
                } else {
                    self.active_particle_list.push(particle);
                }
            }
            self.active_particle_list.sort_by(|a, b| {
                let a = unsafe { &*(*a) };
                let b = unsafe { &*(*b) };
                // println!("pos: {:?}", a.position);
                a.age.partial_cmp(&b.age).unwrap()
            });

            let mut emit_world_matrix = Matrix::identity();
            // let _startWorldRM = Matrix::identity();
            let parent_world_matrix = world_matrix.clone();
            let world_matrix = world_matrix;
            let local_matrix = local_matrix;

            // let mut postion = Vector3::new(0., 0., 0.);
            let mut _start_world_rm = Rotation3::identity();
            // let mut scaling = Vector3::new(1., 1., 1.);
            CoordinateSytem3::matrix4_decompose_rotation(
                &world_matrix,
                None,
                Some(&mut _start_world_rm),
                None,
            );

            // worldMatrix.getRotationMatrixToRef(_startWorldRM);
            // let euler_angles = _startWorldRM.euler_angles()
            // Quaternion::from_euler_angles(euler_angles.0, euler_angles.1, euler_angles.2).mat
            // use ncollide::simba::scalar::SubsetOf;
            let _start_world_rm = _start_world_rm.matrix().try_inverse().unwrap();
            let _start_world_rm = Matrix::new(
                _start_world_rm[0],
                _start_world_rm[1],
                _start_world_rm[2],
                0.,
                _start_world_rm[3],
                _start_world_rm[4],
                _start_world_rm[6],
                0.,
                _start_world_rm[6],
                _start_world_rm[7],
                _start_world_rm[8],
                0.,
                0.,
                0.,
                0.,
                1.,
            );

            self._format_start_info(
                Some(parent_world_matrix),
                local_matrix,
                &mut emit_world_matrix,
            );

            self.apply_modifier();

            // for p in &self.activeParticleList{
            //     let r = unsafe { &mut *(*p) };
            //     println!("1activeParticleList pos{:?}: {:?}", *p, r.position);
            // }
            // for p in &newParticleList{
            //     let r = unsafe { &mut *(*p) };
            //     println!("1newParticleList pos{:?}: {:?}", p, r.position);
            // }

            self.var_init(
                new_particle_list.clone(),
                world_matrix,
                _start_world_rm,
                emit_world_matrix,
            );
            // for p in &self.activeParticleList{
            //     let r = unsafe { &mut *(*p) };
            //     println!("2activeParticleList pos{:?}: {:?}", *p, r.position);
            // }
            // for p in &newParticleList{
            //     let r = unsafe { &mut *(*p) };
            //     println!("2newParticleList pos{:?}: {:?}", p, r.position);
            // }

            self.var_pre(
                delta as f32,
                scale_update_speed as f32,
                world_matrix,
                _start_world_rm,
                emit_world_matrix,
            );

            if let Some(_trail) = self.trail.as_mut() {
                if self._enable_trail {
                    let _mat = (self.get_camera_matrix)();

                    // trail.modify(
                    //     Vector3::new(mat[3], mat[7], mat[11]),
                    //     &self.activeParticleList,
                    // );
                }
            }
        }
    }

    fn apply_modifier(&mut self) {
        self.over_lifetime_modifier_list.clear();
        if self.enable_color_over_life_time {
            self.over_lifetime_modifier_list
                .push(Box::new(self.color_over_lifetime_interpolation.clone()));
        }
        if self.enable_force_over_life_time {
            self.over_lifetime_modifier_list
                .push(Box::new(self.force_over_lifetime_interpolation.clone()));
        }
        if self.enable_rotation_over_life_time {
            self.over_lifetime_modifier_list
                .push(Box::new(self.rotation_over_lifetime_interpolation.clone()));
        }
        if self.enable_size_over_life_time {
            self.over_lifetime_modifier_list
                .push(Box::new(self.size_over_lifetime_interpolation.clone()));
        }
        if self.enable_velocity_over_life_time {
            self.over_lifetime_modifier_list
                .push(Box::new(self.velocity_over_lifetime_interpolation.clone()));
        }

        if self.enable_noise {
            self.over_lifetime_modifier_list
                .push(Box::new(self.noise_modifier.clone()));
        }

        self.over_lifetime_modifier_list
            .push(Box::new(self.local_position_modifier.clone()));

        if self.enable_color_by_speed {
            self.over_lifetime_modifier_list
                .push(Box::new(self.color_by_speed_interpolation.clone()));
        }
        if self.enable_rotation_by_speed {
            self.over_lifetime_modifier_list
                .push(Box::new(self.rotation_by_speed_interpolation.clone()));
        }
        if self.enable_size_by_speed {
            self.over_lifetime_modifier_list
                .push(Box::new(self.size_by_speed_interpolation.clone()));
        }

        if self.enable_limit_velocity_over_life_time {
            self.over_lifetime_modifier_list.push(Box::new(
                self.limit_velocity_over_lifetime_interpolation.clone(),
            ));
        }
        if self.enable_texture_sheet {
            self.over_lifetime_modifier_list
                .push(Box::new(self.texture_sheet_interpolation.clone()));
        }
        if self.enable_custom_data_for_main_uv {
            self.over_lifetime_modifier_list
                .push(Box::new(self.custom_data_for_main_uv.clone()));
        }
    }

    /**
     * 初始化
     */
    pub fn var_init(
        &mut self,
        new_particle_list: Vec<*mut Particle>,
        world_matrix: Matrix,
        start_world_matrix_invert: Matrix,
        emit_world_matrix: Matrix,
    ) {
        let new_count = new_particle_list.len();

        let progress = self._particle_system_age % self.duration / self.duration;
        let mut i = 0;
        for particle in new_particle_list {
            let particle = unsafe { &mut *particle };

            particle.reset();

            particle.global_record_list.clear();
            particle.local_record_list.clear();
            particle.start_world_matrix = world_matrix;

            particle.start_world_matrix_invert = start_world_matrix_invert;
            particle.emit_world_matrix = emit_world_matrix;
            // println!(
            //     "particle.emit_world_matrix {:?}",
            //     particle.emit_world_matrix
            // );
            self.init_new(particle, progress, i, new_count);

            i += 1;
        }
    }

    pub fn init_new(
        &mut self,
        particle: &mut Particle,
        progress: u64,
        emission_index: usize,
        emission_total: usize,
    ) {
        let emission_loop = self.emission_loop;
        let emission_progress = self.emission_progress;
        let mut rng = rand::thread_rng();

        let start_speed = self
            .start_speed_interpolation
            .interpolate(progress as f32, rng.gen::<f32>());
        // println!("startSpeed: {:?}", startSpeed);
        let mut start_local_position = Vector3::zeros();
        if let Some(emitter_shape) = &self.emitter_shape {
            emitter_shape.start_position_function(
                particle.start_world_matrix,
                &mut start_local_position,
                emission_loop,
                emission_progress,
                emission_index as f32,
                emission_total as f32,
                true,
            );
        }

        // 发射器局部矩阵影响
        start_local_position = self
            .emitter_shape
            .as_mut()
            .unwrap()
            .get_local_matrix()
            .transform_vector(&start_local_position);

        let mut start_direction = Vector3::zeros();
        // println!(":startDirection0 {:?}, particle.start_world_matrix: {:?}", startDirection, particle.start_world_matrix);
        // println!("startLocalPosition: {:?}", startLocalPosition);
        if let Some(emitter_shape) = &self.emitter_shape {
            // println!("emitterShape: {:?}", emitterShape);
            emitter_shape.start_direction_function(
                particle.start_world_matrix,
                &mut start_direction,
                Vector3::zeros(),
                start_local_position,
                true,
            );
        }
        // println!("startDirection0: {:?}", startDirection);
        start_direction *= start_speed;
        // println!("initNew {:?}", 1);
        particle.age = 0.;
        particle.lifetime = self
            .start_lifetime_interpolation
            .interpolate(progress as f32, rng.gen::<f32>());
        // println!("startLocalPosition: {:?}", startLocalPosition);
        particle.position = start_local_position;
        // println!("startDirection: {:?}", startDirection);
        particle.direction = start_direction;

        // emitRotation
        let x_axis = start_direction[0];
        let y_axis = start_direction[1];
        let z_axis = start_direction[2];
        let yaw = -z_axis.atan2(x_axis) + std::f32::consts::PI / 2.;
        let len = (x_axis * x_axis + z_axis * z_axis).sqrt();
        let pitch = -(y_axis).atan2(len);
        particle.emit_rotation = Vector3::new(pitch, yaw, 0.);
        let mut progress = progress as f32;
        self.start_color_interpolation
            .modify(particle, progress, 0.);
        self.start_rotation_interpolation
            .modify(particle, progress, 0.);
        self.start_size_interpolation.modify(particle, progress, 0.);
        if self.enable_texture_sheet {
            self.texture_sheet_interpolation.set_run_as_start(true);
            self.texture_sheet_interpolation
                .modify(particle, &mut (progress), 0.);
        }

        particle.trial_lifetime = 0.;
        particle.trial_width = 1.;
        if let Some(trail) = self.trail.as_mut() {
            if self._enable_trail {
                if rng.gen::<f32>() <= trail.ratio {
                    particle.trial_lifetime =
                        trail.lifetime.interpolate(progress, rng.gen::<f32>()) * particle.lifetime;
                    if trail.size_affects_lifetime {
                        particle.trial_lifetime *= particle.start_scaling.magnitude();
                    }
                }
                if trail.width_over_trail.mode == EInterpolationCurveMode::Constant
                    || trail.width_over_trail.mode == EInterpolationCurveMode::TwoConstants
                {
                    particle.trial_width = trail
                        .width_over_trail
                        .interpolate(progress, rng.gen::<f32>());
                }
            }
        }

        start_direction = normalize(&start_direction);
        start_direction = start_direction * (0.01);
        particle.local_record_list.push([
            -0.002,
            start_local_position[0] - start_direction[0],
            start_local_position[1] - start_direction[1],
            start_local_position[2] - start_direction[2],
        ]);
        particle.local_record_list.push([
            -0.001,
            start_local_position[0],
            start_local_position[1],
            start_local_position[2],
        ]);
        start_local_position = particle
            .start_world_matrix
            .transform_vector(&start_local_position);
        start_direction = particle
            .start_world_matrix
            .transform_vector(&start_direction);

        particle.global_record_list.push([
            -0.002,
            start_local_position[0] - start_direction[0],
            start_local_position[1] - start_direction[1],
            start_local_position[2] - start_direction[2],
        ]);
        particle.global_record_list.push([
            -0.001,
            start_local_position[0],
            start_local_position[1],
            start_local_position[2],
        ]);
    }

    /**
     * 变量预处理
     * @param index
     */
    pub fn var_pre(
        &mut self,
        delta_time_ms: f32,
        scale_update_speed: f32,
        world_matrix: Matrix,
        start_world_matrix_invert: Matrix,
        emit_world_matrix: Matrix,
    ) {
        let active_particles = &mut self.active_particle_list;
        let count = active_particles.len();
        let mut global_amount = self._particle_system_age % self.duration / self.duration;
        global_amount = 0u64.max(1u64.min(global_amount));

        self._max_lifetime_overage = 0;
        for i in 0..count {
            let particle = unsafe { &mut *active_particles[i] };
            // println!("varPre particle.pos:{}", particle.position);
            // if i == 0 {
            //     println!("{}particle.age:{}", i, particle.age);
            // }
            particle.age += delta_time_ms;
            // if i == 0 {
            //     println!("{}particle.age:{}", i, particle.age);
            // }
            self._max_lifetime_overage = self._max_lifetime_overage.max(
                (particle.lifetime - particle.age
                    + particle.trial_lifetime * self._trail_lifetime_scale) as u64,
            );

            if particle.age <= particle.lifetime {
                if self.simulation_space == EMeshParticleSpaceMode::Local {
                    particle.start_world_matrix = world_matrix;
                    particle.start_world_matrix_invert = start_world_matrix_invert;
                    particle.emit_world_matrix = emit_world_matrix;
                }

                let mut particle_amount = particle.age / particle.lifetime;
                particle_amount = 1.0f32.min(particle_amount);

                self.gravity_interpolation.modify(
                    particle,
                    global_amount as f32,
                    scale_update_speed,
                );

                self.texture_sheet_interpolation.set_run_as_start(false);

                // 重置scaling
                particle.color = particle.start_color;
                particle.scaling = particle.start_scaling;

                let modifier_count = self.over_lifetime_modifier_list.len();
                // println!("=========== modifierCount:{}", modifierCount);

                // let r = unsafe { &mut *(*p) };
                // println!("varPre1 pos: {:?}", particle.position);

                for j in 0..modifier_count {
                    let modifier = &mut self.over_lifetime_modifier_list[j];
                    modifier.modify(particle, &mut particle_amount, scale_update_speed);
                }
                // println!("varPre2 pos: {:?}", particle.position);
            }
        }
    }
}

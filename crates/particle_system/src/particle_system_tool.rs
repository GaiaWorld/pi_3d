// use std::time::UNIX_EPOCH;

// use pi_scene_math::{
//     coordiante_system::CoordinateSytem3, vector::TToolMatrix, Matrix, Quaternion, Rotation3,
//     Vector3,
// };
// use pi_wy_rng::WyRng;

// use crate::{
//     emitter::IShapeEmitterType,
//     interpolation::{Color4Gradient, FloatInterpolation, IInterpolation},
//     iparticle_system_config::EInterpolationCurveMode,
//     modifier::*,
//     tools::{multiply, normalize, Random, TBurstData},
//     particle::Particle,
// };

// #[derive(Clone, Copy, PartialEq, Default)]
// pub enum EMeshParticleSpaceMode {
//     #[default]
//     Local = 0,
//     /**
//      * 发射在世界空间时, 父级尽量不要有旋转动画, 因为 动画 与 粒子动画的衔接有误差，无法完美适配
//      */
//     World = 1,
// }

// #[derive(Clone, Copy, PartialEq, Default)]
// pub enum EMeshParticleScaleMode {
//     #[default]
//     Hierarchy = 0,
//     Local = 1,
//     Shape = 2,
// }

// #[derive(Clone, Copy, PartialEq, Default)]
// pub enum EParticleRenderAlignment {
//     /**
//      * 粒子面向相机平面。
//      */
//     #[default]
//     View = 0,
//     /**
//      * 粒子与世界轴对齐
//      */
//     World = 1,
//     /**
//      * Local 无需额外适配
//      */
//     Local = 2,
//     /**
//      * 粒子面向相机的位置点
//      */
//     Facing = 3,
//     /**
//      * 粒子朝向它们的速度方向。
//      */
//     Velocity = 4,
// }

// #[derive(Clone, Copy, Default, PartialEq, Debug)]
// pub enum EParticleRenderMode {
//     #[default]
//     Billboard = 0,
//     StretchedBillboard = 1,
//     HorizontalBillboard = 2,
//     VerticalBillboard = 3,
//     Mesh = 4,
//     None = 5,
// }

// /**
//  * 对接 Unity ParticleSystem Mesh Mode
//  */
// pub struct ParticleSystemTool {
//     pub name: String,
//     pub sqrt3: f32,

//     _one_vector3: Vector3,
//     /**
//      * 计算的间隔时间
//      */
//     pub compute_delta_time: u64,
//     /**
//      * 最大粒子数
//      */
//     pub max_particles: usize,
//     // private _maxParticles: number = 100;
//     // public get maxParticles() {
//     //     return self._maxParticles;
//     // }
//     // public set maxParticles(value: number) {
//     //     self._maxParticles = value + 1;
//     // }
//     /**
//      * 是否循环
//      */
//     pub looping: bool,
//     /**
//      * 是否预热
//      */
//     pub prewarm: bool,
//     /**
//      * 粒子系统发射持续时间
//      */
//     pub duration: u64,
//     _loop_count: f32,
//     /**
//      * 延时时间 - 毫秒
//      */
//     pub start_delay: i32,
//     pub emitter_shape: Option<Box<dyn IShapeEmitterType>>,
//     /**
//      * 一次发射循环的时间 - 毫秒
//      */
//     pub emission_time: f32,
//     /**
//      * 当前已循环次数
//      */
//     pub emission_loop: f32,
//     /**
//      * 当前发射循环的进度
//      */
//     pub emission_progress: f32,
//     /**
//      * 每秒发射的粒子数目
//      */
//     pub rate_over_time: f32,
//     /**
//      * 指定时间点 开始 持续 指定时间内 发射指定数目
//      * [开始时间, 发射数目, 循环次数, 间隔]
//      * @tip 需要按 开始时间从小到大排序
//      */
//     pub bursts: Vec<TBurstData>,
//     _bursts_loop_count: Vec<f32>,

//     /**
//      * 发射空间
//      * * Local - 本地: 发射开始时的方向受父级影响, 发射后的位置也受父级影响
//      * * World - 世界空间: 发射开始时的方向受父级影响, 发射后的位置不再受父级影响
//      */
//     pub simulation_space: EMeshParticleSpaceMode,
//     pub scaling_space: EMeshParticleScaleMode,
//     _render_alignment: EParticleRenderAlignment,
//     _render_mode: EParticleRenderMode,

//     pub render_pivot: Vector3,

//     pub start_lifetime_interpolation: FloatInterpolation,
//     pub start_speed_interpolation: FloatInterpolation,
//     pub start_size_interpolation: StartSize,
//     pub start_rotation_interpolation: StartRotation,
//     pub start_color_interpolation: StartColor,

//     pub gravity_interpolation: Gravity,
//     /**
//      * 位移速度
//      * @tip 默认模式 - ConstantsUnite - XYZ 共用一个随机数
//      * @tip 为 Constants | ConstantsUnite 模式应用插值时, 仅创建时使用, 意义为: 创建时速度基础上附加速度
//      * @tip 为 Curves 模式时, 运行过程中使用, 意义为: 创建时速度基础上附加速度 - 每次循环累加 此次循环 与 上次循环 速度值的差值
//      */
//     pub velocity_over_lifetime_interpolation: VelocityOverLifetime,
//     pub enable_velocity_over_life_time: bool,

//     pub limit_velocity_over_lifetime_interpolation: LimitVelocityOverLifetime,
//     pub enable_limit_velocity_over_life_time: bool,

//     pub force_over_lifetime_interpolation: ForceOverLifetime,
//     pub enable_force_over_life_time: bool,

//     pub color_over_lifetime_interpolation: ColorOverLifetime,
//     pub enable_color_over_life_time: bool,
//     pub color_by_speed_interpolation: ColorBySpeed,
//     pub enable_color_by_speed: bool,
//     /**
//      * 缩放
//      * @tip 默认模式 - ConstantsUnite - XYZ 共用一个随机数
//      * @tip 为 Constants | ConstantsUnite 模式应用插值时, 仅创建时使用, 意义为: 仅对创建时大小的影响倍数
//      * @tip 为 Curves 模式时, 运行过程中使用, 意义为: 创建时大小的影响倍数
//      */
//     pub size_over_lifetime_interpolation: SizeOverLifetime,
//     pub enable_size_over_life_time: bool,
//     pub size_by_speed_interpolation: SizeBySpeed,
//     pub enable_size_by_speed: bool,

//     pub local_position_modifier: OrbitVelocityModifier,
//     /**
//      * 旋转
//      * @tip 默认模式 - ConstantsUnite - XYZ 共用一个随机数
//      * @tip Unity 中单一 Angle 对应 只处理 Y 轴
//      * @tip 与 Unity 编辑时不同, 此处单位为 弧度
//      * @tip 为 Constants | ConstantsUnite 模式时, 仅创建时使用, 意义为: 每秒旋转速度
//      * @tip 为 Curves 模式时, 运行过程中使用, 意义为: 每秒旋转速度
//      */
//     pub rotation_over_lifetime_interpolation: RotationOverLifetime,
//     pub enable_rotation_over_life_time: bool,
//     pub rotation_by_speed_interpolation: RotationBySpeed,
//     pub enable_rotation_by_speed: bool,
//     /**
//      * Texture Sheet Animation
//      */
//     pub texture_sheet_interpolation: TextureSheet,
//     pub enable_texture_sheet: bool,

//     pub custom_data_for_main_uv: CustomDataForUV,
//     pub enable_custom_data_for_main_uv: bool,

//     pub noise_modifier: Noise,
//     pub enable_noise: bool,

//     pub stretched_velocity_scale: f32,
//     pub stretched_length_scale: f32,

//     //  scene: pubScene;
//     pub trail: Option<TrailModifier>,
//     _enable_trail: bool,
//     _trail_lifetime_scale: f32,

//     _max_id: usize,

//     _max_lifetime_overage: u64,
//     _step_time: f32,
// }

// impl ParticleSystemTool {
//     pub fn set_render_alignment(&mut self, value: EParticleRenderAlignment) {
//         self._render_alignment = value;
//     }
//     pub fn get_render_alignment(&self) -> EParticleRenderAlignment {
//         return self._render_alignment.clone();
//     }

//     pub fn set_render_mode(&mut self, value: EParticleRenderMode) {
//         self._render_mode = value;
//     }
//     pub fn get_render_mode(&mut self) -> EParticleRenderMode {
//         return self._render_mode;
//     }

//     pub fn set_enable_trail(&mut self, value: bool) {
//         self._enable_trail = value;
//         if let Some(v) = &mut self.trail {
//             v._enabled = value;
//         }
//         // Trail
//         self._trail_lifetime_scale = 0.;
//         if value {
//             if let Some(v) = &mut self.trail {
//                 if !v.die_with_particle {
//                     self._trail_lifetime_scale = 1.;
//                 }
//             }
//         }
//     }
//     pub fn get_enable_trail(&self) -> bool {
//         return self._enable_trail;
//     }

//     pub fn new() -> Self {
//         Self {
//             name: "".to_string(),
//             sqrt3: 3.0f32.sqrt(),
//             _one_vector3: Vector3::new(1., 1., 1.),
//             compute_delta_time: 15,
//             max_particles: 100,
//             looping: true,
//             prewarm: true,
//             duration: 5,
//             _loop_count: 0.,
//             start_delay: 0,
//             emitter_shape: None,
//             emission_time: 1000.,
//             emission_loop: 0.,
//             emission_progress: 0.,
//             rate_over_time: 10.,
//             bursts: vec![],
//             _bursts_loop_count: vec![],
//             simulation_space: EMeshParticleSpaceMode::Local,
//             scaling_space: EMeshParticleScaleMode::Hierarchy,
//             _render_alignment: EParticleRenderAlignment::Local,
//             _render_mode: EParticleRenderMode::Billboard,
//             render_pivot: Vector3::zeros(),
//             start_lifetime_interpolation: FloatInterpolation::default(),
//             start_speed_interpolation: FloatInterpolation::default(),
//             start_size_interpolation: StartSize::default(),
//             start_rotation_interpolation: StartRotation::new(
//                 FloatInterpolation::default(),
//                 FloatInterpolation::default(),
//                 FloatInterpolation::default(),
//             ),
//             start_color_interpolation: StartColor::new(Color4Interpolate::new(
//                 Color4Gradient::default(),
//             )),
//             gravity_interpolation: Gravity::new(FloatInterpolation::new()),
//             velocity_over_lifetime_interpolation: VelocityOverLifetime::new(
//                 FloatInterpolation::default(),
//                 FloatInterpolation::default(),
//                 FloatInterpolation::default(),
//             ),
//             enable_velocity_over_life_time: false,
//             limit_velocity_over_lifetime_interpolation: LimitVelocityOverLifetime::new(
//                 FloatInterpolation::default(),
//             ),
//             enable_limit_velocity_over_life_time: false,
//             force_over_lifetime_interpolation: ForceOverLifetime::new(TranslationInterpolate::new(
//                 FloatInterpolation::default(),
//                 FloatInterpolation::default(),
//                 FloatInterpolation::default(),
//             )),
//             enable_force_over_life_time: false,
//             color_over_lifetime_interpolation: ColorOverLifetime::new(Color4Interpolate::new(
//                 Color4Gradient::default(),
//             )),
//             enable_color_over_life_time: false,
//             color_by_speed_interpolation: ColorBySpeed::new(Color4Interpolate::new(
//                 Color4Gradient::default(),
//             )),
//             enable_color_by_speed: false,
//             size_over_lifetime_interpolation: SizeOverLifetime::new(
//                 FloatInterpolation::default(),
//                 FloatInterpolation::default(),
//                 FloatInterpolation::default(),
//             ),
//             enable_size_over_life_time: false,
//             size_by_speed_interpolation: SizeBySpeed::new(
//                 FloatInterpolation::default(),
//                 FloatInterpolation::default(),
//                 FloatInterpolation::default(),
//             ),
//             enable_size_by_speed: false,
//             local_position_modifier: OrbitVelocityModifier::new(),
//             rotation_over_lifetime_interpolation: RotationOverLifetime::new(
//                 FloatInterpolation::default(),
//                 FloatInterpolation::default(),
//                 FloatInterpolation::default(),
//             ),
//             enable_rotation_over_life_time: false,
//             rotation_by_speed_interpolation: RotationBySpeed::new(
//                 FloatInterpolation::default(),
//                 FloatInterpolation::default(),
//                 FloatInterpolation::default(),
//             ),
//             enable_rotation_by_speed: false,
//             texture_sheet_interpolation: TextureSheet::new(
//                 FloatInterpolation::default(),
//                 FloatInterpolation::default(),
//             ),
//             enable_texture_sheet: false,
//             custom_data_for_main_uv: CustomDataForUV::new(),
//             enable_custom_data_for_main_uv: false,
//             noise_modifier: Noise::new(),
//             enable_noise: false,
//             stretched_velocity_scale: 0.,
//             stretched_length_scale: 1.,
//             trail: Some(TrailModifier::new()),
//             _enable_trail: false,
//             _trail_lifetime_scale: 0.,
//             _max_id: 0,
//             _max_lifetime_overage: 0,
//             _step_time: 16.0,
//         }
//     }
// }

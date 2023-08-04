use crate::{
    emitter::{EBoxShapeMode, EShapeEmitterArcMode},
    interpolation::{ICurve, IGradient},
    modifier::*,
    tools::*,
};


/**
 * ArcMode 类型
 */
pub const TSHAPE_ARC_MODE_RANDOM: isize = 1;
pub const TSHAPE_ARC_MODE_LOOP: isize = 2;
pub const TSHAPE_ARC_MODE_PING_PONG: isize = 3;
pub const TSHAPE_ARC_MODE_BURST_SPREAD: isize = 4;

/**
 * 曲线类型
 */
const TINTERPOLATE_CONSTANT: isize = 1;
const TINTERPOLATE_TWO_CONSTANTS: isize = 2;
const TINTERPOLATE_CURVE: isize = 4;
const TINTERPOLATE_TWO_CURVES: isize = 8;

/**
 * 渐变类型
 */
const TINTERPOLATE_COLOR: isize = 1;
const TINTERPOLATE_TWO_COLORS: isize = 2;
const TINTERPOLATE_GRADIENT: isize = 4;
const TINTERPOLATE_TWO_GRADIENTS: isize = 8;
const TINTERPOLATE_RANDOM: isize = 16;

/**
 * 默认数据配置
 */
pub struct DefaultValue;

impl DefaultValue {
    pub const DURATION: u32 = 5;
    pub const START_SPEED: f32 = 1.0;
    pub const START_LIFETIME: f32 = 2.;
    pub const TEXTURE_SHEET_FRAME: f32 = 0.;
    pub const START_SIZE: [u32; 3] = [1, 1, 1];
    pub const START_ROTATION: [u32; 3] = [0, 0, 0];
    pub const START_COLOR: [u32; 4] = [1, 1, 1, 1];
    pub const GRAVITY: [u32; 3] = [0, 0, 0];
    pub const VELOCITY_OVER_LIFETIME: [u32; 3] = [0, 0, 0];
    pub const LIMIT_VELOCITY_OVER_LIFETIME: [u32; 3] = [999999, 999999, 999999];
    pub const LIMIT_VELOCITY_OVER_LIFETIME_DAMPEN: u32 = 0;
    pub const FORCE_OVER_LIFETIME: [u32; 3] = [0, 0, 0];
    pub const COLOR_OVER_LIFETIME: [u32; 4] = [1, 1, 1, 1];
    pub const SIZE_OVER_LIFETIME: [u32; 3] = [1, 1, 1];
    pub const ROTATION_OVER_LIFETIME: [u32; 3] = [0, 0, 0];
}

pub enum TParamType {
    TParamStartColor = 1,
    TParamStartSpeed,
    TParamStartLifetime,
    TParamStartSize,
    TParamStartRotation,
    TParamGravity,
    TParamVelocityOverLifetime,
    TParamLimitVelocityOverLifetime,
    TParamForceOverLifetime,
    TParamColorOverLifetime,
    TParamSizeOverLifetime,
    TParamRotationOverLifetime,
    TParamTextureSheet,
    None,
}

pub enum IShapeArc {
    IShapeArcRandom(IShapeArcRandom),
    IShapeArcLoop(IShapeArcLoop),
    IShapeArcPingPong(IShapeArcPingPong),
    IShapeArcBurstSpread(IShapeArcBurstSpread),
}

impl Default for IShapeArc {
    fn default() -> Self {
        IShapeArc::IShapeArcRandom(IShapeArcRandom::default())
    }
}
/**
 * 曲线插值模式
 */
#[derive(Debug, PartialEq, Clone)]
pub enum EInterpolationCurveMode {
    /**
     * 静态数值
     */
    Constant = TINTERPOLATE_CONSTANT,
    /**
     * 静态数值随机 - XYZ 随机值相同
     */
    TwoConstants = TINTERPOLATE_TWO_CONSTANTS,
    /**
     * 曲线插值
     */
    Curve = TINTERPOLATE_CURVE,
    /**
     * 曲线插值
     */
    TwoCurves = TINTERPOLATE_TWO_CURVES,
}

impl Default for EInterpolationCurveMode {
    fn default() -> EInterpolationCurveMode {
        EInterpolationCurveMode::Constant
    }
}
/**
 * 渐变插值模式
 */
#[derive(Clone, Debug)]
pub enum EInterpolationGradienMode {
    /**
     * 静态数值
     */
    Color = TINTERPOLATE_COLOR,
    /**
     * 静态数值随机 - XYZ 随机值相同
     */
    TwoColors = TINTERPOLATE_TWO_COLORS,
    /**
     * 曲线插值
     */
    Gradient = TINTERPOLATE_GRADIENT,
    /**
     * 曲线插值
     */
    TwoGradients = TINTERPOLATE_TWO_GRADIENTS,
    /**
     * 曲线插值
     */
    Random = TINTERPOLATE_RANDOM,
}
impl Default for EInterpolationGradienMode {
    fn default() -> EInterpolationGradienMode {
        EInterpolationGradienMode::Color
    }
}
/**
 * 形状发射器 Arc 信息
 */
//  type IShapeArc = IShapeArcRandom | IShapeArcLoop | IShapeArcPingPong | IShapeArcBurstSpread;
/**
 * 弧度范围发射信息描述 - Random 模式
 */
#[derive(Default, Clone)]
pub struct IShapeArcRandom {
    /**
     * 模式 - EShapeEmitterArcMode.Random
     */
    pub mode: EShapeEmitterArcMode,
    /**
     * 发射的弧形范围
     */
    pub value: f32,
    /**
     * 0~1
     */
    pub spread: f32,
    pub speed: f32,
}
/**
 * 弧度范围发射信息描述 - Lopp 模式
 */
pub struct IShapeArcLoop {
    /**
     * 模式 - EShapeEmitterArcMode.Loop
     */
    pub mode: EShapeEmitterArcMode,
    /**
     * 发射的弧形范围 - 角度值
     */
    pub value: f32,
    /**
     * 0~1
     */
    pub spread: f32,
    pub speed: f32,
}
/**
 * 弧度范围发射信息描述 - PingPong 模式
 */
pub struct IShapeArcPingPong {
    /**
     * 模式 - EShapeEmitterArcMode.PingPong
     */
    pub mode: EShapeEmitterArcMode,
    /**
     * 发射的弧形范围
     */
    pub value: f32,
    /**
     * 0~1
     */
    pub spread: f32,
    pub speed: f32,
}
/**
 * 弧度范围发射信息描述 - BurstSpread 模式
 */
pub struct IShapeArcBurstSpread {
    /**
     * 模式 - EShapeEmitterArcMode.BurstSpread
     */
    pub mode: EShapeEmitterArcMode,
    /**
     * 发射的弧形范围
     */
    pub value: f32,
    /**
     * 0~1
     */
    pub spread: f32,
    pub speed: f32,
}
//  type TShapeArcMode  = typeof TShapeArcModeRandom | typeof TShapeArcModeLoop | typeof TShapeArcModePingPong | typeof TShapeArcModeBurstSpread;
/**
 * Cone 形状发射器描述
 */
pub struct IShapeCone {
    /**
     * 类型标识
     */
    pub _type: u32,
    /**
     * 半径
     */
    pub radius: f32,
    /**
     * 角度
     */
    pub angle: f32,
    /**
     * 半径范围
     */
    pub radius_thickness: f32,
    /**
     * 发射弧度信息
     */
    pub arc: IShapeArc,
    /**
     * 沿高度体积发射
     */
    pub emit_as_volume: bool,
    pub height: f32,
    pub scale: Option<[f32; 3]>,
    pub position: Option<[f32; 3]>,
    pub rotation: Option<[f32; 3]>,
    pub align_dir: u32,
    pub randomize: Option<[f32; 3]>,
}
impl Default for IShapeCone {
    fn default() -> Self {
        Self {
            _type: 0,
            radius: 1.,
            angle: 60.,
            radius_thickness: 1.,
            arc: IShapeArc::IShapeArcRandom(IShapeArcRandom { mode: EShapeEmitterArcMode::Random, value: 360., spread: 1., speed: 1. }),
            emit_as_volume: true,
            height: 5.,
            scale: None,
            position: None,
            rotation: None,
            align_dir: 0,
            randomize: None,
        }
    }
}
/**
 * Sphere 形状发射器描述
 */
pub struct IShapeSphere {
    pub _type: u32,
    pub radius: f32,
    pub radius_thickness: f32,
    pub arc: IShapeArc,
    pub scale: Option<[f32; 3]>,
    pub position: Option<[f32; 3]>,
    pub rotation: Option<[f32; 3]>,
    pub align_dir: u32,
    pub randomize: Option<[f32; 3]>,
}
/**
 * Box 形状发射器描述
 */
#[derive(Clone,Default)]
pub struct IShapeBox {
    pub _type: u32,
    pub is_volume: u32,
    pub scale: Option<[f32; 3]>,
    pub position: Option<[f32; 3]>,
    pub rotation: Option<[f32; 3]>,
    pub align_dir: u32,
    pub randomize: Option<[f32; 3]>,
    pub box_emit_mode: Option<EBoxShapeMode>,
}
/**
 * Hemisphere 形状发射器描述
 */
pub struct IShapeHemisphere {
    pub _type: u32,
    pub radius: f32,
    pub radius_thickness: f32,
    pub arc: IShapeArc,
    pub scale: Option<[f32; 3]>,
    pub position: Option<[f32; 3]>,
    pub rotation: Option<[f32; 3]>,
    pub align_dir: u32,
    pub randomize: Option<[f32; 3]>,
}
/**
 * Circle 形状发射器描述
 */
pub struct IShapeCircle {
    pub _type: u32,
    pub radius: f32,
    pub radius_thickness: f32,
    pub arc: IShapeArc,
    pub scale: Option<[f32; 3]>,
    pub position: Option<[f32; 3]>,
    pub rotation: Option<[f32; 3]>,
    pub align_dir: u32,
    pub randomize: Option<[f32; 3]>,
}
/**
 * Edge 形状发射器描述
 */
pub struct IShapeEdge {
    pub _type: u32,
    pub radius: f32,
    pub arc: IShapeArc,
    pub scale: Option<[f32; 3]>,
    pub position: Option<[f32; 3]>,
    pub rotation: Option<[f32; 3]>,
    pub align_dir: u32,
    pub randomize: Option<[f32; 3]>,
}
/**
 * Rectangle 形状发射器描述
 */
pub struct IShapeRectangle {
    pub _type: u32,
    pub scale: Option<[f32; 3]>,
    pub position: Option<[f32; 3]>,
    pub rotation: Option<[f32; 3]>,
    pub align_dir: u32,
    pub randomize: Option<[f32; 3]>,
}
/**
 * 形状发射器类型限制
 */
//  type IShape = IShapeBox | IShapeCircle | IShapeCone | IShapeEdge | IShapeHemisphere | IShapeRectangle | IShapeSphere;
pub enum IShape {
    ShapeBox(IShapeBox),
    ShapeCircle(IShapeCircle),
    ShapeCone(IShapeCone),
    ShapeEdge(IShapeEdge),
    ShapeHemisphere(IShapeHemisphere),
    ShapeRectangle(IShapeRectangle),
    ShapeSphere(IShapeSphere),
}

impl Default for IShape{
    fn default() -> Self {
        IShape::ShapeBox(IShapeBox::default())
    }
}

type OneParam = f32;
type ThreeParam = [f32; 3];
type FourParam = [f32; 4];
type OneParamCurve = ICurve;
type ThreeParamCurve = [ICurve; 3];
type FourParamCurve = [ICurve; 4];
type FourGradient = [Vec<IGradient>; 4];

pub enum ParamInfo {
    OneParamInfo(OneParamInfo),
    ThreeParamInfo(ThreeParamInfo),
}

impl Default for ParamInfo {
    fn default() -> Self {
        ParamInfo::OneParamInfo(OneParamInfo::TInterpolateConstant(0.0))
    }
}

#[derive(Clone)]
pub enum OneParamInfo {
    TInterpolateConstant(OneParam),
    TInterpolateTwoConstants(OneParam, OneParam),
    TInterpolateCurve(OneParamCurve),
    TInterpolateTwoCurves(OneParamCurve, OneParamCurve),
}

impl Default for OneParamInfo {
    fn default() -> Self {
        OneParamInfo::TInterpolateConstant(0.0)
    }
}
    
pub enum ThreeParamInfo {
    TInterpolateConstant(ThreeParam),
    TInterpolateTwoConstants(ThreeParam, ThreeParam),
    TInterpolateCurve(ThreeParamCurve),
    TInterpolateTwoCurves(ThreeParamCurve, ThreeParamCurve),
}

pub enum FourParamInfo {
    TInterpolateConstant(FourParam),
    TInterpolateTwoConstants(FourParam, FourParam),
    TInterpolateCurve(FourParamCurve),
    TInterpolateTwoCurves(FourParamCurve, FourParamCurve),
}

#[derive(Clone, Default)]
pub enum FourGradientInfo {
    #[default] TInterpolateRandom,
    TInterpolateColor(FourParam),
    TInterpolateTwoColors(FourParam, FourParam),
    TInterpolateGradient(FourGradient),
    TInterpolateTwoGradients(FourGradient, FourGradient),
}

pub struct ITextureSheet {
    pub frame_over_time: OneParamInfo,
    pub anim_mode: AnimationMode,
    pub custom_row: f32,
    pub cycles: f32,
    pub row_mode: RowMode,
    pub start_frame: OneParamInfo,
    pub tiles_x: f32,
    pub tiles_y: f32,
    pub time_mode: TimeMode,
}

pub struct ITrail {
    pub ratio: f32,
    pub mode: ETrailMode,
    pub lifetime: OneParamInfo,
    pub ribbon_count: f32,
    pub attach_rtt: u32,
    pub min_dist: f32,
    pub world_space: u32,
    pub die_with: u32,
    pub tex_mode: ETrailTextureMode,
    pub size_awidth: u32,
    pub size_alifetime: u32,
    pub inherit_color: u32,
    pub color_over_life: FourGradientInfo,
    pub width_over_trail: OneParamInfo,
    pub color_over_trail: FourGradientInfo,
    pub material: f32,
}

/**
 * 粒子系统JSON描述
 */

#[derive(Default)]
pub struct IParticleSystemConfig {
    pub name: String,
    pub duration: f32,
    pub start_delay: f32,
    pub looping: u32,
    pub prewarm: bool,
    pub simulation_space_is_world: EMeshParticleSpaceMode,
    pub scaling_mode: EMeshParticleScaleMode,
    pub render_alignment: EParticleRenderAlignment,
    pub render_mode: EParticleRenderMode,
    pub stretched_velocity_scale: f32,
    pub stretched_length_scale: f32,
    pub max_particles: f32,
    pub start_speed: OneParamInfo,
    pub lifetime: OneParamInfo,
    pub delay: Vec<f32>,
    pub start_color: FourGradientInfo,
    pub start_size: ParamInfo,
    pub start_rotation: ParamInfo,
    pub gravity: OneParamInfo,
    pub emission: (f32, Option<Vec<[f32; 4]>>),
    pub shape: IShape,
    pub velocity_over_lifetime: Option<ParamInfo>,
    pub velocity_over_lifetime_is_local: Option<u32>,
    pub limit_velocity_over_lifetime: Option<OneParamInfo>,
    pub limit_velocity_over_lifetime_dampen: Option<f32>,
    pub force_over_lifetime: Option<ParamInfo>,
    pub force_space_is_local: Option<u32>,
    pub color_over_lifetime: Option<FourGradientInfo>,
    pub color_by_speed: Option<(FourGradientInfo, f32, f32)>,
    pub size_over_lifetime: Option<ParamInfo>,
    pub size_by_speed: Option<(OneParamInfo, f32, f32)>,
    pub rotation_over_lifetime: Option<ParamInfo>,
    pub rotation_by_speed: Option<(OneParamInfo, f32, f32)>,
    pub texture_sheet: Option<ITextureSheet>,
    pub texture: Option<String>,
    pub trail: Option<ITrail>,
    pub orbtial_velocity: Option<ParamInfo>,
    pub orbital_offset: Option<ParamInfo>,
    pub orbital_radial: Option<OneParamInfo>,
    pub speed_modifier: Option<OneParamInfo>,
    pub render_pivot: Option<[f32; 3]>,
    pub custom1: Option<[OneParamInfo; 4]>,
}
impl IParticleSystemConfig {
    pub fn new() -> Self {
        let mut result = Self::default();

        result.looping = 1;
        result.duration = 5.;
        result.max_particles = 1000.;
        result.emission = (10., None);
        result.lifetime = OneParamInfo::TInterpolateConstant(1.);
        result.start_speed = OneParamInfo::TInterpolateConstant(1.);
        result.start_size = ParamInfo::OneParamInfo(OneParamInfo::TInterpolateConstant(1.));
        result.start_color = FourGradientInfo::TInterpolateColor([1., 1., 1., 1.]);
        result.start_rotation = ParamInfo::OneParamInfo(OneParamInfo::TInterpolateConstant(0.));
        result.shape = IShape::ShapeCone(IShapeCone::default());

        result
    }
}



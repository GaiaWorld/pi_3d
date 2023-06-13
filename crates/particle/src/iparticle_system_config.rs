use crate::{
    emitter::ishape_emitter_type::{EBoxShapeMode, EShapeEmitterArcMode},
    interpolation::{ICurve, IGradient},
    modifier::texture_sheet::TimeMode,
    modifier::{
        texture_sheet::{AnimationMode, RowMode},
        trail::{ETrailMode, ETrailTextureMode},
    },
    particle_system_tool::{
        EMeshParticleScaleMode, EMeshParticleSpaceMode, ERenderAlignment, ERenderMode,
    },
};
/**
 * 形状发射器类型
 */
const TShapeTypeCone: u32 = 0;
const TShapeTypeSphere: u32 = 1;
const TShapeTypeBox: u32 = 2;
const TShapeTypeCircle: u32 = 3;
const TShapeTypeHemisphere: u32 = 4;
const TShapeTypeEdge: u32 = 5;
const TShapeTypeRectangle: u32 = 6;

/**
 * ArcMode 类型
 */
pub const TShapeArcModeRandom: isize = 1;
pub const TShapeArcModeLoop: isize = 2;
pub const TShapeArcModePingPong: isize = 3;
pub const TShapeArcModeBurstSpread: isize = 4;

/**
 * 曲线类型
 */
const TInterpolateConstant: isize = 1;
const TInterpolateTwoConstants: isize = 2;
const TInterpolateCurve: isize = 4;
const TInterpolateTwoCurves: isize = 8;

/**
 * 渐变类型
 */
const TInterpolateColor: isize = 1;
const TInterpolateTwoColors: isize = 2;
const TInterpolateGradient: isize = 4;
const TInterpolateTwoGradients: isize = 8;
const TInterpolateRandom: isize = 16;

/**
 * josn数据中 Emission 数据Index类型
 */
const TEmissionIndexRate: u32 = 0;
const TEmissionIndexBursts: u32 = 1;

/**
 * 默认数据配置
 */
pub struct DefaultValue;

impl DefaultValue {
    pub const duration: u32 = 5;
    pub const startSpeed: f32 = 1.0;
    pub const startLifetime: f32 = 2.;
    pub const textureSheetFrame: f32 = 0.;
    pub const startSize: [u32; 3] = [1, 1, 1];
    pub const startRotation: [u32; 3] = [0, 0, 0];
    pub const startColor: [u32; 4] = [1, 1, 1, 1];
    pub const gravity: [u32; 3] = [0, 0, 0];
    pub const velocityOverLifetime: [u32; 3] = [0, 0, 0];
    pub const limitVelocityOverLifetime: [u32; 3] = [999999, 999999, 999999];
    pub const limitVelocityOverLifetimeDampen: u32 = 0;
    pub const forceOverLifetime: [u32; 3] = [0, 0, 0];
    pub const colorOverLifetime: [u32; 4] = [1, 1, 1, 1];
    pub const sizeOverLifetime: [u32; 3] = [1, 1, 1];
    pub const rotationOverLifetime: [u32; 3] = [0, 0, 0];
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
/**
 * 曲线插值模式
 */
enum RandomUnit {
    One = 1,
    Three = 3,
}
/**
 * 曲线插值模式
 */
#[derive(Debug, PartialEq, Clone)]
pub enum EInterpolationCurveMode {
    /**
     * 静态数值
     */
    Constant = TInterpolateConstant,
    /**
     * 静态数值随机 - XYZ 随机值相同
     */
    TwoConstants = TInterpolateTwoConstants,
    /**
     * 曲线插值
     */
    Curve = TInterpolateCurve,
    /**
     * 曲线插值
     */
    TwoCurves = TInterpolateTwoCurves,
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
    Color = TInterpolateColor,
    /**
     * 静态数值随机 - XYZ 随机值相同
     */
    TwoColors = TInterpolateTwoColors,
    /**
     * 曲线插值
     */
    Gradient = TInterpolateGradient,
    /**
     * 曲线插值
     */
    TwoGradients = TInterpolateTwoGradients,
    /**
     * 曲线插值
     */
    Random = TInterpolateRandom,
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
    pub radiusThickness: f32,
    /**
     * 发射弧度信息
     */
    pub arc: IShapeArc,
    /**
     * 沿高度体积发射
     */
    pub emitAsVolume: bool,
    pub height: f32,
    pub scale: Option<[f32; 3]>,
    pub position: Option<[f32; 3]>,
    pub rotation: Option<[f32; 3]>,
    pub alignDir: u32,
    pub randomize: Option<[f32; 3]>,
}
/**
 * Sphere 形状发射器描述
 */
pub struct IShapeSphere {
    _type: u32,
    pub radius: f32,
    pub radiusThickness: f32,
    pub arc: IShapeArc,
    pub scale: Option<[f32; 3]>,
    pub position: Option<[f32; 3]>,
    pub rotation: Option<[f32; 3]>,
    pub alignDir: u32,
    pub randomize: Option<[f32; 3]>,
}
/**
 * Box 形状发射器描述
 */
pub struct IShapeBox {
    _type: u32,
    pub isVolume: u32,
    pub scale: Option<[f32; 3]>,
    pub position: Option<[f32; 3]>,
    pub rotation: Option<[f32; 3]>,
    pub alignDir: u32,
    pub randomize: Option<[f32; 3]>,
    pub boxEmitMode: Option<EBoxShapeMode>,
}
/**
 * Hemisphere 形状发射器描述
 */
pub struct IShapeHemisphere {
    _type: u32,
    pub radius: f32,
    pub radiusThickness: f32,
    pub arc: IShapeArc,
    pub scale: Option<[f32; 3]>,
    pub position: Option<[f32; 3]>,
    pub rotation: Option<[f32; 3]>,
    pub alignDir: u32,
    pub randomize: Option<[f32; 3]>,
}
/**
 * Circle 形状发射器描述
 */
pub struct IShapeCircle {
    _type: u32,
    pub radius: f32,
    pub radiusThickness: f32,
    pub arc: IShapeArc,
    pub scale: Option<[f32; 3]>,
    pub position: Option<[f32; 3]>,
    pub rotation: Option<[f32; 3]>,
    pub alignDir: u32,
    pub randomize: Option<[f32; 3]>,
}
/**
 * Edge 形状发射器描述
 */
pub struct IShapeEdge {
    _type: u32,
    pub radius: f32,
    pub arc: IShapeArc,
    pub scale: Option<[f32; 3]>,
    pub position: Option<[f32; 3]>,
    pub rotation: Option<[f32; 3]>,
    pub alignDir: u32,
    pub randomize: Option<[f32; 3]>,
}
/**
 * Rectangle 形状发射器描述
 */
pub struct IShapeRectangle {
    _type: u32,
    pub scale: Option<[f32; 3]>,
    pub position: Option<[f32; 3]>,
    pub rotation: Option<[f32; 3]>,
    pub alignDir: u32,
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
#[derive(Clone)]
pub enum OneParamInfo {
    TInterpolateConstant(OneParam),
    TInterpolateTwoConstants(OneParam, OneParam),
    TInterpolateCurve(OneParamCurve),
    TInterpolateTwoCurves(OneParamCurve, OneParamCurve),
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

pub enum FourGradientInfo {
    TInterpolateRandom,
    TInterpolateColor(FourParam),
    TInterpolateTwoColors(FourParam, FourParam),
    TInterpolateGradient(FourGradient),
    TInterpolateTwoGradients(FourGradient, FourGradient),
}

pub struct ITextureSheet {
    pub frameOverTime: OneParamInfo,
    pub animMode: AnimationMode,
    pub customRow: f32,
    pub cycles: f32,
    pub rowMode: RowMode,
    pub startFrame: OneParamInfo,
    pub tilesX: f32,
    pub tilesY: f32,
    pub timeMode: TimeMode,
}

pub struct ITrail {
    pub ratio: f32,
    pub mode: ETrailMode,
    pub lifetime: OneParamInfo,
    pub ribbonCount: f32,
    pub attachRTT: u32,
    pub minDist: f32,
    pub worldSpace: u32,
    pub dieWith: u32,
    pub texMode: ETrailTextureMode,
    pub sizeAWidth: u32,
    pub sizeALifetime: u32,
    pub inheritColor: u32,
    pub colorOverLife: FourGradientInfo,
    pub widthOverTrail: OneParamInfo,
    pub colorOverTrail: FourGradientInfo,
    pub material: f32,
}

/**
 * 粒子系统JSON描述
 */

pub struct IParticleSystemConfig {
    pub name: String,
    pub duration: f32,
    pub startDelay: f32,
    pub looping: u32,
    pub prewarm: bool,
    pub simulationSpaceIsWorld: EMeshParticleSpaceMode,
    pub scalingMode: EMeshParticleScaleMode,
    pub renderAlignment: ERenderAlignment,
    pub renderMode: ERenderMode,
    pub stretchedVelocityScale: f32,
    pub stretchedLengthScale: f32,
    pub maxParticles: f32,
    pub startSpeed: OneParamInfo,
    pub lifetime: OneParamInfo,
    pub delay: Vec<f32>,
    pub startColor: FourGradientInfo,
    pub startSize: ParamInfo,
    pub startRotation: ParamInfo,
    pub gravity: OneParamInfo,
    pub emission: (f32, Option<Vec<[f32; 4]>>),
    pub shape: IShape,
    pub velocityOverLifetime: Option<ParamInfo>,
    pub velocityOverLifetimeIsLocal: Option<u32>,
    pub limitVelocityOverLifetime: Option<OneParamInfo>,
    pub limitVelocityOverLifetimeDampen: Option<f32>,
    pub forceOverLifetime: Option<ParamInfo>,
    pub forceSpaceIsLocal: Option<u32>,
    pub colorOverLifetime: Option<FourGradientInfo>,
    pub colorBySpeed: Option<(FourGradientInfo, f32, f32)>,
    pub sizeOverLifetime: Option<ParamInfo>,
    pub sizeBySpeed: Option<(OneParamInfo, f32, f32)>,
    pub rotationOverLifetime: Option<ParamInfo>,
    pub rotationBySpeed: Option<(OneParamInfo, f32, f32)>,
    pub textureSheet: Option<ITextureSheet>,
    pub texture: Option<String>,
    pub trail: Option<ITrail>,
    pub orbtialVelocity: Option<ParamInfo>,
    pub orbitalOffset: Option<ParamInfo>,
    pub orbitalRadial: Option<OneParamInfo>,
    pub speedModifier: Option<OneParamInfo>,
    pub renderPivot: Option<[f32; 3]>,
    pub custom1: Option<[OneParamInfo; 4]>,
}

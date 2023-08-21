
use pi_scene_math::{Color4, Matrix, Quaternion, Vector3, Number};

pub const TEMP_VECTOR3_A: Vector3 = Vector3::new(0., 0., 0.);
pub const TEMP_VECTOR3_B: Vector3 = Vector3::new(0., 0., 0.);
pub const TEMP_VECTOR3_C: Vector3 = Vector3::new(0., 0., 0.);
pub const TEMP_VECTOR3_D: Vector3 = Vector3::new(0., 0., 0.);
pub const TEMP_VECTOR3_E: Vector3 = Vector3::new(0., 0., 0.);

pub const TEMP_COLOR4_A: Color4 = Color4::new(1., 1., 1., 1.);
pub const TEMP_COLOR4_B: Color4 = Color4::new(1., 1., 1., 1.);
pub const TEMP_COLOR4_C: Color4 = Color4::new(1., 1., 1., 1.);

/**
 * 数据类型
 */
pub const VALUE_1: u8 = 1;
pub const VALUE_3: u8 = 3;
pub const VALUE_4: u8 = 4;

/**
 * 曲线类型
 */
pub const TINTERPOLATE_CONSTANT: u8 = 1;
pub const TINTERPOLATE_TWO_CONSTANTS: u8 = 2;
pub const TINTERPOLATE_CURVE: u8 = 4;
pub const TINTERPOLATE_TWO_CURVES: u8 = 8;

/**
 * 渐变类型
 */
pub const TINTERPOLATE_COLOR: u8 = 1;
pub const TINTERPOLATE_TWO_COLORS: u8 = 2;
pub const TINTERPOLATE_GRADIENT: u8 = 4;
pub const TINTERPOLATE_TWO_GRADIENTS: u8 = 8;
pub const TINTERPOLATE_RANDOM: u8 = 16;

pub struct BaseRandom {
    pub seed: u64,
    pub base: Number,
    pub x: Number,
    pub y: Number,
    pub z: Number,
    pub w: Number,
}
impl Default for BaseRandom {
    fn default() -> Self {
        Self {
            seed: 0,
            base: 0.,
            x: 0.,
            y: 0.,
            z: 0.,
            w: 0.,
        }
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
    Constant = TINTERPOLATE_CONSTANT as isize,
    /**
     * 静态数值随机 - XYZ 随机值相同
     */
    TwoConstants = TINTERPOLATE_TWO_CONSTANTS as isize,
    /**
     * 曲线插值
     */
    Curve = TINTERPOLATE_CURVE as isize,
    /**
     * 曲线插值
     */
    TwoCurves = TINTERPOLATE_TWO_CURVES as isize,
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
    Color = TINTERPOLATE_COLOR as isize,
    /**
     * 静态数值随机 - XYZ 随机值相同
     */
    TwoColors = TINTERPOLATE_TWO_COLORS as isize,
    /**
     * 曲线插值
     */
    Gradient = TINTERPOLATE_GRADIENT as isize,
    /**
     * 曲线插值
     */
    TwoGradients = TINTERPOLATE_TWO_GRADIENTS as isize,
    /**
     * 曲线插值
     */
    Random = TINTERPOLATE_RANDOM as isize,
}
impl Default for EInterpolationGradienMode {
    fn default() -> EInterpolationGradienMode {
        EInterpolationGradienMode::Color
    }
}

//#region 曲线数据属性值类型
/**
 * 时间点
 */
pub type TCurveTime = f32;

//#region 曲线关键帧 各属性数据在 数组中的序号描述
/**
 * 关键帧 时间信息 的数组序号
 */
pub const KEY_INDEX_FRAME: usize = 0;
/**
 * 关键帧 值信息 的数组序号
 */
pub const KEY_INDEX_VALUE: usize = 1;
/**
 * 关键帧 InTangent 的数组序号
 */
pub const KEY_INDEX_IN_TANGENT: usize = 2;
/**
 * 关键帧 OutTangent 的数组序号
 */
pub const KEY_INDEX_OUT_TANGENT: usize = 3;

/**
 * 曲线 关键帧信息 - 数组形式
 */
pub type ICurveKey = Vec<TCurveTime>;
//#endregion


/**
 * 曲线 值域缩放数据 值类型
 */
type TCurveScalar = f32;
/**
 * 曲线信息 数据 - 数组形式
 */
pub type ICurve = (Vec<ICurveKey>, TCurveScalar);
//#endregion

//#region 可插值数据 值类型描述
/**
 * 一维插值 数据类型
 */
pub type InterpolationData1 = f32;
/**
 * 二维插值 数据类型
 */
pub type InterpolationData2 = [f32; 2];
/**
 * 三维插值 数据类型
 */
pub type InterpolationData3 = [f32; 3];
/**
 * 四维插值 数据类型
 */
pub type InterpolationData4 = [f32; 4];
//#endregion

//#region 渐变信息描述
/**
 * 渐变控制点 时间 数据类型
 */
pub type TGradientTime = f32;

/**
 * 渐变控制点 时间 数据在渐变信息中的序号
 */
pub const GRADIENT_INDEX_FRAME: usize = 0;
/**
 * 渐变控制点 值 数据在渐变信息中的序号
 */
pub const GRADIENT_INDEX_VALUE: usize = 1;
/**
 * 渐变控制点数据 - 数组形式
 */
pub type IGradient = [TGradientTime; 2];
pub type IGradient4 = [Vec<IGradient>; 4];
//#endregion

pub type OneParam = f32;
pub type ThreeParam = [f32; 3];
pub type FourParam = [f32; 4];
pub type OneParamCurve = ICurve;
pub type ThreeParamCurve = [ICurve; 3];
pub type FourParamCurve = [ICurve; 4];
pub type FourGradient = [Vec<IGradient>; 4];


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


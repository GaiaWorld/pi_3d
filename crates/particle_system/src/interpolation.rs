use pi_scene_math::Number;

use crate::{iparticle_system_config::{
    DefaultValue, EInterpolationCurveMode, EInterpolationGradienMode, FourGradientInfo,
    OneParamInfo, TParamType,
}, tools::BaseRandom};

//#region 曲线数据属性值类型
/**
 * 时间点
 */
type TCurveTime = f32;

//#region 曲线关键帧 各属性数据在 数组中的序号描述
/**
 * 关键帧 时间信息 的数组序号
 */
const KEY_INDEX_FRAME: usize = 0;
/**
 * 关键帧 值信息 的数组序号
 */
const KEY_INDEX_VALUE: usize = 1;
/**
 * 关键帧 InTangent 的数组序号
 */
const KEY_INDEX_IN_TANGENT: usize = 2;
/**
 * 关键帧 OutTangent 的数组序号
 */
const KEY_INDEX_OUT_TANGENT: usize = 3;

/**
 * 曲线 关键帧信息 - 数组形式
 */
type ICurveKey = Vec<TCurveTime>;
//#endregion



/**
 * 插值工具模块描述
 */
pub trait IInterpolation<T> {
    /**
     * 插值接口
     * @param progress 插值因子
     */
    fn interpolate(&self, progress: f32, random: f32) -> T;
    /**
     * 销毁
     */
    fn dispose(&mut self);
}

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
type InterpolationData1 = f32;
/**
 * 二维插值 数据类型
 */
type InterpolationData2 = [f32; 2];
/**
 * 三维插值 数据类型
 */
type InterpolationData3 = [f32; 3];
/**
 * 四维插值 数据类型
 */
type InterpolationData4 = [f32; 4];
//#endregion

//#region 随机处理
/**
 * 一维数据间的随机
 * @param min
 * @param max
 * @param random 随机因子
 * @returns
 */
pub fn random_range(
    min: InterpolationData1,
    max: InterpolationData1,
    random: f32,
) -> InterpolationData1 {
    if min == max {
        return min;
    }
    return (random * (max - min)) + min;
}

/**
 * 二维数据间的随机
 * @param min
 * @param max
 * @param result
 * @param random0 第一随机因子
 * @param random1 第二随机因子
 */
pub fn random_range2(
    min: InterpolationData2,
    max: InterpolationData2,
    result: &mut InterpolationData2,
    random0: f32,
    random1: f32,
) {
    result[0] = random_range(min[0], max[0], random0);
    result[1] = random_range(min[1], max[1], random1);
}
/**
 * 三维数据间的随机
 * @param min
 * @param max
 * @param result
 * @param random0 第一随机因子
 * @param random1 第二随机因子
 * @param random2 第三随机因子
 */
pub fn random_range3(
    min: InterpolationData3,
    max: InterpolationData3,
    result: &mut InterpolationData3,
    random0: f32,
    random1: f32,
    random2: f32,
) {
    result[0] = random_range(min[0], max[0], random0);
    result[1] = random_range(min[1], max[1], random1);
    result[2] = random_range(min[2], max[2], random2);
}
/**
 * 四维数据间的随机
 * @param min
 * @param max
 * @param result
 * @param random0 第一随机因子
 * @param random1 第二随机因子
 * @param random2 第三随机因子
 * @param random3 第四随机因子
 */
pub fn random_range4(
    min: InterpolationData4,
    max: InterpolationData4,
    result: &mut InterpolationData4,
    random0: f32,
    random1: f32,
    random2: f32,
    random3: f32,
) {
    result[0] = random_range(min[0], max[0], random0);
    result[1] = random_range(min[1], max[1], random1);
    result[2] = random_range(min[2], max[2], random2);
    result[3] = random_range(min[3], max[3], random3);
}

/**
 * 更新数据缓存
 */
pub fn update_random() {
    // random() = Math.random();
    // random() = Math.random();
    // random() = Math.random();
    // random() = Math.random();
}
// pub fn random() {
//     return Math.random();
// }
//#endregion

//#region 插值方法
/**
 * Hermite 插值
 * @param value1 第一控制点值
 * @param tangent1 第一控制点 切线斜率
 * @param value2 第二控制点值
 * @param tangent2 第二控制点 切线斜率
 * @param amount 插值因子
 * @returns
 */
pub fn hermite(value1: f32, tangent1: f32, value2: f32, tangent2: f32, amount: f32) -> f32 {
    let squared = amount * amount;
    let cubed = amount * squared;
    let part1 = ((2.0 * cubed) - (3.0 * squared)) + 1.0;
    let part2 = (-2.0 * cubed) + (3.0 * squared);
    let part3 = (cubed - (2.0 * squared)) + amount;
    let part4 = cubed - squared;

    return (((value1 * part1) + (value2 * part2)) + (tangent1 * part3)) + (tangent2 * part4);
}
/**
 * 曲线插值
 * @param amount 插值因子
 * @param curve 曲线描述
 * @returns
 */
pub fn interpolation_curve(amount: f32, curve: &ICurve) -> InterpolationData1 {
    let key_count = curve.0.len();

    if key_count == 0 {
        return curve.1;
    }

    let mut pre_index = 0;
    let mut next_index = key_count - 1;
    // let i = 0;
    let mut pre = &curve.0[pre_index];
    let mut next = &curve.0[next_index];

    if key_count == 1 {
        return pre[KEY_INDEX_VALUE] * curve.1;
    }

    for i in 0..key_count {
        pre_index = i;
        next_index = i + 1;

        pre = &curve.0[pre_index];
        next = &curve.0[next_index];

        if pre_index == 0 && amount <= pre[KEY_INDEX_FRAME] {
            next_index = pre_index;
            next = pre;
            break;
        }

        if pre[KEY_INDEX_FRAME] < amount && amount <= next[KEY_INDEX_FRAME] {
            break;
        }

        if next_index == key_count - 1 && next[KEY_INDEX_FRAME] <= amount {
            pre_index = next_index;
            pre = next;
            break;
        }
    }

    if pre_index == next_index {
        return pre[KEY_INDEX_VALUE] * curve.1;
    }

    // https://en.wikipedia.org/wiki/Cubic_Hermite_spline Interpolation on an arbitrary interval
    let delta_t = next[KEY_INDEX_FRAME] - pre[KEY_INDEX_FRAME];
    let amount = (amount - pre[KEY_INDEX_FRAME]) / delta_t;

    return hermite(
        pre[KEY_INDEX_VALUE],
        pre[KEY_INDEX_OUT_TANGENT] * delta_t,
        next[KEY_INDEX_VALUE],
        next[KEY_INDEX_IN_TANGENT] * delta_t,
        amount,
    ) * curve.1;
}
/**
 * 渐变插值
 * @param amount 插值因子
 * @param gradient 渐变控制点数组
 * @returns
 */
pub fn interpolation_gradient(amount: f32, gradient: &Vec<IGradient>) -> f32 {
    let key_count = gradient.len();

    if key_count == 0 {
        return 1.0;
    }

    let mut pre_index = 0;
    let mut next_index = key_count - 1;

    let mut pre = gradient[pre_index];
    let mut next = gradient[next_index];

    if key_count == 1 {
        return pre[GRADIENT_INDEX_VALUE];
    }

    for i in 0..key_count {
        pre_index = i;
        next_index = i + 1;

        pre = gradient[pre_index];
        next = gradient[next_index];

        if pre_index == 0 && amount <= pre[GRADIENT_INDEX_FRAME] {
            next_index = pre_index;
            next = pre;
            break;
        }

        if pre[GRADIENT_INDEX_FRAME] < amount && amount <= next[GRADIENT_INDEX_FRAME] {
            break;
        }

        if next_index == key_count - 1 && next[GRADIENT_INDEX_FRAME] <= amount {
            pre_index = next_index;
            pre = next;
            break;
        }
    }

    if pre_index == next_index {
        return pre[GRADIENT_INDEX_VALUE];
    }

    let amount =
        (amount - pre[GRADIENT_INDEX_FRAME]) / (next[GRADIENT_INDEX_FRAME] - pre[GRADIENT_INDEX_FRAME]);

    return pre[GRADIENT_INDEX_VALUE] + (next[GRADIENT_INDEX_VALUE] - pre[GRADIENT_INDEX_VALUE]) * amount;
}
//#endregion

//#region 曲线插值实现 - 一维 - 三维 - 四维
/**
 * 一维曲线插值
 */
#[derive(Default, Clone)]
pub struct FloatInterpolation {
    pub constant0: Option<InterpolationData1>,
    pub constant1: Option<InterpolationData1>,
    pub min_curve: Option<ICurve>,
    pub max_curve: Option<ICurve>,
    pub mode: EInterpolationCurveMode,
}

impl FloatInterpolation {
    pub fn new(val: Number) -> Self {
        Self {
            constant0: Some(val),
            constant1: Some(val),
            min_curve: None,
            max_curve: None,
            mode: EInterpolationCurveMode::Constant,
        }
    }
}

impl IInterpolation<InterpolationData1> for FloatInterpolation {
    fn interpolate(&self, amount: f32, random: f32) -> InterpolationData1 {
        match self.mode {
            EInterpolationCurveMode::TwoConstants => {
                let constant0 = if self.constant0.is_some() {
                    self.constant0.unwrap()
                } else {
                    0.0
                };
                let constant1 = if self.constant1.is_some() {
                    self.constant1.unwrap()
                } else {
                    0.0
                };
                return random_range(constant0, constant1, random);
            }
            EInterpolationCurveMode::Curve => {
                let curve = if self.min_curve.is_some() {
                    self.min_curve.as_ref().unwrap()
                } else {
                    self.max_curve.as_ref().unwrap()
                };
                return interpolation_curve(amount, curve);
            }
            EInterpolationCurveMode::TwoCurves => {
                // se
                let min = interpolation_curve(amount, self.min_curve.as_ref().unwrap());
                let max = interpolation_curve(amount, self.max_curve.as_ref().unwrap());
                return random_range(min, max, random);
            }
            _ => {
                if let Some(v) = self.constant0 {
                    return v;
                } else if let Some(v) = self.constant1 {
                    return v;
                } else {
                    return 0.;
                };
            }
        }
    }

    fn dispose(&mut self) {
        self.constant0 = None;
        self.constant1 = None;

        self.min_curve = None;
        self.max_curve = None;
    }
}

/**
 * 一维 - 根据目标数据使用场景 创建对应 默认插值模块
 * @param ptype 使用场景标识
 * @returns
 */
pub fn default_float_interpolation(ptype: TParamType) -> Option<FloatInterpolation> {
    let mut interpolate = None;
    match ptype {
        TParamType::TParamStartSpeed => {
            let mut temp = FloatInterpolation::default();
            temp.mode = EInterpolationCurveMode::Constant;
            temp.constant0 = Some(DefaultValue::START_SPEED);
            interpolate = Some(temp);
        }
        TParamType::TParamStartLifetime => {
            let mut temp = FloatInterpolation::default();
            temp.mode = EInterpolationCurveMode::Constant;
            temp.constant0 = Some(DefaultValue::START_LIFETIME);
        }
        TParamType::TParamTextureSheet => {
            let mut temp = FloatInterpolation::default();
            temp.mode = EInterpolationCurveMode::Constant;
            temp.constant0 = Some(DefaultValue::TEXTURE_SHEET_FRAME);
            interpolate = Some(temp);
        }
        _ => {}
    }

    return interpolate;
}
/**
 * 一维 - 从 json 描述创建曲线插值模块
 * @param interpolation 目标插值模块 - 如果外部传入则逻辑为应用json配置到该模块
 * @param config json 描述
 * @param ptype 数据应用场景描述
 * @param scale 数据值域缩放 - 如角度转弧度，秒转毫秒，重力因子转重力值
 * @returns
 */
pub fn parse_float_interpolation(
    interpolation: &mut FloatInterpolation,
    config: &Option<OneParamInfo>,
    ptype: TParamType,
    scale: f32,
) {
    if let Some(config) = config {
        match config {
            OneParamInfo::TInterpolateConstant(param) => {
                interpolation.mode = EInterpolationCurveMode::Constant;
                interpolation.constant0 = Some(param * scale);
                // break;
            }
            OneParamInfo::TInterpolateTwoConstants(param1, param2) => {
                interpolation.mode = EInterpolationCurveMode::TwoConstants;
                interpolation.constant0 = Some(param1 * scale);
                interpolation.constant1 = Some(param2 * scale);
            }
            OneParamInfo::TInterpolateCurve(curve) => {
                interpolation.mode = EInterpolationCurveMode::Curve;
                interpolation.min_curve = Some(scale_curve(curve.clone(), scale));
            }
            OneParamInfo::TInterpolateTwoCurves(curve1, curve2) => {
                interpolation.mode = EInterpolationCurveMode::Curve;
                interpolation.min_curve = Some(scale_curve(curve1.clone(), scale));
                interpolation.max_curve = Some(scale_curve(curve2.clone(), scale));
            }
        }
    } else {
        if let Some(res) = default_float_interpolation(ptype) {
            *interpolation = res;
        }
    }
}

pub fn scale_curve(mut curve: ICurve, scale: f32) -> ICurve {
    curve.1 *= scale;
    return curve;
}
//#endregion

//#region 渐变信息描述
/**
 * 渐变控制点 时间 数据类型
 */
type TGradientTime = f32;

/**
 * 渐变控制点 时间 数据在渐变信息中的序号
 */
const GRADIENT_INDEX_FRAME: usize = 0;
/**
 * 渐变控制点 值 数据在渐变信息中的序号
 */
const GRADIENT_INDEX_VALUE: usize = 1;
/**
 * 渐变控制点数据 - 数组形式
 */
pub type IGradient = [TGradientTime; 2];
type IGradient4 = [Vec<IGradient>; 4];
//#endregion

//#region 渐变插值
#[derive(Clone, Debug)]
pub struct Color4Gradient {
    pub mode: EInterpolationGradienMode,
    pub constant0: Option<[f32; 4]>,
    pub constant1: Option<[f32; 4]>,
    pub min_gradients: Option<IGradient4>,
    pub max_gradients: Option<IGradient4>,
}

impl Default for Color4Gradient {
    fn default() -> Self {
        Self {
            mode: EInterpolationGradienMode::Color,
            constant0: Some([1., 1., 1., 1.]),
            constant1: Some([1., 1., 1., 1.]),
            min_gradients: None,
            max_gradients: None,
        }
    }
}
impl Color4Gradient {
    pub fn interpolate(&self, amount: f32, result: &mut [f32; 4], randoms: &BaseRandom) {
        match self.mode {
            EInterpolationGradienMode::Color => {
                let temp = if self.constant0.is_some() {
                    self.constant0.as_ref().unwrap()
                } else {
                    self.constant1.as_ref().unwrap()
                };
                result[0] = temp[0];
                result[1] = temp[1];
                result[2] = temp[2];
                result[3] = temp[3];

                return;
            }
            EInterpolationGradienMode::TwoColors => {
                let temp_min = if self.constant0.is_some() {
                    self.constant0.as_ref().unwrap()
                } else {
                    self.constant1.as_ref().unwrap()
                };
                let temp_max = if self.constant1.is_some() {
                    self.constant1.as_ref().unwrap()
                } else {
                    self.constant0.as_ref().unwrap()
                };

                // let minR = tempMin[0];
                // let minG = tempMin[1];
                // let minB = tempMin[2];
                // let minA = tempMin[3];

                // let maxR = tempMax[0];
                // let maxG = tempMax[1];
                // let maxB = tempMax[2];
                // let maxA = tempMax[3];

                // result[0] = RandomRange(minR, maxR, random());
                // result[1] = RandomRange(minG, maxG, random());
                // result[2] = RandomRange(minB, maxB, random());
                // result[3] = RandomRange(minA, maxA, random());

                let use_min = randoms.base > 0.5;
                if use_min {
                    result[0] = temp_min[0];
                    result[1] = temp_min[1];
                    result[2] = temp_min[2];
                    result[3] = temp_min[3];
                } else {
                    result[0] = temp_max[0];
                    result[1] = temp_max[1];
                    result[2] = temp_max[2];
                    result[3] = temp_max[3];
                }

                return;
            }
            EInterpolationGradienMode::Gradient => {
                let gradient0 = if self.min_gradients.is_some() {
                    &self.min_gradients.as_ref().unwrap()[0]
                } else {
                    &self.max_gradients.as_ref().unwrap()[0]
                };
                let gradient1 = if self.min_gradients.is_some() {
                    &self.min_gradients.as_ref().unwrap()[1]
                } else {
                    &self.max_gradients.as_ref().unwrap()[1]
                };
                let gradient2 = if self.min_gradients.is_some() {
                    &self.min_gradients.as_ref().unwrap()[2]
                } else {
                    &self.max_gradients.as_ref().unwrap()[2]
                };
                let gradient3 = if self.min_gradients.is_some() {
                    &self.min_gradients.as_ref().unwrap()[3]
                } else {
                    &self.max_gradients.as_ref().unwrap()[3]
                };

                result[0] = interpolation_gradient(amount, gradient0);
                result[1] = interpolation_gradient(amount, gradient1);
                result[2] = interpolation_gradient(amount, gradient2);
                result[3] = interpolation_gradient(amount, gradient3);

                return;
            }
            EInterpolationGradienMode::TwoGradients => {
                let min_gradient0 = &self.min_gradients.as_ref().unwrap()[0];
                let min_gradient1 = &self.min_gradients.as_ref().unwrap()[1];
                let min_gradient2 = &self.min_gradients.as_ref().unwrap()[2];
                let min_gradient3 = &self.min_gradients.as_ref().unwrap()[3];

                let max_gradient0 = &self.max_gradients.as_ref().unwrap()[0];
                let max_gradient1 = &self.max_gradients.as_ref().unwrap()[1];
                let max_gradient2 = &self.max_gradients.as_ref().unwrap()[2];
                let max_gradient3 = &self.max_gradients.as_ref().unwrap()[3];

                let use_min = randoms.base > 0.5;
                if use_min {
                    result[0] = interpolation_gradient(amount, min_gradient0);
                    result[1] = interpolation_gradient(amount, min_gradient1);
                    result[2] = interpolation_gradient(amount, min_gradient2);
                    result[3] = interpolation_gradient(amount, min_gradient3);
                } else {
                    result[0] = interpolation_gradient(amount, max_gradient0);
                    result[1] = interpolation_gradient(amount, max_gradient1);
                    result[2] = interpolation_gradient(amount, max_gradient2);
                    result[3] = interpolation_gradient(amount, max_gradient3);
                }

                return;
            }
            _ => {
                // TODO
                result[0] = randoms.x;
                result[1] = randoms.y;
                result[2] = randoms.z;
                result[3] = randoms.w;

                return;
            }
        }
    }

    pub fn dispose(&mut self) {
        self.constant0 = None;
        self.constant1 = None;
    }
}

/**
 * 根据json表达创建渐变插值模块
 * @param interpolation
 * @param config
 * @param ptype
 * @returns
 */
pub fn parse_color4_gradient(
    interpolation: &mut Color4Gradient,
    config: Option<&FourGradientInfo>,
    _ptype: TParamType,
) {
    if let Some(config) = config {
        // if interpolation.is_some() {
        //     *interpolation = Some(Color4Gradient::default());
        // }
        // let config = config.as_ref().unwarp();

        // let interpolation = interpolation.as_mut().unwrap();
        match config {
            FourGradientInfo::TInterpolateRandom => {
                interpolation.mode = EInterpolationGradienMode::Random;
            }
            FourGradientInfo::TInterpolateColor(param) => {
                interpolation.mode = EInterpolationGradienMode::Color;
                interpolation.constant0 = Some(*param);
                interpolation.constant1 = Some(*param);
            }
            FourGradientInfo::TInterpolateTwoColors(param1, param2) => {
                interpolation.mode = EInterpolationGradienMode::TwoColors;
                interpolation.constant0 = Some(*param1);
                interpolation.constant1 = Some(*param2);
            }
            FourGradientInfo::TInterpolateGradient(graient) => {
                interpolation.mode = EInterpolationGradienMode::Gradient;
                interpolation.min_gradients = Some(graient.clone());
                interpolation.max_gradients = Some(graient.clone());
            }
            FourGradientInfo::TInterpolateTwoGradients(graient1, graient2) => {
                interpolation.mode = EInterpolationGradienMode::TwoGradients;
                interpolation.min_gradients = Some(graient1.clone());
                interpolation.max_gradients = Some(graient2.clone());
            }
        }
    } else {
        // interpolation = defaultFloat4Interpolation(ptype);
    }

    // return interpolation;
}
//#endregion

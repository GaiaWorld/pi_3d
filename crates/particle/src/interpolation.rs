use crate::iparticle_system_config::{
    DefaultValue, EInterpolationCurveMode, EInterpolationGradienMode, FourGradientInfo,
    OneParamInfo, TParamType,
};

//#region 曲线数据属性值类型
/**
 * 时间点
 */
type TCurveTime = f32;
/**
 * 当前时间点对应值
 */
type TCurveValue = f32;
/**
 * 当前时间点的进入时切线斜率
 */
type TCurveInTangent = f32;
/**
 * 当前时间点的离开时切线斜率
 */
type TCurveOutTangent = f32;
/**
 * 曲线模式
 */
type TCurveMode = f32;
//#endregion

//#region 曲线关键帧 各属性数据在 数组中的序号描述
/**
 * 关键帧 时间信息 的数组序号
 */
const KeyIndexFrame: usize = 0;
/**
 * 关键帧 值信息 的数组序号
 */
const KeyIndexValue: usize = 1;
/**
 * 关键帧 InTangent 的数组序号
 */
const KeyIndexInTangent: usize = 2;
/**
 * 关键帧 OutTangent 的数组序号
 */
const KeyIndexOutTangent: usize = 3;
/**
 * 关键帧 曲线模式 的数组序号
 */
const KeyIndexMode: usize = 4;
/**
 * 曲线 关键帧信息 - 数组形式
 */
type ICurveKey = Vec<TCurveTime>;
//#endregion

/**
 * Enum for the animation key frame interpolation type
 */
enum AnimationKeyInterpolation {
    /**
     * Do not interpolate between keys and use the start key value only. Tangents are ignored
     */
    STEP = 1,
}

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

//#region 曲线信息描述
/**
 * 曲线 关键帧信息数组 在曲线信息 数据中的序号
 */
const CurveIndexKeys: u32 = 0;
/**
 * 曲线 值域缩放数据 在曲线信息 数据中的序号
 */
const CurveIndexScalar: u32 = 1;
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
pub fn RandomRange(
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
pub fn RandomRange2(
    min: InterpolationData2,
    max: InterpolationData2,
    result: &mut InterpolationData2,
    random0: f32,
    random1: f32,
) {
    result[0] = RandomRange(min[0], max[0], random0);
    result[1] = RandomRange(min[1], max[1], random1);
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
pub fn RandomRange3(
    min: InterpolationData3,
    max: InterpolationData3,
    result: &mut InterpolationData3,
    random0: f32,
    random1: f32,
    random2: f32,
) {
    result[0] = RandomRange(min[0], max[0], random0);
    result[1] = RandomRange(min[1], max[1], random1);
    result[2] = RandomRange(min[2], max[2], random2);
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
pub fn RandomRange4(
    min: InterpolationData4,
    max: InterpolationData4,
    result: &mut InterpolationData4,
    random0: f32,
    random1: f32,
    random2: f32,
    random3: f32,
) {
    result[0] = RandomRange(min[0], max[0], random0);
    result[1] = RandomRange(min[1], max[1], random1);
    result[2] = RandomRange(min[2], max[2], random2);
    result[3] = RandomRange(min[3], max[3], random3);
}
/**
 * 全局随机数缓存
 */
const RandomList: [u32; 4] = [0, 0, 0, 0];
/**
 * 更新数据缓存
 */
pub fn UpdateRandom() {
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
pub fn Hermite(value1: f32, tangent1: f32, value2: f32, tangent2: f32, amount: f32) -> f32 {
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
pub fn InterpolationCurve(amount: f32, curve: &ICurve) -> InterpolationData1 {
    let keyCount = curve.0.len();

    if keyCount == 0 {
        return curve.1;
    }

    let mut preIndex = 0;
    let mut nextIndex = keyCount - 1;
    // let i = 0;
    let mut pre = &curve.0[preIndex];
    let mut next = &curve.0[nextIndex];

    if keyCount == 1 {
        return pre[KeyIndexValue] * curve.1;
    }

    for i in 0..keyCount {
        preIndex = i;
        nextIndex = i + 1;

        pre = &curve.0[preIndex];
        next = &curve.0[nextIndex];

        if preIndex == 0 && amount <= pre[KeyIndexFrame] {
            nextIndex = preIndex;
            next = pre;
            break;
        }

        if pre[KeyIndexFrame] < amount && amount <= next[KeyIndexFrame] {
            break;
        }

        if nextIndex == keyCount - 1 && next[KeyIndexFrame] <= amount {
            preIndex = nextIndex;
            pre = next;
            break;
        }
    }

    if preIndex == nextIndex {
        return pre[KeyIndexValue] * curve.1;
    }

    // https://en.wikipedia.org/wiki/Cubic_Hermite_spline Interpolation on an arbitrary interval
    let deltaT = next[KeyIndexFrame] - pre[KeyIndexFrame];
    let amount = (amount - pre[KeyIndexFrame]) / deltaT;

    return Hermite(
        pre[KeyIndexValue],
        pre[KeyIndexOutTangent] * deltaT,
        next[KeyIndexValue],
        next[KeyIndexInTangent] * deltaT,
        amount,
    ) * curve.1;
}
/**
 * 渐变插值
 * @param amount 插值因子
 * @param gradient 渐变控制点数组
 * @returns
 */
pub fn InterpolationGradient(amount: f32, gradient: &Vec<IGradient>) -> f32 {
    let keyCount = gradient.len();

    if keyCount == 0 {
        return 1.0;
    }

    let mut preIndex = 0;
    let mut nextIndex = keyCount - 1;

    let mut pre = gradient[preIndex];
    let mut next = gradient[nextIndex];

    if keyCount == 1 {
        return pre[GradientIndexValue];
    }

    for i in 0..keyCount {
        preIndex = i;
        nextIndex = i + 1;

        pre = gradient[preIndex];
        next = gradient[nextIndex];

        if preIndex == 0 && amount <= pre[GradientIndexFrame] {
            nextIndex = preIndex;
            next = pre;
            break;
        }

        if pre[GradientIndexFrame] < amount && amount <= next[GradientIndexFrame] {
            break;
        }

        if nextIndex == keyCount - 1 && next[GradientIndexFrame] <= amount {
            preIndex = nextIndex;
            pre = next;
            break;
        }
    }

    if preIndex == nextIndex {
        return pre[GradientIndexValue];
    }

    let amount =
        (amount - pre[GradientIndexFrame]) / (next[GradientIndexFrame] - pre[GradientIndexFrame]);

    return pre[GradientIndexValue] + (next[GradientIndexValue] - pre[GradientIndexValue]) * amount;
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
    pub minCurve: Option<ICurve>,
    pub maxCurve: Option<ICurve>,
    pub mode: EInterpolationCurveMode,
}

impl FloatInterpolation {
    pub fn new() -> Self {
        Self {
            constant0: Some(0.),
            constant1: Some(0.),
            minCurve: None,
            maxCurve: None,
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
                return RandomRange(constant0, constant1, random);
            }
            EInterpolationCurveMode::Curve => {
                let curve = if self.minCurve.is_some() {
                    self.minCurve.as_ref().unwrap()
                } else {
                    self.maxCurve.as_ref().unwrap()
                };
                return InterpolationCurve(amount, curve);
            }
            EInterpolationCurveMode::TwoCurves => {
                // se
                let min = InterpolationCurve(amount, self.minCurve.as_ref().unwrap());
                let max = InterpolationCurve(amount, self.maxCurve.as_ref().unwrap());
                return RandomRange(min, max, random);
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

        self.minCurve = None;
        self.maxCurve = None;
    }
}
//#endregion

//#region 从 json 描述创建曲线插值模块
const ZeroInterpolateConstant: InterpolationData1 = 0.;
const ZeroInterpolateCurve: ICurve = (vec![], 1.);
const OneInterpolateConstant: InterpolationData1 = 1.;
const OneInterpolateCurve: ICurve = (vec![], 0.);
/**
 * 一维 - 根据目标数据使用场景 创建对应 默认插值模块
 * @param ptype 使用场景标识
 * @returns
 */
pub fn defaultFloatInterpolation(ptype: TParamType) -> Option<FloatInterpolation> {
    let mut interpolate = None;
    match ptype {
        TParamType::TParamStartSpeed => {
            let mut temp = FloatInterpolation::default();
            temp.mode = EInterpolationCurveMode::Constant;
            temp.constant0 = Some(DefaultValue::startSpeed);
            interpolate = Some(temp);
        }
        TParamType::TParamStartLifetime => {
            let mut temp = FloatInterpolation::default();
            temp.mode = EInterpolationCurveMode::Constant;
            temp.constant0 = Some(DefaultValue::startLifetime);
        }
        TParamType::TParamTextureSheet => {
            let mut temp = FloatInterpolation::default();
            temp.mode = EInterpolationCurveMode::Constant;
            temp.constant0 = Some(DefaultValue::textureSheetFrame);
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
pub fn parseFloatInterpolation(
    interpolation: &mut FloatInterpolation,
    mut config: &Option<OneParamInfo>,
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
                interpolation.minCurve = Some(scaleCurve(curve.clone(), scale));
            }
            OneParamInfo::TInterpolateTwoCurves(curve1, curve2) => {
                interpolation.mode = EInterpolationCurveMode::Curve;
                interpolation.minCurve = Some(scaleCurve(curve1.clone(), scale));
                interpolation.maxCurve = Some(scaleCurve(curve2.clone(), scale));
            }
        }
    } else {
        if let Some(res) = defaultFloatInterpolation(ptype) {
            *interpolation = res;
        }
    }
}

pub fn scaleCurve(mut curve: ICurve, scale: f32) -> ICurve {
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
 * 渐变控制点 值 数据类型
 */
type TGradientValue = f32;
/**
 * 渐变控制点 时间 数据在渐变信息中的序号
 */
const GradientIndexFrame: usize = 0;
/**
 * 渐变控制点 值 数据在渐变信息中的序号
 */
const GradientIndexValue: usize = 1;
/**
 * 渐变控制点数据 - 数组形式
 */
pub type IGradient = [TGradientTime; 2];
type IGradient4 = [Vec<IGradient>; 4];
//#endregion

//#region 渐变插值
#[derive(Default, Clone, Debug)]
pub struct Color4Gradient {
    pub mode: EInterpolationGradienMode,
    pub constant0: Option<[f32; 4]>,
    pub constant1: Option<[f32; 4]>,
    pub minGradients: Option<IGradient4>,
    pub maxGradients: Option<IGradient4>,
}

impl Color4Gradient {
    pub fn interpolate(&self, amount: f32, result: &mut [f32; 4], startAmout: f32) {
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

                let use_min = startAmout > 0.5;
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
                let gradient0 = if self.minGradients.is_some() {
                    &self.minGradients.as_ref().unwrap()[0]
                } else {
                    &self.maxGradients.as_ref().unwrap()[0]
                };
                let gradient1 = if self.minGradients.is_some() {
                    &self.minGradients.as_ref().unwrap()[1]
                } else {
                    &self.maxGradients.as_ref().unwrap()[1]
                };
                let gradient2 = if self.minGradients.is_some() {
                    &self.minGradients.as_ref().unwrap()[2]
                } else {
                    &self.maxGradients.as_ref().unwrap()[2]
                };
                let gradient3 = if self.minGradients.is_some() {
                    &self.minGradients.as_ref().unwrap()[3]
                } else {
                    &self.maxGradients.as_ref().unwrap()[3]
                };

                result[0] = InterpolationGradient(amount, gradient0);
                result[1] = InterpolationGradient(amount, gradient1);
                result[2] = InterpolationGradient(amount, gradient2);
                result[3] = InterpolationGradient(amount, gradient3);

                return;
            }
            EInterpolationGradienMode::TwoGradients => {
                let minGradient0 = &self.minGradients.as_ref().unwrap()[0];
                let minGradient1 = &self.minGradients.as_ref().unwrap()[1];
                let minGradient2 = &self.minGradients.as_ref().unwrap()[2];
                let minGradient3 = &self.minGradients.as_ref().unwrap()[3];

                let maxGradient0 = &self.maxGradients.as_ref().unwrap()[0];
                let maxGradient1 = &self.maxGradients.as_ref().unwrap()[1];
                let maxGradient2 = &self.maxGradients.as_ref().unwrap()[2];
                let maxGradient3 = &self.maxGradients.as_ref().unwrap()[3];

                let useMin = startAmout > 0.5;
                if useMin {
                    result[0] = InterpolationGradient(amount, minGradient0);
                    result[1] = InterpolationGradient(amount, minGradient1);
                    result[2] = InterpolationGradient(amount, minGradient2);
                    result[3] = InterpolationGradient(amount, minGradient3);
                } else {
                    result[0] = InterpolationGradient(amount, maxGradient0);
                    result[1] = InterpolationGradient(amount, maxGradient1);
                    result[2] = InterpolationGradient(amount, maxGradient2);
                    result[3] = InterpolationGradient(amount, maxGradient3);
                }

                return;
            }
            _ => {
                // TODO
                // result[0] = random();
                // result[1] = random();
                // result[2] = random();
                // result[3] = random();

                return;
            }
        }
    }

    fn dispose(&mut self) {
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
pub fn parseColor4Gradient(
    interpolation: &mut Color4Gradient,
    config: Option<&FourGradientInfo>,
    ptype: TParamType,
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
                interpolation.minGradients = Some(graient.clone());
                interpolation.maxGradients = Some(graient.clone());
            }
            FourGradientInfo::TInterpolateTwoGradients(graient1, graient2) => {
                interpolation.mode = EInterpolationGradienMode::TwoGradients;
                interpolation.minGradients = Some(graient1.clone());
                interpolation.maxGradients = Some(graient2.clone());
            }
        }
    } else {
        // interpolation = defaultFloat4Interpolation(ptype);
    }

    // return interpolation;
}
//#endregion

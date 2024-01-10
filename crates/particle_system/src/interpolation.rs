
use pi_scene_shell::prelude::*;

use crate::prelude::{TParamType, DefaultValue};

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
        float_interpolation(interpolation, config, scale)
    } else {
        if let Some(res) = default_float_interpolation(ptype) {
            *interpolation = res;
        }
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

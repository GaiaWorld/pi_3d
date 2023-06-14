use std::sync::Arc;

use pi_scene_math::{Color4, Matrix, Quaternion, Vector3};

use crate::{
    interpolation::{Color4Gradient, FloatInterpolation, IInterpolation},
    iparticle_system_config::{
        EInterpolationCurveMode, EInterpolationGradienMode, FourGradientInfo, OneParamInfo,
        ParamInfo, ThreeParamInfo,
    },
    particle::Particle,
};

pub const TEMP_VECTOR3_A: Vector3 = Vector3::new(0., 0., 0.);
pub const TEMP_VECTOR3_B: Vector3 = Vector3::new(0., 0., 0.);
pub const TEMP_VECTOR3_C: Vector3 = Vector3::new(0., 0., 0.);
pub const TEMP_VECTOR3_D: Vector3 = Vector3::new(0., 0., 0.);
pub const TEMP_VECTOR3_E: Vector3 = Vector3::new(0., 0., 0.);

pub const TEMP_COLOR4_A: Color4 = Color4::new(1., 1., 1., 1.);
pub const TEMP_COLOR4_B: Color4 = Color4::new(1., 1., 1., 1.);
pub const TEMP_COLOR4_C: Color4 = Color4::new(1., 1., 1., 1.);

lazy_static! {
    pub static ref TEMP_QUATERNION_A: Quaternion = Quaternion::default();
    pub static ref TEMP_QUATERNION_B: Quaternion = Quaternion::default();
    pub static ref TEMP_QUATERNION_C: Quaternion = Quaternion::default();
    pub static ref TEMP_MATRIX_A: Matrix = Matrix::identity();
    pub static ref TEMP_MATRIX_B: Matrix = Matrix::identity();
    pub static ref TEMP_MATRIX_C: Matrix = Matrix::identity();
}

pub trait IParticleModifier {
    fn modify(&mut self, particle: &mut Particle, amount: &mut f32, delta_seconds: f32);
}

pub fn transform_vector_as_local_space(
    _source: &Vector3,
    _transform_invert: Matrix,
    _result: &mut Vector3,
) {
    // no thing
}

pub fn transform_vector_as_world_space(
    source: &Vector3,
    transform_invert: Matrix,
    result: &mut Vector3,
) {
    *result = transform_invert.transform_vector(source);
}

pub fn interpolate_three(
    force_interpolation_x: &FloatInterpolation,
    force_interpolation_y: &FloatInterpolation,
    force_interpolation_z: &FloatInterpolation,
    amount: f32,
    random: f32,
    result: &mut Vector3,
) {
    let x = force_interpolation_x.interpolate(amount, random);
    let y = force_interpolation_y.interpolate(amount, random);
    let z = force_interpolation_z.interpolate(amount, random);
    *result = Vector3::new(x, y, z);
}

pub fn interpolate_translation_no_axis(
    force_interpolation_x: &FloatInterpolation,
    _force_interpolation_y: &FloatInterpolation,
    _force_interpolation_z: &FloatInterpolation,
    amount: f32,
    random: f32,
    result: &mut Vector3,
) {
    let x = force_interpolation_x.interpolate(amount, random);
    *result = Vector3::new(x, x, x);
}

pub fn interpolate_rotation_no_axis(
    force_interpolation_x: &FloatInterpolation,
    _force_interpolation_y: &FloatInterpolation,
    _force_interpolation_z: &FloatInterpolation,
    amount: f32,
    random: f32,
    result: &mut Vector3,
) {
    let z = force_interpolation_x.interpolate(amount, random);
    *result = Vector3::new(0., 0., z);
}

pub fn interpolate_scaling_no_axis(
    force_interpolation_x: &FloatInterpolation,
    _force_interpolation_y: &FloatInterpolation,
    _force_interpolation_z: &FloatInterpolation,
    amount: f32,
    random: f32,
    result: &mut Vector3,
) {
    let x = force_interpolation_x.interpolate(amount, random);
    *result = Vector3::new(x, x, x);
}

pub fn interpolate_four(
    force_interpolation_x: &FloatInterpolation,
    force_interpolation_y: &FloatInterpolation,
    force_interpolation_z: &FloatInterpolation,
    amount: f32,
    random: f32,
    result: &mut Color4,
) {
    let r = force_interpolation_x.interpolate(amount, random);
    let g = force_interpolation_y.interpolate(amount, random);
    let b = force_interpolation_z.interpolate(amount, random);
    let a = force_interpolation_z.interpolate(amount, random);
    *result = Color4::new(r, g, b, a);
}

#[derive(Clone)]
pub struct Vector3Interpolate {
    pub x: FloatInterpolation,
    pub y: FloatInterpolation,
    pub z: FloatInterpolation,
}

impl Vector3Interpolate {
    pub fn new(x: FloatInterpolation, y: FloatInterpolation, z: FloatInterpolation) -> Self {
        Self { x, y, z }
    }
}

impl Default for Vector3Interpolate {
    fn default() -> Self {
        Self {
            x: FloatInterpolation::default(),
            y: FloatInterpolation::default(),
            z: FloatInterpolation::default(),
        }
    }
}

pub fn apply_vector3_interpolation(config: &ParamInfo, target: &mut Vector3Interpolate) {
    apply_float_interpolation_from_multi(config, 0, &mut target.x);
    apply_float_interpolation_from_multi(config, 1, &mut target.y);
    apply_float_interpolation_from_multi(config, 2, &mut target.z);
}

pub fn apply_float_interpolation_from_multi(
    config: &ParamInfo,
    index: usize,
    target: &mut FloatInterpolation,
) {
    // let temp = target;

    match config {
        ParamInfo::OneParamInfo(info) => apply_float_interpolation_from_one_param_info(info, target),
        ParamInfo::ThreeParamInfo(info) => {
            apply_float_interpolation_from_three_param_info(info, index, target)
        }
    }
}

pub fn apply_float_interpolation_from_one_param_info(
    config: &OneParamInfo,
    target: &mut FloatInterpolation,
) {
    match config {
        OneParamInfo::TInterpolateConstant(one_param) => {
            target.mode = EInterpolationCurveMode::Constant;
            target.constant0 = Some(*one_param);
            target.constant1 = Some(*one_param);
        }
        OneParamInfo::TInterpolateTwoConstants(one_param1, one_param2) => {
            target.mode = EInterpolationCurveMode::TwoConstants;
            target.constant0 = Some(*one_param1);
            target.constant1 = Some(*one_param2);
        }
        OneParamInfo::TInterpolateCurve(one_param_curve) => {
            target.mode = EInterpolationCurveMode::Curve;
            target.min_curve = Some(one_param_curve.clone());
            target.max_curve = Some(one_param_curve.clone());
        }
        OneParamInfo::TInterpolateTwoCurves(one_param_curve1, one_param_curve2) => {
            target.mode = EInterpolationCurveMode::Curve;
            target.min_curve = Some(one_param_curve1.clone());
            target.max_curve = Some(one_param_curve2.clone());
        }
    }
}

pub fn apply_float_interpolation_from_three_param_info(
    config: &ThreeParamInfo,
    index: usize,
    target: &mut FloatInterpolation,
) {
    match config {
        ThreeParamInfo::TInterpolateConstant(three_param) => {
            target.mode = EInterpolationCurveMode::Constant;
            target.constant0 = Some(three_param[index]);
            target.constant1 = Some(three_param[index]);
        }
        ThreeParamInfo::TInterpolateTwoConstants(three_param1, three_param2) => {
            target.mode = EInterpolationCurveMode::TwoConstants;
            target.constant0 = Some(three_param1[index]);
            target.constant1 = Some(three_param2[index]);
        }
        ThreeParamInfo::TInterpolateCurve(three_param) => {
            target.mode = EInterpolationCurveMode::Curve;
            target.min_curve = Some(three_param[index].clone());
            target.max_curve = Some(three_param[index].clone());
        }
        ThreeParamInfo::TInterpolateTwoCurves(three_param1, three_param2) => {
            target.mode = EInterpolationCurveMode::Curve;
            target.min_curve = Some(three_param1[index].clone());
            target.max_curve = Some(three_param2[index].clone());
        }
    }
}

#[derive(Clone)]
pub struct TranslationInterpolate {
    pub _is_axis: bool,
    vector3_interpolate: Vector3Interpolate,
    interpolate: Arc<
        dyn Fn(
            &FloatInterpolation,
            &FloatInterpolation,
            &FloatInterpolation,
            f32,
            f32,
            &mut Vector3,
        ),
    >,
    pub transform_force: Arc<dyn Fn(&Vector3, Matrix, &mut Vector3)>,
}

impl TranslationInterpolate {
    pub fn set_is_axis(&mut self, value: bool) {
        if self._is_axis != value {
            self._is_axis = value;
            if value {
                self.interpolate = Arc::new(interpolate_three);
            } else {
                self.interpolate = Arc::new(interpolate_translation_no_axis);
            }
        }
    }
    pub fn get_is_axis(&self) -> bool {
        return self._is_axis;
    }

    pub fn new(x: FloatInterpolation, y: FloatInterpolation, z: FloatInterpolation) -> Self {
        Self {
            _is_axis: false,
            vector3_interpolate: Vector3Interpolate { x, y, z },
            interpolate: Arc::new(interpolate_translation_no_axis),
            transform_force: Arc::new(transform_vector_as_local_space),
        }
    }

    pub fn compute(&self, amount: f32, random: f32, transform_invert: Matrix, result: &mut Vector3) {
        (self.interpolate)(
            &self.vector3_interpolate.x,
            &self.vector3_interpolate.y,
            &self.vector3_interpolate.z,
            amount,
            random,
            result,
        );
        (self.transform_force)(&result.clone(), transform_invert, result);
    }

    pub fn format(config: &ParamInfo, target: &mut TranslationInterpolate) {
        if let ParamInfo::OneParamInfo(_info) = &config {
            target._is_axis = true;
        }

        apply_vector3_interpolation(config, &mut target.vector3_interpolate);
    }
}

#[derive(Clone)]
pub struct RotationInterpolate {
    _is_axis: bool,
    vector3_interpolate: Vector3Interpolate,
    interpolate: Arc<
        dyn Fn(
            &FloatInterpolation,
            &FloatInterpolation,
            &FloatInterpolation,
            f32,
            f32,
            &mut Vector3,
        ),
    >,
}

// impl Default for RotationInterpolate{
//     fn default() -> Self {
//         Self { _isAxis: false, vector3Interpolate: Vector3Interpolate::default(), interpolate: () }
//     }
// }

impl RotationInterpolate {
    pub fn set_is_axis(&mut self, value: bool) {
        if self._is_axis != value {
            self._is_axis = value;
            if value {
                self.interpolate = Arc::new(interpolate_three);
            } else {
                self.interpolate = Arc::new(interpolate_rotation_no_axis);
            }
        }
    }

    pub fn get_is_axis(&mut self) -> bool {
        return self._is_axis;
    }

    pub fn new(x: FloatInterpolation, y: FloatInterpolation, z: FloatInterpolation) -> Self {
        Self {
            _is_axis: false,
            vector3_interpolate: Vector3Interpolate { x, y, z },
            interpolate: Arc::new(interpolate_rotation_no_axis),
        }
    }

    pub fn compute(&self, amount: f32, random: f32, result: &mut Vector3) {
        (self.interpolate)(
            &self.vector3_interpolate.x,
            &self.vector3_interpolate.y,
            &self.vector3_interpolate.z,
            amount,
            random,
            result,
        );
    }

    pub fn format(config: &ParamInfo, target: &mut RotationInterpolate) {
        if let ParamInfo::OneParamInfo(_) = &config {
            target._is_axis = true;
        }

        apply_vector3_interpolation(config, &mut target.vector3_interpolate);
    }
}

#[derive(Clone)]
pub struct ScalingInterpolate {
    _is_axis: bool,
    vector3_interpolate: Vector3Interpolate,
    interpolate: Arc<
        dyn Fn(
            &FloatInterpolation,
            &FloatInterpolation,
            &FloatInterpolation,
            f32,
            f32,
            &mut Vector3,
        ),
    >,
}

impl Default for ScalingInterpolate {
    fn default() -> Self {
        Self {
            _is_axis: false,
            vector3_interpolate: Vector3Interpolate::default(),
            interpolate: Arc::new(interpolate_scaling_no_axis),
        }
    }
}

impl ScalingInterpolate {
    pub fn set_is_axis(&mut self, value: bool) {
        if self._is_axis != value {
            self._is_axis = value;
            if value {
                self.interpolate = Arc::new(interpolate_three);
            } else {
                self.interpolate = Arc::new(interpolate_scaling_no_axis);
            }
        }
    }
    pub fn get_is_axis(&self) -> bool {
        return self._is_axis;
    }

    pub fn new(x: FloatInterpolation, y: FloatInterpolation, z: FloatInterpolation) -> Self {
        Self {
            _is_axis: false,
            vector3_interpolate: Vector3Interpolate { x, y, z },
            interpolate: Arc::new(interpolate_scaling_no_axis),
        }
    }

    pub fn compute(&self, amount: f32, random: f32, result: &mut Vector3) {
        (self.interpolate)(
            &self.vector3_interpolate.x,
            &self.vector3_interpolate.y,
            &self.vector3_interpolate.z,
            amount,
            random,
            result,
        );
    }

    pub fn format(config: &ParamInfo, target: &mut ScalingInterpolate) {
        if let ParamInfo::OneParamInfo(_) = &config {
            target._is_axis = true;
        }

        apply_vector3_interpolation(config, &mut target.vector3_interpolate);
    }
}

#[derive(Clone, Debug)]
pub struct Color4Interpolate {
    pub temp_result: [f32; 4],
    pub gradient: Color4Gradient,
}
impl Color4Interpolate {
    pub fn compute(&mut self, amount: f32, result: &mut Color4, start_amount: f32) {
        self.gradient
            .interpolate(amount, &mut self.temp_result, start_amount);

        *result = Color4::new(
            self.temp_result[0],
            self.temp_result[1],
            self.temp_result[2],
            self.temp_result[3],
        )
    }

    pub fn format(config: &FourGradientInfo, target: &mut Color4Interpolate) {
        // let interpolation = &mut target.gradient;

        match config {
            FourGradientInfo::TInterpolateRandom => {
                target.gradient.mode = EInterpolationGradienMode::Random;
            }
            FourGradientInfo::TInterpolateColor(config) => {
                target.gradient.mode = EInterpolationGradienMode::Color;
                target.gradient.constant0 = Some(config.clone());
                target.gradient.constant1 = Some(*config);
            }
            FourGradientInfo::TInterpolateTwoColors(param1, param2) => {
                target.gradient.mode = EInterpolationGradienMode::TwoColors;
                println!("param1:{:?},param2:{:?}", param1, param2);
                target.gradient.constant0 = Some(*param1);
                target.gradient.constant1 = Some(*param2);
            }
            FourGradientInfo::TInterpolateGradient(gradient) => {
                target.gradient.mode = EInterpolationGradienMode::Gradient;
                target.gradient.min_gradients = Some(gradient.clone());
                target.gradient.max_gradients = Some(gradient.clone());
            }
            FourGradientInfo::TInterpolateTwoGradients(gradient1, gradient2) => {
                target.gradient.mode = EInterpolationGradienMode::TwoGradients;
                target.gradient.min_gradients = Some(gradient1.clone());
                target.gradient.max_gradients = Some(gradient2.clone());
            }
        }
    }

    pub fn new(gradient: Color4Gradient) -> Self {
        Self {
            temp_result: [1.; 4],
            gradient,
        }
    }
}

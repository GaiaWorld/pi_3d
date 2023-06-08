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

pub const TempVector3A: Vector3 = Vector3::new(0., 0., 0.);
pub const TempVector3B: Vector3 = Vector3::new(0., 0., 0.);
pub const TempVector3C: Vector3 = Vector3::new(0., 0., 0.);
pub const TempVector3D: Vector3 = Vector3::new(0., 0., 0.);
pub const TempVector3E: Vector3 = Vector3::new(0., 0., 0.);

pub const TempColor4A: Color4 = Color4::new(1., 1., 1., 1.);
pub const TempColor4B: Color4 = Color4::new(1., 1., 1., 1.);
pub const TempColor4C: Color4 = Color4::new(1., 1., 1., 1.);

lazy_static! {
    pub static ref TempQuaternionA: Quaternion = Quaternion::default();
    pub static ref TempQuaternionB: Quaternion = Quaternion::default();
    pub static ref TempQuaternionC: Quaternion = Quaternion::default();
    pub static ref TempMatrixA: Matrix = Matrix::identity();
    pub static ref TempMatrixB: Matrix = Matrix::identity();
    pub static ref TempMatrixC: Matrix = Matrix::identity();
}

pub trait IParticleModifier {
    fn modify(&mut self, particle: &mut Particle, amount: &mut f32, deltaSeconds: f32);
}

pub fn transformVectorAsLocalSpace(
    source: &Vector3,
    transformInvert: Matrix,
    result: &mut Vector3,
) {
    // no thing
}

pub fn transformVectorAsWorldSpace(
    source: &Vector3,
    transformInvert: Matrix,
    result: &mut Vector3,
) {
    *result = transformInvert.transform_vector(source);
}

pub fn interpolateThree(
    forceInterpolationX: &FloatInterpolation,
    forceInterpolationY: &FloatInterpolation,
    forceInterpolationZ: &FloatInterpolation,
    amount: f32,
    random: f32,
    result: &mut Vector3,
) {
    let x = forceInterpolationX.interpolate(amount, random);
    let y = forceInterpolationY.interpolate(amount, random);
    let z = forceInterpolationZ.interpolate(amount, random);
    *result = Vector3::new(x, y, z);
}

pub fn interpolateTranslationNoAxis(
    forceInterpolationX: &FloatInterpolation,
    forceInterpolationY: &FloatInterpolation,
    forceInterpolationZ: &FloatInterpolation,
    amount: f32,
    random: f32,
    result: &mut Vector3,
) {
    let x = forceInterpolationX.interpolate(amount, random);
    *result = Vector3::new(x, x, x);
}

pub fn interpolateRotationNoAxis(
    forceInterpolationX: &FloatInterpolation,
    forceInterpolationY: &FloatInterpolation,
    forceInterpolationZ: &FloatInterpolation,
    amount: f32,
    random: f32,
    result: &mut Vector3,
) {
    let z = forceInterpolationX.interpolate(amount, random);
    *result = Vector3::new(0., 0., z);
}

pub fn interpolateScalingNoAxis(
    forceInterpolationX: &FloatInterpolation,
    forceInterpolationY: &FloatInterpolation,
    forceInterpolationZ: &FloatInterpolation,
    amount: f32,
    random: f32,
    result: &mut Vector3,
) {
    let x = forceInterpolationX.interpolate(amount, random);
    *result = Vector3::new(x, x, x);
}

pub fn interpolateFour(
    forceInterpolationX: &FloatInterpolation,
    forceInterpolationY: &FloatInterpolation,
    forceInterpolationZ: &FloatInterpolation,
    amount: f32,
    random: f32,
    result: &mut Color4,
) {
    let r = forceInterpolationX.interpolate(amount, random);
    let g = forceInterpolationY.interpolate(amount, random);
    let b = forceInterpolationZ.interpolate(amount, random);
    let a = forceInterpolationZ.interpolate(amount, random);
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

pub fn applyVector3Interpolation(config: &ParamInfo, target: &mut Vector3Interpolate) {
    applyFloatInterpolationFromMulti(config, 0, &mut target.x);
    applyFloatInterpolationFromMulti(config, 1, &mut target.y);
    applyFloatInterpolationFromMulti(config, 2, &mut target.z);
}

pub fn applyFloatInterpolationFromMulti(
    config: &ParamInfo,
    index: usize,
    target: &mut FloatInterpolation,
) {
    // let temp = target;

    match config {
        ParamInfo::OneParamInfo(info) => applyFloatInterpolationFromOneParamInfo(info, target),
        ParamInfo::ThreeParamInfo(info) => {
            applyFloatInterpolationFromThreeParamInfo(info, index, target)
        }
    }
}

pub fn applyFloatInterpolationFromOneParamInfo(
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
            target.minCurve = Some(one_param_curve.clone());
            target.maxCurve = Some(one_param_curve.clone());
        }
        OneParamInfo::TInterpolateTwoCurves(one_param_curve1, one_param_curve2) => {
            target.mode = EInterpolationCurveMode::Curve;
            target.minCurve = Some(one_param_curve1.clone());
            target.maxCurve = Some(one_param_curve2.clone());
        }
    }
}

pub fn applyFloatInterpolationFromThreeParamInfo(
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
            target.minCurve = Some(three_param[index].clone());
            target.maxCurve = Some(three_param[index].clone());
        }
        ThreeParamInfo::TInterpolateTwoCurves(three_param1, three_param2) => {
            target.mode = EInterpolationCurveMode::Curve;
            target.minCurve = Some(three_param1[index].clone());
            target.maxCurve = Some(three_param2[index].clone());
        }
    }
}

#[derive(Clone)]
pub struct TranslationInterpolate {
    pub _isAxis: bool,
    vector3Interpolate: Vector3Interpolate,
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
    pub transformForce: Arc<dyn Fn(&Vector3, Matrix, &mut Vector3)>,
}

impl TranslationInterpolate {
    pub fn set_isAxis(&mut self, value: bool) {
        if (self._isAxis != value) {
            self._isAxis = value;
            if (value) {
                self.interpolate = Arc::new(interpolateThree);
            } else {
                self.interpolate = Arc::new(interpolateTranslationNoAxis);
            }
        }
    }
    pub fn get_isAxis(&self) -> bool {
        return self._isAxis;
    }

    pub fn new(x: FloatInterpolation, y: FloatInterpolation, z: FloatInterpolation) -> Self {
        Self {
            _isAxis: false,
            vector3Interpolate: Vector3Interpolate { x, y, z },
            interpolate: Arc::new(interpolateTranslationNoAxis),
            transformForce: Arc::new(transformVectorAsLocalSpace),
        }
    }

    pub fn compute(&self, amount: f32, random: f32, transformInvert: Matrix, result: &mut Vector3) {
        (self.interpolate)(
            &self.vector3Interpolate.x,
            &self.vector3Interpolate.y,
            &self.vector3Interpolate.z,
            amount,
            random,
            result,
        );
        (self.transformForce)(&result.clone(), transformInvert, result);
    }

    pub fn format(config: &ParamInfo, target: &mut TranslationInterpolate) {
        if let ParamInfo::OneParamInfo(info) = &config {
            target._isAxis = true;
        }

        applyVector3Interpolation(config, &mut target.vector3Interpolate);
    }
}

#[derive(Clone)]
pub struct RotationInterpolate {
    _isAxis: bool,
    vector3Interpolate: Vector3Interpolate,
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
    pub fn set_isAxis(&mut self, value: bool) {
        if (self._isAxis != value) {
            self._isAxis = value;
            if (value) {
                self.interpolate = Arc::new(interpolateThree);
            } else {
                self.interpolate = Arc::new(interpolateRotationNoAxis);
            }
        }
    }

    pub fn get_isAxis(&mut self) -> bool {
        return self._isAxis;
    }

    pub fn new(x: FloatInterpolation, y: FloatInterpolation, z: FloatInterpolation) -> Self {
        Self {
            _isAxis: false,
            vector3Interpolate: Vector3Interpolate { x, y, z },
            interpolate: Arc::new(interpolateRotationNoAxis),
        }
    }

    pub fn compute(&self, amount: f32, random: f32, result: &mut Vector3) {
        (self.interpolate)(
            &self.vector3Interpolate.x,
            &self.vector3Interpolate.y,
            &self.vector3Interpolate.z,
            amount,
            random,
            result,
        );
    }

    pub fn format(config: &ParamInfo, target: &mut RotationInterpolate) {
        if let ParamInfo::OneParamInfo(_) = &config {
            target._isAxis = true;
        }

        applyVector3Interpolation(config, &mut target.vector3Interpolate);
    }
}

#[derive(Clone)]
pub struct ScalingInterpolate {
    _isAxis: bool,
    vector3Interpolate: Vector3Interpolate,
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
            _isAxis: false,
            vector3Interpolate: Vector3Interpolate::default(),
            interpolate: Arc::new(interpolateScalingNoAxis),
        }
    }
}

impl ScalingInterpolate {
    pub fn set_isAxis(&mut self, value: bool) {
        if (self._isAxis != value) {
            self._isAxis = value;
            if (value) {
                self.interpolate = Arc::new(interpolateThree);
            } else {
                self.interpolate = Arc::new(interpolateScalingNoAxis);
            }
        }
    }
    pub fn get_isAxis(&self) -> bool {
        return self._isAxis;
    }

    pub fn new(x: FloatInterpolation, y: FloatInterpolation, z: FloatInterpolation) -> Self {
        Self {
            _isAxis: false,
            vector3Interpolate: Vector3Interpolate { x, y, z },
            interpolate: Arc::new(interpolateScalingNoAxis),
        }
    }

    pub fn compute(&self, amount: f32, random: f32, result: &mut Vector3) {
        (self.interpolate)(
            &self.vector3Interpolate.x,
            &self.vector3Interpolate.y,
            &self.vector3Interpolate.z,
            amount,
            random,
            result,
        );
    }

    pub fn format(config: &ParamInfo, target: &mut ScalingInterpolate) {
        if let ParamInfo::OneParamInfo(_) = &config {
            target._isAxis = true;
        }

        applyVector3Interpolation(config, &mut target.vector3Interpolate);
    }
}

#[derive(Clone, Debug)]
pub struct Color4Interpolate {
    pub tempResult: [f32; 4],
    pub gradient: Color4Gradient,
}
impl Color4Interpolate {
    pub fn compute(&mut self, amount: f32, result: &mut Color4, startAmount: f32) {
        self.gradient
            .interpolate(amount, &mut self.tempResult, startAmount);

        *result = Color4::new(
            self.tempResult[0],
            self.tempResult[1],
            self.tempResult[2],
            self.tempResult[3],
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
                target.gradient.minGradients = Some(gradient.clone());
                target.gradient.maxGradients = Some(gradient.clone());
            }
            FourGradientInfo::TInterpolateTwoGradients(gradient1, gradient2) => {
                target.gradient.mode = EInterpolationGradienMode::TwoGradients;
                target.gradient.minGradients = Some(gradient1.clone());
                target.gradient.maxGradients = Some(gradient2.clone());
            }
        }
    }

    pub fn new(gradient: Color4Gradient) -> Self {
        Self {
            tempResult: [1.; 4],
            gradient,
        }
    }
}

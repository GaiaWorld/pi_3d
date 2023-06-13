use pi_scene_math::{Quaternion, Vector3};

use crate::{
    interpolation::{FloatInterpolation, IInterpolation},
    iparticle_system_config::EInterpolationCurveMode,
    particle::Particle,
};

use super::base::{
    IParticleModifier, TempQuaternionA, TempVector3A, TempVector3B, TempVector3C, TempVector3D,
    TranslationInterpolate,
};

#[derive(Clone)]
pub struct LocalPosition {
    pub orbitalRotateSpeed: TranslationInterpolate,
    pub orbitalOffset: TranslationInterpolate,
    pub radial: FloatInterpolation,
    pub speedModifier: FloatInterpolation,
}

impl LocalPosition {
    pub fn new() -> Self {
        let mut speedModifier = FloatInterpolation::new();
        speedModifier.mode = EInterpolationCurveMode::Constant;
        speedModifier.constant0 = Some(1.0);
        Self {
            orbitalRotateSpeed: TranslationInterpolate::new(
                FloatInterpolation::new(),
                FloatInterpolation::new(),
                FloatInterpolation::new(),
            ),
            orbitalOffset: TranslationInterpolate::new(
                FloatInterpolation::new(),
                FloatInterpolation::new(),
                FloatInterpolation::new(),
            ),
            radial: FloatInterpolation::new(),
            speedModifier,
        }
    }
}

impl IParticleModifier for LocalPosition {
    fn modify(&mut self, particle: &mut Particle, amount: &mut f32, deltaSeconds: f32) {
        particle.direction_length = particle.direction.magnitude();

        let mut orbitalRotate = TempVector3B;
        self.orbitalRotateSpeed.compute(
            *amount,
            particle.base_random,
            particle.start_world_matrix_invert,
            &mut orbitalRotate,
        );

        let speedModifier = self
            .speedModifier
            .interpolate(*amount, particle.base_random);

        let radial = self.radial.interpolate(*amount, particle.base_random);

        let mut centerOffset = TempVector3C;
        self.orbitalOffset.compute(
            *amount,
            particle.base_random,
            particle.start_world_matrix_invert,
            &mut centerOffset,
        );

        // 旋转量
        orbitalRotate = orbitalRotate * (deltaSeconds * speedModifier);

        // 旋转的目标位置
        let mut orbtialDiff = TempVector3D;
        orbtialDiff = particle.position - centerOffset;

        if orbtialDiff.magnitude_squared() > 0. {
            let rotate =
                Quaternion::from_euler_angles(orbitalRotate[0], orbitalRotate[1], orbitalRotate[2]);

            let temp = rotate.transform_vector(&(particle.position - centerOffset));
            orbtialDiff = temp + centerOffset;

            // 基础径向向量
            // orbtialDiff.subtractToRef(centerOffset, orbtialDiff);
            orbtialDiff = orbtialDiff - centerOffset;

            let len = orbtialDiff.magnitude();
            let scale = (len + radial * deltaSeconds) / len;
            orbtialDiff = orbtialDiff * scale;

            // 新的位置
            orbtialDiff = centerOffset + orbtialDiff;

            // 最终径向位移
            orbtialDiff = orbtialDiff - particle.position;
        } else {
            orbtialDiff = Vector3::new(0., 0., 0.);
        }

        let mut localResult = TempVector3A;
        // particle.direction.scaleToRef(deltaSeconds, localResult);
        localResult = particle.direction * deltaSeconds;
        localResult = localResult * speedModifier;
        // println!("particle.direction: {:?}, deltaSeconds: {:?}, speedModifier: {:?}",particle.direction, deltaSeconds, speedModifier);
        particle.readldirection = orbtialDiff;

        particle.readldirection = particle.readldirection + localResult;
        // println!(
        //     "LocalPosition1 particle.position: {:?}, particle.readldirection: {:?}",
        //     particle.position, particle.readldirection
        // );
        particle.position = particle.position + (particle.readldirection);
        // println!(
        //     "LocalPosition2 particle.position: {:?}, particle.readldirection: {:?}",
        //     particle.position, particle.readldirection
        // );

        if (deltaSeconds != 0.) {
            particle.readldirection = particle.readldirection * (1. / deltaSeconds.abs());
            particle.direction_length = particle.readldirection.magnitude();
        }
    }
}

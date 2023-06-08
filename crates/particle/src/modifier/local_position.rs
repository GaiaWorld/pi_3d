use pi_scene_math::{Quaternion, Vector3};

use crate::{
    interpolation::{FloatInterpolation, IInterpolation},
    particle::Particle,
};

use super::base::{
    TempQuaternionA, TempVector3A, TempVector3B, TempVector3C, TempVector3D, TranslationInterpolate, IParticleModifier,
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
            speedModifier: FloatInterpolation::new(),
        }
    }
}

impl IParticleModifier for LocalPosition{
    fn modify(&mut self, particle: &mut Particle, amount: &mut f32, deltaSeconds: f32) {
        particle.direction_length = particle.direction.magnitude();

        let mut orbitalRotate = TempVector3B;
        self.orbitalRotateSpeed.compute(
            *amount,
            particle.base_random,
            particle.start_world_matrix_invert,
            &mut orbitalRotate,
        );

        let speedModifier = self.speedModifier.interpolate(*amount, particle.base_random);

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

        if (orbtialDiff.magnitude_squared() > 0.) {
            let mut rotate =
                Quaternion::from_euler_angles(orbitalRotate[0], orbitalRotate[1], orbitalRotate[2]);

            orbtialDiff = rotate.transform_vector(&(centerOffset + (-particle.position)));
            // let rotated_v = Rotation3::from_quaternion(rotation_quaternion) * rotated_v + rotation_point.coords;

            // particle
            //     .position
            //     .rotateByQuaternionAroundPointToRef(rotate, centerOffset, orbtialDiff);

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

        particle.readldirection = (orbtialDiff);
        particle.readldirection = particle.readldirection + (localResult);

        particle.position = particle.position + (particle.readldirection);

        if (deltaSeconds != 0.) {
            particle.readldirection = particle.readldirection * (1. / deltaSeconds.abs());
            particle.direction_length = particle.readldirection.magnitude();
        }
    }
}
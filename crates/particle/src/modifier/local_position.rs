use pi_scene_math::{Quaternion, Vector3};

use crate::{
    interpolation::{FloatInterpolation, IInterpolation},
    iparticle_system_config::EInterpolationCurveMode,
    particle::Particle,
};

use super::base::{
    IParticleModifier, TranslationInterpolate, TEMP_VECTOR3_A, TEMP_VECTOR3_B, TEMP_VECTOR3_C,
    TEMP_VECTOR3_D,
};

#[derive(Clone)]
pub struct LocalPosition {
    pub orbital_rotate_speed: TranslationInterpolate,
    pub orbital_offset: TranslationInterpolate,
    pub radial: FloatInterpolation,
    pub speed_modifier: FloatInterpolation,
}

impl LocalPosition {
    pub fn new() -> Self {
        let mut speed_modifier = FloatInterpolation::new();
        speed_modifier.mode = EInterpolationCurveMode::Constant;
        speed_modifier.constant0 = Some(1.0);
        Self {
            orbital_rotate_speed: TranslationInterpolate::new(
                FloatInterpolation::new(),
                FloatInterpolation::new(),
                FloatInterpolation::new(),
            ),
            orbital_offset: TranslationInterpolate::new(
                FloatInterpolation::new(),
                FloatInterpolation::new(),
                FloatInterpolation::new(),
            ),
            radial: FloatInterpolation::new(),
            speed_modifier,
        }
    }
}

impl IParticleModifier for LocalPosition {
    fn modify(&mut self, particle: &mut Particle, amount: &mut f32, delta_seconds: f32) {
        particle.direction_length = particle.direction.magnitude();

        let mut orbital_rotate = TEMP_VECTOR3_B;
        self.orbital_rotate_speed.compute(
            *amount,
            particle.base_random,
            particle.start_world_matrix_invert,
            &mut orbital_rotate,
        );

        let speed_modifier = self
            .speed_modifier
            .interpolate(*amount, particle.base_random);

        let radial = self.radial.interpolate(*amount, particle.base_random);

        let mut center_offset = TEMP_VECTOR3_C;
        self.orbital_offset.compute(
            *amount,
            particle.base_random,
            particle.start_world_matrix_invert,
            &mut center_offset,
        );

        // 旋转量
        orbital_rotate = orbital_rotate * (delta_seconds * speed_modifier);

        // 旋转的目标位置
        let mut _orbtial_diff = TEMP_VECTOR3_D;
        _orbtial_diff = particle.position - center_offset;

        if _orbtial_diff.magnitude_squared() > 0. {
            let rotate = Quaternion::from_euler_angles(
                orbital_rotate[0],
                orbital_rotate[1],
                orbital_rotate[2],
            );

            let temp = rotate.transform_vector(&(particle.position - center_offset));
            _orbtial_diff = temp + center_offset;

            // 基础径向向量
            // orbtialDiff.subtractToRef(centerOffset, orbtialDiff);
            _orbtial_diff = _orbtial_diff - center_offset;

            let len = _orbtial_diff.magnitude();
            let scale = (len + radial * delta_seconds) / len;
            _orbtial_diff = _orbtial_diff * scale;

            // 新的位置
            _orbtial_diff = center_offset + _orbtial_diff;

            // 最终径向位移
            _orbtial_diff = _orbtial_diff - particle.position;
        } else {
            _orbtial_diff = Vector3::new(0., 0., 0.);
        }

        let mut _local_result = TEMP_VECTOR3_A;
        _local_result = particle.direction * delta_seconds;
        _local_result = _local_result * speed_modifier;

        particle.readldirection = _orbtial_diff;
        particle.readldirection = particle.readldirection + _local_result;
        particle.position = particle.position + (particle.readldirection);

        if delta_seconds != 0. {
            particle.readldirection = particle.readldirection * (1. / delta_seconds.abs());
            particle.direction_length = particle.readldirection.magnitude();
        }
    }
}

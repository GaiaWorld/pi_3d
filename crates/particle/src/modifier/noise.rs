use pi_scene_math::Vector3;
use rand::Rng;

use crate::particle::Particle;

use super::base::IParticleModifier;

#[derive(Clone)]
pub struct Noise {
    position_amount: f32,
    rotation_amount: f32,
    sizen_amount: f32,
    _frequency: f32,
    last_record: f32,
    randoms: [f32; 9],
}

impl Noise {
    
    pub fn new() -> Self {
        Self {
            position_amount: 1.,
            rotation_amount: 0.,
            sizen_amount: 0.,
            _frequency: 1.,
            last_record: -1.,
            randoms: [1.; 9],
        }
    }
}

impl IParticleModifier for Noise {
    fn modify(&mut self, particle: &mut Particle, _amount: &mut f32, _delta_seconds: f32) {
        let delta_seconds = 1.;
        let tt = particle.age / particle.lifetime * 4.;
        let t = tt.floor();
        let s = ((tt - t) * std::f32::consts::PI * 2.).sin();

        let mut rng = rand::thread_rng();

        if self.last_record != t {
            self.last_record = t;
            self.randoms[0] = (rng.gen::<f32>() - 0.5) * self.position_amount * delta_seconds;
            self.randoms[1] = (rng.gen::<f32>() - 0.5) * self.position_amount * delta_seconds;
            self.randoms[2] = (rng.gen::<f32>() - 0.5) * self.position_amount * delta_seconds;

            self.randoms[3] = (rng.gen::<f32>() - 0.5) * self.rotation_amount * delta_seconds;
            self.randoms[4] = (rng.gen::<f32>() - 0.5) * self.rotation_amount * delta_seconds;
            self.randoms[5] = (rng.gen::<f32>() - 0.5) * self.rotation_amount * delta_seconds;

            self.randoms[6] = (rng.gen::<f32>() - 0.5) * self.sizen_amount * delta_seconds;
            self.randoms[7] = (rng.gen::<f32>() - 0.5) * self.sizen_amount * delta_seconds;
            self.randoms[8] = (rng.gen::<f32>() - 0.5) * self.sizen_amount * delta_seconds;
        }
        particle.position = particle.position
            + Vector3::new(
                self.randoms[0] * s,
                self.randoms[1] * s,
                self.randoms[2] * s,
            );

        particle.rotation = particle.rotation
            + Vector3::new(
                self.randoms[3] * s,
                self.randoms[4] * s,
                self.randoms[5] * s,
            );

        particle.scaling = particle.scaling
            + Vector3::new(
                self.randoms[6] * s,
                self.randoms[7] * s,
                self.randoms[8] * s,
            );
    }
}

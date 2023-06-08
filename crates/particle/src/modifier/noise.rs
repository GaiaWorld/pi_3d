use pi_scene_math::Vector3;
use rand::Rng;

use crate::particle::Particle;

use super::base::IParticleModifier;

#[derive(Clone)]
pub struct Noise {
    positionAmount: f32,
    rotationAmount: f32,
    sizenAmount: f32,
    frequency: f32,
    lastRecord: f32,
    randoms: [f32; 9],
}

impl Noise {
    
    pub fn new() -> Self {
        Self {
            positionAmount: 1.,
            rotationAmount: 0.,
            sizenAmount: 0.,
            frequency: 1.,
            lastRecord: -1.,
            randoms: [1.; 9],
        }
    }
}

impl IParticleModifier for Noise {
    fn modify(&mut self, particle: &mut Particle, amount: &mut f32, deltaSeconds: f32) {
        let deltaSeconds = 1.;
        let tt = particle.age / particle.lifetime * 4.;
        let t = tt.floor();
        let s = ((tt - t) * std::f32::consts::PI * 2.).sin();

        let mut rng = rand::thread_rng();

        if (self.lastRecord != t) {
            self.lastRecord = t;
            self.randoms[0] = (rng.gen::<f32>() - 0.5) * self.positionAmount * deltaSeconds;
            self.randoms[1] = (rng.gen::<f32>() - 0.5) * self.positionAmount * deltaSeconds;
            self.randoms[2] = (rng.gen::<f32>() - 0.5) * self.positionAmount * deltaSeconds;

            self.randoms[3] = (rng.gen::<f32>() - 0.5) * self.rotationAmount * deltaSeconds;
            self.randoms[4] = (rng.gen::<f32>() - 0.5) * self.rotationAmount * deltaSeconds;
            self.randoms[5] = (rng.gen::<f32>() - 0.5) * self.rotationAmount * deltaSeconds;

            self.randoms[6] = (rng.gen::<f32>() - 0.5) * self.sizenAmount * deltaSeconds;
            self.randoms[7] = (rng.gen::<f32>() - 0.5) * self.sizenAmount * deltaSeconds;
            self.randoms[8] = (rng.gen::<f32>() - 0.5) * self.sizenAmount * deltaSeconds;
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

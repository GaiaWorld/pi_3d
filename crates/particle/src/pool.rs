use pi_scene_math::Vector3;

use crate::particle::Particle;

pub struct Pool {
    particleList: Vec<Particle>,
    vector3List: Vec<Vector3>,
}

impl Pool {
    pub fn createParticle(&mut self) -> Particle {
        if let Some(value) = self.particleList.pop() {
            return value;
        }

        return Particle::default();
    }

    pub fn recycleParticle(&mut self, mut value: Particle) {
        value.reset();
        self.particleList.push(value);
    }

    pub fn createVector3(&mut self) -> Vector3 {
        if let Some(value) = self.vector3List.pop() {
            return value;
        }

        return Vector3::zeros();
    }

    pub fn recycleVector3(&mut self, value: Vector3) {
        self.vector3List.push(value);
    }

    pub fn new() -> Self {
        Self {
            particleList: vec![],
            vector3List: vec![],
        }
    }
}

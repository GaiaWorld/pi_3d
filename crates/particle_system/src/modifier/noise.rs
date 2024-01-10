use pi_scene_shell::prelude::*;

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
    pub fn modify(&mut self, result: &mut [f32; 9], amount: f32, randoms: &BaseRandom) {
        let delta_seconds = 1.;
        // let tt = particle.age / particle.lifetime * 4.;
        let tt = amount * 4.;
        let t = tt.floor();
        let s = ((tt - t) * std::f32::consts::PI * 2.).sin();

        if self.last_record != t {
            self.last_record = t;
            self.randoms[0] = (randoms.x - 0.5) * self.position_amount * delta_seconds;
            self.randoms[1] = (randoms.y - 0.5) * self.position_amount * delta_seconds;
            self.randoms[2] = (randoms.z - 0.5) * self.position_amount * delta_seconds;
            self.randoms[3] = (randoms.x - 0.5) * self.rotation_amount * delta_seconds;
            self.randoms[4] = (randoms.y - 0.5) * self.rotation_amount * delta_seconds;
            self.randoms[5] = (randoms.z - 0.5) * self.rotation_amount * delta_seconds;
            self.randoms[6] = (randoms.x - 0.5) * self.sizen_amount * delta_seconds;
            self.randoms[7] = (randoms.y - 0.5) * self.sizen_amount * delta_seconds;
            self.randoms[8] = (randoms.z - 0.5) * self.sizen_amount * delta_seconds;
        }
        result[0] = self.randoms[0] * s;
        result[1] = self.randoms[1] * s;
        result[2] = self.randoms[2] * s;
        result[3] = self.randoms[3] * s;
        result[4] = self.randoms[4] * s;
        result[5] = self.randoms[5] * s;
        result[6] = self.randoms[6] * s;
        result[7] = self.randoms[7] * s;
        result[8] = self.randoms[8] * s;
    }
}

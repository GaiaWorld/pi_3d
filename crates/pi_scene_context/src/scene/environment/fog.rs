

use pi_engine_shell::prelude::*;


use super::BindSceneEffect;


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum EFogMode {
    None,
    Linear,
    Exp,
    Exp2,
    AltitudeBase,
}

#[derive(Component)]
pub struct SceneFog {
    mode: EFogMode,
    color: (f32, f32, f32),
    start: f32,
    end: f32,
    intensity: f32,
    pub dirty: bool,
}
impl SceneFog {
    pub const FOG_PARAM: usize = 4;
    pub const FOG_COLOR: usize = 4;

    pub const FOG_PARAM_OFFSIZE: usize = 0 * 4;
    pub const FOG_COLOR_OFFSIZE: usize = Self::FOG_PARAM_OFFSIZE + Self::FOG_PARAM_OFFSIZE * 4;

    pub fn new(
    ) -> Self {
        Self {
            mode: EFogMode::Linear,
            color: (0.1, 0.5, 0.1),
            start: 10.,
            end: 100.,
            intensity: 1.0,
            dirty: true,
        }
    }

    pub fn mode(&mut self, mode: EFogMode) {
        if self.mode == mode {

        } else {
            self.dirty = true;
            self.mode = mode;
        }
    }
    pub fn color(&mut self, value: (f32, f32, f32)) {
        if self.color.0 != value.0 || self.color.1 != value.1 || self.color.2 != value.2 {
            self.dirty = true;
            self.color = value;
        }
    }
    pub fn start(&mut self, value: f32) {
        if self.start != value {
            self.dirty = true;
            self.start = value;
        }
    }
    pub fn end(&mut self, value: f32) {
        if self.end != value {
            self.dirty = true;
            self.end = value;
        }
    }
    pub fn intensity(&mut self, value: f32) {
        if self.intensity != value {
            self.dirty = true;
            self.intensity = value;
        }
    }
    pub fn data(&self, data: &mut Vec<f32>) {
        let mode: f32 = match self.mode {
            EFogMode::None => 0.,
            EFogMode::Linear => 1.,
            EFogMode::Exp => 2.,
            EFogMode::Exp2 => 3.,
            EFogMode::AltitudeBase => 4.,
        };

        let temp = [
            self.color.0, self.color.1, self.color.2, 1.,
            mode, self.start, self.end, self.intensity
        ];
        
        temp.iter().for_each(|v| {
            data.push(*v);
        });
    }
    pub fn update(&self, bind: &BindSceneEffect) {
        let mode: f32 = match self.mode {
            EFogMode::None => 0.,
            EFogMode::Linear => 1.,
            EFogMode::Exp => 2.,
            EFogMode::Exp2 => 3.,
            EFogMode::AltitudeBase => 4.,
        };
        let values = vec![
            self.color.0, self.color.1, self.color.2, 1.
            ,mode, self.start, self.end, self.intensity
        ];
        bind.0.data().write_data(ShaderBindSceneAboutEffect::OFFSET_FOG_INFO as usize, bytemuck::cast_slice(&values));
    }
}
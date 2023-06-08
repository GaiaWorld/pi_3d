use std::sync::Arc;

use rand::Rng;

use crate::{
    interpolation::{FloatInterpolation, IInterpolation, UpdateRandom},
    particle::Particle,
};

use super::base::IParticleModifier;
#[derive(Clone)]
pub enum RowMode {
    Custom = 0,
    Random = 1,
}

#[derive(Clone)]
pub enum TimeMode {
    Liftime = 0,
    Speed = 1,
}

#[derive(Clone)]
pub enum AnimationMode {
    WholeSheet = 0,
    SingleRow = 1,
}

pub const ATTRIBUTE_PS_UV_SHEET: &'static str = "uv_sheet";

#[derive(Clone)]
pub struct TextureSheet {
    pub rowMode: RowMode,
    pub customRow: f32,
    pub timeMode: TimeMode,
    pub animMode: AnimationMode,
    pub _tilesX: f32,
    pub _tilesY: f32,
    //  _tilesXY: f32 = 1;
    uScale: f32,
    vScale: f32,
    pub cycles: f32,
    pub active: bool,
    modifyCall: Arc<dyn Fn(&mut Particle, f32, f32, &TextureSheet)>,
    pub frameOverTime: FloatInterpolation,
    pub startFrame: FloatInterpolation,
}

impl TextureSheet {
    pub fn set_tilesX(&mut self, v: f32) {
        self._tilesX = v;
        self.uScale = 1. / v;
        // self._tilesXY = self._tilesX * self._tilesY;
    }
    pub fn get_tilesX(&mut self) -> f32 {
        return self._tilesX;
    }
    pub fn set_tilesY(&mut self, v: f32) {
        self._tilesY = v;
        self.vScale = 1. / v;
        // self._tilesXY = self._tilesX * self._tilesY;
    }
    pub fn get_tilesY(&mut self) -> f32 {
        return self._tilesY;
    }

    pub fn new(frameOverTime: FloatInterpolation, startFrame: FloatInterpolation) -> Self {
        Self {
            rowMode: RowMode::Random,
            customRow: 0.,
            timeMode: TimeMode::Liftime,
            animMode: AnimationMode::WholeSheet,
            _tilesX: 1.,
            _tilesY: 1.,
            uScale: 1.,
            vScale: 1.,
            cycles: 1.,
            active: true,
            modifyCall: Arc::new(TextureSheet::modifyForStart),
            frameOverTime,
            startFrame,
        }
    }

    pub fn set_runAsStart(&mut self, value: bool) {
        if (value) {
            self.modifyCall = Arc::new(TextureSheet::modifyForStart);
        } else {
            self.modifyCall = Arc::new(TextureSheet::modifyForOverLifetime);
        }
    }

    pub fn modifyForStart(
        particle: &mut Particle,
        amount: f32,
        deltaSeconds: f32,
        modifier: &TextureSheet,
    ) {
        particle.texture_start_frame = modifier.startFrame.interpolate(0., particle.base_random);

        if let RowMode::Custom = modifier.rowMode {
            particle.texture_row = modifier.customRow;
        } else {
            let mut rng = rand::thread_rng();
            let a: f32 = rng.gen();
            particle.texture_row = (a * modifier._tilesY).round() % modifier._tilesY;
        }
    }

    pub fn modifyForOverLifetime(
        particle: &mut Particle,
        amount: f32,
        deltaSeconds: f32,
        modifier: &TextureSheet,
    ) {
        if (modifier.active) {
            let interpolation = modifier
                .frameOverTime
                .interpolate((amount * modifier.cycles) % 1., particle.base_random);
            let start = particle.texture_start_frame;
            let mut cellId = 0.;
            let mut cellX = 0.;
            let mut cellY = 0.;

            if let AnimationMode::SingleRow = modifier.animMode {
                cellY = particle.texture_row;
                cellX = (((start + interpolation) * modifier._tilesX) % modifier._tilesX).floor();
            } else {
                cellId = (start + interpolation) * modifier._tilesX * modifier._tilesY;
                cellX = (cellId % modifier._tilesX).floor();
                cellY = (cellId / modifier._tilesX).floor();
            }

            particle.uv[2] = cellX / modifier._tilesX;
            particle.uv[3] = (modifier._tilesY - cellY - 1.) / modifier._tilesY;
        } else {
            particle.uv[2] = 0.;
            particle.uv[3] = (modifier._tilesY - 0. - 1.) / modifier._tilesY;
        }

        particle.uv[0] = 1. / modifier._tilesX;
        particle.uv[1] = 1. / modifier._tilesY;
    }
}

impl IParticleModifier for TextureSheet {
    fn modify(&mut self, particle: &mut Particle, amount: &mut f32, deltaSeconds: f32) {
        (self.modifyCall)(particle, *amount, deltaSeconds, self);
    }
}

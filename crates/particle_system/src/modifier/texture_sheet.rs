use std::sync::Arc;

use rand::Rng;

use crate::{
    interpolation::{FloatInterpolation, IInterpolation},
    particle::Particle, tools::{TextureUV, BaseRandom},
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
    pub row_mode: RowMode,
    pub custom_row: f32,
    pub time_mode: TimeMode,
    pub anim_mode: AnimationMode,
    pub _tiles_x: f32,
    pub _tiles_y: f32,
    //  _tilesXY: f32 = 1;
    u_scale: f32,
    v_scale: f32,
    pub cycles: f32,
    pub active: bool,
    pub frame_over_time: FloatInterpolation,
    pub start_frame: FloatInterpolation,
}

impl TextureSheet {
    pub fn set_tiles_x(&mut self, v: f32) {
        self._tiles_x = v;
        self.u_scale = 1. / v;
        // self._tilesXY = self._tilesX * self._tilesY;
    }
    pub fn get_tiles_x(&mut self) -> f32 {
        return self._tiles_x;
    }
    pub fn set_tiles_y(&mut self, v: f32) {
        self._tiles_y = v;
        self.v_scale = 1. / v;
        // self._tilesXY = self._tilesX * self._tilesY;
    }
    pub fn get_tiles_y(&mut self) -> f32 {
        return self._tiles_y;
    }

    pub fn new(frame_over_time: FloatInterpolation, start_frame: FloatInterpolation) -> Self {
        Self {
            row_mode: RowMode::Random,
            custom_row: 0.,
            time_mode: TimeMode::Liftime,
            anim_mode: AnimationMode::WholeSheet,
            _tiles_x: 1.,
            _tiles_y: 1.,
            u_scale: 1.,
            v_scale: 1.,
            cycles: 1.,
            active: true,
            frame_over_time,
            start_frame,
        }
    }

    pub fn modify_for_start(
        &self,
        particle: &mut TextureUV,
        randoms: &BaseRandom
    ) {
        particle.start_frame = self.start_frame.interpolate(0., randoms.base);

        if let RowMode::Custom = self.row_mode {
            particle.row = self.custom_row;
        } else {
            particle.row = (randoms.w * self._tiles_y).round() % self._tiles_y;
        }
    }

    pub fn modify_for_over_lifetime(
        &self,
        particle: &mut TextureUV,
        amount: f32,
        randoms: &BaseRandom
    ) {
        if self.active {
            let interpolation = self.frame_over_time.interpolate((amount * self.cycles) % 1., randoms.base);
            let start = particle.start_frame;
            let mut _cell_id = 0.;
            let mut _cell_x = 0.;
            let mut _cell_y = 0.;

            if let AnimationMode::SingleRow = self.anim_mode {
                _cell_y = particle.row;
                _cell_x = (((start + interpolation) * self._tiles_x) % self._tiles_x).floor();
            } else {
                _cell_id = (start + interpolation) * self._tiles_x * self._tiles_y;
                _cell_x = (_cell_id % self._tiles_x).floor();
                _cell_y = (_cell_id / self._tiles_x).floor();
            }

            particle.uoffset = _cell_x / self._tiles_x;
            particle.voffset = (self._tiles_y - _cell_y - 1.) / self._tiles_y;
        } else {
            particle.uoffset = 0.;
            particle.voffset = (self._tiles_y - 0. - 1.) / self._tiles_y;
        }

        particle.uscale = 1. / self._tiles_x;
        particle.vscale = 1. / self._tiles_y;
    }
}
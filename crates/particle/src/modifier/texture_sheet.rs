use std::sync::Arc;

use rand::Rng;

use crate::{
    interpolation::{FloatInterpolation, IInterpolation},
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
    modify_call: Arc<dyn Fn(&mut Particle, f32, f32, &TextureSheet)>,
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
            modify_call: Arc::new(TextureSheet::modify_for_start),
            frame_over_time,
            start_frame,
        }
    }

    pub fn set_run_as_start(&mut self, value: bool) {
        if value {
            self.modify_call = Arc::new(TextureSheet::modify_for_start);
        } else {
            self.modify_call = Arc::new(TextureSheet::modify_for_over_lifetime);
        }
    }

    pub fn modify_for_start(
        particle: &mut Particle,
        _amount: f32,
        _delta_seconds: f32,
        modifier: &TextureSheet,
    ) {
        particle.texture_start_frame = modifier.start_frame.interpolate(0., particle.base_random);

        if let RowMode::Custom = modifier.row_mode {
            particle.texture_row = modifier.custom_row;
        } else {
            let mut rng = rand::thread_rng();
            let a: f32 = rng.gen();
            particle.texture_row = (a * modifier._tiles_y).round() % modifier._tiles_y;
        }
    }

    pub fn modify_for_over_lifetime(
        particle: &mut Particle,
        amount: f32,
        _delta_seconds: f32,
        modifier: &TextureSheet,
    ) {
        if modifier.active {
            let interpolation = modifier
                .frame_over_time
                .interpolate((amount * modifier.cycles) % 1., particle.base_random);
            let start = particle.texture_start_frame;
            let mut _cell_id = 0.;
            let mut _cell_x = 0.;
            let mut _cell_y = 0.;

            if let AnimationMode::SingleRow = modifier.anim_mode {
                _cell_y = particle.texture_row;
                _cell_x = (((start + interpolation) * modifier._tiles_x) % modifier._tiles_x).floor();
            } else {
                _cell_id = (start + interpolation) * modifier._tiles_x * modifier._tiles_y;
                _cell_x = (_cell_id % modifier._tiles_x).floor();
                _cell_y = (_cell_id / modifier._tiles_x).floor();
            }

            particle.uv[2] = _cell_x / modifier._tiles_x;
            particle.uv[3] = (modifier._tiles_y - _cell_y - 1.) / modifier._tiles_y;
        } else {
            particle.uv[2] = 0.;
            particle.uv[3] = (modifier._tiles_y - 0. - 1.) / modifier._tiles_y;
        }

        particle.uv[0] = 1. / modifier._tiles_x;
        particle.uv[1] = 1. / modifier._tiles_y;
    }
}

impl IParticleModifier for TextureSheet {
    fn modify(&mut self, particle: &mut Particle, amount: &mut f32, delta_seconds: f32) {
        (self.modify_call)(particle, *amount, delta_seconds, self);
    }
}

// use pi_scene_math::Vector3;
use pi_scene_shell::prelude::*;

// use crate::{
//     tools::*,
//     particle::Particle,
// };

use super::color_over_lifetime::ColorOverLifetime;

#[derive(Clone, Copy)]
pub enum ETrailMode {
    Particles = 0,
}

#[derive(Clone, Copy)]
pub enum ETrailTextureMode {
    Stretch = 0,
    Tiled = 1,
    DistributePerSegment = 2,
    RepeatPerSegment = 3,
}
impl From<u8> for ETrailTextureMode {
    fn from(value: u8) -> Self {
        match value {
            1 => { ETrailTextureMode::Tiled }
            2 => { ETrailTextureMode::DistributePerSegment }
            3 => { ETrailTextureMode::RepeatPerSegment }
            _ => {
                ETrailTextureMode::Stretch
            }
        }
    }
}

pub struct TrailModifier {
    pub mode: ETrailMode,

    /**
     * 可生成路径轨迹的粒子的百分比
     */
    pub ratio: f32,

    /**
     * 使用粒子多少生命百分比内的路径点
     */
    pub lifetime: FloatInterpolation,

    /**
     * ribbon 模式 间隔多少个粒子进行连接
     */
    pub ribbon_count: f32,

    /**
     * ribbon 末尾是否连接到粒子系统的位置
     */
    pub attach_ribbons_to_transfoem: bool,

    /**
     * 新路径点与作为轨迹用的最后一个点之间距离超过多少，才将新作为轨迹用的点
     */
    pub minimun_vertex_distance: f32,

    pub use_world_space: bool,

    /**
     * 粒子的Size是否实时影响拖尾宽度
     */
    pub size_affects_width: bool,
    /**
     * 粒子的Size是否影响拖尾的 Lifetime 参数
     */
    pub size_affects_lifetime: bool,
    /**
     * 粒子的实时颜色是否影响拖尾颜色的整体变化
     */
    pub inherit_particle_color: bool,
    /**
     * 拖尾的颜色随生命增长的整体变化
     */
    pub color_over_lifetime: ColorOverLifetime,
    /**
     * 拖尾的初始宽度
     */
    pub width_over_trail: FloatInterpolation,
    /**
     * 拖尾颜色在路径线上的分布情况
     */
    pub color_over_trail: ColorOverLifetime,

    // positions: Vec<f32>,
    // indices: Vec<f32>,
    // colors: Vec<f32>,
    // uvs: Vec<f32>,

    // _geometry: Vec<f32>,
    // pub mesh: Vec<f32>,

    // max_key_point: f32,
    // modify_call: Box<dyn Fn(Vector3, &mut Vec<Particle>, &mut TrailModifier)>,

    /**
     * 拖尾是否在粒子死亡时即死亡
     */
    pub die_with_particle: bool,

    pub texture_mode: ETrailTextureMode,
    // pub _enabled: bool,
}

impl TrailModifier {

    pub fn new() -> Self {
        Self {
            mode: ETrailMode::Particles,
            ratio: 1.,
            lifetime: FloatInterpolation::new(0.),
            ribbon_count: 1.,
            attach_ribbons_to_transfoem: false,
            minimun_vertex_distance: 1.,
            use_world_space: false,
            size_affects_width: false,
            size_affects_lifetime: false,
            inherit_particle_color: false,
            color_over_lifetime: ColorOverLifetime::default(),
            width_over_trail: FloatInterpolation::new(0.),
            color_over_trail: ColorOverLifetime::default(),
            // positions: vec![],
            // indices: vec![],
            // colors: vec![],
            // uvs: vec![],
            // _geometry: vec![],
            // mesh: vec![],
            // max_key_point: 65000. / 2.,
            // modify_call: Box::new(TrailModifier::for_particles_local_space),
            die_with_particle: true,
            texture_mode: ETrailTextureMode::Stretch,
            // _enabled: false,
        }
    }
}

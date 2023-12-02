

mod light;
mod shadow;
mod brdf;
mod shader;
mod lighting;
mod screen;
mod environment_lighting;

pub use light::*;
pub use shadow::*;
pub use brdf::*;
pub use lighting::*;
pub use screen::*;
pub use environment_lighting::*;


/// 一个场景内最大有效灯光数目
/// * 一个灯光的数据:
///   * LightData - [f32;4]
///   * Color - [f32;4]
///     * ~~DiffuseColor - [f32;4]~~
///     * ~~SpeculerColor - [f32;4]~~
///   * OtherA - [f32;4]
///     * PointLight 和 SpotLight
///       * Falloff
///     * HemiLight
///       * LightGround
///   * OtherB - [f32;4]
///     * SpotLight
///       * LightDirection
///     * HemiLight
///       * LightGround
pub const MAX_SCENE_LIGHTING_LIGHT_COUNT: usize = 1024; // 65536 byte = 1024 * (4 + 4 + 4 + 4) * 4, min_bind_buffer_size: 65536
/// 一个场景内最大有效阴影生成器数目
/// * 一个阴影灯光的数据:
///   * ProjectionMatrix - [f32;16]
///   * ProjectionMatrix - [f32;16]
pub const MAX_SCENE_SHADOW_CASTER_COUNT: usize = 4;

/// 一个模型最大有效灯光数目
pub const MAX_MODEL_LIGHTING_COUNT: usize = 15;

/// 一个模型最大有效阴影数目
pub const MAX_MODEL_SHADOWMAP_COUNT: usize = 3;

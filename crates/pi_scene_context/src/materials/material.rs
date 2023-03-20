

use pi_hash::XHashMap;
use pi_scene_math::{Number, Matrix, Vector4, Vector2, Matrix2};

use crate::{object::{ObjectID}, renderers::render_mode::ERenderMode};

pub trait TMaterial {
    fn render_mode(&self) -> ERenderMode;
}

///
/// 材质单独与 GameObject 关联
/// Mesh 使用

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MaterialID (pub ObjectID);

/// 材质被哪些实体使用
#[derive(Debug, Default)]
pub struct MaterialUsedList(pub XHashMap<ObjectID, ObjectID>);

pub struct DirtyMaterialUsedList;

#[derive(Debug, Clone, Copy)]
pub enum UniformModifier {
    Mat4(usize, Matrix),
    Mat2(usize, Matrix2),
    Vec4(usize, Vector4),
    Vec2(usize, Vector2),
    Float(usize, Number),
    Int32(usize, i32),
    Uint32(usize, u32),
}

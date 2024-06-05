
use pi_scene_shell::prelude::*;
use pi_scene_math::{Number, Matrix, Vector4, Vector2, Matrix2};

use crate::renderers::prelude::*;


pub trait TMaterial {
    fn render_mode(&self) -> ERenderMode;
}

///
/// 材质单独与 GameObject 关联
/// Mesh 使用

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Component, Default)]
pub struct LinkedMaterialID (pub Entity);
impl TEntityRef for LinkedMaterialID {
    fn id(&self) -> Entity {
        self.0
    }
}

#[derive(Clone, Component, Default)]
pub struct DirtyMaterialRefs;

/// 材质被哪些实体使用
pub type MaterialRefs = EntityRefInfo<DirtyMaterialRefs>;

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

#[derive(Debug, Clone, Copy, Resource)]
pub struct SingleIDBaseDefaultMaterial(pub Entity);
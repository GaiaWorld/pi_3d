
use pi_engine_shell::prelude::*;
use pi_scene_math::{Vector4, Matrix, Matrix2, Vector2, Number};
use pi_share::ThreadSync;

pub trait Value {
    fn name(&self) -> &str;
}

pub trait FromValueUniformStatistics: ThreadSync + 'static {
    fn new(value: &ShaderBindEffectValue) -> Self;
}

pub trait ValueSlotIndex {
    const SLOT_INDEX: usize;
}

pub trait ValueMat4 {
    fn value(&self) -> Matrix;
}
pub trait ValueMat2 {
    fn value(&self) -> Matrix2;
}
pub trait ValueVec4 {
    fn value(&self) -> Vector4;
}
pub trait ValueVec2 {
    fn value(&self) -> Vector2;
}
pub trait ValueFloat {
    fn value(&self) -> Number;
}
pub trait ValueInt {
    fn value(&self) -> i32;
}
pub trait ValueUint {
    fn value(&self) -> u32;
}
pub trait ValueBoolean {
    fn value(&self) -> bool;
}
pub trait ValueByte {
    fn value(&self) -> u8;
}

pub trait UniformBind {
    fn bind(&self) -> u32;
}

pub trait SlotActiveRequire {
    const ASK_SLOT_COUNT: u8;
}

#[derive(Component)]
pub struct AnimatorableUniformVec4A { pub value: Entity }
#[derive(Component)]
pub struct AnimatorableUniformVec4B { pub value: Entity }
#[derive(Component)]
pub struct AnimatorableUniformVec4C { pub value: Entity }
#[derive(Component)]
pub struct AnimatorableUniformVec4D { pub value: Entity }
#[derive(Component)]
pub struct AnimatorableUniformVec4E { pub value: Entity }
#[derive(Component)]
pub struct AnimatorableUniformVec4F { pub value: Entity }
#[derive(Component)]
pub struct AnimatorableUniformVec4G { pub value: Entity }
#[derive(Component)]
pub struct AnimatorableUniformVec4H { pub value: Entity }

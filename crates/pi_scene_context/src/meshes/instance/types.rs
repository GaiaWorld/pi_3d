use pi_scene_math::{Matrix, Matrix2, Vector4, Vector2, Number};
use render_geometry::vertex_data::EVertexDataKind;

pub trait TInstancedData {
    fn vertex_kind(&self) -> &EVertexDataKind;
    fn value(&self) -> &InstancedValue;
    fn local_offset() -> u32;
    // fn write_to_buffer(&self, offset: u32, buffer: &mut Instances)
}

pub enum InstancedValue {
    Mat4(Matrix),
    Mat2(Matrix2),
    Vec4(Vector4),
    Vec2(Vector2),
    Float(Number),
    Int(i32),
    Uint(u32),
}

// impl 
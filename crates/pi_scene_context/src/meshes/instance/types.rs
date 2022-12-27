
use pi_scene_math::{Matrix, Matrix2, Vector4, Vector2, Number};
use render_data_container::VertexBuffer;
use render_geometry::vertex_data::EVertexDataKind;

use crate::transforms::transform_node::WorldMatrix;

pub trait TInstancedData {
    fn vertex_kind(&self) -> EVertexDataKind;
    fn value(&self) -> &InstancedValue;
    fn size(&self) -> usize;
    fn local_offset(&self) -> usize;
    fn write_instance_buffer(&self, buffer: &mut VertexBuffer, offset: usize);
}

impl TInstancedData for WorldMatrix {
    fn vertex_kind(&self) -> EVertexDataKind {
        EVertexDataKind::InsWorldRow1
    }

    fn value(&self) -> &InstancedValue {
        todo!()
    }

    fn size(&self) -> usize {
        16
    }

    fn local_offset(&self) -> usize {
        0
    }

    fn write_instance_buffer(&self, buffer: &mut VertexBuffer, offset: usize) {
        buffer.update_f32(self.0.as_slice(), offset);
    }
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

impl InstancedValue {
    pub fn size(&self) -> usize {
        match self {
            InstancedValue::Mat4(value) => {
                16
            },
            InstancedValue::Mat2(value) =>  {
                4
            },
            InstancedValue::Vec4(value) =>  {
                4
            },
            InstancedValue::Vec2(value) =>  {
                2
            },
            InstancedValue::Float(value) =>  {
                1
            },
            InstancedValue::Int(value) =>  {
                1
            },
            InstancedValue::Uint(value) =>  {
                1
            },
        }
    }
    pub fn write(&self, buffer: &mut VertexBuffer, offset: usize) {
        match self {
            InstancedValue::Mat4(value) => {
                buffer.update_f32(value.as_slice(), offset);
            },
            InstancedValue::Mat2(value) =>  {
                buffer.update_f32(value.as_slice(), offset);
            },
            InstancedValue::Vec4(value) =>  {
                buffer.update_f32(value.as_slice(), offset);
            },
            InstancedValue::Vec2(value) =>  {
                buffer.update_f32(value.as_slice(), offset);
            },
            InstancedValue::Float(value) =>  {
                buffer.update_f32(&[*value], offset);
            },
            InstancedValue::Int(value) =>  {
                buffer.update_i32(&[*value], offset);
            },
            InstancedValue::Uint(value) =>  {
                buffer.update_u32(&[*value], offset);
            },
        }
    }
}

use pi_assets::{asset::Handle, mgr::AssetMgr};
use pi_render::{renderer::{attributes::EVertexDataKind, vertex_buffer::{EVertexBufferRange, VertexBufferAllocator, KeyVertexBuffer}}, rhi::{device::RenderDevice, RenderQueue}};
use pi_scene_math::{Matrix, Matrix2, Vector4, Vector2, Number};
use pi_share::Share;

pub trait TInstanceFlag {
    fn dirty(&self) -> bool;
    fn reset(&mut self);
}

pub trait TInstanceData {
    fn vertex_kind(&self) -> EVertexDataKind;
    fn collect(list: &Vec<&Self>, key: KeyVertexBuffer, device: &RenderDevice, queue: &RenderQueue, allocator: &mut VertexBufferAllocator, asset_mgr: &Share<AssetMgr<EVertexBufferRange>>) -> Option<Handle<EVertexBufferRange>> ;
    // fn size() -> usize;
    // fn bytes_size() -> usize;
    // fn local_offset(&self) -> usize;
}

pub fn instance_datas<T: TInstanceData>(
    list: &[T],
) {
    
}

pub enum InstanceValue {
    Mat4(Matrix),
    Mat2(Matrix2),
    Vec4(Vector4),
    Vec2(Vector2),
    Float(Number),
    Int(i32),
    Uint(u32),
}

impl InstanceValue {
    pub fn size(&self) -> usize {
        match self {
            InstanceValue::Mat4(value) => {
                16
            },
            InstanceValue::Mat2(value) =>  {
                4
            },
            InstanceValue::Vec4(value) =>  {
                4
            },
            InstanceValue::Vec2(value) =>  {
                2
            },
            InstanceValue::Float(value) =>  {
                1
            },
            InstanceValue::Int(value) =>  {
                1
            },
            InstanceValue::Uint(value) =>  {
                1
            },
        }
    }
    pub fn write(&self, buffer: &EVertexBufferRange, offset: usize) {
        match buffer {
            EVertexBufferRange::Updatable(buffer, _) => {
                match self {
                    InstanceValue::Mat4(value) => {
                        buffer.write_data(offset, bytemuck::cast_slice(value.as_slice()))
                    },
                    InstanceValue::Mat2(value) =>  {
                        buffer.write_data(offset, bytemuck::cast_slice(value.as_slice()))
                    },
                    InstanceValue::Vec4(value) =>  {
                        buffer.write_data(offset, bytemuck::cast_slice(value.as_slice()))
                    },
                    InstanceValue::Vec2(value) =>  {
                        buffer.write_data(offset, bytemuck::cast_slice(value.as_slice()))
                    },
                    InstanceValue::Float(value) =>  {
                        buffer.write_data(offset, bytemuck::cast_slice(&[*value]))
                    },
                    InstanceValue::Int(value) =>  {
                        buffer.write_data(offset, bytemuck::cast_slice(&[*value]))
                    },
                    InstanceValue::Uint(value) =>  {
                        buffer.write_data(offset, bytemuck::cast_slice(&[*value]))
                    },
                }
            },
            EVertexBufferRange::NotUpdatable(_) => todo!(),
        }
    }
}
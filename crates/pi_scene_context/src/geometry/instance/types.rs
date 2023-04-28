
use pi_assets::{asset::Handle, mgr::AssetMgr};
use pi_render::{renderer::{attributes::EVertexDataKind, vertex_buffer::{EVertexBufferRange, VertexBufferAllocator, KeyVertexBuffer}}, rhi::{device::RenderDevice, RenderQueue}};
use pi_scene_math::{Matrix, Matrix2, Vector4, Vector2, Number};
use pi_share::Share;

use crate::transforms::transform_node::WorldMatrix;

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

impl TInstanceData for WorldMatrix {
    fn vertex_kind(&self) -> EVertexDataKind {
        EVertexDataKind::InsWorldRow1
    }

    // fn size() -> usize {
    //     16
    // }

    // fn bytes_size() -> usize {
    //     16 * 4
    // }

    // fn local_offset(&self) -> usize {
    //     0
    // }

    fn collect(list: &Vec<&Self>, key: KeyVertexBuffer, device: &RenderDevice, queue: &RenderQueue, allocator: &mut VertexBufferAllocator, asset_mgr: &Share<AssetMgr<EVertexBufferRange>>) -> Option<Handle<EVertexBufferRange>> {
        let mut result = vec![];

        list.iter().for_each(|v| {
            v.0.as_slice().iter().for_each(|v| {
                result.push(*v);
            })
        });

        if let Some(buffer) = allocator.create_not_updatable_buffer(device, queue, bytemuck::cast_slice(&result)) {
            asset_mgr.insert(key, buffer)
        } else {
            None
        }
    }
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
use std::{ops::Range, time::Instant, marker::PhantomData};

use pi_ecs::{prelude::{Query, ResMut, Res, Component, Commands}, query::{Changed}};
use pi_ecs_macros::setup;
use pi_engine_shell::{object::{ObjectID, GameObject}, run_stage::TSystemStageInfo};
use pi_render::rhi::{device::RenderDevice, RenderQueue};
use render_data_container::{VertexBufferPool};

use crate::{geometry::{vertex_buffer_useinfo, geometry::RenderGeometryEable, instance::{instanced_buffer::TInstancedBuffer, types::TInstancedData}, sys_vertex_buffer_use::{SysGeometryStatesInit}}, meshes::command::SysMeshCreateCommand};

use super::{InstanceList, types::TInstanceFlag};

///
/// T: Mesh 中 保存实例数据的buffer
/// D: 实例数据
/// F: 实例数据在Mesh上的脏标识
/// S: 脏标识更新的System
pub struct SysInstanceBufferUpdateFunc<T: TInstancedBuffer + Component, D: TInstancedData + Component, F: TInstanceFlag + Component, S: TSystemStageInfo>(PhantomData<(T, D, F, S)>);
impl<T: TInstancedBuffer + Component, D: TInstancedData + Component, F: TInstanceFlag + Component, S: TSystemStageInfo> TSystemStageInfo for SysInstanceBufferUpdateFunc<T, D, F, S> {
    // fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
    //     vec![
    //         S::key(), SysInstancedBufferInitFunc::<T>::key()
    //     ]
    // }
}
#[setup]
impl<T: TInstancedBuffer + Component, D: TInstancedData + Component, F: TInstanceFlag + Component, S: TSystemStageInfo + 'static> SysInstanceBufferUpdateFunc<T, D, F, S> {
    #[system]
    pub fn tick(
        instances: Query<GameObject, &D>,
        mut sources: Query<
            GameObject,
            (
                &InstanceList, &mut T, &mut F, &mut RenderGeometryEable,
                ( 
                    Option<&mut vertex_buffer_useinfo::AssetDescVBSlot01>, Option<&mut vertex_buffer_useinfo::AssetDescVBSlot02>, Option<&mut vertex_buffer_useinfo::AssetDescVBSlot03>, Option<&mut vertex_buffer_useinfo::AssetDescVBSlot04>,
                    Option<&mut vertex_buffer_useinfo::AssetDescVBSlot05>, Option<&mut vertex_buffer_useinfo::AssetDescVBSlot06>, Option<&mut vertex_buffer_useinfo::AssetDescVBSlot07>, Option<&mut vertex_buffer_useinfo::AssetDescVBSlot08>, 
                    Option<&mut vertex_buffer_useinfo::AssetDescVBSlot09>, Option<&mut vertex_buffer_useinfo::AssetDescVBSlot10>, Option<&mut vertex_buffer_useinfo::AssetDescVBSlot11>, Option<&mut vertex_buffer_useinfo::AssetDescVBSlot12>,  
                )
            ),
        >,
        mut vbpool: ResMut<VertexBufferPool>,
        device: Res<RenderDevice>,
        queue: Res<RenderQueue>,
    ) {
        let time = Instant::now();
        sources.iter_mut().for_each(|(
            inslist, buffer, mut flag, mut geodisable,
            (
                desc01, desc02, desc03, desc04, 
                desc05, desc06, desc07, desc08, 
                desc09, desc10, desc11, desc12, 
            )
        )| {
            // log::trace!("SysInstanceBufferUpdateFunc:");
            if flag.dirty() == false {
                return;
            }
            // log::debug!("SysInstanceBufferUpdateFunc: A, {:?}", inslist.list.len());
            let mut list = vec![];
            inslist.list.iter().for_each(|insid| {
                if let Some(instance) = instances.get(insid.clone()) {
                    list.push(instance);
                }
            });
            let bytes = list.len() * D::bytes_size();

            if list.len() == 0 {
                geodisable.0 = false;
            } else {
                geodisable.0 = true;
                flag.reset();
                buffer.update::<D>(list.as_slice(), &mut vbpool, &device, &queue);
                // log::debug!("SysInstanceBufferUpdateFunc: B, {:?}", buffer.slot());
                match buffer.slot() {
                    vertex_buffer_useinfo::EVertexBufferSlot::Slot01 => {
                        if let Some(mut desc) = desc01 {
                            desc.desc.update_range(Some(Range { start: 0, end: bytes as wgpu::BufferAddress }));
                        }
                    },
                    vertex_buffer_useinfo::EVertexBufferSlot::Slot02 => {
                        if let Some(mut desc) = desc02 {
                            desc.desc.update_range(Some(Range { start: 0, end: bytes as wgpu::BufferAddress }));
                        }
                    },
                    vertex_buffer_useinfo::EVertexBufferSlot::Slot03 => {
                        if let Some(mut desc) = desc03 {
                            desc.desc.update_range(Some(Range { start: 0, end: bytes as wgpu::BufferAddress }));
                        }
                    },
                    vertex_buffer_useinfo::EVertexBufferSlot::Slot04 => {
                        if let Some(mut desc) = desc04 {
                            desc.desc.update_range(Some(Range { start: 0, end: bytes as wgpu::BufferAddress }));
                        }
                    },
                    vertex_buffer_useinfo::EVertexBufferSlot::Slot05 => {
                        if let Some(mut desc) = desc05 {
                            desc.desc.update_range(Some(Range { start: 0, end: bytes as wgpu::BufferAddress }));
                        }
                    },
                    vertex_buffer_useinfo::EVertexBufferSlot::Slot06 => {
                        if let Some(mut desc) = desc06 {
                            desc.desc.update_range(Some(Range { start: 0, end: bytes as wgpu::BufferAddress }));
                        }
                    },
                    vertex_buffer_useinfo::EVertexBufferSlot::Slot07 => {
                        if let Some(mut desc) = desc07 {
                            desc.desc.update_range(Some(Range { start: 0, end: bytes as wgpu::BufferAddress }));
                        }
                    },
                    vertex_buffer_useinfo::EVertexBufferSlot::Slot08 => {
                        if let Some(mut desc) = desc08 {
                            desc.desc.update_range(Some(Range { start: 0, end: bytes as wgpu::BufferAddress }));
                        }
                    },
                    vertex_buffer_useinfo::EVertexBufferSlot::Slot09 => {
                        if let Some(mut desc) = desc09 {
                            desc.desc.update_range(Some(Range { start: 0, end: bytes as wgpu::BufferAddress }));
                        }
                    },
                    vertex_buffer_useinfo::EVertexBufferSlot::Slot10 => {
                        if let Some(mut desc) = desc10 {
                            desc.desc.update_range(Some(Range { start: 0, end: bytes as wgpu::BufferAddress }));
                        }
                    },
                    vertex_buffer_useinfo::EVertexBufferSlot::Slot11 => {
                        if let Some(mut desc) = desc11 {
                            desc.desc.update_range(Some(Range { start: 0, end: bytes as wgpu::BufferAddress }));
                        }
                    },
                    vertex_buffer_useinfo::EVertexBufferSlot::Slot12 => {
                        if let Some(mut desc) = desc12 {
                            desc.desc.update_range(Some(Range { start: 0, end: bytes as wgpu::BufferAddress }));
                        }
                    },
                    vertex_buffer_useinfo::EVertexBufferSlot::Slot13 => {
                        // if let Some(mut desc) = desc13 {
                        //     desc.desc.update_range(Some(Range { start: 0, end: bytes as wgpu::BufferAddress }));
                        // }
                    },
                    vertex_buffer_useinfo::EVertexBufferSlot::Slot14 => {
                        // if let Some(desc) = desc14.get_mut() {
                        //     desc.desc.update_range(Some(Range { start: 0, end: bytes as wgpu::BufferAddress }));
                        // }
                    },
                    vertex_buffer_useinfo::EVertexBufferSlot::Slot15 => {
                        // if let Some(desc) = desc15.get_mut() {
                        //     desc.desc.update_range(Some(Range { start: 0, end: bytes as wgpu::BufferAddress }));
                        // }
                    },
                    vertex_buffer_useinfo::EVertexBufferSlot::Slot16 => {
                        // if let Some(desc) = desc16.get_mut() {
                        //     desc.desc.update_range(Some(Range { start: 0, end: bytes as wgpu::BufferAddress }));
                        // }
                    },
                    vertex_buffer_useinfo::EVertexBufferSlot::Slot17 => {
                        // if let Some(desc) = desc17.get_mut() {
                        //     desc.desc.update_range(Some(Range { start: 0, end: bytes as wgpu::BufferAddress }));
                        // }
                    },
                    vertex_buffer_useinfo::EVertexBufferSlot::Slot18 => {
                        // if let Some(desc) = desc18.get_mut() {
                        //     desc.desc.update_range(Some(Range { start: 0, end: bytes as wgpu::BufferAddress }));
                        // }
                    },
                }
            }
        });
        
        let time1 = Instant::now();
        log::info!("SysInstancedBufferUpdate<{}>: {:?}", T::display_name(), time1 - time);
    }
}


///
/// 实例数据 Buffer 组件初始化
/// T: 实例数据buffer
pub struct SysInstancedBufferInitFunc<T: TInstancedBuffer + Component>(PhantomData<T>);
impl<T: TInstancedBuffer + Component> TSystemStageInfo for SysInstancedBufferInitFunc<T> {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysMeshCreateCommand::key()
        ]
    }
}
#[setup]
impl<T: TInstancedBuffer + Component> SysInstancedBufferInitFunc<T> {
    #[system]
    pub fn tick(
        mut sources: Query<
            GameObject,
            (
                ObjectID,
                &T,
            ),
            Changed<T>
        >,
        mut res01: Commands<GameObject, vertex_buffer_useinfo::AssetResVBSlot01>,
        mut res02: Commands<GameObject, vertex_buffer_useinfo::AssetResVBSlot02>,
        mut res03: Commands<GameObject, vertex_buffer_useinfo::AssetResVBSlot03>,
        mut res04: Commands<GameObject, vertex_buffer_useinfo::AssetResVBSlot04>,
        mut res05: Commands<GameObject, vertex_buffer_useinfo::AssetResVBSlot05>,
        mut res06: Commands<GameObject, vertex_buffer_useinfo::AssetResVBSlot06>,
        mut res07: Commands<GameObject, vertex_buffer_useinfo::AssetResVBSlot07>,
        mut res08: Commands<GameObject, vertex_buffer_useinfo::AssetResVBSlot08>,
        mut res09: Commands<GameObject, vertex_buffer_useinfo::AssetResVBSlot09>,
        mut res10: Commands<GameObject, vertex_buffer_useinfo::AssetResVBSlot10>,
        mut res11: Commands<GameObject, vertex_buffer_useinfo::AssetResVBSlot11>,
        mut res12: Commands<GameObject, vertex_buffer_useinfo::AssetResVBSlot12>,
        mut res13: Commands<GameObject, vertex_buffer_useinfo::AssetResVBSlot13>,
    ) {
        sources.iter_mut().for_each(|(
            id_source, buffer,
        )| {
            log::info!(">> Sys{}Init", T::display_name());
    
            match buffer.slot() {
                vertex_buffer_useinfo::EVertexBufferSlot::Slot01 => {
                    res01.insert(id_source, vertex_buffer_useinfo::AssetResVBSlot01::from(buffer.key()));
                },
                vertex_buffer_useinfo::EVertexBufferSlot::Slot02 => {
                    res02.insert(id_source, vertex_buffer_useinfo::AssetResVBSlot02::from(buffer.key()));
                },
                vertex_buffer_useinfo::EVertexBufferSlot::Slot03 => {
                    res03.insert(id_source, vertex_buffer_useinfo::AssetResVBSlot03::from(buffer.key()));
                },
                vertex_buffer_useinfo::EVertexBufferSlot::Slot04 => {
                    res04.insert(id_source, vertex_buffer_useinfo::AssetResVBSlot04::from(buffer.key()));
                },
                vertex_buffer_useinfo::EVertexBufferSlot::Slot05 => {
                    res05.insert(id_source, vertex_buffer_useinfo::AssetResVBSlot05::from(buffer.key()));
                },
                vertex_buffer_useinfo::EVertexBufferSlot::Slot06 => {
                    res06.insert(id_source, vertex_buffer_useinfo::AssetResVBSlot06::from(buffer.key()));
                },
                vertex_buffer_useinfo::EVertexBufferSlot::Slot07 => {
                    res07.insert(id_source, vertex_buffer_useinfo::AssetResVBSlot07::from(buffer.key()));
                },
                vertex_buffer_useinfo::EVertexBufferSlot::Slot08 => {
                    res08.insert(id_source, vertex_buffer_useinfo::AssetResVBSlot08::from(buffer.key()));
                },
                vertex_buffer_useinfo::EVertexBufferSlot::Slot09 => {
                    res09.insert(id_source, vertex_buffer_useinfo::AssetResVBSlot09::from(buffer.key()));
                },
                vertex_buffer_useinfo::EVertexBufferSlot::Slot10 => {
                    res10.insert(id_source, vertex_buffer_useinfo::AssetResVBSlot10::from(buffer.key()));
                },
                vertex_buffer_useinfo::EVertexBufferSlot::Slot11 => {
                    res11.insert(id_source, vertex_buffer_useinfo::AssetResVBSlot11::from(buffer.key()));
                },
                vertex_buffer_useinfo::EVertexBufferSlot::Slot12 => {
                    res12.insert(id_source, vertex_buffer_useinfo::AssetResVBSlot12::from(buffer.key()));
                },
                vertex_buffer_useinfo::EVertexBufferSlot::Slot13 => {
                    res13.insert(id_source, vertex_buffer_useinfo::AssetResVBSlot13::from(buffer.key()));
                },
                vertex_buffer_useinfo::EVertexBufferSlot::Slot14 => {
                    // if let Some(desc) = desc14.get_mut() {
                    //     desc.desc.update_range(Some(Range { start: 0, end: bytes as wgpu::BufferAddress }));
                    // }
                },
                vertex_buffer_useinfo::EVertexBufferSlot::Slot15 => {
                    // if let Some(desc) = desc15.get_mut() {
                    //     desc.desc.update_range(Some(Range { start: 0, end: bytes as wgpu::BufferAddress }));
                    // }
                },
                vertex_buffer_useinfo::EVertexBufferSlot::Slot16 => {
                    // if let Some(desc) = desc16.get_mut() {
                    //     desc.desc.update_range(Some(Range { start: 0, end: bytes as wgpu::BufferAddress }));
                    // }
                },
                vertex_buffer_useinfo::EVertexBufferSlot::Slot17 => {
                    // if let Some(desc) = desc17.get_mut() {
                    //     desc.desc.update_range(Some(Range { start: 0, end: bytes as wgpu::BufferAddress }));
                    // }
                },
                vertex_buffer_useinfo::EVertexBufferSlot::Slot18 => {
                    // if let Some(desc) = desc18.get_mut() {
                    //     desc.desc.update_range(Some(Range { start: 0, end: bytes as wgpu::BufferAddress }));
                    // }
                },
            }
        });
    }
}
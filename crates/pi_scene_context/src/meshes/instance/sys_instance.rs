use std::{ops::Range, sync::Arc, time::Instant, marker::PhantomData};

use pi_ecs::{prelude::{Query, ResMut, Res, Component}, query::{Or, Changed, Write}};
use pi_ecs_macros::setup;
use pi_engine_shell::object::{ObjectID, GameObject};
use pi_render::rhi::{device::RenderDevice, RenderQueue};
use render_data_container::{VertexBufferPool};

use crate::{geometry::{vertex_buffer_useinfo, geometry::RenderGeometryEable, instance::{instanced_buffer::TInstancedBuffer, types::TInstancedData}}};

use super::{instanced_mesh::InstanceList};

pub struct SysInstanceBufferUpdate<T: TInstancedBuffer + Component, D: TInstancedData + Component, F: Component>(PhantomData<(T, D, F)>);
#[setup]
impl<T: TInstancedBuffer + Component, D: TInstancedData + Component, F: Component> SysInstanceBufferUpdate<T, D, F> {
    #[system]
    pub fn tick(
        instances: Query<GameObject, &D>,
        mut sources: Query<
            GameObject,
            (
                &InstanceList, &mut T, Write<RenderGeometryEable>,
                Write<vertex_buffer_useinfo::AssetDescVBSlot1>, Write<vertex_buffer_useinfo::AssetDescVBSlot2>, Write<vertex_buffer_useinfo::AssetDescVBSlot3>, Write<vertex_buffer_useinfo::AssetDescVBSlot4>,
                Write<vertex_buffer_useinfo::AssetDescVBSlot5>, Write<vertex_buffer_useinfo::AssetDescVBSlot6>, Write<vertex_buffer_useinfo::AssetDescVBSlot7>, Write<vertex_buffer_useinfo::AssetDescVBSlot8>, 
                Write<vertex_buffer_useinfo::AssetDescVBSlot9>, Write<vertex_buffer_useinfo::AssetDescVBSlot10>, Write<vertex_buffer_useinfo::AssetDescVBSlot11>, Write<vertex_buffer_useinfo::AssetDescVBSlot12>,  
                Write<vertex_buffer_useinfo::AssetDescVBSlot13>,
            ),
            Or<(Changed<InstanceList>, Changed<F>)>
        >,
        mut vbpool: ResMut<VertexBufferPool>,
        device: Res<RenderDevice>,
        queue: Res<RenderQueue>,
    ) {
        let time = Instant::now();
        sources.iter_mut().for_each(|(
            inslist, buffer, mut geodisable,
            mut desc1, mut desc2, mut desc3, mut desc4, 
            mut desc5, mut desc6, mut desc7, mut desc8, 
            mut desc9, mut desc10, mut desc11, mut desc12, 
            mut desc13,
        )| {
            let mut list = vec![];
            inslist.list.iter().for_each(|insid| {
                if let Some(instance) = instances.get(insid.clone()) {
                    list.push(instance);
                }
            });
            let bytes = list.len() * D::bytes_size();

            if list.len() == 0 {
                geodisable.write(RenderGeometryEable(false));
            } else {
                geodisable.write(RenderGeometryEable(true));
                buffer.update::<D>(list.as_slice(), &mut vbpool, &device, &queue);
                match buffer.slot() {
                    vertex_buffer_useinfo::EVertexBufferSlot::Slot01 => {
                        if let Some(desc) = desc1.get_mut() {
                            desc.desc.update_range(Some(Range { start: 0, end: bytes as wgpu::BufferAddress }));
                        }
                    },
                    vertex_buffer_useinfo::EVertexBufferSlot::Slot02 => {
                        if let Some(desc) = desc2.get_mut() {
                            desc.desc.update_range(Some(Range { start: 0, end: bytes as wgpu::BufferAddress }));
                        }
                    },
                    vertex_buffer_useinfo::EVertexBufferSlot::Slot03 => {
                        if let Some(desc) = desc3.get_mut() {
                            desc.desc.update_range(Some(Range { start: 0, end: bytes as wgpu::BufferAddress }));
                        }
                    },
                    vertex_buffer_useinfo::EVertexBufferSlot::Slot04 => {
                        if let Some(desc) = desc4.get_mut() {
                            desc.desc.update_range(Some(Range { start: 0, end: bytes as wgpu::BufferAddress }));
                        }
                    },
                    vertex_buffer_useinfo::EVertexBufferSlot::Slot05 => {
                        if let Some(desc) = desc5.get_mut() {
                            desc.desc.update_range(Some(Range { start: 0, end: bytes as wgpu::BufferAddress }));
                        }
                    },
                    vertex_buffer_useinfo::EVertexBufferSlot::Slot06 => {
                        if let Some(desc) = desc6.get_mut() {
                            desc.desc.update_range(Some(Range { start: 0, end: bytes as wgpu::BufferAddress }));
                        }
                    },
                    vertex_buffer_useinfo::EVertexBufferSlot::Slot07 => {
                        if let Some(desc) = desc7.get_mut() {
                            desc.desc.update_range(Some(Range { start: 0, end: bytes as wgpu::BufferAddress }));
                        }
                    },
                    vertex_buffer_useinfo::EVertexBufferSlot::Slot08 => {
                        if let Some(desc) = desc8.get_mut() {
                            desc.desc.update_range(Some(Range { start: 0, end: bytes as wgpu::BufferAddress }));
                        }
                    },
                    vertex_buffer_useinfo::EVertexBufferSlot::Slot09 => {
                        if let Some(desc) = desc9.get_mut() {
                            desc.desc.update_range(Some(Range { start: 0, end: bytes as wgpu::BufferAddress }));
                        }
                    },
                    vertex_buffer_useinfo::EVertexBufferSlot::Slot10 => {
                        if let Some(desc) = desc10.get_mut() {
                            desc.desc.update_range(Some(Range { start: 0, end: bytes as wgpu::BufferAddress }));
                        }
                    },
                    vertex_buffer_useinfo::EVertexBufferSlot::Slot11 => {
                        if let Some(desc) = desc11.get_mut() {
                            desc.desc.update_range(Some(Range { start: 0, end: bytes as wgpu::BufferAddress }));
                        }
                    },
                    vertex_buffer_useinfo::EVertexBufferSlot::Slot12 => {
                        if let Some(desc) = desc12.get_mut() {
                            desc.desc.update_range(Some(Range { start: 0, end: bytes as wgpu::BufferAddress }));
                        }
                    },
                    vertex_buffer_useinfo::EVertexBufferSlot::Slot13 => {
                        if let Some(desc) = desc13.get_mut() {
                            desc.desc.update_range(Some(Range { start: 0, end: bytes as wgpu::BufferAddress }));
                        }
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
        println!("SysInstancedBufferUpdate<{}>: {:?}", T::display_name(), time1 - time);
    }
}


pub struct SysInstancedBufferInit<T: TInstancedBuffer + Component>(PhantomData<T>);
#[setup]
impl<T: TInstancedBuffer + Component> SysInstancedBufferInit<T> {
    #[system]
    pub fn tick(
        mut sources: Query<
            GameObject,
            (
                &T,
                Write<vertex_buffer_useinfo::AssetResVBSlot1>, Write<vertex_buffer_useinfo::AssetResVBSlot2>, Write<vertex_buffer_useinfo::AssetResVBSlot3>, Write<vertex_buffer_useinfo::AssetResVBSlot4>,
                Write<vertex_buffer_useinfo::AssetResVBSlot5>, Write<vertex_buffer_useinfo::AssetResVBSlot6>, Write<vertex_buffer_useinfo::AssetResVBSlot7>, Write<vertex_buffer_useinfo::AssetResVBSlot8>, 
                Write<vertex_buffer_useinfo::AssetResVBSlot9>, Write<vertex_buffer_useinfo::AssetResVBSlot10>, Write<vertex_buffer_useinfo::AssetResVBSlot11>, Write<vertex_buffer_useinfo::AssetResVBSlot12>,  
                Write<vertex_buffer_useinfo::AssetResVBSlot13>,
            ),
            Changed<T>
        >,
    ) {
        sources.iter_mut().for_each(|(
            buffer,
            mut res1, mut res2, mut res3, mut res4, 
            mut res5, mut res6, mut res7, mut res8, 
            mut res9, mut res10, mut res11, mut res12, 
            mut res13,
        )| {
            println!(">> Sys{}Init", T::display_name());
    
            match buffer.slot() {
                vertex_buffer_useinfo::EVertexBufferSlot::Slot01 => {
                    res1.write(vertex_buffer_useinfo::AssetResVBSlot1::from(buffer.key()));
                },
                vertex_buffer_useinfo::EVertexBufferSlot::Slot02 => {
                    res2.write(vertex_buffer_useinfo::AssetResVBSlot2::from(buffer.key()));
                },
                vertex_buffer_useinfo::EVertexBufferSlot::Slot03 => {
                    res3.write(vertex_buffer_useinfo::AssetResVBSlot3::from(buffer.key()));
                },
                vertex_buffer_useinfo::EVertexBufferSlot::Slot04 => {
                    res4.write(vertex_buffer_useinfo::AssetResVBSlot4::from(buffer.key()));
                },
                vertex_buffer_useinfo::EVertexBufferSlot::Slot05 => {
                    res5.write(vertex_buffer_useinfo::AssetResVBSlot5::from(buffer.key()));
                },
                vertex_buffer_useinfo::EVertexBufferSlot::Slot06 => {
                    res6.write(vertex_buffer_useinfo::AssetResVBSlot6::from(buffer.key()));
                },
                vertex_buffer_useinfo::EVertexBufferSlot::Slot07 => {
                    res7.write(vertex_buffer_useinfo::AssetResVBSlot7::from(buffer.key()));
                },
                vertex_buffer_useinfo::EVertexBufferSlot::Slot08 => {
                    res8.write(vertex_buffer_useinfo::AssetResVBSlot8::from(buffer.key()));
                },
                vertex_buffer_useinfo::EVertexBufferSlot::Slot09 => {
                    res9.write(vertex_buffer_useinfo::AssetResVBSlot9::from(buffer.key()));
                },
                vertex_buffer_useinfo::EVertexBufferSlot::Slot10 => {
                    res10.write(vertex_buffer_useinfo::AssetResVBSlot10::from(buffer.key()));
                },
                vertex_buffer_useinfo::EVertexBufferSlot::Slot11 => {
                    res11.write(vertex_buffer_useinfo::AssetResVBSlot11::from(buffer.key()));
                },
                vertex_buffer_useinfo::EVertexBufferSlot::Slot12 => {
                    res12.write(vertex_buffer_useinfo::AssetResVBSlot12::from(buffer.key()));
                },
                vertex_buffer_useinfo::EVertexBufferSlot::Slot13 => {
                    res13.write(vertex_buffer_useinfo::AssetResVBSlot13::from(buffer.key()));
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
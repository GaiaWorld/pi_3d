
use std::{hash::{Hash, Hasher}, ops::Range};

use pi_scene_shell::prelude::*;

use super::{
    base::GeometryDesc, geometry::*, vertex_buffer_useinfo::*, FlagGeometryDirty, MeshInstanceState
};

pub fn sys_vertex_buffer_slots_loaded(
    addeds: ComponentAdded<FlagGeometryDirty>,
    changes: ComponentChanged<FlagGeometryDirty>,
    items: Query<
        (
            Entity, 
            (&MeshID, &GeometryDesc, &IndicesBufferDescComp, &AssetResBufferIndicesComp, &AssetKeyBufferIndices)
            , &AssetDescVBSlots, &AssetResVBSlots, &LoadedKeyVBSlots
        )
    >,
    mut geometries: Query<&mut RenderGeometryComp>,
    mut meshes: Query<(&mut RenderGeometryEable, &MeshInstanceState)>,
    entitysets: Res<EntityFilterForComponentChanged>,
    // mut performance: ResMut<Performance>,
) {
    // performance.systems.push(String::from("sys_vertex_buffer_slots_loaded"));
    let mut counter = 0;
    let mut entities = entitysets.pop();
    addeds.iter().for_each(|entity| {
        entities.insert(*entity);
    });
    changes.iter().for_each(|entity| {
        entities.insert(*entity);
    });
    entities.iter().for_each(|entity| {
        if let Ok((
            idgeo, 
            (idmesh, geodesc, indicesdesc, indices, indiceskey)
            , desclist, datalist, _keyslist
        )) = items.get(*entity) {
            if let (Ok(mut geometry), Ok((mut rendergeo, instancestate))) = (geometries.get_mut(idgeo), meshes.get_mut(idmesh.0)) {
                counter += 1;

                let mut values = vec![];
                let mut instance_memory = None;
                let mut isready = true;

                let max: usize = geodesc.slot_count();
                for slot in 0..max {
                    match (desclist.get(slot), datalist.get(slot)) {
                        (Some(Some(desc)), Some(Some(data))) => {
                            // log::warn!("Ready Slot {:?}", slot);
                            match &data.0 {
                                EVerticesBufferTmp::Instance(mem) => { instance_memory = Some(mem.clone()); },
                                EVerticesBufferTmp::Buffer(buf) => {
                                    let buff = RenderVertices {
                                        slot: slot as u32,
                                        buffer: buf.clone(),
                                        buffer_range: desc.range(),
                                        size_per_value: desc.0.stride()
                                    };
                                    values.push((desc.0.step_mode(), buff));
                                },
                            }
                        },
                        (None, None) => {
                            break;
                        },
                        _ => {
                            // log::error!("Not Ready Slot {:?}", (slot, max, idgeo, idmesh.0, desclist.get(slot)));
                            isready = false;
                            break;
                        }
                    }
                }
    
                if isready {
                    // if instance_memory.is_none() {
                    //     log::error!("Geo Ready {:?}", (idgeo, idmesh.0, desclist.get(0)));
                    // }
                    match (&indicesdesc.0, &indiceskey.0, &indices.0) {
                        (Some(inddesc), Some(key), Some(_data)) => {
                            if &inddesc.buffer == key {
                                let mut hasher = DefaultHasher::default();
                                geodesc.hash_resource(&mut hasher);
                                if instancestate.use_single_instancebuffer {
                                    idgeo.hash(&mut hasher);
                                }
                                geometry.0 = Some(RenderGeometry::create(values, (indicesdesc.0.as_ref() , indices.0.as_ref()), instance_memory, hasher.finish()));
                                
                                *rendergeo = RenderGeometryEable(true);
                            } else {
                                if rendergeo.0 == true {
                                    *rendergeo = RenderGeometryEable(false);
                                }
                            }
                        },
                        (None, None, None) => {
                            let mut hasher = DefaultHasher::default();
                            geodesc.hash_resource(&mut hasher);
                            if instancestate.use_single_instancebuffer {
                                idgeo.hash(&mut hasher);
                            }
                            geometry.0 = Some(RenderGeometry::create(values, (indicesdesc.0.as_ref() , indices.0.as_ref()), instance_memory, hasher.finish()));
                            *rendergeo = RenderGeometryEable(true);
                        },
                        _ => {
                            // isready = false;
                            if rendergeo.0 == true {
                                *rendergeo = RenderGeometryEable(false);
                            }
                        }
                    }
                }
            }
        }
    });
    entitysets.push(entities);
    // log::error!("sys_vertex_buffer_slots_loaded {:?}", counter);
}

pub fn sys_geometry_enable(
    addeds: ComponentAdded<RenderGeometryComp>,
    changes: ComponentChanged<RenderGeometryComp>,
    geometries: Query<(&RenderGeometryComp, &MeshID)>,
    mut meshes: Query<&mut RenderGeometryEable>,
    // mut performance: ResMut<Performance>,
) {
    // performance.systems.push(String::from("sys_geometry_enable"));
    addeds.iter().chain(changes.iter()).for_each(|entity| {
        if let Ok((geometrycomp, idmesh)) = geometries.get(*entity) {
            if let Ok(mut state) = meshes.get_mut(idmesh.0) {
                *state = RenderGeometryEable(geometrycomp.is_some());
            }
        }
    });
    // changes.iter().for_each(|entity| {
    //     if let Ok((geometrycomp, idmesh)) = geometries.get(*entity) {
    //         if let Ok(mut state) = meshes.get_mut(idmesh.0) {
    //             *state = RenderGeometryEable(geometrycomp.is_some());
    //         }
    //     }
    // });
}
use std::{mem::replace, sync::Arc};

use pi_engine_shell::prelude::*;

use crate::{prelude::{instance_buffer_update, RenderGeometryComp, ActionListDisposeCan, ActionListDisposeReady, OpsDisposeCan}, object::ActionEntity};

use super::{
    vertex_buffer_useinfo::*,
    base::*,
    instance::{instance_world_matrix::InstanceBufferWorldMatrix, instance_color::InstanceBufferColor, instance_tilloff::InstanceBufferTillOff, InstanceMesh},
};

use super::command::*;


pub fn sys_create_geometry(
    mut commands: Commands,
    mut meshes: Query<&mut GeometryID>,
    mut cmds: ResMut<ActionListGeometryCreate>,
    mut geoloader: ResMut<GeometryVBLoader>,
    mut vb_data_map: ResMut<VertexBufferDataMap3D>,
    asset_mgr: Res<ShareAssetMgr<EVertexBufferRange>>,
    mut allocator: ResMut<VertexBufferAllocator3D>,
    device: Res<PiRenderDevice>,
    queue: Res<PiRenderQueue>,
    mut disposereadylist: ResMut<ActionListDisposeReady>,
    mut disposecanlist: ResMut<ActionListDisposeCan>,
) {
    cmds.drain().drain(..).for_each(|OpsGeomeryCreate(id_mesh, entity, vertex_desc, indices_desc, count)| {
        if let Ok(mut mesh) = meshes.get_mut(id_mesh) {
            *mesh = GeometryID(entity);
        } else if count < 2 {
            cmds.push(OpsGeomeryCreate(id_mesh, entity, vertex_desc, indices_desc, count + 1));
            return;
        } else {
            // Geometry 不在应用层, 依附于 Mesh
            disposecanlist.push(OpsDisposeCan::ops(entity));
            return;
        }
        
        let mut geocommands = if let Some(cmd) = commands.get_entity(entity) {
            cmd
        } else {
            return;
        };

        ActionGeometry::init(&mut geocommands, &vertex_desc, indices_desc.clone(), id_mesh);

        let instancesource = InstanceMesh(id_mesh);

        let geo_desc = GeometryDesc { list: vertex_desc };
    
        geocommands
            .remove::<AssetDescVBSlot01>()
            .remove::<AssetDescVBSlot02>()
            .remove::<AssetDescVBSlot03>()
            .remove::<AssetDescVBSlot04>()
            .remove::<AssetDescVBSlot05>()
            .remove::<AssetDescVBSlot06>()
            .remove::<AssetDescVBSlot07>()
            .remove::<AssetDescVBSlot08>()
            .remove::<AssetDescVBSlot09>()
            .remove::<AssetDescVBSlot10>()
            .remove::<AssetDescVBSlot11>()
            .remove::<AssetDescVBSlot12>()
            .remove::<AssetDescVBSlot13>()
            .remove::<AssetDescVBSlot14>()
            .remove::<AssetDescVBSlot15>()
            .remove::<AssetDescVBSlot16>();

        let mut instance_code = EVerticeExtendCode(EVerticeExtendCode::NONE);
        init_slot::<AssetDescVBSlot01, AssetResVBSlot01>(&instancesource, entity, &geo_desc, &mut instance_code, &mut geoloader.loader_01,  &mut vb_data_map, &asset_mgr, &mut geocommands, &mut allocator, &device, &queue);
        init_slot::<AssetDescVBSlot02, AssetResVBSlot02>(&instancesource, entity, &geo_desc, &mut instance_code, &mut geoloader.loader_02,  &mut vb_data_map, &asset_mgr,  &mut geocommands, &mut allocator, &device, &queue);
        init_slot::<AssetDescVBSlot03, AssetResVBSlot03>(&instancesource, entity, &geo_desc, &mut instance_code, &mut geoloader.loader_03,  &mut vb_data_map, &asset_mgr,  &mut geocommands, &mut allocator, &device, &queue);
        init_slot::<AssetDescVBSlot04, AssetResVBSlot04>(&instancesource, entity, &geo_desc, &mut instance_code, &mut geoloader.loader_04,  &mut vb_data_map, &asset_mgr,  &mut geocommands, &mut allocator, &device, &queue);
        init_slot::<AssetDescVBSlot05, AssetResVBSlot05>(&instancesource, entity, &geo_desc, &mut instance_code, &mut geoloader.loader_05,  &mut vb_data_map, &asset_mgr,  &mut geocommands, &mut allocator, &device, &queue);
        init_slot::<AssetDescVBSlot06, AssetResVBSlot06>(&instancesource, entity, &geo_desc, &mut instance_code, &mut geoloader.loader_06,  &mut vb_data_map, &asset_mgr,  &mut geocommands, &mut allocator, &device, &queue);
        init_slot::<AssetDescVBSlot07, AssetResVBSlot07>(&instancesource, entity, &geo_desc, &mut instance_code, &mut geoloader.loader_07,  &mut vb_data_map, &asset_mgr,  &mut geocommands, &mut allocator, &device, &queue);
        init_slot::<AssetDescVBSlot08, AssetResVBSlot08>(&instancesource, entity, &geo_desc, &mut instance_code, &mut geoloader.loader_08,  &mut vb_data_map, &asset_mgr,  &mut geocommands, &mut allocator, &device, &queue);
        init_slot::<AssetDescVBSlot09, AssetResVBSlot09>(&instancesource, entity, &geo_desc, &mut instance_code, &mut geoloader.loader_09,  &mut vb_data_map, &asset_mgr,  &mut geocommands, &mut allocator, &device, &queue);
        init_slot::<AssetDescVBSlot10, AssetResVBSlot10>(&instancesource, entity, &geo_desc, &mut instance_code, &mut geoloader.loader_10,  &mut vb_data_map, &asset_mgr,  &mut geocommands, &mut allocator, &device, &queue);
        init_slot::<AssetDescVBSlot11, AssetResVBSlot11>(&instancesource, entity, &geo_desc, &mut instance_code, &mut geoloader.loader_11,  &mut vb_data_map, &asset_mgr,  &mut geocommands, &mut allocator, &device, &queue);
        init_slot::<AssetDescVBSlot12, AssetResVBSlot12>(&instancesource, entity, &geo_desc, &mut instance_code, &mut geoloader.loader_12,  &mut vb_data_map, &asset_mgr,  &mut geocommands, &mut allocator, &device, &queue);

        // log::debug!(">>>>  GeometryDesc ");
        geocommands.insert(geo_desc);
        geocommands.insert(EVerticeExtendCodeComp(instance_code));
        
        if let Some(indices_desc) = indices_desc {
            if let Some(data) = asset_mgr.get(&indices_desc.buffer.asset_u64()) {
                // log::warn!("Indice Buffer Ok!");
                geocommands.insert(AssetResBufferIndices::from(EVerticesBufferUsage::Other(data)));
            } else {
                geoloader.loader_indices.request(entity, &indices_desc.buffer, None, &mut vb_data_map);
            }
        } else {
            geocommands.remove::<AssetResBufferIndices>();
        }
    });
}

pub fn sys_act_geomettry_instance_world_matrix(
    mut cmds: ResMut<ActionListInstanceWorldMatrixs>,
    mut geoloader: ResMut<GeometryVBLoader>,
    mut vb_data_map: ResMut<VertexBufferDataMap3D>,
    mut geometrys: Query<&mut InstanceBufferWorldMatrix>,
    mut slots: (
        Query<&mut AssetResVBSlot01>,
        Query<&mut AssetResVBSlot02>,
        Query<&mut AssetResVBSlot03>,
        Query<&mut AssetResVBSlot04>,
        Query<&mut AssetResVBSlot05>,
        Query<&mut AssetResVBSlot06>,
        Query<&mut AssetResVBSlot07>,
        Query<&mut AssetResVBSlot08>,
        Query<&mut AssetResVBSlot09>,
        Query<&mut AssetResVBSlot10>,
        Query<&mut AssetResVBSlot11>,
        Query<&mut AssetResVBSlot12>,
        Query<&mut AssetResVBSlot13>,
        Query<&mut AssetResVBSlot14>,
        Query<&mut AssetResVBSlot15>,
        Query<&mut AssetResVBSlot16>,
    ),
    mut allocator: ResMut<VertexBufferAllocator3D>,
    device: Res<PiRenderDevice>,
    queue: Res<PiRenderQueue>,
) {
    cmds.drain().drain(..).for_each(|OpsInstanceWorldMatrixs(geo, data, count)| {
        if let Ok(mut buffer) = geometrys.get_mut(geo) {
            instance_buffer_update::<InstanceBufferWorldMatrix>(data, geo, &mut buffer, &mut slots, &mut allocator, &device, &queue);
        } else {
            if count < 1 {
                cmds.push(OpsInstanceWorldMatrixs(geo, data, count + 1));
            }
        }
    });
}

pub fn sys_act_geomettry_instance_color(
    mut cmds: ResMut<ActionListInstanceColors>,
    mut geoloader: ResMut<GeometryVBLoader>,
    mut vb_data_map: ResMut<VertexBufferDataMap3D>,
    mut geometrys: Query<&mut InstanceBufferColor>,
    mut slots: (
        Query<&mut AssetResVBSlot01>,
        Query<&mut AssetResVBSlot02>,
        Query<&mut AssetResVBSlot03>,
        Query<&mut AssetResVBSlot04>,
        Query<&mut AssetResVBSlot05>,
        Query<&mut AssetResVBSlot06>,
        Query<&mut AssetResVBSlot07>,
        Query<&mut AssetResVBSlot08>,
        Query<&mut AssetResVBSlot09>,
        Query<&mut AssetResVBSlot10>,
        Query<&mut AssetResVBSlot11>,
        Query<&mut AssetResVBSlot12>,
        Query<&mut AssetResVBSlot13>,
        Query<&mut AssetResVBSlot14>,
        Query<&mut AssetResVBSlot15>,
        Query<&mut AssetResVBSlot16>,
    ),
    mut allocator: ResMut<VertexBufferAllocator3D>,
    device: Res<PiRenderDevice>,
    queue: Res<PiRenderQueue>,
) {
    cmds.drain().drain(..).for_each(|OpsInstanceColors(geo, data, count)| {
        if let Ok(mut buffer) = geometrys.get_mut(geo) {
            instance_buffer_update::<InstanceBufferColor>(data, geo, &mut buffer, &mut slots, &mut allocator, &device, &queue);
            // geometry_update_instance_buffer::<InstanceBufferColor>(Some(data), geo, &mut buffer, &mut geoloader, &mut vb_data_map);
        } else {
            if count < 1 {
                cmds.push(OpsInstanceColors(geo, data, count + 1));
            }
        }
    });
}

pub fn sys_act_geomettry_instance_tilloff(
    mut cmds: ResMut<ActionListInstanceTilloffs>,
    mut geoloader: ResMut<GeometryVBLoader>,
    mut vb_data_map: ResMut<VertexBufferDataMap3D>,
    mut geometrys: Query<&mut InstanceBufferTillOff>,
    mut slots: (
        Query<&mut AssetResVBSlot01>,
        Query<&mut AssetResVBSlot02>,
        Query<&mut AssetResVBSlot03>,
        Query<&mut AssetResVBSlot04>,
        Query<&mut AssetResVBSlot05>,
        Query<&mut AssetResVBSlot06>,
        Query<&mut AssetResVBSlot07>,
        Query<&mut AssetResVBSlot08>,
        Query<&mut AssetResVBSlot09>,
        Query<&mut AssetResVBSlot10>,
        Query<&mut AssetResVBSlot11>,
        Query<&mut AssetResVBSlot12>,
        Query<&mut AssetResVBSlot13>,
        Query<&mut AssetResVBSlot14>,
        Query<&mut AssetResVBSlot15>,
        Query<&mut AssetResVBSlot16>,
    ),
    mut allocator: ResMut<VertexBufferAllocator3D>,
    device: Res<PiRenderDevice>,
    queue: Res<PiRenderQueue>,
) {
    cmds.drain().drain(..).for_each(|OpsInstanceTilloffs(geo, data, count)| {
        if let Ok(mut buffer) = geometrys.get_mut(geo) {
            instance_buffer_update::<InstanceBufferTillOff>(data, geo, &mut buffer, &mut slots, &mut allocator, &device, &queue);
            // geometry_update_instance_buffer::<InstanceBufferTillOff>(Some(data), geo, &mut buffer, &mut geoloader, &mut vb_data_map);
        } else {
            if count < 1 {
                cmds.push(OpsInstanceTilloffs(geo, data, count + 1));
            }
        }
    });
}

pub struct ActionVertexBuffer;
impl ActionVertexBuffer {
    pub fn check(
        asset_mgr: &ShareAssetMgr<EVertexBufferRange>,
        key: KeyVertexBuffer,
    ) -> bool {
        asset_mgr.contains_key(&key.asset_u64())
    }
    pub fn create(
        data_map: &mut SingleVertexBufferDataMap,
        key: KeyVertexBuffer,
        buffer: Vec<u8>,
    ) {
        data_map.add(&key, buffer);
    }
    pub fn create_indices(
        data_map: &mut SingleVertexBufferDataMap,
        key: KeyVertexBuffer,
        buffer: Vec<u8>,
    ) {
        data_map.add_indices(&key, buffer);
    }
}

pub struct ActionGeometry;
impl ActionGeometry {
    pub fn init(
        cmds: &mut EntityCommands,
        vertex_desc: &Vec<VertexBufferDesc>,
        indices_desc: Option<IndicesBufferDesc>,
        id_mesh: Entity,
    ) {
        ActionEntity::init(cmds);
        cmds
            .insert(VertexBufferLayoutsComp(VertexBufferLayouts::from(vertex_desc)))
            .insert(MeshID(id_mesh))
            .insert(RenderGeometryComp::default())
            ;
        if let Some(indices_desc) = indices_desc {
            cmds.insert(indices_desc);
        } else {
            cmds.remove::<IndicesBufferDesc>();
        }
    }
}

fn init_slot<
    D: TVertexBufferUseInfo + Component,
    D1: From<EVerticesBufferUsage> + Component,
>(
    instancesource: &InstanceMesh,
    id_geo: ObjectID,
    geodesc: &GeometryDesc,
    instance_code: &mut EVerticeExtendCode,
    loader: &mut VertexBufferLoader<ObjectID, D1>,
    vb_data_map: &mut SingleVertexBufferDataMap,
    asset_mgr: &ShareAssetMgr<EVertexBufferRange>,
    commands: &mut EntityCommands,
    allocator: &mut VertexBufferAllocator3D,
    device: &RenderDevice,
    queue: &RenderQueue,
) {
    let slot_index = D::ASK_SLOT_COUNT as usize - 1;
    if slot_index >= geodesc.list.len() {
        commands.remove::<D>();
        commands.remove::<D1>();
    } else {
        let desc = geodesc.get_desc(slot_index);
        let instance_kind = desc.instance_kind();
        match instance_kind {
            EInstanceKind::None => {
                if let Some(data) = asset_mgr.get(&desc.key.asset_u64()) {
                    // log::warn!("Vertex Buffer Ok!");
                    commands.insert(D1::from(EVerticesBufferUsage::Other(data)));
                } else {
                    commands.remove::<D1>();
                    loader.request(id_geo, &desc.key, None, vb_data_map);
                }
            },
            _ => {
    
                let buff_id = instancesource.id();
                let mut temp = Vec::with_capacity(64);
                for _ in 0..64 {
                    temp.push(0);
                }
    
                match instance_kind {
                    EInstanceKind::WorldMatrix => {
                        let buff = InstanceBufferWorldMatrix { slot: slot_index, index: KeyVertexBuffer::from((buff_id + "WorldMatrix").as_str()) };
                        commands.insert(buff);
                        if let Some(buffer) = allocator.create_not_updatable_buffer_pre(device, queue, &temp, None) {
                            commands.insert(D1::from(EVerticesBufferUsage::EVBRange(Arc::new(EVertexBufferRange::NotUpdatable(buffer, 0, 0)))));
                        }
                        instance_code.0 = instance_code.0 | EVerticeExtendCode::INSTANCE_BASE;
                    },
                    EInstanceKind::Color => {
                        let buff = InstanceBufferColor { slot: slot_index, index: KeyVertexBuffer::from((buff_id + "Color").as_str()) };
                        commands.insert(buff);
                        if let Some(buffer) = allocator.create_not_updatable_buffer_pre(device, queue, &temp, None) {
                            commands.insert(D1::from(EVerticesBufferUsage::EVBRange(Arc::new(EVertexBufferRange::NotUpdatable(buffer, 0, 0)))));
                        }
                        // log::debug!("Instance Color");
                        instance_code.0 = instance_code.0 | EVerticeExtendCode::INSTANCE_COLOR;
                    },
                    EInstanceKind::TillOffset => {
                        let buff = InstanceBufferTillOff { slot: slot_index, index: KeyVertexBuffer::from((buff_id + "TillOff").as_str()) };
                        commands.insert(buff);
                        if let Some(buffer) = allocator.create_not_updatable_buffer_pre(device, queue, &temp, None) {
                            commands.insert(D1::from(EVerticesBufferUsage::EVBRange(Arc::new(EVertexBufferRange::NotUpdatable(buffer, 0, 0)))));
                        }
                        // log::debug!("Instance TillOffset");
                        instance_code.0 = instance_code.0 | EVerticeExtendCode::INSTANCE_TILL_OFF_1;
                    },
                    _ => { },
                }
            },
        };
        
        commands.insert(D::from(desc));
    }
}

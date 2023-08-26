use std::sync::Arc;

use pi_engine_shell::prelude::*;

use crate::{prelude::{RenderGeometryComp, ActionListDisposeCan, ActionListDisposeReady, OpsDisposeCan, MeshInstanceState}, object::ActionEntity};

use super::{
    vertex_buffer_useinfo::*,
    base::*,
    instance::{instance_world_matrix::InstanceBufferWorldMatrix, instance_color::InstanceBufferColor, instance_tilloff::InstanceBufferTillOff, instanced_buffer::{InstancedInfo, InstanceBufferAllocator}},
};

use super::command::*;


pub fn sys_create_geometry(
    mut commands: Commands,
    mut meshes: Query<(&mut GeometryID, &MeshInstanceState)>,
    mut cmds: ResMut<ActionListGeometryCreate>,
    mut geoloader: ResMut<GeometryVBLoader>,
    mut vb_data_map: ResMut<VertexBufferDataMap3D>,
    asset_mgr: Res<ShareAssetMgr<EVertexBufferRange>>,
    mut instanceallocator: ResMut<InstanceBufferAllocator>,
    mut _disposereadylist: ResMut<ActionListDisposeReady>,
    mut disposecanlist: ResMut<ActionListDisposeCan>,
) {
    cmds.drain().drain(..).for_each(|OpsGeomeryCreate(id_mesh, entity, mut vertex_desc, indices_desc, count)| {
        
        let instancestate =if let Ok((mut mesh, insstate)) = meshes.get_mut(id_mesh) {
            *mesh = GeometryID(entity);
            insstate.state
        } else if count < 2 {
            cmds.push(OpsGeomeryCreate(id_mesh, entity, vertex_desc, indices_desc, count + 1));
            return;
        } else {
            // Geometry 不在应用层, 依附于 Mesh
            disposecanlist.push(OpsDisposeCan::ops(entity));
            return;
        };

        let mut geocommands = if let Some(cmd) = commands.get_entity(entity) {
            cmd
        } else {
            return;
        };

        if instancestate > 0 {
            vertex_desc.push(VertexBufferDesc {
                key: KeyVertexBuffer::from(""),
                range: None,
                attrs: InstanceState::attributes(instancestate),
                step_mode: wgpu::VertexStepMode::Instance,
                instance: true,
            });
        }

        ActionGeometry::init(&mut geocommands, &vertex_desc, indices_desc.clone(), id_mesh);

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


        init_slot::<AssetDescVBSlot01, AssetResVBSlot01>(entity, &geo_desc, &mut geoloader.loader_01,  &mut vb_data_map, &asset_mgr, &mut geocommands,   &mut instanceallocator, instancestate);
        init_slot::<AssetDescVBSlot02, AssetResVBSlot02>(entity, &geo_desc, &mut geoloader.loader_02,  &mut vb_data_map, &asset_mgr,  &mut geocommands,  &mut instanceallocator, instancestate);
        init_slot::<AssetDescVBSlot03, AssetResVBSlot03>(entity, &geo_desc, &mut geoloader.loader_03,  &mut vb_data_map, &asset_mgr,  &mut geocommands,  &mut instanceallocator, instancestate);
        init_slot::<AssetDescVBSlot04, AssetResVBSlot04>(entity, &geo_desc, &mut geoloader.loader_04,  &mut vb_data_map, &asset_mgr,  &mut geocommands,  &mut instanceallocator, instancestate);
        init_slot::<AssetDescVBSlot05, AssetResVBSlot05>(entity, &geo_desc, &mut geoloader.loader_05,  &mut vb_data_map, &asset_mgr,  &mut geocommands,  &mut instanceallocator, instancestate);
        init_slot::<AssetDescVBSlot06, AssetResVBSlot06>(entity, &geo_desc, &mut geoloader.loader_06,  &mut vb_data_map, &asset_mgr,  &mut geocommands,  &mut instanceallocator, instancestate);
        init_slot::<AssetDescVBSlot07, AssetResVBSlot07>(entity, &geo_desc, &mut geoloader.loader_07,  &mut vb_data_map, &asset_mgr,  &mut geocommands,  &mut instanceallocator, instancestate);
        init_slot::<AssetDescVBSlot08, AssetResVBSlot08>(entity, &geo_desc, &mut geoloader.loader_08,  &mut vb_data_map, &asset_mgr,  &mut geocommands,  &mut instanceallocator, instancestate);
        init_slot::<AssetDescVBSlot09, AssetResVBSlot09>(entity, &geo_desc, &mut geoloader.loader_09,  &mut vb_data_map, &asset_mgr,  &mut geocommands,  &mut instanceallocator, instancestate);
        init_slot::<AssetDescVBSlot10, AssetResVBSlot10>(entity, &geo_desc, &mut geoloader.loader_10,  &mut vb_data_map, &asset_mgr,  &mut geocommands,  &mut instanceallocator, instancestate);
        init_slot::<AssetDescVBSlot11, AssetResVBSlot11>(entity, &geo_desc, &mut geoloader.loader_11,  &mut vb_data_map, &asset_mgr,  &mut geocommands,  &mut instanceallocator, instancestate);
        init_slot::<AssetDescVBSlot12, AssetResVBSlot12>(entity, &geo_desc, &mut geoloader.loader_12,  &mut vb_data_map, &asset_mgr,  &mut geocommands,  &mut instanceallocator, instancestate);

        // log::debug!(">>>>  GeometryDesc ");
        geocommands.insert(geo_desc);
        let instance_code = EVerticeExtendCode(EVerticeExtendCode::NONE + instancestate);
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
    mut _cmds: ResMut<ActionListInstanceWorldMatrixs>,
    mut _geometrys: Query<&mut InstanceBufferWorldMatrix>,
    mut _slots: (
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
    mut _allocator: ResMut<VertexBufferAllocator3D>,
    _device: Res<PiRenderDevice>,
    _queue: Res<PiRenderQueue>,
) {
    // cmds.drain().drain(..).for_each(|OpsInstanceWorldMatrixs(geo, data, count)| {
        // if let Ok(mut buffer) = geometrys.get_mut(geo) {
        //     instance_buffer_update::<InstanceBufferWorldMatrix>(data, geo, &mut buffer, &mut slots, &mut allocator, &device, &queue);
        // } else {
        //     if count < 1 {
        //         cmds.push(OpsInstanceWorldMatrixs(geo, data, count + 1));
        //     }
        // }
    // });
}

pub fn sys_act_geomettry_instance_color(
    mut _cmds: ResMut<ActionListInstanceColors>,
    mut _geometrys: Query<&mut InstanceBufferColor>,
    mut _slots: (
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
    mut _allocator: ResMut<VertexBufferAllocator3D>,
    _device: Res<PiRenderDevice>,
    _queue: Res<PiRenderQueue>,
) {
    // cmds.drain().drain(..).for_each(|OpsInstanceColors(geo, data, count)| {
        // if let Ok(mut buffer) = geometrys.get_mut(geo) {
        //     instance_buffer_update::<InstanceBufferColor>(data, geo, &mut buffer, &mut slots, &mut allocator, &device, &queue);
        //     // geometry_update_instance_buffer::<InstanceBufferColor>(Some(data), geo, &mut buffer, &mut geoloader, &mut vb_data_map);
        // } else {
        //     if count < 1 {
        //         cmds.push(OpsInstanceColors(geo, data, count + 1));
        //     }
        // }
    // });
}

pub fn sys_act_geomettry_instance_tilloff(
    mut _cmds: ResMut<ActionListInstanceTilloffs>,
    mut _geometrys: Query<&mut InstanceBufferTillOff>,
    mut _slots: (
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
    mut _allocator: ResMut<VertexBufferAllocator3D>,
    _device: Res<PiRenderDevice>,
    _queue: Res<PiRenderQueue>,
) {
    // cmds.drain().drain(..).for_each(|OpsInstanceTilloffs(geo, data, count)| {
        // if let Ok(mut buffer) = geometrys.get_mut(geo) {
        //     instance_buffer_update::<InstanceBufferTillOff>(data, geo, &mut buffer, &mut slots, &mut allocator, &device, &queue);
        //     // geometry_update_instance_buffer::<InstanceBufferTillOff>(Some(data), geo, &mut buffer, &mut geoloader, &mut vb_data_map);
        // } else {
        //     if count < 1 {
        //         cmds.push(OpsInstanceTilloffs(geo, data, count + 1));
        //     }
        // }
    // });
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
    id_geo: ObjectID,
    geodesc: &GeometryDesc,
    loader: &mut VertexBufferLoader<ObjectID, D1>,
    vb_data_map: &mut SingleVertexBufferDataMap,
    asset_mgr: &ShareAssetMgr<EVertexBufferRange>,
    commands: &mut EntityCommands,
    instanceallocator: &mut InstanceBufferAllocator,
    instancestate: u32,
) {
    let slot_index = D::ASK_SLOT_COUNT as usize - 1;
    if slot_index >= geodesc.list.len() {
        commands.remove::<D>();
        commands.remove::<D1>();
    } else {
        let desc = geodesc.get_desc(slot_index);
        if desc.instance() == false {
            if let Some(data) = asset_mgr.get(&desc.key.asset_u64()) {
                // log::warn!("Vertex Buffer Ok!");
                commands.insert(D1::from(EVerticesBufferUsage::Other(data)));
            } else {
                commands.remove::<D1>();
                loader.request(id_geo, &desc.key, None, vb_data_map);
            }
        } else {
            let info = InstancedInfo::new(instancestate, EVertexBufferSlot::from_u8_unsafe(slot_index as u8));
            // log::warn!("Geometry: {:?}", EVertexBufferSlot::from_u8_unsafe(slot_index as u8));
            let data = instanceallocator.instance_initial_buffer();
            commands.insert(D1::from(EVerticesBufferUsage::EVBRange(Arc::new(EVertexBufferRange::NotUpdatable(data.0, data.1, data.2)))));
            commands.insert(info);
        }
        commands.insert(D::from(desc));
    }
}

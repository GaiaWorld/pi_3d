use pi_engine_shell::prelude::*;
use pi_render::{
    renderer::{
        vertex_buffer_desc::*,
        indices::{IndicesBufferDesc, AssetResBufferIndices, AssetKeyBufferIndices},
        vertex_buffer_loader::{SingleVertexBufferDataMap, VertexBufferLoader},
        vertex_buffer::EVertexBufferRange, vertices::EVerticesBufferUsage, instance::EInstanceKind
    },
    render_3d::shader::instance_code::EInstanceCode
};

use crate::meshes::MeshID;

use super::{
    vertex_buffer_useinfo::*,
    GeometryDesc,
    geometry::RenderGeometryEable,
    instance::{InstanceSourceRecord, instance_world_matrix::InstancedBufferWorldMatrix, instance_color::InstancedBufferColor, instance_tilloff::InstancedBufferTillOff}, base::VBLoaderSlot
};

#[derive(Debug)]
pub enum ECommand {
    Desc(ObjectID, Vec<VertexBufferDesc>, Option<IndicesBufferDesc>)
}

#[derive(Debug, Default)]
pub struct SingleGeometryVBCommands(pub Vec<ECommand>);

pub struct ActionGeometry;
impl ActionGeometry {
    pub fn create(
        commands: &mut EntityCommands,
        id_geo: Entity,
        vertex_desc: Vec<VertexBufferDesc>,
        indices_desc: Option<IndicesBufferDesc>,
        loader_01: &mut VertexBufferLoader<ObjectID, AssetResVBSlot01>,
        loader_02: &mut VertexBufferLoader<ObjectID, AssetResVBSlot02>,
        loader_03: &mut VertexBufferLoader<ObjectID, AssetResVBSlot03>,
        loader_04: &mut VertexBufferLoader<ObjectID, AssetResVBSlot04>,
        loader_05: &mut VertexBufferLoader<ObjectID, AssetResVBSlot05>,
        loader_06: &mut VertexBufferLoader<ObjectID, AssetResVBSlot06>,
        loader_07: &mut VertexBufferLoader<ObjectID, AssetResVBSlot07>,
        loader_08: &mut VertexBufferLoader<ObjectID, AssetResVBSlot08>,
        loader_09: &mut VertexBufferLoader<ObjectID, AssetResVBSlot09>,
        loader_10: &mut VertexBufferLoader<ObjectID, AssetResVBSlot10>,
        loader_11: &mut VertexBufferLoader<ObjectID, AssetResVBSlot11>,
        loader_12: &mut VertexBufferLoader<ObjectID, AssetResVBSlot12>,
        loader_indices: &mut VertexBufferLoader<ObjectID, AssetResBufferIndices>,
        vb_data_map: &mut SingleVertexBufferDataMap,
        asset_mgr: ShareAssetMgr<EVertexBufferRange>,
        instance_source_record: &mut InstanceSourceRecord,
    ) {
        commands.insert(VertexBufferLayouts::from(&vertex_desc));
        let geo_desc = GeometryDesc { list: vertex_desc };
    
        commands
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

        
        let mut instance_code = EInstanceCode(EInstanceCode::NONE);
        init_slot::<AssetDescVBSlot01, AssetResVBSlot01>(id_geo, &geo_desc, &mut instance_code, &mut loader_01,  &mut vb_data_map, &asset_mgr, &mut instance_source_record, &mut commands);
        init_slot::<AssetDescVBSlot02, AssetResVBSlot02>(id_geo, &geo_desc, &mut instance_code, &mut loader_02,  &mut vb_data_map, &asset_mgr, &mut instance_source_record, &mut commands);
        init_slot::<AssetDescVBSlot03, AssetResVBSlot03>(id_geo, &geo_desc, &mut instance_code, &mut loader_03,  &mut vb_data_map, &asset_mgr, &mut instance_source_record, &mut commands);
        init_slot::<AssetDescVBSlot04, AssetResVBSlot04>(id_geo, &geo_desc, &mut instance_code, &mut loader_04,  &mut vb_data_map, &asset_mgr, &mut instance_source_record, &mut commands);
        init_slot::<AssetDescVBSlot05, AssetResVBSlot05>(id_geo, &geo_desc, &mut instance_code, &mut loader_05,  &mut vb_data_map, &asset_mgr, &mut instance_source_record, &mut commands);
        init_slot::<AssetDescVBSlot06, AssetResVBSlot06>(id_geo, &geo_desc, &mut instance_code, &mut loader_06,  &mut vb_data_map, &asset_mgr, &mut instance_source_record, &mut commands);
        init_slot::<AssetDescVBSlot07, AssetResVBSlot07>(id_geo, &geo_desc, &mut instance_code, &mut loader_07,  &mut vb_data_map, &asset_mgr, &mut instance_source_record, &mut commands);
        init_slot::<AssetDescVBSlot08, AssetResVBSlot08>(id_geo, &geo_desc, &mut instance_code, &mut loader_08,  &mut vb_data_map, &asset_mgr, &mut instance_source_record, &mut commands);
        init_slot::<AssetDescVBSlot09, AssetResVBSlot09>(id_geo, &geo_desc, &mut instance_code, &mut loader_09,  &mut vb_data_map, &asset_mgr, &mut instance_source_record, &mut commands);
        init_slot::<AssetDescVBSlot10, AssetResVBSlot10>(id_geo, &geo_desc, &mut instance_code, &mut loader_10,  &mut vb_data_map, &asset_mgr, &mut instance_source_record, &mut commands);
        init_slot::<AssetDescVBSlot11, AssetResVBSlot11>(id_geo, &geo_desc, &mut instance_code, &mut loader_11,  &mut vb_data_map, &asset_mgr, &mut instance_source_record, &mut commands);
        init_slot::<AssetDescVBSlot12, AssetResVBSlot12>(id_geo, &geo_desc, &mut instance_code, &mut loader_12,  &mut vb_data_map, &asset_mgr, &mut instance_source_record, &mut commands);

        // log::debug!(">>>>  GeometryDesc ");
        commands.insert(geo_desc);
        commands.insert(instance_code);
        
        if let Some(indices_desc) = indices_desc {
            if let Some(data) = asset_mgr.get(&indices_desc.buffer) {
                commands.insert(AssetResBufferIndices::from(EVerticesBufferUsage::Other(data)));
            } else {
                commands.remove::<IndicesBufferDesc>();
                loader_indices.request(id_geo, &indices_desc.buffer, None, &mut vb_data_map);
            }
            commands.insert(indices_desc);
        } else {
            commands.remove::<IndicesBufferDesc>();
            commands.remove::<AssetResBufferIndices>();
        }
    }
}


fn init_slot<
    D: TVertexBufferUseInfo + Component,
    D1: From<EVerticesBufferUsage> + Component,
>(
    id_geo: ObjectID,
    geodesc: &GeometryDesc,
    instance_code: &mut EInstanceCode,
    loader_01: &mut VBLoaderSlot<ObjectID, D1>,
    vb_data_map: &mut SingleVertexBufferDataMap,
    asset_mgr: &ShareAssetMgr<EVertexBufferRange>,
    instance_source_record: &mut InstanceSourceRecord,
    commands: &mut EntityCommands,
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
                if let Some(data) = asset_mgr.get(&desc.key) {
                    commands.insert(D1::from(EVerticesBufferUsage::Other(data)));
                } else {
                    commands.remove::<D1>();
                    loader_01.request(id_geo, &desc.key, None, vb_data_map);
                }
            },
            _ => {
    
                let buff_id = instance_source_record.id().to_string();
    
                match instance_kind {
                    EInstanceKind::WorldMatrix => {
                        let buff = InstancedBufferWorldMatrix { slot: slot_index, id: String::from(buff_id + "WorldMatrix"), index: 0 };
                        commands.insert(buff);
                        instance_code.0 = instance_code.0 | EInstanceCode::BASE;
                    },
                    EInstanceKind::Color => {
                        let buff = InstancedBufferColor { slot: slot_index, id: String::from(buff_id + "Color"), index: 0 };
                        commands.insert(buff);
                        // log::debug!("Instance Color");
                        instance_code.0 = instance_code.0 | EInstanceCode::COLOR;
                    },
                    EInstanceKind::TillOffset => {
                        let buff = InstancedBufferTillOff { slot: slot_index, id: String::from(buff_id + "TillOff"), index: 0 };
                        commands.insert(buff);
                        // log::debug!("Instance TillOffset");
                        instance_code.0 = instance_code.0 | EInstanceCode::TILL_OFF_1;
                    },
                    _ => { },
                }
            },
        };
        
        commands.insert(D::from(desc));
    }
}

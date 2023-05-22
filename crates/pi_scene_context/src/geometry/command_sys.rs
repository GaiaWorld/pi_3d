use std::mem::replace;

use pi_engine_shell::prelude::*;

use super::{
    vertex_buffer_useinfo::*,
    base::*,
    instance::{instance_world_matrix::InstanceBufferWorldMatrix, instance_color::InstanceBufferColor, instance_tilloff::InstanceBufferTillOff, InstanceSourceID},
};

use super::command::*;


pub fn sys_geometry_create(
    mut cmds: ResMut<ActionListGeometryCreate>,
    mut commands: Commands,
    mut geoloader: ResMut<GeometryVBLoader>,
    mut vb_data_map: ResMut<VertexBufferDataMap3D>,
    asset_mgr: Res<ShareAssetMgr<EVertexBufferRange>>,
) {
    cmds.drain().drain(..).for_each(|OpsGeomeryCreate(id_mesh, entity, vertex_desc, indices_desc)| {
        commands.entity(id_mesh).insert(GeometryID(entity));

        let mut geocommands = commands.entity(entity);
        geocommands
            .insert(VertexBufferLayoutsComp(VertexBufferLayouts::from(&vertex_desc)))
            .insert(MeshID(id_mesh));

        let instancesource = InstanceSourceID(id_mesh);

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

        let mut instance_code = EInstanceCode(EInstanceCode::NONE);
        init_slot::<AssetDescVBSlot01, AssetResVBSlot01>(&instancesource, entity, &geo_desc, &mut instance_code, &mut geoloader.loader_01,  &mut vb_data_map, &asset_mgr, &mut geocommands);
        init_slot::<AssetDescVBSlot02, AssetResVBSlot02>(&instancesource, entity, &geo_desc, &mut instance_code, &mut geoloader.loader_02,  &mut vb_data_map, &asset_mgr,  &mut geocommands);
        init_slot::<AssetDescVBSlot03, AssetResVBSlot03>(&instancesource, entity, &geo_desc, &mut instance_code, &mut geoloader.loader_03,  &mut vb_data_map, &asset_mgr,  &mut geocommands);
        init_slot::<AssetDescVBSlot04, AssetResVBSlot04>(&instancesource, entity, &geo_desc, &mut instance_code, &mut geoloader.loader_04,  &mut vb_data_map, &asset_mgr,  &mut geocommands);
        init_slot::<AssetDescVBSlot05, AssetResVBSlot05>(&instancesource, entity, &geo_desc, &mut instance_code, &mut geoloader.loader_05,  &mut vb_data_map, &asset_mgr,  &mut geocommands);
        init_slot::<AssetDescVBSlot06, AssetResVBSlot06>(&instancesource, entity, &geo_desc, &mut instance_code, &mut geoloader.loader_06,  &mut vb_data_map, &asset_mgr,  &mut geocommands);
        init_slot::<AssetDescVBSlot07, AssetResVBSlot07>(&instancesource, entity, &geo_desc, &mut instance_code, &mut geoloader.loader_07,  &mut vb_data_map, &asset_mgr,  &mut geocommands);
        init_slot::<AssetDescVBSlot08, AssetResVBSlot08>(&instancesource, entity, &geo_desc, &mut instance_code, &mut geoloader.loader_08,  &mut vb_data_map, &asset_mgr,  &mut geocommands);
        init_slot::<AssetDescVBSlot09, AssetResVBSlot09>(&instancesource, entity, &geo_desc, &mut instance_code, &mut geoloader.loader_09,  &mut vb_data_map, &asset_mgr,  &mut geocommands);
        init_slot::<AssetDescVBSlot10, AssetResVBSlot10>(&instancesource, entity, &geo_desc, &mut instance_code, &mut geoloader.loader_10,  &mut vb_data_map, &asset_mgr,  &mut geocommands);
        init_slot::<AssetDescVBSlot11, AssetResVBSlot11>(&instancesource, entity, &geo_desc, &mut instance_code, &mut geoloader.loader_11,  &mut vb_data_map, &asset_mgr,  &mut geocommands);
        init_slot::<AssetDescVBSlot12, AssetResVBSlot12>(&instancesource, entity, &geo_desc, &mut instance_code, &mut geoloader.loader_12,  &mut vb_data_map, &asset_mgr,  &mut geocommands);

        // log::debug!(">>>>  GeometryDesc ");
        geocommands.insert(geo_desc);
        geocommands.insert(EInstanceCodeComp(instance_code));
        
        if let Some(indices_desc) = indices_desc {
            if let Some(data) = asset_mgr.get(&indices_desc.buffer) {
                geocommands.insert(AssetResBufferIndices::from(EVerticesBufferUsage::Other(data)));
            } else {
                geocommands.remove::<IndicesBufferDesc>();
                geoloader.loader_indices.request(entity, &indices_desc.buffer, None, &mut vb_data_map);
            }
            geocommands.insert(indices_desc);
        } else {
            geocommands.remove::<IndicesBufferDesc>();
            geocommands.remove::<AssetResBufferIndices>();
        }
    });
}

pub struct ActionVertexBuffer;
impl ActionVertexBuffer {
    pub fn check(
        asset_mgr: &ShareAssetMgr<EVertexBufferRange>,
        key: KeyVertexBuffer,
    ) -> bool {
        asset_mgr.contains_key(&key)
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
    pub(crate) fn create(
        cmds: &mut ActionListGeometryCreate,
        id_geo: Entity,
        id_mesh: Entity,
        vertex_desc: Vec<VertexBufferDesc>,
        indices_desc: Option<IndicesBufferDesc>,
    ) {
        cmds.push(OpsGeomeryCreate::ops(id_mesh, id_geo, vertex_desc, indices_desc));
    }
}

fn init_slot<
    D: TVertexBufferUseInfo + Component,
    D1: From<EVerticesBufferUsage> + Component,
>(
    instancesource: &InstanceSourceID,
    id_geo: ObjectID,
    geodesc: &GeometryDesc,
    instance_code: &mut EInstanceCode,
    loader: &mut VertexBufferLoader<ObjectID, D1>,
    vb_data_map: &mut SingleVertexBufferDataMap,
    asset_mgr: &ShareAssetMgr<EVertexBufferRange>,
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
                    loader.request(id_geo, &desc.key, None, vb_data_map);
                }
            },
            _ => {
    
                let buff_id = instancesource.id();
    
                match instance_kind {
                    EInstanceKind::WorldMatrix => {
                        let buff = InstanceBufferWorldMatrix { slot: slot_index, id: String::from(buff_id + "WorldMatrix"), index: 0 };
                        commands.insert(buff);
                        instance_code.0 = instance_code.0 | EInstanceCode::BASE;
                    },
                    EInstanceKind::Color => {
                        let buff = InstanceBufferColor { slot: slot_index, id: String::from(buff_id + "Color"), index: 0 };
                        commands.insert(buff);
                        // log::debug!("Instance Color");
                        instance_code.0 = instance_code.0 | EInstanceCode::COLOR;
                    },
                    EInstanceKind::TillOffset => {
                        let buff = InstanceBufferTillOff { slot: slot_index, id: String::from(buff_id + "TillOff"), index: 0 };
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

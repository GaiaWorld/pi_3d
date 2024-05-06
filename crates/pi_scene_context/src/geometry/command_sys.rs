use std::{hash::{Hash, Hasher}, sync::Arc};

use pi_scene_shell::prelude::*;

use crate::{prelude::{RenderGeometryComp, ActionListDisposeCan, ActionListDisposeReadyForRef, OpsDisposeCan, MeshInstanceState}, object::ActionEntity};

use super::{
    vertex_buffer_useinfo::*,
    base::*,
    instance::instanced_buffer::{InstancedInfo, InstanceBufferAllocator},
};

use super::command::*;

pub type SysCreateGeometryBunble = (
    AssetDescVBSlot01, 
    AssetDescVBSlot02, 
    AssetDescVBSlot03,
    AssetDescVBSlot04,
    AssetDescVBSlot05,
    AssetDescVBSlot06,
    AssetDescVBSlot07,
    AssetDescVBSlot08,
    AssetResVBSlot01,
    AssetResVBSlot02,
    AssetResVBSlot03,
    AssetResVBSlot04,
    AssetResVBSlot05,
    AssetResVBSlot06,
    AssetResVBSlot07,
    AssetResVBSlot08
);

pub fn sys_create_geometry(
    // mut commands: Commands,
    mut alter0:  Alter<(), (), (GeometryResourceHash, GeometryDesc), SysCreateGeometryBunble>,
    // mut alter1: Alter<(), (), (GeometryResourceHash,), ()>,
    mut alter2: Alter<(), (), (DisposeReady, DisposeCan), ()>,
    mut alter3: Alter<(), (), (VertexBufferLayoutsComp, MeshID, RenderGeometryComp), ()>,
    mut alter4: Alter<(), (), (IndicesBufferDesc,), ()>,
    mut alter5: Alter<(), (), (), (IndicesBufferDesc,)>,
    mut alter6: Alter<(), (), (InstancedInfo,), ()>,
    // mut alter7: Alter<(), (), (GeometryDesc,), ()>,
    mut alter8: Alter<(), (), (AssetResBufferIndices,), ()>,
    mut alter9: Alter<(), (), (), (AssetResBufferIndices,)>,
    
    mut geocommands0: Alter<(), (), (AssetResVBSlot01, ), ()>,
    mut geocommands1: Alter<(), (), (AssetDescVBSlot01, ), ()>,
    mut geocommands2: Alter<(), (), (AssetResVBSlot02, ), ()>,
    mut geocommands3: Alter<(), (), (AssetDescVBSlot02, ), ()>,
    mut geocommands4: Alter<(), (), (AssetResVBSlot03, ), ()>,
    mut geocommands5: Alter<(), (), (AssetDescVBSlot03, ), ()>,
    mut geocommands6: Alter<(), (), (AssetResVBSlot04, ), ()>,
    mut geocommands7: Alter<(), (), (AssetDescVBSlot04, ), ()>,
    mut geocommands8: Alter<(), (), (AssetResVBSlot05, ), ()>,
    mut geocommands9: Alter<(), (), (AssetDescVBSlot05, ), ()>,
    mut geocommands10: Alter<(), (), (AssetResVBSlot06, ), ()>,
    mut geocommands11: Alter<(), (), (AssetDescVBSlot06, ), ()>,
    mut geocommands12: Alter<(), (), (AssetResVBSlot07, ), ()>,
    mut geocommands13: Alter<(), (), (AssetDescVBSlot07, ), ()>,
    mut geocommands14: Alter<(), (), (AssetResVBSlot08, ), ()>,
    mut geocommands15: Alter<(), (), (AssetDescVBSlot08, ), ()>,

    mut meshes: Query<(&mut GeometryID, &MeshInstanceState)>,
    mut cmds: ResMut<ActionListGeometryCreate>,
    mut geoloader: ResMut<GeometryVBLoader>,
    mut vb_data_map: ResMut<VertexBufferDataMap3D>,
    asset_mgr: Res<ShareAssetMgr<EVertexBufferRange>>,
    mut instanceallocator: ResMut<InstanceBufferAllocator>,
    mut _disposereadylist: ResMut<ActionListDisposeReadyForRef>,
    mut disposecanlist: ResMut<ActionListDisposeCan>,
) {
    cmds.drain().drain(..).for_each(|OpsGeomeryCreate(id_mesh, entity, mut vertex_desc, indices_desc)| {
        
        let instancestate = if let Ok((mut mesh, insstate)) = meshes.get_mut(id_mesh) {
            *mesh = GeometryID(entity); insstate
        } else { disposecanlist.push(OpsDisposeCan::ops(entity)); return; };

        if !alter0.get(entity).is_ok() {
            return;
        };

        let mut attrs = vec![];
        if instancestate.instance_matrix {
            attrs.push(EVertexAttribute::Buildin(EBuildinVertexAtribute::InsWorldRow1));
            attrs.push(EVertexAttribute::Buildin(EBuildinVertexAtribute::InsWorldRow2));
            attrs.push(EVertexAttribute::Buildin(EBuildinVertexAtribute::InsWorldRow3));
            attrs.push(EVertexAttribute::Buildin(EBuildinVertexAtribute::InsWorldRow4));
        }
        instancestate.instances.iter().for_each(|attr| {
            attrs.push(EVertexAttribute::Custom(attr.clone()));
        });

        if attrs.len() > 0 {
            vertex_desc.push(VertexBufferDesc::new(KeyVertexBuffer::from(""), VertexBufferDescRange::new(0, 0), attrs, true));
        }

        // if instancestate > 0 {
        //     vertex_desc.push(VertexBufferDesc::new(KeyVertexBuffer::from(""), VertexBufferDescRange::default(), attrs, true));
        // }

        ActionGeometry::init(entity, &mut alter2,&mut alter3, &mut alter4, &mut alter5, &vertex_desc, indices_desc.clone(), id_mesh);

        let geo_desc = GeometryDesc { list: vertex_desc };
        let mut hasher = DefaultHasher::default();
        geo_desc.hash_resource(&mut hasher);
        if instancestate.use_single_instancebuffer {
            entity.hash(&mut hasher);
        }
        // alter1.alter(entity, (GeometryResourceHash(hasher.finish()), ));
    
        // geocommands
        //     .remove::<AssetDescVBSlot01>()
        //     .remove::<AssetDescVBSlot02>()
        //     .remove::<AssetDescVBSlot03>()
        //     .remove::<AssetDescVBSlot04>()
        //     .remove::<AssetDescVBSlot05>()
        //     .remove::<AssetDescVBSlot06>()
        //     .remove::<AssetDescVBSlot07>()
        //     .remove::<AssetDescVBSlot08>()
        //     // .remove::<AssetDescVBSlot09>()
        //     // .remove::<AssetDescVBSlot10>()
        //     // .remove::<AssetDescVBSlot11>()
        //     // .remove::<AssetDescVBSlot12>()
        //     // .remove::<AssetDescVBSlot13>()
        //     // .remove::<AssetDescVBSlot14>()
        //     // .remove::<AssetDescVBSlot15>()
        //     // .remove::<AssetDescVBSlot16>()
        //     .remove::<AssetResVBSlot01>()
        //     .remove::<AssetResVBSlot02>()
        //     .remove::<AssetResVBSlot03>()
        //     .remove::<AssetResVBSlot04>()
        //     .remove::<AssetResVBSlot05>()
        //     .remove::<AssetResVBSlot06>()
        //     .remove::<AssetResVBSlot07>()
        //     .remove::<AssetResVBSlot08>()
        //     ;

        let loader = &mut geoloader.loader_01;
        if let Some((desc, buff)) = init_slot(&geo_desc, &mut vb_data_map, &asset_mgr, entity, &mut alter6,  &mut instanceallocator, AssetDescVBSlot01::ASK_SLOT_COUNT as usize - 1) {
            if let Some(buff) = buff { 
                geocommands0.alter(entity, (AssetResVBSlot01::from(buff),)); 
            } else { 
                loader.request(entity, &desc.key, None, &mut vb_data_map); 
            }

            geocommands1.alter(entity,(AssetDescVBSlot01::from(desc),));
        }
        let loader = &mut geoloader.loader_02;
        if let Some((desc, buff)) = init_slot(&geo_desc, &mut vb_data_map, &asset_mgr, entity, &mut alter6,  &mut instanceallocator, AssetDescVBSlot02::ASK_SLOT_COUNT as usize - 1) {
            if let Some(buff) = buff { geocommands2.alter(entity,(AssetResVBSlot02::from(buff),)); } else { loader.request(entity, &desc.key, None, &mut vb_data_map); }
            geocommands3.alter(entity,(AssetDescVBSlot02::from(desc),));
        }
        let loader = &mut geoloader.loader_03;
        if let Some((desc, buff)) = init_slot(&geo_desc, &mut vb_data_map, &asset_mgr, entity, &mut alter6,  &mut instanceallocator, AssetDescVBSlot03::ASK_SLOT_COUNT as usize - 1) {
            if let Some(buff) = buff { geocommands4.alter(entity,(AssetResVBSlot03::from(buff),)); } else { loader.request(entity, &desc.key, None, &mut vb_data_map); }
            geocommands5.alter(entity,(AssetDescVBSlot03::from(desc),));
        }
        let loader = &mut geoloader.loader_04;
        if let Some((desc, buff)) = init_slot(&geo_desc, &mut vb_data_map, &asset_mgr, entity, &mut alter6,  &mut instanceallocator, AssetDescVBSlot04::ASK_SLOT_COUNT as usize - 1) {
            if let Some(buff) = buff { geocommands6.alter(entity,(AssetResVBSlot04::from(buff),)); } else { loader.request(entity, &desc.key, None, &mut vb_data_map); }
            geocommands7.alter(entity,(AssetDescVBSlot04::from(desc),));
        }
        let loader = &mut geoloader.loader_05;
        if let Some((desc, buff)) = init_slot(&geo_desc, &mut vb_data_map, &asset_mgr, entity, &mut alter6,  &mut instanceallocator, AssetDescVBSlot05::ASK_SLOT_COUNT as usize - 1) {
            if let Some(buff) = buff { geocommands8.alter(entity,(AssetResVBSlot05::from(buff),)); } else { loader.request(entity, &desc.key, None, &mut vb_data_map); }
            geocommands9.alter(entity,(AssetDescVBSlot05::from(desc),));
        }
        let loader = &mut geoloader.loader_06;
        if let Some((desc, buff)) = init_slot(&geo_desc, &mut vb_data_map, &asset_mgr, entity, &mut alter6,  &mut instanceallocator, AssetDescVBSlot06::ASK_SLOT_COUNT as usize - 1) {
            if let Some(buff) = buff { geocommands10.alter(entity,(AssetResVBSlot06::from(buff),)); } else { loader.request(entity, &desc.key, None, &mut vb_data_map); }
            geocommands11.alter(entity,(AssetDescVBSlot06::from(desc),));
        }
        let loader = &mut geoloader.loader_07;
        if let Some((desc, buff)) = init_slot(&geo_desc, &mut vb_data_map, &asset_mgr, entity, &mut alter6,  &mut instanceallocator, AssetDescVBSlot07::ASK_SLOT_COUNT as usize - 1) {
            if let Some(buff) = buff { geocommands12.alter(entity,(AssetResVBSlot07::from(buff),)); } else { loader.request(entity, &desc.key, None, &mut vb_data_map); }
            geocommands13.alter(entity,(AssetDescVBSlot07::from(desc),));
        }
        let loader = &mut geoloader.loader_08;
        if let Some((desc, buff)) = init_slot(&geo_desc, &mut vb_data_map, &asset_mgr, entity, &mut alter6,  &mut instanceallocator, AssetDescVBSlot08::ASK_SLOT_COUNT as usize - 1) {
            if let Some(buff) = buff { geocommands14.alter(entity,(AssetResVBSlot08::from(buff),)); } else { loader.request(entity, &desc.key, None, &mut vb_data_map); }
            geocommands15.alter(entity,(AssetDescVBSlot08::from(desc),));
        }
        alter0.alter(entity, (GeometryResourceHash(hasher.finish()), geo_desc));
        // init_slot::<AssetDescVBSlot01, AssetResVBSlot01>(entity, &geo_desc, &mut geoloader.loader_01,  &mut vb_data_map, &asset_mgr, &mut geocommands,   &mut instanceallocator);
        // init_slot::<AssetDescVBSlot02, AssetResVBSlot02>(entity, &geo_desc, &mut geoloader.loader_02,  &mut vb_data_map, &asset_mgr,  &mut geocommands,  &mut instanceallocator);
        // init_slot::<AssetDescVBSlot03, AssetResVBSlot03>(entity, &geo_desc, &mut geoloader.loader_03,  &mut vb_data_map, &asset_mgr,  &mut geocommands,  &mut instanceallocator);
        // init_slot::<AssetDescVBSlot04, AssetResVBSlot04>(entity, &geo_desc, &mut geoloader.loader_04,  &mut vb_data_map, &asset_mgr,  &mut geocommands,  &mut instanceallocator);
        // init_slot::<AssetDescVBSlot05, AssetResVBSlot05>(entity, &geo_desc, &mut geoloader.loader_05,  &mut vb_data_map, &asset_mgr,  &mut geocommands,  &mut instanceallocator);
        // init_slot::<AssetDescVBSlot06, AssetResVBSlot06>(entity, &geo_desc, &mut geoloader.loader_06,  &mut vb_data_map, &asset_mgr,  &mut geocommands,  &mut instanceallocator);
        // init_slot::<AssetDescVBSlot07, AssetResVBSlot07>(entity, &geo_desc, &mut geoloader.loader_07,  &mut vb_data_map, &asset_mgr,  &mut geocommands,  &mut instanceallocator);
        // init_slot::<AssetDescVBSlot08, AssetResVBSlot08>(entity, &geo_desc, &mut geoloader.loader_08,  &mut vb_data_map, &asset_mgr,  &mut geocommands,  &mut instanceallocator);

        // init_slot::<AssetDescVBSlot09, AssetResVBSlot09>(entity, &geo_desc, &mut geoloader.loader_09,  &mut vb_data_map, &asset_mgr,  &mut geocommands,  &mut instanceallocator);
        // init_slot::<AssetDescVBSlot10, AssetResVBSlot10>(entity, &geo_desc, &mut geoloader.loader_10,  &mut vb_data_map, &asset_mgr,  &mut geocommands,  &mut instanceallocator);
        // init_slot::<AssetDescVBSlot11, AssetResVBSlot11>(entity, &geo_desc, &mut geoloader.loader_11,  &mut vb_data_map, &asset_mgr,  &mut geocommands,  &mut instanceallocator);
        // init_slot::<AssetDescVBSlot12, AssetResVBSlot12>(entity, &geo_desc, &mut geoloader.loader_12,  &mut vb_data_map, &asset_mgr,  &mut geocommands,  &mut instanceallocator);

        // log::error!(">>>>  GeometryDesc ");
        // alter7.alter(entity, (geo_desc,));
        // let instance_code = EVerticeExtendCode(EVerticeExtendCode::NONE + instancestate);
        // geocommands.insert(EVerticeExtendCodeComp(instance_code));
        
        if let Some(indices_desc) = indices_desc {
            if let Some(data) = asset_mgr.get(&indices_desc.buffer.asset_u64()) {
                // log::warn!("Indice Buffer Ok!");
                alter8.alter(entity,(AssetResBufferIndices::from(EVerticesBufferUsage::Other(data)),));
            } else {
                geoloader.loader_indices.request(entity, &indices_desc.buffer, None, &mut vb_data_map);
            }
        } else {
            alter9.alter(entity, ());
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
        entity: Entity,  
        alter0: &mut Alter<(), (), (DisposeReady, DisposeCan)>,
        alter1: &mut Alter<(), (), (VertexBufferLayoutsComp, MeshID, RenderGeometryComp)>,
        alter2: &mut Alter<(), (), (IndicesBufferDesc,)>,
        alter3: &mut Alter<(), (), (), (IndicesBufferDesc,)>,
        // cmds: &mut EntityCommands,
        vertex_desc: &Vec<VertexBufferDesc>,
        indices_desc: Option<IndicesBufferDesc>,
        id_mesh: Entity,
    ) {
        // log::warn!("{:?}", vertex_desc);
        ActionEntity::init(entity, alter0);
        let _ = alter1.alter(entity, 
            (
                VertexBufferLayoutsComp(VertexBufferLayouts::from(vertex_desc), KeyShaderFromAttributes::new(vertex_desc)),
                MeshID(id_mesh),
                RenderGeometryComp::default()
            ));
        if let Some(indices_desc) = indices_desc {
            alter2.alter(entity, (indices_desc,));
        } else {
            alter3.alter(entity, ());
        }
    }
}

fn init_slot
// <
//     D: TVertexBufferUseInfo + 
//     D1: From<EVerticesBufferUsage> + 
// >
(
    // id_geo: ObjectID,
    geodesc: &GeometryDesc,
    // loader: &mut VertexBufferLoader<ObjectID, D1>,
    vb_data_map: &mut SingleVertexBufferDataMap,
    asset_mgr: &ShareAssetMgr<EVertexBufferRange>,
    entity: Entity,  
    alter0: &mut Alter<(), (), (InstancedInfo, )>,
    // commands: &mut EntityCommands,
    instanceallocator: &mut InstanceBufferAllocator,
    // instancestate: u32,
    slot_index: usize,
) -> Option<(VertexBufferDesc, Option<EVerticesBufferUsage>)> {
    // let slot_index = D::ASK_SLOT_COUNT as usize - 1;
    if slot_index < geodesc.list.len() {
        let mut buffer = None;
        let desc: VertexBufferDesc = geodesc.get_desc(slot_index);
        if desc.instance() == false {
            if let Some(data) = asset_mgr.get(&desc.key.asset_u64()) {
                // log::warn!("Vertex Buffer Ok!");
                buffer = Some(EVerticesBufferUsage::Other(data));
                // commands.insert(D1::from());
            // } else {
            //     loader.request(id_geo, &desc.key, None, vb_data_map);
            }
        } else {
            let info = InstancedInfo::new(desc.stride() as u32, EVertexBufferSlot::from_u8_unsafe(slot_index as u8));
            // log::warn!("Geometry Instance: {:?}", EVertexBufferSlot::from_u8_unsafe(slot_index as u8));
            let data = instanceallocator.instance_initial_buffer();
            buffer = Some(EVerticesBufferUsage::EVBRange(Arc::new(EVertexBufferRange::NotUpdatable(data.0, data.1, data.2))));
            // commands.insert(D1::from(EVerticesBufferUsage::EVBRange(Arc::new(EVertexBufferRange::NotUpdatable(data.0, data.1, data.2)))));
            alter0.alter(entity, (info,));
        }
        // commands.insert(D::from(desc));
        Some((desc, buffer))
    } else {
        None
    }
}

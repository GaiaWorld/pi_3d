use std::{hash::{Hash, Hasher}, sync::Arc};

use pi_scene_shell::prelude::*;

use crate::{prelude::{RenderGeometryComp, ActionListDisposeCan, ActionListDisposeReadyForRef, OpsDisposeCan, MeshInstanceState}, object::ActionEntity};

use super::{
    base::*, instance::instanced_buffer::{InstanceBufferAllocator, InstancedInfo, InstancedInfoComp}, vertex_buffer_useinfo::*
};

use super::command::*;

pub type BundleGeometry = (
    (DisposeReady, DisposeCan),
    GeometryDesc,
    (VertexBufferLayoutsComp, MeshID, RenderGeometryComp, IndicesBufferDescComp, AssetKeyBufferIndices, AssetDescVBSlots, LoadedKeyVBSlots, AssetResVBSlots),
    AssetResBufferIndicesComp, InstancedInfoComp, GeometryResourceHash
);

pub fn sys_create_geometry(
    mut commands: Commands,
    mut meshes: Query<(&mut GeometryID, &MeshInstanceState)>,
    mut cmds: ResMut<ActionListGeometryCreate>,
    mut geoloader: ResMut<GeometryVBLoader>,
    mut vb_data_map: ResMut<VertexBufferDataMap3D>,
    asset_mgr: Res<ShareAssetMgr<EVertexBufferRange>>,
    mut instanceallocator: ResMut<InstanceBufferAllocator>,
    mut _disposereadylist: ResMut<ActionListDisposeReadyForRef>,
    mut disposecanlist: ResMut<ActionListDisposeCan>,
    // mut cmdgeo: Alter<(), (), BundleGeometry, ()>,
) {
    cmds.drain().drain(..).for_each(|OpsGeomeryCreate(id_mesh, entity, mut vertex_desc, indices_desc)| {
        
        let instancestate = if let Ok((mut mesh, insstate)) = meshes.get_mut(id_mesh) {
            *mesh = GeometryID(entity); insstate
        } else { disposecanlist.push(OpsDisposeCan::ops(entity)); return; };

        let mut geocommands = if let Some(cmd) = commands.get_entity(entity) {
            cmd
        } else { return; };

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

        let (comp1, comp2, comp3, comp4, comp5, mut comp6) = ActionGeometry::init(&vertex_desc, indices_desc.clone(), id_mesh);

        let geo_desc = GeometryDesc { list: vertex_desc };
        let mut hasher = DefaultHasher::default();
        geo_desc.hash_resource(&mut hasher);
        if instancestate.use_single_instancebuffer {
            entity.hash(&mut hasher);
        }

        let mut desclist = AssetDescVBSlots::default();
        let mut keyslist = LoadedKeyVBSlots::default();
        let mut datalist = AssetResVBSlots::default();
        let mut instacned = InstancedInfoComp(None);

        let loader = &mut geoloader.loader_01;
        for slot in 0..VB_SLOTS_COUNT {
            if let Some((desc, buff)) = init_slot(&geo_desc, &asset_mgr, &mut instanceallocator, &mut instacned, slot) {
                if let Some(buff) = buff {
                    datalist[slot] = Some(AssetResVBSlot::from(buff));
                    keyslist[slot] = Some(desc.key.clone());
                } else {
                    loader.request((entity, slot as u8), &desc.key, None, &mut vb_data_map);
                }
                desclist[slot] = Some(AssetDescVBSlot::from(desc));
            }
        }

        let mut indicesres = AssetResBufferIndicesComp(None);
        if let Some(indices_desc) = indices_desc {
            if let Some(data) = asset_mgr.get(&indices_desc.buffer.asset_u64()) {
                indicesres.0 = Some(AssetResBufferIndices::from(EVerticesBufferUsage::Other(data)));
                comp6.0 = Some(indices_desc.buffer.clone());
            } else {
                geoloader.loader_indices.request(entity, &indices_desc.buffer, None, &mut vb_data_map);
            }
        }

        let bundle = (
            comp1,
            geo_desc,
            (comp2, comp3, comp4, comp5, comp6, desclist, keyslist, datalist),
            indicesres,
            instacned,
            GeometryResourceHash(hasher.finish()),
        );
        geocommands.insert(bundle);
        // cmdgeo.alter(entity, bundle);
        // log::warn!("sys_create_geometry");
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

pub type GeometryInitBundle = (
    BundleEntity,
    VertexBufferLayoutsComp,
    MeshID,
    RenderGeometryComp,
    IndicesBufferDescComp,
    AssetKeyBufferIndices,
);

pub struct ActionGeometry;
impl ActionGeometry {
    pub fn init(
        vertex_desc: &Vec<VertexBufferDesc>,
        indices_desc: Option<IndicesBufferDesc>,
        id_mesh: Entity,
    ) -> GeometryInitBundle {
        (
            ActionEntity::init(),
            VertexBufferLayoutsComp(VertexBufferLayouts::from(vertex_desc), KeyShaderFromAttributes::new(vertex_desc)),
            MeshID(id_mesh),
            RenderGeometryComp::default(),
            IndicesBufferDescComp(indices_desc),
            AssetKeyBufferIndices(None),
        )
    }
}

fn init_slot
(
    geodesc: &GeometryDesc,
    asset_mgr: &ShareAssetMgr<EVertexBufferRange>,
    instanceallocator: &mut InstanceBufferAllocator,
    instancecomp: &mut InstancedInfoComp,
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
            }
        } else {
            let info = InstancedInfo::new(desc.stride() as u32, EVertexBufferSlot::from_u8_unsafe(slot_index as u8));
            // log::warn!("Geometry Instance: {:?}", EVertexBufferSlot::from_u8_unsafe(slot_index as u8));
            let data = instanceallocator.instance_initial_buffer();
            buffer = Some(EVerticesBufferUsage::EVBRange(Arc::new(EVertexBufferRange::NotUpdatable(data.0, data.1, data.2))));
            instancecomp.0 = Some(info);
        }
        Some((desc, buffer))
    } else {
        None
    }
}

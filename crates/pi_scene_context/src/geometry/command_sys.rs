use std::{hash::{Hash, Hasher}, sync::Arc};

use pi_render::components;
use pi_scene_shell::{add_component, prelude::{pi_world::editor::{self, EntityEditor}, *}};

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
    mut editor: EntityEditor,
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

        if !editor.contains_entity(entity) {
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

        ActionGeometry::init(entity, &mut editor, &vertex_desc, indices_desc.clone(), id_mesh);

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

        let mut index = editor.init_component::<AssetResVBSlot01>();
        let loader = &mut geoloader.loader_01;
        if let Some((desc, buff)) = init_slot(&geo_desc, &mut vb_data_map, &asset_mgr, entity, &mut editor,  &mut instanceallocator, AssetDescVBSlot01::ASK_SLOT_COUNT as usize - 1) {
            if let Some(buff) = buff {
                // let index = editor.init_component::<AssetResVBSlot01>();
                editor.add_components(entity, &[index]);
                *editor.get_component_unchecked_mut_by_id(entity, index) = AssetResVBSlot01::from(buff); 
            } else { 
                loader.request(entity, &desc.key, None, &mut vb_data_map); 
            }
            index = editor.init_component::<AssetDescVBSlot01>();
            editor.add_components(entity, &[index]);
            *editor.get_component_unchecked_mut_by_id(entity, index) = AssetDescVBSlot01::from(desc); 

            // editor.add_components(entity,(AssetDescVBSlot01::from(desc),));
        }
        let loader = &mut geoloader.loader_02;
        if let Some((desc, buff)) = init_slot(&geo_desc, &mut vb_data_map, &asset_mgr, entity, &mut editor,  &mut instanceallocator, AssetDescVBSlot02::ASK_SLOT_COUNT as usize - 1) {
            if let Some(buff) = buff { index = editor.init_component::<AssetResVBSlot02>(); editor.add_components(entity, &[index]); *editor.get_component_unchecked_mut_by_id(entity, index) = AssetResVBSlot02::from(buff); } else { loader.request(entity, &desc.key, None, &mut vb_data_map); }
            
            index = editor.init_component::<AssetDescVBSlot02>();
            editor.add_components(entity, &[index]);
            *editor.get_component_unchecked_mut_by_id(entity, index) = AssetDescVBSlot02::from(desc); 

            // editor.add_components(entity,(AssetDescVBSlot02::from(desc),));
        }
        let loader = &mut geoloader.loader_03;
        if let Some((desc, buff)) = init_slot(&geo_desc, &mut vb_data_map, &asset_mgr, entity, &mut editor,  &mut instanceallocator, AssetDescVBSlot03::ASK_SLOT_COUNT as usize - 1) {
            if let Some(buff) = buff { index = editor.init_component::<AssetDescVBSlot03>(); editor.add_components(entity, &[index]); *editor.get_component_unchecked_mut_by_id(entity, index) = AssetResVBSlot03::from(buff); } else { loader.request(entity, &desc.key, None, &mut vb_data_map); }
            // editor.add_components(entity,(AssetDescVBSlot03::from(desc),));
            index = editor.init_component::<AssetDescVBSlot03>();
            editor.add_components(entity, &[index]);
            *editor.get_component_unchecked_mut_by_id(entity, index) = AssetDescVBSlot03::from(desc); 
        }
        let loader = &mut geoloader.loader_04;
        if let Some((desc, buff)) = init_slot(&geo_desc, &mut vb_data_map, &asset_mgr, entity, &mut editor,  &mut instanceallocator, AssetDescVBSlot04::ASK_SLOT_COUNT as usize - 1) {
            if let Some(buff) = buff { index = editor.init_component::<AssetDescVBSlot03>(); editor.add_components(entity, &[index]); *editor.get_component_unchecked_mut_by_id(entity, index) = AssetResVBSlot04::from(buff); } else { loader.request(entity, &desc.key, None, &mut vb_data_map); }
            // editor.add_components(entity,(AssetDescVBSlot04::from(desc),));
            index = editor.init_component::<AssetDescVBSlot04>();
            editor.add_components(entity, &[index]);
            *editor.get_component_unchecked_mut_by_id(entity, index) = AssetDescVBSlot04::from(desc); 
        }
        let loader = &mut geoloader.loader_05;
        if let Some((desc, buff)) = init_slot(&geo_desc, &mut vb_data_map, &asset_mgr, entity, &mut editor,  &mut instanceallocator, AssetDescVBSlot05::ASK_SLOT_COUNT as usize - 1) {
            if let Some(buff) = buff { index = editor.init_component::<AssetDescVBSlot03>(); editor.add_components(entity, &[index]); *editor.get_component_unchecked_mut_by_id(entity, index) = AssetResVBSlot05::from(buff); } else { loader.request(entity, &desc.key, None, &mut vb_data_map); }
            index = editor.init_component::<AssetDescVBSlot05>();
            editor.add_components(entity, &[index]);
            *editor.get_component_unchecked_mut_by_id(entity, index) = AssetDescVBSlot05::from(desc); 
            // editor.add_components(entity,(AssetDescVBSlot05::from(desc),));
        }
        let loader = &mut geoloader.loader_06;
        if let Some((desc, buff)) = init_slot(&geo_desc, &mut vb_data_map, &asset_mgr, entity, &mut editor,  &mut instanceallocator, AssetDescVBSlot06::ASK_SLOT_COUNT as usize - 1) {
            if let Some(buff) = buff { index = editor.init_component::<AssetDescVBSlot03>(); editor.add_components(entity, &[index]); *editor.get_component_unchecked_mut_by_id(entity, index) = AssetResVBSlot06::from(buff); } else { loader.request(entity, &desc.key, None, &mut vb_data_map); }
            index = editor.init_component::<AssetDescVBSlot06>();
            editor.add_components(entity, &[index]);
            *editor.get_component_unchecked_mut_by_id(entity, index) = AssetDescVBSlot06::from(desc); 
            // editor.add_components(entity,(AssetDescVBSlot06::from(desc),));
        }
        let loader = &mut geoloader.loader_07;
        if let Some((desc, buff)) = init_slot(&geo_desc, &mut vb_data_map, &asset_mgr, entity, &mut editor,  &mut instanceallocator, AssetDescVBSlot07::ASK_SLOT_COUNT as usize - 1) {
            if let Some(buff) = buff { index = editor.init_component::<AssetDescVBSlot03>(); editor.add_components(entity, &[index]); *editor.get_component_unchecked_mut_by_id(entity, index) = AssetResVBSlot07::from(buff); } else { loader.request(entity, &desc.key, None, &mut vb_data_map); }
            // editor.add_components(entity,(AssetDescVBSlot07::from(desc),));
            index = editor.init_component::<AssetDescVBSlot07>();
            editor.add_components(entity, &[index]);
            *editor.get_component_unchecked_mut_by_id(entity, index) = AssetDescVBSlot07::from(desc); 
        }
        let loader = &mut geoloader.loader_08;
        if let Some((desc, buff)) = init_slot(&geo_desc, &mut vb_data_map, &asset_mgr, entity, &mut editor,  &mut instanceallocator, AssetDescVBSlot08::ASK_SLOT_COUNT as usize - 1) {
            if let Some(buff) = buff { index = editor.init_component::<AssetDescVBSlot03>(); editor.add_components(entity, &[index]); *editor.get_component_unchecked_mut_by_id(entity, index) = AssetResVBSlot08::from(buff); } else { loader.request(entity, &desc.key, None, &mut vb_data_map); }
            // editor.add_components(entity,(AssetDescVBSlot08::from(desc),));
            index = editor.init_component::<AssetDescVBSlot08>();
            editor.add_components(entity, &[index]);
            *editor.get_component_unchecked_mut_by_id(entity, index) = AssetDescVBSlot08::from(desc); 
        }
        // editor.add_components(entity, (GeometryResourceHash(hasher.finish()), geo_desc));

        let indexs = [editor.init_component::<GeometryResourceHash>(), editor.init_component::<GeometryDesc>()];
        editor.add_components(entity, &indexs);
        *editor.get_component_unchecked_mut_by_id(entity, indexs[0]) = GeometryResourceHash(hasher.finish()); 
        *editor.get_component_unchecked_mut_by_id(entity, indexs[1]) = geo_desc; 
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
                add_component(&mut editor, entity,AssetResBufferIndices::from(EVerticesBufferUsage::Other(data)));
            } else {
                geoloader.loader_indices.request(entity, &indices_desc.buffer, None, &mut vb_data_map);
            }
        } else {
            editor.destroy(entity);
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
        editor: &mut EntityEditor,
        vertex_desc: &Vec<VertexBufferDesc>,
        indices_desc: Option<IndicesBufferDesc>,
        id_mesh: Entity,
    ) {
        // log::warn!("{:?}", vertex_desc);
        ActionEntity::init(entity, editor);
        let mut components = vec![
            editor.init_component::<VertexBufferLayoutsComp>(),
            editor.init_component::<MeshID>(),
            editor.init_component::<RenderGeometryComp>(),
        ];
        let _ = editor.add_components(entity, &components);

        *editor.get_component_unchecked_mut_by_id(entity, components[0]) = VertexBufferLayoutsComp(VertexBufferLayouts::from(vertex_desc), KeyShaderFromAttributes::new(vertex_desc));
        *editor.get_component_unchecked_mut_by_id(entity, components[1]) = MeshID(id_mesh);
        *editor.get_component_unchecked_mut_by_id(entity, components[2]) = RenderGeometryComp::default();
   
        let index = editor.init_component::<IndicesBufferDesc>();
        if let Some(indices_desc) = indices_desc {
            // let _ = editor.add_components(entity, &[index]);
            add_component(editor, entity, indices_desc).unwrap();
            // *editor.get_component_unchecked_mut_by_id(entity, index) = indices_desc;
        } else {
            editor.remove_components(entity, &[index]);
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
    editor: &mut EntityEditor,
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
            add_component(editor, entity, info).unwrap();
        }
        // commands.insert(D::from(desc));
        Some((desc, buffer))
    } else {
        None
    }
}

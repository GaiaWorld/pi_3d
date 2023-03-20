use std::mem::replace;

use pi_assets::{mgr::AssetMgr, asset::GarbageEmpty};
use pi_atom::Atom;
use pi_ecs::{prelude::{ResMut, Setup, Commands, Query, EntityDelete, EntityCommands, Res, Component}};
use pi_ecs_macros::setup;
use pi_engine_shell::{engine_shell::EnginShell, object::GameObject, assets::sync_load::{AssetSyncWait, PluginAssetSyncLoad, InterfaceAssetSyncCreate}, run_stage::{TSystemStageInfo, ERunStageChap}};
use pi_render::{
    renderer::{
        vertex_buffer_loader::{VertexBufferLoader, SingleVertexBufferDataMap},
        vertex_buffer_desc::VertexBufferDesc,
        vertex_buffer::{VertexBufferLayouts, KeyVertexBuffer, VertexBufferAllocator, EVertexBufferRange},
        indices::{AssetKeyBufferIndices, AssetResBufferIndices, IndicesBufferDesc}, vertices::EVerticesBufferUsage, instance::EInstanceKind
    },
    render_3d::shader::instance_code::EInstanceCode,
    rhi::{RenderQueue, device::RenderDevice}
};
use pi_share::Share;
///
/// 网格信息单独与 GameObject 绑定

use crate::{object::ObjectID, plugin::Plugin, meshes::command::SysMeshCreateCommand};

use self::{
    vertex_buffer_useinfo::*,
    sys_vertex_buffer_use::{PluginVertexBuffers},
    geometry::RenderGeometryEable, load::SysVertexBufferLoad, instance::{InstanceSourceRecord, instance_world_matrix::InstancedBufferWorldMatrix, instance_color::InstancedBufferColor, instance_tilloff::InstancedBufferTillOff}
};

pub mod vertex_buffer_useinfo;
pub mod sys_vertex_buffer_use;
pub mod geometry;
pub mod indices;
pub mod instance;
pub mod load;

pub type VDK = usize;
pub type GBID = Atom;

pub struct GeometryDesc {
    pub list: Vec<VertexBufferDesc>,
}
impl GeometryDesc {
    pub fn slot_count(&self) -> usize {
        self.list.len()
    }
    pub fn get_desc(&self, slot: usize) -> VertexBufferDesc {
        self.list.get(slot).unwrap().clone()
    }
}

#[derive(Debug)]
pub enum ECommand {
    Desc(ObjectID, Vec<VertexBufferDesc>, Option<IndicesBufferDesc>)
}
#[derive(Debug, Default)]
pub struct SingleGeometryVBCommands(pub Vec<ECommand>);

pub struct SysGeometryVBCommand;
impl TSystemStageInfo for SysGeometryVBCommand {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysMeshCreateCommand::key()
        ]
    }
}
#[setup]
impl SysGeometryVBCommand {
    #[system]
    fn cmd(
        mut cmds: ResMut<SingleGeometryVBCommands>,
        meshes: Query<GameObject, &GeometryID>,
        mut id_geo_cmd: Commands<GameObject, GeometryID>,
        mut id_mesh_cmd: Commands<GameObject, MeshID>,
        mut geo_desc_cmd: Commands<GameObject, GeometryDesc>,
        mut geo_vb_cmd: Commands<GameObject, VertexBufferLayouts>,
        mut geo_enable_cmd: Commands<GameObject, RenderGeometryEable>,

        mut indices_desc_cmd: Commands<GameObject, IndicesBufferDesc>,
        mut indices_res_cmd: Commands<GameObject, AssetResBufferIndices>,
        mut loader_indices: ResMut<VertexBufferLoader<ObjectID, AssetResBufferIndices>>,
        
        mut vb_data_map: ResMut<SingleVertexBufferDataMap>,
        asset_mgr: Res<Share<AssetMgr<EVertexBufferRange>>>,
        mut instance_source_record: ResMut<InstanceSourceRecord>,
        mut ins_wm_cmd: Commands<GameObject, InstancedBufferWorldMatrix>,
        mut ins_color_cmd: Commands<GameObject, InstancedBufferColor>,
        mut ins_tilloff_cmd: Commands<GameObject, InstancedBufferTillOff>,
        mut inscode_cmd: Commands<GameObject, EInstanceCode>,

        mut key01_cmd: Commands<GameObject, AssetDescVBSlot01>,
        mut key02_cmd: Commands<GameObject, AssetDescVBSlot02>,
        mut key03_cmd: Commands<GameObject, AssetDescVBSlot03>,
        mut key04_cmd: Commands<GameObject, AssetDescVBSlot04>,
        mut key05_cmd: Commands<GameObject, AssetDescVBSlot05>,
        mut key06_cmd: Commands<GameObject, AssetDescVBSlot06>,
        mut key07_cmd: Commands<GameObject, AssetDescVBSlot07>,
        mut key08_cmd: Commands<GameObject, AssetDescVBSlot08>,
        mut key09_cmd: Commands<GameObject, AssetDescVBSlot09>,
        mut key10_cmd: Commands<GameObject, AssetDescVBSlot10>,
        mut key11_cmd: Commands<GameObject, AssetDescVBSlot11>,
        mut key12_cmd: Commands<GameObject, AssetDescVBSlot12>,
        // mut key13_cmd: Commands<GameObject, AssetDescVBSlot13>,
        // mut key14_cmd: Commands<GameObject, AssetDescVBSlot14>,
        // mut key15_cmd: Commands<GameObject, AssetDescVBSlot15>,
        // mut key16_cmd: Commands<GameObject, AssetDescVBSlot16>,

        mut vb01_cmd: Commands<GameObject, AssetResVBSlot01>,
        mut vb02_cmd: Commands<GameObject, AssetResVBSlot02>,
        mut vb03_cmd: Commands<GameObject, AssetResVBSlot03>,
        mut vb04_cmd: Commands<GameObject, AssetResVBSlot04>,
        mut vb05_cmd: Commands<GameObject, AssetResVBSlot05>,
        mut vb06_cmd: Commands<GameObject, AssetResVBSlot06>,
        mut vb07_cmd: Commands<GameObject, AssetResVBSlot07>,
        mut vb08_cmd: Commands<GameObject, AssetResVBSlot08>,
        mut vb09_cmd: Commands<GameObject, AssetResVBSlot09>,
        mut vb10_cmd: Commands<GameObject, AssetResVBSlot10>,
        mut vb11_cmd: Commands<GameObject, AssetResVBSlot11>,
        mut vb12_cmd: Commands<GameObject, AssetResVBSlot12>,
        // mut vb13_cmd: Commands<GameObject, AssetResVBSlot13>,
        // mut vb14_cmd: Commands<GameObject, AssetResVBSlot14>,
        // mut vb15_cmd: Commands<GameObject, AssetResVBSlot15>,
        // mut vb16_cmd: Commands<GameObject, AssetResVBSlot16>,
        
        mut loader_01: ResMut<VertexBufferLoader<ObjectID, AssetResVBSlot01>>,
        mut loader_02: ResMut<VertexBufferLoader<ObjectID, AssetResVBSlot02>>,
        mut loader_03: ResMut<VertexBufferLoader<ObjectID, AssetResVBSlot03>>,
        mut loader_04: ResMut<VertexBufferLoader<ObjectID, AssetResVBSlot04>>,
        mut loader_05: ResMut<VertexBufferLoader<ObjectID, AssetResVBSlot05>>,
        mut loader_06: ResMut<VertexBufferLoader<ObjectID, AssetResVBSlot06>>,
        mut loader_07: ResMut<VertexBufferLoader<ObjectID, AssetResVBSlot07>>,
        mut loader_08: ResMut<VertexBufferLoader<ObjectID, AssetResVBSlot08>>,
        mut loader_09: ResMut<VertexBufferLoader<ObjectID, AssetResVBSlot09>>,
        mut loader_10: ResMut<VertexBufferLoader<ObjectID, AssetResVBSlot10>>,
        mut loader_11: ResMut<VertexBufferLoader<ObjectID, AssetResVBSlot11>>,
        mut loader_12: ResMut<VertexBufferLoader<ObjectID, AssetResVBSlot12>>,
        // mut loader_13: ResMut<VertexBufferLoader<ObjectID, AssetResVBSlot13>>,
        // mut loader_14: ResMut<VertexBufferLoader<ObjectID, AssetResVBSlot14>>,
        // mut loader_15: ResMut<VertexBufferLoader<ObjectID, AssetResVBSlot15>>,
        // mut loader_16: ResMut<VertexBufferLoader<ObjectID, AssetResVBSlot16>>,
    ) {
        let mut list = replace(&mut cmds.0, vec![]);

        list.drain(..).for_each(|cmd| {
            match cmd {
                ECommand::Desc(entity, descs, indices_desc) => {
                    geo_enable_cmd.insert(entity, RenderGeometryEable(false));
                    if let Some(geo) = meshes.get(entity) {

                        let id_geo = geo.0.clone();

                        id_geo_cmd.insert(entity, GeometryID(id_geo));
    
                        id_mesh_cmd.insert(id_geo, MeshID(entity));
                        geo_vb_cmd.insert(id_geo, VertexBufferLayouts::from(&descs));
                        let geo_desc = GeometryDesc { list: descs };
                    
                        vb01_cmd.delete(id_geo);
                        vb02_cmd.delete(id_geo);
                        vb03_cmd.delete(id_geo);
                        vb04_cmd.delete(id_geo);
                        vb05_cmd.delete(id_geo);
                        vb06_cmd.delete(id_geo);
                        vb07_cmd.delete(id_geo);
                        vb08_cmd.delete(id_geo);
                        vb09_cmd.delete(id_geo);
                        vb10_cmd.delete(id_geo);
                        vb11_cmd.delete(id_geo);
                        vb12_cmd.delete(id_geo);
                        // vb13_cmd.delete(id_geo);
                        // vb14_cmd.delete(id_geo);
                        // vb15_cmd.delete(id_geo);
                        // vb16_cmd.delete(id_geo);
                        
                        let mut instance_code = EInstanceCode(EInstanceCode::NONE);
                        init_slot::<AssetDescVBSlot01, AssetResVBSlot01>(id_geo, &geo_desc, &mut instance_code, &mut key01_cmd, &mut vb01_cmd, &mut loader_01, &mut ins_wm_cmd, &mut ins_color_cmd, &mut ins_tilloff_cmd, &mut vb_data_map, &asset_mgr, &mut instance_source_record);                        init_slot::<AssetDescVBSlot01, AssetResVBSlot01>(id_geo, &geo_desc, &mut instance_code, &mut key01_cmd, &mut vb01_cmd, &mut loader_01, &mut ins_wm_cmd, &mut ins_color_cmd, &mut ins_tilloff_cmd, &mut vb_data_map, &asset_mgr, &mut instance_source_record);
                        init_slot::<AssetDescVBSlot02, AssetResVBSlot02>(id_geo, &geo_desc, &mut instance_code, &mut key02_cmd, &mut vb02_cmd, &mut loader_02, &mut ins_wm_cmd, &mut ins_color_cmd, &mut ins_tilloff_cmd, &mut vb_data_map, &asset_mgr, &mut instance_source_record);
                        init_slot::<AssetDescVBSlot03, AssetResVBSlot03>(id_geo, &geo_desc, &mut instance_code, &mut key03_cmd, &mut vb03_cmd, &mut loader_03, &mut ins_wm_cmd, &mut ins_color_cmd, &mut ins_tilloff_cmd, &mut vb_data_map, &asset_mgr, &mut instance_source_record);
                        init_slot::<AssetDescVBSlot04, AssetResVBSlot04>(id_geo, &geo_desc, &mut instance_code, &mut key04_cmd, &mut vb04_cmd, &mut loader_04, &mut ins_wm_cmd, &mut ins_color_cmd, &mut ins_tilloff_cmd, &mut vb_data_map, &asset_mgr, &mut instance_source_record);
                        init_slot::<AssetDescVBSlot05, AssetResVBSlot05>(id_geo, &geo_desc, &mut instance_code, &mut key05_cmd, &mut vb05_cmd, &mut loader_05, &mut ins_wm_cmd, &mut ins_color_cmd, &mut ins_tilloff_cmd, &mut vb_data_map, &asset_mgr, &mut instance_source_record);
                        init_slot::<AssetDescVBSlot06, AssetResVBSlot06>(id_geo, &geo_desc, &mut instance_code, &mut key06_cmd, &mut vb06_cmd, &mut loader_06, &mut ins_wm_cmd, &mut ins_color_cmd, &mut ins_tilloff_cmd, &mut vb_data_map, &asset_mgr, &mut instance_source_record);
                        init_slot::<AssetDescVBSlot07, AssetResVBSlot07>(id_geo, &geo_desc, &mut instance_code, &mut key07_cmd, &mut vb07_cmd, &mut loader_07, &mut ins_wm_cmd, &mut ins_color_cmd, &mut ins_tilloff_cmd, &mut vb_data_map, &asset_mgr, &mut instance_source_record);
                        init_slot::<AssetDescVBSlot08, AssetResVBSlot08>(id_geo, &geo_desc, &mut instance_code, &mut key08_cmd, &mut vb08_cmd, &mut loader_08, &mut ins_wm_cmd, &mut ins_color_cmd, &mut ins_tilloff_cmd, &mut vb_data_map, &asset_mgr, &mut instance_source_record);
                        init_slot::<AssetDescVBSlot09, AssetResVBSlot09>(id_geo, &geo_desc, &mut instance_code, &mut key09_cmd, &mut vb09_cmd, &mut loader_09, &mut ins_wm_cmd, &mut ins_color_cmd, &mut ins_tilloff_cmd, &mut vb_data_map, &asset_mgr, &mut instance_source_record);
                        init_slot::<AssetDescVBSlot10, AssetResVBSlot10>(id_geo, &geo_desc, &mut instance_code, &mut key10_cmd, &mut vb10_cmd, &mut loader_10, &mut ins_wm_cmd, &mut ins_color_cmd, &mut ins_tilloff_cmd, &mut vb_data_map, &asset_mgr, &mut instance_source_record);
                        init_slot::<AssetDescVBSlot11, AssetResVBSlot11>(id_geo, &geo_desc, &mut instance_code, &mut key11_cmd, &mut vb11_cmd, &mut loader_11, &mut ins_wm_cmd, &mut ins_color_cmd, &mut ins_tilloff_cmd, &mut vb_data_map, &asset_mgr, &mut instance_source_record);
                        init_slot::<AssetDescVBSlot12, AssetResVBSlot12>(id_geo, &geo_desc, &mut instance_code, &mut key12_cmd, &mut vb12_cmd, &mut loader_12, &mut ins_wm_cmd, &mut ins_color_cmd, &mut ins_tilloff_cmd, &mut vb_data_map, &asset_mgr, &mut instance_source_record);


                        // log::debug!(">>>>  GeometryDesc ");
                        geo_desc_cmd.insert(id_geo.clone(), geo_desc);
                        inscode_cmd.insert(id_geo.clone(), instance_code);
                        
                        if let Some(indices_desc) = indices_desc {
                            if let Some(data) = asset_mgr.get(&indices_desc.buffer) {
                                indices_res_cmd.insert(id_geo, AssetResBufferIndices::from(EVerticesBufferUsage::Other(data)));
                            } else {
                                indices_res_cmd.delete(id_geo);
                                loader_indices.request(id_geo, &indices_desc.buffer, None, &mut vb_data_map);
                            }
                            indices_desc_cmd.insert(id_geo.clone(), indices_desc);
                        } else {
                            indices_res_cmd.delete(id_geo);
                            indices_desc_cmd.delete(id_geo);
                        }
                    }
                },
            }
        });
    }
}

fn init_slot<
    D: TVertexBufferUseInfo + Component,
    D1: From<EVerticesBufferUsage> + Component,
>(
    id_geo: ObjectID,
    geodesc: &GeometryDesc,
    instance_code: &mut EInstanceCode,
    slot_cmd: &mut Commands<GameObject, D>,
    res_cmd: &mut Commands<GameObject, D1>,
    loader_01: &mut VertexBufferLoader<ObjectID, D1>,
    ins_wm_cmd: &mut Commands<GameObject, InstancedBufferWorldMatrix>,
    ins_color_cmd: &mut Commands<GameObject, InstancedBufferColor>,
    ins_tilloff_cmd: &mut Commands<GameObject, InstancedBufferTillOff>,
    vb_data_map: &mut SingleVertexBufferDataMap,
    asset_mgr: &Share<AssetMgr<EVertexBufferRange>>,
    instance_source_record: &mut InstanceSourceRecord,
) {
    let slot_index = D::ASK_SLOT_COUNT as usize - 1;
    if slot_index >= geodesc.list.len() {
        slot_cmd.delete(id_geo);
        res_cmd.delete(id_geo);
    } else {
        let desc = geodesc.get_desc(slot_index);
        let instance_kind = desc.instance_kind();
        match instance_kind {
            EInstanceKind::None => {
                if let Some(data) = asset_mgr.get(&desc.key) {
                    res_cmd.insert(id_geo, D1::from(EVerticesBufferUsage::Other(data)));
                } else {
                    res_cmd.delete(id_geo);
                    loader_01.request(id_geo, &desc.key, None, vb_data_map);
                }
            },
            _ => {
    
                let buff_id = instance_source_record.id().to_string();
    
                match instance_kind {
                    EInstanceKind::WorldMatrix => {
                        let buff = InstancedBufferWorldMatrix { slot: slot_index, id: String::from(buff_id + "WorldMatrix"), index: 0 };
                        ins_wm_cmd.insert(id_geo.clone(), buff);
                        instance_code.0 = instance_code.0 | EInstanceCode::BASE;
                    },
                    EInstanceKind::Color => {
                        let buff = InstancedBufferColor { slot: slot_index, id: String::from(buff_id + "Color"), index: 0 };
                        ins_color_cmd.insert(id_geo.clone(), buff);
                        // log::debug!("Instance Color");
                        instance_code.0 = instance_code.0 | EInstanceCode::COLOR;
                    },
                    EInstanceKind::TillOffset => {
                        let buff = InstancedBufferTillOff { slot: slot_index, id: String::from(buff_id + "TillOff"), index: 0 };
                        ins_tilloff_cmd.insert(id_geo.clone(), buff);
                        // log::debug!("Instance TillOffset");
                        instance_code.0 = instance_code.0 | EInstanceCode::TILL_OFF_1;
                    },
                    _ => { },
                }
            },
        };
        
        slot_cmd.insert(id_geo.clone(), D::from(desc));
    }
}

pub trait TInterfaceGeomtery {
    fn create_vertex_buffer(
        &self,
        key: KeyVertexBuffer,
        buffer: Vec<u8>,
    ) -> &Self;
    fn use_geometry(
        &self,
        entity: ObjectID,
        vertices: Vec<VertexBufferDesc>,
        indices: Option<IndicesBufferDesc>,
    ) -> &Self;
}

impl TInterfaceGeomtery for EnginShell {
    fn create_vertex_buffer(
        &self,
        key: KeyVertexBuffer,
        buffer: Vec<u8>,
    ) -> &Self {
        let world = self.world();
        let assert_mgr = world.get_resource::<Share<AssetMgr<EVertexBufferRange>>>().unwrap();
        if !assert_mgr.check_asset(&key) {
            let data_map = world.get_resource_mut::<SingleVertexBufferDataMap>().unwrap();
            data_map.add(&key, buffer);
        }

        self
    }
    fn use_geometry(
        &self,
        entity: ObjectID,
        descs: Vec<VertexBufferDesc>,
        indices: Option<IndicesBufferDesc>,
    ) -> &Self {
        let commands = self.world().get_resource_mut::<SingleGeometryVBCommands>().unwrap();
        commands.0.push(ECommand::Desc(entity, descs, indices));

        self
    }
}

pub struct PluginGeometry;
impl Plugin for PluginGeometry {
    fn init(
        &mut self,
        engine: &mut crate::engine::Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {
        let world = engine.world_mut();
        world.insert_resource(SingleGeometryVBCommands::default());
        world.insert_resource(VertexBufferAllocator::new());
        world.insert_resource(AssetMgr::<EVertexBufferRange>::new(GarbageEmpty(), false, 1 * 1024 * 1024, 10 * 1000));

        world.insert_resource(VertexBufferLoader::<ObjectID, AssetResVBSlot01>::default());
        world.insert_resource(VertexBufferLoader::<ObjectID, AssetResVBSlot02>::default());
        world.insert_resource(VertexBufferLoader::<ObjectID, AssetResVBSlot03>::default());
        world.insert_resource(VertexBufferLoader::<ObjectID, AssetResVBSlot04>::default());
        world.insert_resource(VertexBufferLoader::<ObjectID, AssetResVBSlot05>::default());
        world.insert_resource(VertexBufferLoader::<ObjectID, AssetResVBSlot06>::default());
        world.insert_resource(VertexBufferLoader::<ObjectID, AssetResVBSlot07>::default());
        world.insert_resource(VertexBufferLoader::<ObjectID, AssetResVBSlot08>::default());
        world.insert_resource(VertexBufferLoader::<ObjectID, AssetResVBSlot09>::default());
        world.insert_resource(VertexBufferLoader::<ObjectID, AssetResVBSlot10>::default());
        world.insert_resource(VertexBufferLoader::<ObjectID, AssetResVBSlot11>::default());
        world.insert_resource(VertexBufferLoader::<ObjectID, AssetResVBSlot12>::default());
        world.insert_resource(VertexBufferLoader::<ObjectID, AssetResVBSlot13>::default());
        world.insert_resource(VertexBufferLoader::<ObjectID, AssetResVBSlot14>::default());
        world.insert_resource(VertexBufferLoader::<ObjectID, AssetResVBSlot15>::default());
        world.insert_resource(VertexBufferLoader::<ObjectID, AssetResVBSlot16>::default());
        world.insert_resource(VertexBufferLoader::<ObjectID, AssetResBufferIndices>::default());

        SysGeometryVBCommand::setup(world, stages.query_stage::<SysGeometryVBCommand>(ERunStageChap::Initial));
        SysVertexBufferLoad::setup(world, stages.query_stage::<SysVertexBufferLoad>(ERunStageChap::Draw));
        PluginVertexBuffers.init(engine, stages);

        Ok(())
    }
}

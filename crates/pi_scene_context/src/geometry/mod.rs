

use pi_atom::Atom;

use pi_engine_shell::prelude::*;
///
/// 网格信息单独与 GameObject 绑定

use crate::{object::ObjectID, plugin::Plugin};

use self::{
    vertex_buffer_useinfo::*,
    sys_vertex_buffer_use::*,
    load::sys_vertex_buffer_loaded,
    instance::*, command::*,
};

pub mod base;
pub mod command;
pub mod vertex_buffer_useinfo;
pub mod sys_vertex_buffer_use;
pub mod geometry;
pub mod indices;
pub mod instance;
pub mod load;

pub type VDK = usize;
pub type GBID = Atom;

#[derive(Resource)]
pub struct GeometryVBLoader {
    pub loader_01: VertexBufferLoader<ObjectID, AssetResVBSlot01>,
    pub loader_02: VertexBufferLoader<ObjectID, AssetResVBSlot02>,
    pub loader_03: VertexBufferLoader<ObjectID, AssetResVBSlot03>,
    pub loader_04: VertexBufferLoader<ObjectID, AssetResVBSlot04>,
    pub loader_05: VertexBufferLoader<ObjectID, AssetResVBSlot05>,
    pub loader_06: VertexBufferLoader<ObjectID, AssetResVBSlot06>,
    pub loader_07: VertexBufferLoader<ObjectID, AssetResVBSlot07>,
    pub loader_08: VertexBufferLoader<ObjectID, AssetResVBSlot08>,
    pub loader_09: VertexBufferLoader<ObjectID, AssetResVBSlot09>,
    pub loader_10: VertexBufferLoader<ObjectID, AssetResVBSlot10>,
    pub loader_11: VertexBufferLoader<ObjectID, AssetResVBSlot11>,
    pub loader_12: VertexBufferLoader<ObjectID, AssetResVBSlot12>,
    pub loader_13: VertexBufferLoader<ObjectID, AssetResVBSlot13>,
    pub loader_14: VertexBufferLoader<ObjectID, AssetResVBSlot14>,
    pub loader_15: VertexBufferLoader<ObjectID, AssetResVBSlot15>,
    pub loader_16: VertexBufferLoader<ObjectID, AssetResVBSlot16>,
    pub loader_indices: VertexBufferLoader<ObjectID, AssetResBufferIndices>,
}
impl Default for GeometryVBLoader {
    fn default() -> Self {
        Self {
            loader_01: VertexBufferLoader::<ObjectID, AssetResVBSlot01>::default(),
            loader_02: VertexBufferLoader::<ObjectID, AssetResVBSlot02>::default(),
            loader_03: VertexBufferLoader::<ObjectID, AssetResVBSlot03>::default(),
            loader_04: VertexBufferLoader::<ObjectID, AssetResVBSlot04>::default(),
            loader_05: VertexBufferLoader::<ObjectID, AssetResVBSlot05>::default(),
            loader_06: VertexBufferLoader::<ObjectID, AssetResVBSlot06>::default(),
            loader_07: VertexBufferLoader::<ObjectID, AssetResVBSlot07>::default(),
            loader_08: VertexBufferLoader::<ObjectID, AssetResVBSlot08>::default(),
            loader_09: VertexBufferLoader::<ObjectID, AssetResVBSlot09>::default(),
            loader_10: VertexBufferLoader::<ObjectID, AssetResVBSlot10>::default(),
            loader_11: VertexBufferLoader::<ObjectID, AssetResVBSlot11>::default(),
            loader_12: VertexBufferLoader::<ObjectID, AssetResVBSlot12>::default(),
            loader_13: VertexBufferLoader::<ObjectID, AssetResVBSlot13>::default(),
            loader_14: VertexBufferLoader::<ObjectID, AssetResVBSlot14>::default(),
            loader_15: VertexBufferLoader::<ObjectID, AssetResVBSlot15>::default(),
            loader_16: VertexBufferLoader::<ObjectID, AssetResVBSlot16>::default(),
            loader_indices: VertexBufferLoader::<ObjectID, AssetResBufferIndices>::default(),
        }
    }
}

#[derive(Component)]
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

// impl TInterfaceGeomtery for EnginShell {
//     fn create_vertex_buffer(
//         &self,
//         key: KeyVertexBuffer,
//         buffer: Vec<u8>,
//     ) -> &Self {
//         let world = self.world();
//         let assert_mgr = world.get_resource::<Share<AssetMgr<EVertexBufferRange>>>().unwrap();
//         if !assert_mgr.check_asset(&key) {
//             let data_map = world.get_resource_mut::<SingleVertexBufferDataMap>().unwrap();
//             data_map.add(&key, buffer);
//         }

//         self
//     }
//     fn use_geometry(
//         &self,
//         entity: ObjectID,
//         descs: Vec<VertexBufferDesc>,
//         indices: Option<IndicesBufferDesc>,
//     ) -> &Self {
//         let commands = self.world().get_resource_mut::<SingleGeometryVBCommands>().unwrap();
//         commands.0.push(ECommand::Desc(entity, descs, indices));

//         self
//     }
// }

pub struct PluginGeometry;
impl Plugin for PluginGeometry {
    // fn init(
    //     &mut self,
    //     engine: &mut crate::engine::Engine,
    //     stages: &mut crate::run_stage::RunStage,
    // ) -> Result<(), crate::plugin::ErrorPlugin> {
    //     let world = engine.world_mut();
    //     world.insert_resource(SingleGeometryVBCommands::default());
    //     world.insert_resource(VBAllocator::new());
    //     world.insert_resource(AssetMgr::<EVertexBufferRange>::new(GarbageEmpty(), false, 1 * 1024 * 1024, 10 * 1000));

    //     world.insert_resource(VBLoaderSlot::<ObjectID, AssetResVBSlot01>::default());
    //     world.insert_resource(VBLoaderSlot::<ObjectID, AssetResVBSlot02>::default());
    //     world.insert_resource(VBLoaderSlot::<ObjectID, AssetResVBSlot03>::default());
    //     world.insert_resource(VBLoaderSlot::<ObjectID, AssetResVBSlot04>::default());
    //     world.insert_resource(VBLoaderSlot::<ObjectID, AssetResVBSlot05>::default());
    //     world.insert_resource(VBLoaderSlot::<ObjectID, AssetResVBSlot06>::default());
    //     world.insert_resource(VBLoaderSlot::<ObjectID, AssetResVBSlot07>::default());
    //     world.insert_resource(VBLoaderSlot::<ObjectID, AssetResVBSlot08>::default());
    //     world.insert_resource(VBLoaderSlot::<ObjectID, AssetResVBSlot09>::default());
    //     world.insert_resource(VBLoaderSlot::<ObjectID, AssetResVBSlot10>::default());
    //     world.insert_resource(VBLoaderSlot::<ObjectID, AssetResVBSlot11>::default());
    //     world.insert_resource(VBLoaderSlot::<ObjectID, AssetResVBSlot12>::default());
    //     world.insert_resource(VBLoaderSlot::<ObjectID, AssetResVBSlot13>::default());
    //     world.insert_resource(VBLoaderSlot::<ObjectID, AssetResVBSlot14>::default());
    //     world.insert_resource(VBLoaderSlot::<ObjectID, AssetResVBSlot15>::default());
    //     world.insert_resource(VBLoaderSlot::<ObjectID, AssetResVBSlot16>::default());
    //     world.insert_resource(VBLoaderSlot::<ObjectID, AssetResBufferIndices>::default());

    //     SysGeometryVBCommand::setup(world, stages.query_stage::<SysGeometryVBCommand>(ERunStageChap::Initial));
    //     SysVertexBufferLoad::setup(world, stages.query_stage::<SysVertexBufferLoad>(ERunStageChap::Draw));
    //     PluginVertexBuffers.init(engine, stages);

    //     Ok(())
    // }

    fn build(&self, app: &mut bevy::prelude::App) {
        // app.world.insert_resource(SingleGeometryVBCommands::default());
        app.insert_resource(ActionListGeometryCreate::default());
        app.insert_resource(VertexBufferDataMap3D(SingleVertexBufferDataMap::default()));
        app.insert_resource(VertexBufferAllocator3D(VertexBufferAllocator::new()));
        app.insert_resource(ShareAssetMgr::<EVertexBufferRange>::new(GarbageEmpty(), false, 1 * 1024 * 1024, 10 * 1000));
        app.insert_resource(GeometryVBLoader::default());

        app.add_systems(
            (
                sys_geometry_create.in_set(ERunStageChap::Initial),
                sys_vertex_buffer_loaded.in_set(ERunStageChap::Command),
            )
        );
        app.add_systems(
            (
                sys_vertex_buffer_loaded_01,
                sys_vertex_buffer_loaded_02,
                sys_vertex_buffer_loaded_03,
                sys_vertex_buffer_loaded_04,
                sys_vertex_buffer_loaded_05,
                sys_vertex_buffer_loaded_06,
            ).in_set(ERunStageChap::Uniform)
    );
    }
}

// use pi_assets::mgr::AssetMgr;
// use pi_ecs::{prelude::{ResMut, Setup, Commands, Res}};
// use pi_ecs_macros::setup;
// use pi_engine_shell::{assets::sync_load::{PluginAssetSyncLoad, AssetSyncLoad}, run_stage::{TSystemStageInfo, ERunStageChap}};
// use pi_render::renderer::{indices::{AssetKeyBufferIndices, AssetResBufferIndices, IndicesBufferDesc}, vertex_buffer::{KeyVertexBuffer, EVertexBufferRange}, vertex_buffer_loader::{VertexBufferLoader, SingleVertexBufferDataMap}, vertices::EVerticesBufferUsage};
// use pi_share::Share;

// use crate::{object::{ObjectID, GameObject}, plugin::Plugin, engine::Engine};

// #[derive(Debug)]
// pub enum ECommand {
//     Use(ObjectID, IndicesBufferDesc),
// }
// #[derive(Debug, Default)]
// pub struct CommandListBufferIndices {
//     pub list: Vec<ECommand>,
// }
// pub struct SysGeometryIndicesCommand;
// impl TSystemStageInfo for SysGeometryIndicesCommand {

// }
// #[setup]
// impl SysGeometryIndicesCommand {
//     #[system]
//     pub fn cmd(
//         mut cmds: ResMut<CommandListBufferIndices>,
//         mut desc_cmd: Commands<GameObject, IndicesBufferDesc>,
//         mut res_cmd: Commands<GameObject, AssetResBufferIndices>,
//         mut vb_data_map: ResMut<SingleVertexBufferDataMap>,
//         mut loader_01: ResMut<VertexBufferLoader<ObjectID, AssetResBufferIndices>>,
//         asset_mgr: Res<Share<AssetMgr<EVertexBufferRange>>>,
//     ) {
//         let mut list = std::mem::replace(&mut cmds.list, vec![]);
//         list.drain(..).for_each(|cmd| {
//             match cmd {
//                 ECommand::Use(entity, desc) => {
//                     if let Some(data) = asset_mgr.get(&desc.buffer) {
//                         res_cmd.insert(entity, AssetResBufferIndices::from(EVerticesBufferUsage::Other(data)));
//                     } else {
//                         loader_01.request(entity, &desc.buffer, None, &mut vb_data_map);
//                     }
//                     desc_cmd.insert(entity.clone(), desc);
//                 },
//             }
//         });
//     }
// }

// pub trait InterfaceBufferIndices {
//     fn use_indices(
//         & self,
//         entity: ObjectID,
//         info: IndicesBufferDesc,
//     ) -> &Self;
// }
// impl InterfaceBufferIndices for Engine {
//     fn use_indices(
//         & self,
//         entity: ObjectID,
//         info: IndicesBufferDesc,
//     ) -> &Self {
//         let commands = self.world().get_resource_mut::<CommandListBufferIndices>().unwrap();
//         commands.list.push(ECommand::Use(entity, info));

//         self
//     }
// }

// pub struct PluginBufferIndices;
// impl Plugin for PluginBufferIndices {
//     fn init(
//         &mut self,
//         engine: &mut crate::engine::Engine,
//         stages: &mut crate::run_stage::RunStage,
//     ) -> Result<(), crate::plugin::ErrorPlugin> {

//         let world = engine.world_mut();
//         world.insert_resource(CommandListBufferIndices::default());
//         SysGeometryIndicesCommand::setup(world, stages.query_stage::<SysGeometryIndicesCommand>(ERunStageChap::Initial));

//         Ok(())
//     }
// }

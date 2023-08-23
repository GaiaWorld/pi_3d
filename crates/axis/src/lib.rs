
use axis::AxisBuilder;
use pi_engine_shell::prelude::*;
use pi_scene_context::prelude::*;

pub mod shader;
pub mod axis;
pub mod interface;

// use crate::shader::AxisShader;


// fn setup(
//     asset_mgr: Res<ShareAssetMgr<EVertexBufferRange>>,
//     mut data_map: ResMut<VertexBufferDataMap3D>,
// ) {
//     if !ActionVertexBuffer::check(&asset_mgr, KeyVertexBuffer::from(AxisBuilder::KEY_BUFFER_POSITION)) {
//         ActionVertexBuffer::create(&mut data_map, KeyVertexBuffer::from(AxisBuilder::KEY_BUFFER_POSITION), bytemuck::cast_slice(&AxisBuilder::position()).iter().map(|v| *v).collect::<Vec<u8>>());
//     }
//     if !ActionVertexBuffer::check(&asset_mgr, KeyVertexBuffer::from(AxisBuilder::KEY_BUFFER_COLOR4)) {
//         ActionVertexBuffer::create(&mut data_map, KeyVertexBuffer::from(AxisBuilder::KEY_BUFFER_COLOR4), bytemuck::cast_slice(&AxisBuilder::colors()).iter().map(|v| *v).collect::<Vec<u8>>());
//     }
//     if !ActionVertexBuffer::check(&asset_mgr, KeyVertexBuffer::from(AxisBuilder::KEY_BUFFER_INDICES)) {
//         ActionVertexBuffer::create(&mut data_map, KeyVertexBuffer::from(AxisBuilder::KEY_BUFFER_INDICES), bytemuck::cast_slice(&AxisBuilder::indices()).iter().map(|v| *v).collect::<Vec<u8>>());
//     }
// }

pub struct PluginAxis;
impl Plugin for PluginAxis {
    fn build(&self, app: &mut App) {
        // app.add_startup_system(setup);
        let asset_mgr = app.world.get_resource::<ShareAssetMgr<EVertexBufferRange>>().unwrap().clone();
        let mut data_map = app.world.get_resource_mut::<VertexBufferDataMap3D>().unwrap();
        if !ActionVertexBuffer::check(&asset_mgr, KeyVertexBuffer::from(AxisBuilder::KEY_BUFFER_POSITION)) {
            ActionVertexBuffer::create(&mut data_map, KeyVertexBuffer::from(AxisBuilder::KEY_BUFFER_POSITION), bytemuck::cast_slice(&AxisBuilder::position()).iter().map(|v| *v).collect::<Vec<u8>>());
        }
        if !ActionVertexBuffer::check(&asset_mgr, KeyVertexBuffer::from(AxisBuilder::KEY_BUFFER_COLOR4)) {
            ActionVertexBuffer::create(&mut data_map, KeyVertexBuffer::from(AxisBuilder::KEY_BUFFER_COLOR4), bytemuck::cast_slice(&AxisBuilder::colors()).iter().map(|v| *v).collect::<Vec<u8>>());
        }
        if !ActionVertexBuffer::check(&asset_mgr, KeyVertexBuffer::from(AxisBuilder::KEY_BUFFER_INDICES)) {
            ActionVertexBuffer::create(&mut data_map, KeyVertexBuffer::from(AxisBuilder::KEY_BUFFER_INDICES), bytemuck::cast_slice(&AxisBuilder::indices()).iter().map(|v| *v).collect::<Vec<u8>>());
        }
    }
    // fn init(
    //     &mut self,
    //     engine: &mut EnginShell,
    //     stages: &mut RunStage,
    // ) -> Result<(), ErrorPlugin> {
    //     log::debug!("PluginAxis");
    //     let key = KeyShaderMeta::from(AxisShader::KEY);
    //     engine.regist_material_meta(key, AxisShader::meta());

    //     Ok(())
    // }
}
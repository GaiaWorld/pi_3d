// use pi_assets::{asset::GarbageEmpty, mgr::AssetMgr};
// use pi_engine_shell::plugin::Plugin;
// use pi_render::rhi::asset::{TextureRes, RenderRes};
// use pi_share::Share;
// use pi_scene_math::Number;

// pub mod sampler_load;
// pub mod main_texture;
// pub mod emissive_texture;

// #[derive(Debug, Clone, Copy)]
// pub struct Texture2DScaleOffset {
//     pub u_tiling: Number,
//     pub v_tiling: Number,
//     pub u_offset: Number,
//     pub v_offset: Number,
// }
// impl Default for Texture2DScaleOffset {
//     fn default() -> Self {
//         Self { u_tiling: 1., v_tiling: 1., u_offset: 0., v_offset: 0. }
//     }
// }

// pub struct PluginMaterialTextures;
// impl Plugin for PluginMaterialTextures {
//     fn init(
//         &mut self,
//         engine: &mut pi_engine_shell::engine_shell::EnginShell,
//         stages: &mut pi_engine_shell::run_stage::RunStage,
//     ) -> Result<(), pi_engine_shell::plugin::ErrorPlugin> {

//         if engine.world().get_resource::<Share<AssetMgr<TextureRes>>>().is_none() {
//             engine.world_mut().insert_resource(
//                 AssetMgr::<TextureRes>::new(GarbageEmpty(), false, 60 * 1024 * 1024, 60 * 1000)
//             );
//         }
        
//         Ok(())
//     }
// }
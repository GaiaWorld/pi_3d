use pi_mesh_builder::quad::QuadBuilder;
use pi_node_materials::prelude::{BlockMainTexture, BlockUVAtlas, NodeMaterialBuilder};
use pi_scene_shell::prelude::*;
use pi_scene_context::prelude::*;
use pi_slotmap::Key;

pub fn main() {}

pub struct PluginImageCopy;
impl Plugin for PluginImageCopy {
    fn build(&self, app: &mut App) {
        let asset_mgr = app.world.get_resource::<ShareAssetMgr<ShaderEffectMeta>>().unwrap().clone();
    }
}
impl PluginImageCopy {
    pub fn toscreen(
        commands: &mut Commands,
        actions: &mut pi_3d::ActionSets,
        scene: Entity,
        pre_renderer: Entity,
        source_render_target: Option<KeyCustomRenderTarget>,
    ) -> (Entity, Entity) {

        // {
            let copymat = commands.spawn_empty_id();
            actions.material.create.push(OpsMaterialCreate::ops_with_matarray(copymat, ShaderImageCopy::KEY));
            
            if let Some(pre_render_target) = source_render_target {
                match pre_render_target {
                    KeyCustomRenderTarget::Custom(pre_render_target) => {
                        actions.material.valb.push(OpsUniformValB::texture_from_target(copymat, UniformTextureWithSamplerParam { slotname: Atom::from(BlockMainTexture::KEY_TEX), ..Default::default() }, pre_render_target, Atom::from(BlockMainTexture::KEY_TILLOFF)));
                    },
                    KeyCustomRenderTarget::FinalRender(_) => {},
                }

            }

            let id_geo = commands.spawn_empty_id();
            let attrs = QuadBuilder::attrs_meta();
            
            let plane = commands.spawn_empty_id(); actions.transform.tree.push(OpsTransformNodeParent::ops(plane, scene));
            actions.mesh.create.push(OpsMeshCreation::ops(scene, plane, MeshInstanceState { ..Default::default() }));

            // actions.mesh.depth_compare.push(OpsDepthCompare::ops(plane, CompareFunction::Always));
            actions.mesh.render_state.push(OpsRenderState::depth_state(plane, PassTag::PASS_TAG_01, EDepthState::Compare(CompareFunction::Always)));
            actions.mesh.render_state.push(OpsRenderState::depth_state(plane, PassTag::PASS_TAG_02, EDepthState::Compare(CompareFunction::Always)));
            actions.mesh.render_state.push(OpsRenderState::depth_state(plane, PassTag::PASS_TAG_03, EDepthState::Compare(CompareFunction::Always)));
            actions.mesh.render_state.push(OpsRenderState::depth_state(plane, PassTag::PASS_TAG_04, EDepthState::Compare(CompareFunction::Always)));
            actions.mesh.render_state.push(OpsRenderState::depth_state(plane, PassTag::PASS_TAG_05, EDepthState::Compare(CompareFunction::Always)));
            actions.mesh.render_state.push(OpsRenderState::depth_state(plane, PassTag::PASS_TAG_06, EDepthState::Compare(CompareFunction::Always)));
            actions.mesh.render_state.push(OpsRenderState::depth_state(plane, PassTag::PASS_TAG_07, EDepthState::Compare(CompareFunction::Always)));
            actions.mesh.render_state.push(OpsRenderState::depth_state(plane, PassTag::PASS_TAG_08, EDepthState::Compare(CompareFunction::Always)));
            actions.mesh.render_state.push(OpsRenderState::PrimitiveState(plane, PassTag::PASS_TAG_01, EPrimitiveState::CCullMode(CullMode::Off)));
            actions.mesh.render_state.push(OpsRenderState::PrimitiveState(plane, PassTag::PASS_TAG_02, EPrimitiveState::CCullMode(CullMode::Off)));
            actions.mesh.render_state.push(OpsRenderState::PrimitiveState(plane, PassTag::PASS_TAG_03, EPrimitiveState::CCullMode(CullMode::Off)));
            actions.mesh.render_state.push(OpsRenderState::PrimitiveState(plane, PassTag::PASS_TAG_04, EPrimitiveState::CCullMode(CullMode::Off)));
            actions.mesh.render_state.push(OpsRenderState::PrimitiveState(plane, PassTag::PASS_TAG_05, EPrimitiveState::CCullMode(CullMode::Off)));
            actions.mesh.render_state.push(OpsRenderState::PrimitiveState(plane, PassTag::PASS_TAG_06, EPrimitiveState::CCullMode(CullMode::Off)));
            actions.mesh.render_state.push(OpsRenderState::PrimitiveState(plane, PassTag::PASS_TAG_07, EPrimitiveState::CCullMode(CullMode::Off)));
            actions.mesh.render_state.push(OpsRenderState::PrimitiveState(plane, PassTag::PASS_TAG_08, EPrimitiveState::CCullMode(CullMode::Off)));

            actions.geometry.create.push(OpsGeomeryCreate::ops(plane, id_geo, attrs, None));
            actions.material.usemat.push(OpsMaterialUse::ops(plane, copymat, PassTag::PASS_TAG_01));
            
            let copycamera = commands.spawn_empty_id(); actions.transform.tree.push(OpsTransformNodeParent::ops(copycamera, scene));
            actions.camera.create.push(OpsCameraCreation::ops(scene, copycamera, Entity::null()));
            actions.mesh.layermask.push(OpsLayerMask::ops(copycamera, 0));
            actions.mesh.layermask.push(OpsLayerMask::ops(plane, 0));
            actions.camera.forceinclude.push(OpsViewerForceInclude::ops(copycamera, plane, true));
            actions.camera.param.push(OpsCameraModify::ops( copycamera, ECameraModify::Active( true )));
            
            let copy_renderer = commands.spawn_empty_id(); actions.renderer.create.push(OpsRendererCreate::ops(copy_renderer, String::from("ImageCopy") + copy_renderer.index().to_string().as_str(), copycamera, PassTag::PASS_TAG_01, false, false, false));
            actions.renderer.modify.push(OpsRendererCommand::AutoClearColor(copy_renderer, false));
            actions.renderer.modify.push(OpsRendererCommand::AutoClearDepth(copy_renderer, false));
            actions.renderer.modify.push(OpsRendererCommand::AutoClearStencil(copy_renderer, false));
            actions.renderer.connect.push(OpsRendererConnect::ops(pre_renderer, copy_renderer, false));
            actions.renderer.target.push(OpsRendererTarget::Custom(copy_renderer, KeyCustomRenderTarget::FinalRender(true), false));
        // }
        (copy_renderer, copycamera)
    }
    pub fn init(
        commands: &mut Commands,
        actions: &mut pi_3d::ActionSets,
        scene: Entity,
        pre_renderer: Entity,
        next_renderer: Entity,
        source_render_target: Option<KeyCustomRenderTarget>,
        dst_render_target: Option<KeyCustomRenderTarget>,
    ) -> (Entity, Entity) {

        // {
            let copymat = commands.spawn_empty_id();
            actions.material.create.push(OpsMaterialCreate::ops_with_matarray(copymat, ShaderImageCopy::KEY));
            
            if let Some(pre_render_target) = source_render_target {
                match pre_render_target {
                    KeyCustomRenderTarget::Custom(pre_render_target) => {
                        actions.material.valb.push(OpsUniformValB::texture_from_target(copymat, UniformTextureWithSamplerParam { slotname: Atom::from(BlockMainTexture::KEY_TEX), ..Default::default() }, pre_render_target, Atom::from(BlockMainTexture::KEY_TILLOFF)));
                    },
                    KeyCustomRenderTarget::FinalRender(_) => {},
                }

            }

            let id_geo = commands.spawn_empty_id();
            let attrs = QuadBuilder::attrs_meta();
            
            let plane = commands.spawn_empty_id(); actions.transform.tree.push(OpsTransformNodeParent::ops(plane, scene));
            actions.mesh.create.push(OpsMeshCreation::ops(scene, plane, MeshInstanceState { ..Default::default() }));

            // actions.mesh.depth_compare.push(OpsDepthCompare::ops(plane, CompareFunction::Always));
            actions.mesh.render_state.push(OpsRenderState::depth_state(plane, PassTag::PASS_TAG_01, EDepthState::Compare(CompareFunction::Always)));
            actions.mesh.render_state.push(OpsRenderState::depth_state(plane, PassTag::PASS_TAG_02, EDepthState::Compare(CompareFunction::Always)));
            actions.mesh.render_state.push(OpsRenderState::depth_state(plane, PassTag::PASS_TAG_03, EDepthState::Compare(CompareFunction::Always)));
            actions.mesh.render_state.push(OpsRenderState::depth_state(plane, PassTag::PASS_TAG_04, EDepthState::Compare(CompareFunction::Always)));
            actions.mesh.render_state.push(OpsRenderState::depth_state(plane, PassTag::PASS_TAG_05, EDepthState::Compare(CompareFunction::Always)));
            actions.mesh.render_state.push(OpsRenderState::depth_state(plane, PassTag::PASS_TAG_06, EDepthState::Compare(CompareFunction::Always)));
            actions.mesh.render_state.push(OpsRenderState::depth_state(plane, PassTag::PASS_TAG_07, EDepthState::Compare(CompareFunction::Always)));
            actions.mesh.render_state.push(OpsRenderState::depth_state(plane, PassTag::PASS_TAG_08, EDepthState::Compare(CompareFunction::Always)));
            actions.mesh.render_state.push(OpsRenderState::PrimitiveState(plane, PassTag::PASS_TAG_01, EPrimitiveState::CCullMode(CullMode::Off)));
            actions.mesh.render_state.push(OpsRenderState::PrimitiveState(plane, PassTag::PASS_TAG_02, EPrimitiveState::CCullMode(CullMode::Off)));
            actions.mesh.render_state.push(OpsRenderState::PrimitiveState(plane, PassTag::PASS_TAG_03, EPrimitiveState::CCullMode(CullMode::Off)));
            actions.mesh.render_state.push(OpsRenderState::PrimitiveState(plane, PassTag::PASS_TAG_04, EPrimitiveState::CCullMode(CullMode::Off)));
            actions.mesh.render_state.push(OpsRenderState::PrimitiveState(plane, PassTag::PASS_TAG_05, EPrimitiveState::CCullMode(CullMode::Off)));
            actions.mesh.render_state.push(OpsRenderState::PrimitiveState(plane, PassTag::PASS_TAG_06, EPrimitiveState::CCullMode(CullMode::Off)));
            actions.mesh.render_state.push(OpsRenderState::PrimitiveState(plane, PassTag::PASS_TAG_07, EPrimitiveState::CCullMode(CullMode::Off)));
            actions.mesh.render_state.push(OpsRenderState::PrimitiveState(plane, PassTag::PASS_TAG_08, EPrimitiveState::CCullMode(CullMode::Off)));

            actions.geometry.create.push(OpsGeomeryCreate::ops(plane, id_geo, attrs, None));
            actions.material.usemat.push(OpsMaterialUse::ops(plane, copymat, PassTag::PASS_TAG_01));
            
            let copycamera = commands.spawn_empty_id(); actions.transform.tree.push(OpsTransformNodeParent::ops(copycamera, scene));
            actions.camera.create.push(OpsCameraCreation::ops(scene, copycamera, Entity::null()));
            actions.mesh.layermask.push(OpsLayerMask::ops(copycamera, 0));
            actions.mesh.layermask.push(OpsLayerMask::ops(plane, 0));
            actions.camera.forceinclude.push(OpsViewerForceInclude::ops(copycamera, plane, true));
            actions.camera.param.push(OpsCameraModify::ops( copycamera, ECameraModify::Active( true )));
            
            let copy_renderer = commands.spawn_empty_id(); actions.renderer.create.push(OpsRendererCreate::ops(copy_renderer, String::from("ImageCopy") + copy_renderer.index().to_string().as_str(), copycamera, PassTag::PASS_TAG_01, false, false, false));
            actions.renderer.modify.push(OpsRendererCommand::AutoClearColor(copy_renderer, false));
            actions.renderer.modify.push(OpsRendererCommand::AutoClearDepth(copy_renderer, false));
            actions.renderer.modify.push(OpsRendererCommand::AutoClearStencil(copy_renderer, false));
            actions.renderer.connect.push(OpsRendererConnect::ops(pre_renderer, copy_renderer, false));
            actions.renderer.connect.push(OpsRendererConnect::ops(copy_renderer, next_renderer, false));
            actions.renderer.target.push(OpsRendererTarget::Custom(copy_renderer, dst_render_target.unwrap(), false));
        // }
        (copy_renderer, copycamera)
    }
}

pub struct ShaderImageCopy;
impl ShaderImageCopy {
    pub const KEY: &'static str = "ImageCopy";
    pub fn res(engineopt: &EngineCustomPlugins) -> ShaderEffectMeta {
        let mut nodemat = NodeMaterialBuilder::new();
        nodemat.fs_define = String::from("
        layout(location = 0) out vec4 gl_FragColor;
        ");

        nodemat.binddefines = BindDefines::EFFECT_VALUE_BIND | BindDefines::EFFECT_TEXTURE_BIND;

        nodemat.vs = String::from("
        gl_Position = vec4(A_POSITION, 1.);
        v_uv = (gl_Position.xy + 0.5) * matParam.uMainTilloff.xy + matParam.uMainTilloff.zw;
        gl_Position.xy *= 2.;
        ");
        nodemat.fs = String::from("
        gl_FragColor = mainTexture(v_uv);
        ");

        nodemat.varyings = Varyings(
            vec![
                Varying { format: Atom::from("vec2"), name: Atom::from("v_uv"), },
            ]
        );
        
        nodemat.apply::<BlockUVAtlas>();
        nodemat.apply::<BlockMainTexture>();

        nodemat.meta(engineopt)
    }
}

use bevy::app::Plugin;
use pi_mesh_builder::quad::QuadBuilder;
use pi_node_materials::prelude::{NodeMaterialBuilder, BlockMainTexture};
use pi_engine_shell::prelude::*;
use pi_scene_context::prelude::*;


pub fn main() {}

pub struct PluginImageCopy;
impl Plugin for PluginImageCopy {
    fn build(&self, app: &mut bevy::prelude::App) {
        let asset_mgr = app.world.get_resource::<ShareAssetMgr<ShaderEffectMeta>>().unwrap().clone();
        ActionMaterial::regist_material_meta(&asset_mgr, KeyShaderMeta::from(ShaderImageCopy::KEY), ShaderImageCopy::res());
    }
}
impl PluginImageCopy {
    pub fn toscreen(
        commands: &mut Commands,
        materialcmd: &mut ActionSetMaterial,
        meshcmds: &mut ActionSetMesh,
        geometrycmd: &mut ActionSetGeometry,
        cameracmds: &mut ActionSetCamera,
        transformcmds: &mut ActionSetTransform,
        renderercmds: &mut ActionSetRenderer,
        scene: Entity,
        pre_renderer: Entity,
        source_render_target: Option<KeyCustomRenderTarget>,
    ) -> (Entity, Entity) {

        // {
            let copymat = commands.spawn_empty().id();
            materialcmd.create.push(OpsMaterialCreate::ops(copymat, ShaderImageCopy::KEY));
            
            if let Some(pre_render_target) = source_render_target {
                match pre_render_target {
                    KeyCustomRenderTarget::Custom(pre_render_target) => {
                        materialcmd.texturefromtarget.push(OpsUniformTextureFromRenderTarget::ops(copymat, UniformTextureWithSamplerParam { slotname: Atom::from(BlockMainTexture::KEY_TEX), ..Default::default() }, pre_render_target, Atom::from(BlockMainTexture::KEY_TILLOFF)));
                    },
                    KeyCustomRenderTarget::FinalRender => {},
                }

            }

            let id_geo = commands.spawn_empty().id();
            let attrs = QuadBuilder::attrs_meta();
            
            let plane = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(plane, scene));
            meshcmds.create.push(OpsMeshCreation::ops(scene, plane, MeshInstanceState { ..Default::default() }));
            meshcmds.depth_compare.push(OpsDepthCompare::ops(plane, CompareFunction::Always));
            geometrycmd.create.push(OpsGeomeryCreate::ops(plane, id_geo, attrs, Some(QuadBuilder::indices_meta())));
            materialcmd.usemat.push(OpsMaterialUse::ops(plane, copymat, PassTag::PASS_TAG_01));
            
            let copycamera = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(copycamera, scene));
            cameracmds.create.push(OpsCameraCreation::ops(scene, copycamera, false));
            meshcmds.layermask.push(OpsLayerMask::ops(copycamera, 0));
            meshcmds.layermask.push(OpsLayerMask::ops(plane, 0));
            cameracmds.forceinclude.push(OpsViewerForceInclude::ops(copycamera, plane, true));
            cameracmds.active.push(OpsCameraActive::ops(copycamera, true));
            
            let copy_renderer = commands.spawn_empty().id(); renderercmds.create.push(OpsRendererCreate::ops(copy_renderer, String::from("ImageCopy") + copy_renderer.to_bits().to_string().as_str(), copycamera, PassTag::PASS_TAG_01, false));
            renderercmds.modify.push(OpsRendererCommand::AutoClearColor(copy_renderer, false));
            renderercmds.modify.push(OpsRendererCommand::AutoClearDepth(copy_renderer, false));
            renderercmds.modify.push(OpsRendererCommand::AutoClearStencil(copy_renderer, false));
            renderercmds.connect.push(OpsRendererConnect::ops(pre_renderer, copy_renderer, false));
            renderercmds.target.push(OpsRendererTarget::Custom(copy_renderer, KeyCustomRenderTarget::FinalRender));
        // }
        (copy_renderer, copycamera)
    }
    pub fn init(
        commands: &mut Commands,
        materialcmd: &mut ActionSetMaterial,
        meshcmds: &mut ActionSetMesh,
        geometrycmd: &mut ActionSetGeometry,
        cameracmds: &mut ActionSetCamera,
        transformcmds: &mut ActionSetTransform,
        renderercmds: &mut ActionSetRenderer,
        scene: Entity,
        pre_renderer: Entity,
        next_renderer: Entity,
        source_render_target: Option<KeyCustomRenderTarget>,
        dst_render_target: Option<KeyCustomRenderTarget>,
    ) -> (Entity, Entity) {

        // {
            let copymat = commands.spawn_empty().id();
            materialcmd.create.push(OpsMaterialCreate::ops(copymat, ShaderImageCopy::KEY));
            
            if let Some(pre_render_target) = source_render_target {
                match pre_render_target {
                    KeyCustomRenderTarget::Custom(pre_render_target) => {
                        materialcmd.texturefromtarget.push(OpsUniformTextureFromRenderTarget::ops(copymat, UniformTextureWithSamplerParam { slotname: Atom::from(BlockMainTexture::KEY_TEX), ..Default::default() }, pre_render_target, Atom::from(BlockMainTexture::KEY_TILLOFF)));
                    },
                    KeyCustomRenderTarget::FinalRender => {},
                }

            }

            let id_geo = commands.spawn_empty().id();
            let attrs = QuadBuilder::attrs_meta();
            
            let plane = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(plane, scene));
            meshcmds.create.push(OpsMeshCreation::ops(scene, plane, MeshInstanceState { ..Default::default() }));
            meshcmds.depth_compare.push(OpsDepthCompare::ops(plane, CompareFunction::Always));
            geometrycmd.create.push(OpsGeomeryCreate::ops(plane, id_geo, attrs, Some(QuadBuilder::indices_meta())));
            materialcmd.usemat.push(OpsMaterialUse::ops(plane, copymat, PassTag::PASS_TAG_01));
            
            let copycamera = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(copycamera, scene));
            cameracmds.create.push(OpsCameraCreation::ops(scene, copycamera, false));
            meshcmds.layermask.push(OpsLayerMask::ops(copycamera, 0));
            meshcmds.layermask.push(OpsLayerMask::ops(plane, 0));
            cameracmds.forceinclude.push(OpsViewerForceInclude::ops(copycamera, plane, true));
            cameracmds.active.push(OpsCameraActive::ops(copycamera, true));
            
            let copy_renderer = commands.spawn_empty().id(); renderercmds.create.push(OpsRendererCreate::ops(copy_renderer, String::from("ImageCopy") + copy_renderer.to_bits().to_string().as_str(), copycamera, PassTag::PASS_TAG_01, false));
            renderercmds.modify.push(OpsRendererCommand::AutoClearColor(copy_renderer, false));
            renderercmds.modify.push(OpsRendererCommand::AutoClearDepth(copy_renderer, false));
            renderercmds.modify.push(OpsRendererCommand::AutoClearStencil(copy_renderer, false));
            renderercmds.connect.push(OpsRendererConnect::ops(pre_renderer, copy_renderer, false));
            renderercmds.connect.push(OpsRendererConnect::ops(copy_renderer, next_renderer, false));
            renderercmds.target.push(OpsRendererTarget::Custom(copy_renderer, dst_render_target.unwrap()));
        // }
        (copy_renderer, copycamera)
    }
}

pub struct ShaderImageCopy;
impl ShaderImageCopy {
    pub const KEY: &'static str = "ImageCopy";
    pub fn res() -> ShaderEffectMeta {
        let mut nodemat = NodeMaterialBuilder::new();
        nodemat.fs_define = String::from("
        layout(location = 0) out vec4 gl_FragColor;
        ");

        nodemat.binddefines = BindDefines::EFFECT_VALUE_BIND;

        nodemat.vs = String::from("
        gl_Position = vec4(A_POSITION, 1.);
        v_uv = gl_Position.xy + 0.5;
        gl_Position.xy *= 2.;
        ");
        nodemat.fs = String::from("
        gl_FragColor = mainTexture(v_uv, vec2(0., 0.));
        ");

        nodemat.varyings = Varyings(
            vec![
                Varying { format: Atom::from("vec2"), name: Atom::from("v_uv"), },
            ]
        );
        
        nodemat.apply::<BlockMainTexture>();

        nodemat.meta()
    }
}

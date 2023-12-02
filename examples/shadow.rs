use pi_engine_shell::prelude::*;
use pi_scene_context::prelude::*;

pub fn main() {}

pub struct DemoShadow;
impl DemoShadow {
    pub fn init(
        commands:&mut Commands,
        scene: Entity,
        light: Entity,
        pass: PassTag,
        pre_renderer: Option<Entity>,
        next_renderer: Entity,
        rendertarget: Option<KeyRenderTarget>,
        renderercmds: &mut ActionSetRenderer,
        shadowcmds: &mut ActionSetShadow,
    ) -> Entity {
        let shadow = commands.spawn_empty().id(); 
        shadowcmds.create.push(OpsShadowGenerator::ops(shadow, scene, light, pass));
        shadowcmds.param.push(OpsShadowGeneratorParam::Bias(shadow, 0.005));
        shadowcmds.param.push(OpsShadowGeneratorParam::NormalBias(shadow, 0.005));
        shadowcmds.param.push(OpsShadowGeneratorParam::ShadowFrustumSize(shadow, 10.0));
        shadowcmds.param.push(OpsShadowGeneratorParam::ShadowMinz(shadow, -100.0));
        shadowcmds.param.push(OpsShadowGeneratorParam::ShadowMaxz(shadow, 100.));

        renderercmds.create.push(OpsRendererCreate::ops(shadow, String::from("Shadow01"), shadow, pass, false));
        if let Some(pre_renderer) = pre_renderer {
            renderercmds.connect.push(OpsRendererConnect::ops(pre_renderer, shadow, false));
        }
        renderercmds.connect.push(OpsRendererConnect::ops(shadow, next_renderer, false));
        renderercmds.modify.push(OpsRendererCommand::AutoClearColor(shadow, true));
        renderercmds.modify.push(OpsRendererCommand::AutoClearDepth(shadow, true));
        renderercmds.modify.push(OpsRendererCommand::DepthClear(shadow, RenderDepthClear(1.)));
        renderercmds.modify.push(OpsRendererCommand::ColorClear(shadow, RenderColorClear(0, 0, 0, 0)));
        if let Some(key) = rendertarget {
            renderercmds.target.push(OpsRendererTarget::Custom(shadow, KeyCustomRenderTarget::Custom(key)));
        }

        shadow
    }
}
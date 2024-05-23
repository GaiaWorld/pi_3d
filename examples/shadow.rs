use pi_scene_shell::prelude::*;
use pi_scene_context::prelude::*;

pub fn main() {}

#[path = "./base.rs"]
mod base;

pub struct DemoShadow;
impl DemoShadow {
    // pub fn init(
    //     commands:&Insert<()>,
    //     scene: Entity,
    //     light: Entity,
    //     pass: PassTag,
    //     pre_renderer: Option<Entity>,
    //     next_renderer: Entity,
    //     rendertarget: Option<KeyRenderTarget>,
    //     actions: &mut pi_3d::ActionSets,
    // ) -> Entity {
    //     let shadow = commands.insert(()); 
    //     actions.shadow.create.push(OpsShadowGenerator::ops(shadow, scene, light, pass));
    //     actions.shadow.param.push(OpsShadowGeneratorParam::Bias(shadow, 0.005));
    //     actions.shadow.param.push(OpsShadowGeneratorParam::NormalBias(shadow, 0.005));
    //     actions.shadow.param.push(OpsShadowGeneratorParam::ShadowFrustumSize(shadow, 10.0));
    //     actions.shadow.param.push(OpsShadowGeneratorParam::ShadowMinz(shadow, -100.0));
    //     actions.shadow.param.push(OpsShadowGeneratorParam::ShadowMaxz(shadow, 100.));

    //     actions.renderer.create.push(OpsRendererCreate::ops(shadow, String::from("Shadow01"), shadow, pass, false));
    //     if let Some(pre_renderer) = pre_renderer {
    //         actions.renderer.connect.push(OpsRendererConnect::ops(pre_renderer, shadow, false));
    //     }
    //     actions.renderer.connect.push(OpsRendererConnect::ops(shadow, next_renderer, false));
    //     actions.renderer.modify.push(OpsRendererCommand::AutoClearColor(shadow, true));
    //     actions.renderer.modify.push(OpsRendererCommand::AutoClearDepth(shadow, true));
    //     actions.renderer.modify.push(OpsRendererCommand::DepthClear(shadow, RenderDepthClear(1.)));
    //     actions.renderer.modify.push(OpsRendererCommand::ColorClear(shadow, RenderColorClear(0, 0, 0, 0)));
    //     if let Some(key) = rendertarget {
    //         actions.renderer.target.push(OpsRendererTarget::Custom(shadow, KeyCustomRenderTarget::Custom(key)));
    //     }

    //     shadow
    // }
}
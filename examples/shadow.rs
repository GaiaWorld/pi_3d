use pi_scene_shell::prelude::*;
use pi_scene_context::prelude::*;

pub fn main() {}

#[path = "./base.rs"]
mod base;

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
        actions: &mut pi_3d::ActionSets,
    ) -> Entity {
        let shadow = commands.spawn_empty_id(); 
        actions.shadow.create.push(OpsShadowGenerator::ops(shadow, scene, light, pass, Entity::null()));
        actions.shadow.param.push(OpsShadowGeneratorParam::ops(shadow, EShadowGeneratorParam::Bias( 20. / 1024. * 0.001 )));
        actions.shadow.param.push(OpsShadowGeneratorParam::ops(shadow, EShadowGeneratorParam::NormalBias( 20. / 1024. * 0.001 )));
        actions.shadow.param.push(OpsShadowGeneratorParam::ops(shadow, EShadowGeneratorParam::ShadowFrustumSize( 20.0 )));
        actions.shadow.param.push(OpsShadowGeneratorParam::ops(shadow, EShadowGeneratorParam::ShadowMinz( 1.0 )));
        actions.shadow.param.push(OpsShadowGeneratorParam::ops(shadow, EShadowGeneratorParam::ShadowMaxz( 101. )));

        actions.renderer.create.push(OpsRendererCreate::ops(shadow, String::from("Shadow01"), shadow, pass, false, false, false));
        if let Some(pre_renderer) = pre_renderer {
            actions.renderer.connect.push(OpsRendererConnect::ops(pre_renderer, shadow, false));
        }
        actions.renderer.connect.push(OpsRendererConnect::ops(shadow, next_renderer, false));
        actions.renderer.modify.push(OpsRendererCommand::ops( shadow, ERendererCommand::AutoClearColor( true) ));
        actions.renderer.modify.push(OpsRendererCommand::ops( shadow, ERendererCommand::AutoClearDepth( true) ));
        actions.renderer.modify.push(OpsRendererCommand::ops( shadow, ERendererCommand::DepthClear( RenderDepthClear(1.)) ));
        actions.renderer.modify.push(OpsRendererCommand::ops( shadow, ERendererCommand::ColorClear( RenderColorClear(0, 0, 0, 0)) ));
        if let Some(key) = rendertarget {
            actions.renderer.target.push(OpsRendererTarget::new(shadow, ERendererTarget::Custom( KeyCustomRenderTarget::Custom(key), false) ));
        }

        shadow
    }
}
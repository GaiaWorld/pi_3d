
use pi_scene_shell::prelude::*;
use pi_scene_context::prelude::*;
use pi_scene_math::Vector3;

#[path = "./base.rs"]
mod base;

pub fn main() { }

pub struct DemoLight;
impl DemoLight {
    pub fn directlight(
        commands: &mut Commands,
        scene: Entity,
        parent: Entity,
        actions: &mut pi_3d::ActionSets,
    ) -> Entity {
        let light = commands.spawn_empty().id(); actions.transform.tree.push(OpsTransformNodeParent::ops(light, parent));
        actions.transform.localsrt.push(OpsTransformNodeLocal::ops(light, ETransformSRT::Translation(0., 20., -10.)));
        actions.mesh.layermask.push(OpsLayerMask::ops(light, 0xFFFFFFFF));
        actions.light.create.push(OpsLightCreate::ops(scene, light, ELightType::Direct));
        actions.light.param.push(OpsLightParam::ops(light, ELightModify::Directional( Vector3::new(-0., -2., 1.) )));
        actions.light.param.push(OpsLightParam::ops(light, ELightModify::Color(1. * 1.8, 1. * 1.8, 1. * 1.8)));

        light
    }

    pub fn directlight_custom(
        commands: &mut Commands,
        scene: Entity,
        parent: Entity,
        actions: &mut pi_3d::ActionSets,
        position: (f32, f32, f32),
        direction: (f32, f32, f32),
        color: (f32, f32, f32),
        layer: u32,
    ) -> Entity {
        let light = commands.spawn_empty().id(); actions.transform.tree.push(OpsTransformNodeParent::ops(light, parent));
        actions.transform.localsrt.push(OpsTransformNodeLocal::ops(light, ETransformSRT::Translation(position.0, position.1, position.2)));
        actions.mesh.layermask.push(OpsLayerMask::ops(light, layer));
        actions.light.create.push(OpsLightCreate::ops(scene, light, ELightType::Direct));
        actions.light.param.push(OpsLightParam::ops(light, ELightModify::Directional(Vector3::new(direction.0, direction.1, direction.2))));
        actions.light.param.push(OpsLightParam::ops(light, ELightModify::Color(color.0, color.1, color.2)));

        light
    }

    pub fn pointlight(
        commands: &mut Commands,
        scene: Entity,
        parent: Entity,
        actions: &mut pi_3d::ActionSets,
        position: (f32, f32, f32),
        color: (f32, f32, f32),
        layer: u32,
    ) -> Entity {
        let light = commands.spawn_empty().id(); actions.transform.tree.push(OpsTransformNodeParent::ops(light, parent));
        actions.transform.localsrt.push(OpsTransformNodeLocal::ops(light, ETransformSRT::Translation(position.0, position.1, position.2)));
        actions.mesh.layermask.push(OpsLayerMask::ops(light, layer));
        actions.light.create.push(OpsLightCreate::ops(scene, light, ELightType::Point));
        actions.light.param.push(OpsLightParam::ops(light, ELightModify::Color(color.0, color.1, color.2)));

        light
    }

    pub fn spotlight(
        commands: &mut Commands,
        scene: Entity,
        parent: Entity,
        actions: &mut pi_3d::ActionSets,
        position: (f32, f32, f32),
        direction: (f32, f32, f32),
        color: (f32, f32, f32),
        layer: u32,
    ) -> Entity {
        let light = commands.spawn_empty().id(); actions.transform.tree.push(OpsTransformNodeParent::ops(light, parent));
        actions.transform.localsrt.push(OpsTransformNodeLocal::ops(light, ETransformSRT::Translation(position.0, position.1, position.2)));
        actions.mesh.layermask.push(OpsLayerMask::ops(light, layer));
        actions.light.create.push(OpsLightCreate::ops(scene, light, ELightType::Spot));
        actions.light.param.push(OpsLightParam::ops(light, ELightModify::Directional(Vector3::new(direction.0, direction.1, direction.2))));
        actions.light.param.push(OpsLightParam::ops(light, ELightModify::Color(color.0, color.1, color.2)));

        light
    }
}


use pi_engine_shell::prelude::*;
use pi_scene_context::prelude::*;
use pi_scene_math::Vector3;

pub fn main() { }

pub struct DemoLight;
impl DemoLight {
    pub fn directlight(
        commands: &mut Commands,
        scene: Entity,
        parent: Entity,
        transformcmds: &mut ActionSetTransform,
        lightingcmds: &mut ActionSetLighting,
        layermask: &mut ActionListLayerMask,
    ) -> Entity {
        let light = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(light, parent));
        transformcmds.localpos.push(OpsTransformNodeLocalPosition::ops(light, 0., 20., -10.));
        layermask.push(OpsLayerMask::ops(light, 0xFFFFFFFF));
        lightingcmds.create.push(OpsLightCreate::ops(scene, light, ELightType::Direct));
        lightingcmds.param.push(ELightModifyCommand::Directional(light, Vector3::new(-0., -2., 1.)));
        lightingcmds.color.push(OpsLightColor::ops(light, 1. * 1.8, 1. * 1.8, 1. * 1.8));

        light
    }

    pub fn directlight_custom(
        commands: &mut Commands,
        scene: Entity,
        parent: Entity,
        transformcmds: &mut ActionSetTransform,
        lightingcmds: &mut ActionSetLighting,
        layermask: &mut ActionListLayerMask,
        position: (f32, f32, f32),
        direction: (f32, f32, f32),
        color: (f32, f32, f32),
        layer: u32,
    ) -> Entity {
        let light = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(light, parent));
        transformcmds.localpos.push(OpsTransformNodeLocalPosition::ops(light, position.0, position.1, position.2));
        layermask.push(OpsLayerMask::ops(light, layer));
        lightingcmds.create.push(OpsLightCreate::ops(scene, light, ELightType::Direct));
        lightingcmds.param.push(ELightModifyCommand::Directional(light, Vector3::new(direction.0, direction.1, direction.2)));
        lightingcmds.color.push(OpsLightColor::ops(light, color.0, color.1, color.2));

        light
    }

    pub fn pointlight(
        commands: &mut Commands,
        scene: Entity,
        parent: Entity,
        transformcmds: &mut ActionSetTransform,
        lightingcmds: &mut ActionSetLighting,
        layermask: &mut ActionListLayerMask,
        position: (f32, f32, f32),
        color: (f32, f32, f32),
        layer: u32,
    ) -> Entity {
        let light = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(light, parent));
        transformcmds.localpos.push(OpsTransformNodeLocalPosition::ops(light, position.0, position.1, position.2));
        layermask.push(OpsLayerMask::ops(light, layer));
        lightingcmds.create.push(OpsLightCreate::ops(scene, light, ELightType::Point));
        lightingcmds.color.push(OpsLightColor::ops(light, color.0, color.1, color.2));

        light
    }

    pub fn spotlight(
        commands: &mut Commands,
        scene: Entity,
        parent: Entity,
        transformcmds: &mut ActionSetTransform,
        lightingcmds: &mut ActionSetLighting,
        layermask: &mut ActionListLayerMask,
        position: (f32, f32, f32),
        direction: (f32, f32, f32),
        color: (f32, f32, f32),
        layer: u32,
    ) -> Entity {
        let light = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(light, parent));
        transformcmds.localpos.push(OpsTransformNodeLocalPosition::ops(light, position.0, position.1, position.2));
        layermask.push(OpsLayerMask::ops(light, layer));
        lightingcmds.create.push(OpsLightCreate::ops(scene, light, ELightType::Spot));
        lightingcmds.param.push(ELightModifyCommand::Directional(light, Vector3::new(direction.0, direction.1, direction.2)));
        lightingcmds.color.push(OpsLightColor::ops(light, color.0, color.1, color.2));

        light
    }
}

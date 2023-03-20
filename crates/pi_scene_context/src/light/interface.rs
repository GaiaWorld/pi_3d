use pi_atom::Atom;
use pi_engine_shell::{object::{ObjectID, InterfaceObject}, engine_shell::EnginShell};
use pi_scene_math::Vector3;

use crate::{scene::interface::InterfaceScene, transforms::interface::InterfaceTransformNode};

use super::{base::{LightingMode, Light}, command::{SingleLightCreateCommands, SingleLightModifyCommands, ELightModifyCommand}, shadow_generator::base::{ShadowMinZ, ShadowMaxZ, ShadowFrustumSize}};


pub trait TLight {
    fn create_light(&self, scene: ObjectID, name: Atom) -> ObjectID;
    fn light_direction(&self, entity: ObjectID, value: Vector3) -> &Self;
    fn light_mode(&self, entity: ObjectID, value: Light) -> &Self;
    fn lighting_mode(&self, entity: ObjectID, value: LightingMode) -> &Self;
    fn shadow_enable(&self, entity: ObjectID, value: bool) -> &Self;
    fn shadow_mixz(&self, entity: ObjectID, value: f32) -> &Self;
    fn shadow_maxz(&self, entity: ObjectID, value: f32) -> &Self;
    fn shadow_frustumsize(&self, entity: ObjectID, value: f32) -> &Self;
    fn shadow_bias(&self, entity: ObjectID, value: f32) -> &Self;
    fn shadow_normal_bias(&self, entity: ObjectID, value: f32) -> &Self;
    fn shadow_depth_scale(&self, entity: ObjectID, value: f32) -> &Self;
}

impl TLight  for EnginShell {
    fn create_light(&self, scene: ObjectID, name: Atom) -> ObjectID {
        let entity = self.new_object();

        self.add_to_scene(entity, scene);

        self.as_transform_node(entity);

        self.transform_parent(entity, scene);

        let cmds = self.world().get_resource_mut::<SingleLightCreateCommands>().unwrap();
        cmds.0.push((entity, name));

        entity
    }

    fn light_mode(&self, entity: ObjectID, value: Light) -> &Self {
        
        let cmds = self.world().get_resource_mut::<SingleLightModifyCommands>().unwrap();
        cmds.0.push(ELightModifyCommand::LightType(entity, value));

        self
    }

    fn lighting_mode(&self, entity: ObjectID, value: LightingMode) -> &Self {
        
        let cmds = self.world().get_resource_mut::<SingleLightModifyCommands>().unwrap();
        cmds.0.push(ELightModifyCommand::LightingType(entity, value));

        self
    }

    fn shadow_enable(&self, entity: ObjectID, value: bool) -> &Self {
        
        let cmds = self.world().get_resource_mut::<SingleLightModifyCommands>().unwrap();
        cmds.0.push(ELightModifyCommand::ShadowEnable(entity, value));

        self
    }

    fn shadow_mixz(&self, entity: ObjectID, value: f32) -> &Self {
        
        let cmds = self.world().get_resource_mut::<SingleLightModifyCommands>().unwrap();
        cmds.0.push(ELightModifyCommand::ShadowMinz(entity, value));

        self
    }

    fn shadow_maxz(&self, entity: ObjectID, value: f32) -> &Self {
        
        let cmds = self.world().get_resource_mut::<SingleLightModifyCommands>().unwrap();
        cmds.0.push(ELightModifyCommand::ShadowMaxz(entity, value));

        self
    }

    fn shadow_frustumsize(&self, entity: ObjectID, value: f32) -> &Self {
        
        let cmds = self.world().get_resource_mut::<SingleLightModifyCommands>().unwrap();
        cmds.0.push(ELightModifyCommand::ShadowFrustumSize(entity, value));

        self
    }

    fn shadow_bias(&self, entity: ObjectID, value: f32) -> &Self {
        
        let cmds = self.world().get_resource_mut::<SingleLightModifyCommands>().unwrap();
        cmds.0.push(ELightModifyCommand::Bias(entity, value));

        self
    }

    fn shadow_normal_bias(&self, entity: ObjectID, value: f32) -> &Self {
        
        let cmds = self.world().get_resource_mut::<SingleLightModifyCommands>().unwrap();
        cmds.0.push(ELightModifyCommand::NormalBias(entity, value));

        self
    }

    fn shadow_depth_scale(&self, entity: ObjectID, value: f32) -> &Self {
        
        let cmds = self.world().get_resource_mut::<SingleLightModifyCommands>().unwrap();
        cmds.0.push(ELightModifyCommand::DepthScale(entity, value));

        self
    }

    fn light_direction(&self, entity: ObjectID, value: Vector3) -> &Self {
        
        let cmds = self.world().get_resource_mut::<SingleLightModifyCommands>().unwrap();
        cmds.0.push(ELightModifyCommand::Directional(entity, value));

        self
    }

}
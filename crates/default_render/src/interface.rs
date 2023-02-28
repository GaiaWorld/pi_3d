
use pi_atom::Atom;
use pi_engine_shell::{object::InterfaceObject};

use pi_render::renderer::shader::KeyShaderMeta;
use pi_scene_context::{object::ObjectID, materials::{interface::{InterfaceMaterial}, interface::InterfaceMaterialMeta}, pass::EPassTag};

use crate::shader::DefaultShader;

use super::{command::{SingeDefaultMaterialCommandList, DefaultMaterialCommand}, SingleIDBaseDefaultMaterial};


pub trait InterfaceDefaultMaterial {
    fn create_default_material(
        & self,
        pass: EPassTag,
    ) -> ObjectID;
    fn use_default_material(
        &self,
        entity: ObjectID,
    ) -> &Self;
    fn emissive_color(
        &self,
        entity: ObjectID,
        color: (f32, f32, f32),
    ) -> &Self;
    fn emissive_intensity(
        &self,
        entity: ObjectID,
        intensity: f32,
    ) -> &Self;
}

impl InterfaceDefaultMaterial for crate::engine::Engine {
    fn create_default_material(
        & self,
        pass: EPassTag,
    ) -> ObjectID {
        //  log::debug!("create_default_material");
        let entity = self.new_object();

        self.as_material(entity, KeyShaderMeta::from(DefaultShader::KEY), pass);

        entity
    }
    fn use_default_material(
        &self,
        entity: ObjectID,
    ) -> &Self {

        let id = self.world().get_resource::<SingleIDBaseDefaultMaterial>().unwrap();
        self.use_material(entity, id.0.0.clone());

        self
    }
    fn emissive_color(
        &self,
        entity: ObjectID,
        color: (f32, f32, f32),
    ) -> &Self {
        let world = self.world();
        let commands = world.get_resource_mut::<SingeDefaultMaterialCommandList>().unwrap();
        commands.list.push(DefaultMaterialCommand::EmissiveColor(entity, color));

        log::debug!("emissive_color >>>>>>>>>>");

        self
    }
    fn emissive_intensity(
        &self,
        entity: ObjectID,
        intensity: f32,
    ) -> &Self {
        let world = self.world();
        let commands = world.get_resource_mut::<SingeDefaultMaterialCommandList>().unwrap();
        commands.list.push(DefaultMaterialCommand::EmissiveIntensity(entity, intensity));

        self
    }
}
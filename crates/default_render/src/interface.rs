
use pi_atom::Atom;
use pi_engine_shell::{object::InterfaceObject};

use pi_scene_context::{object::ObjectID, materials::{material::{InterfaceMaterial}, material_meta::InterfaceMaterialMeta}};
use render_shader::shader::{KeyPreShader};

use crate::shader::DefaultShader;

use super::{command::{SingeDefaultMaterialCommandList, DefaultMaterialCommand}, SingleIDBaseDefaultMaterial};


pub trait InterfaceDefaultMaterial {
    fn create_default_material(
        & self,
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
    ) -> ObjectID {
        //  println!("create_default_material");
        let entity = self.new_object();

        self.as_material(entity, KeyPreShader(Atom::from(DefaultShader::KEY)));
        self.emissive_color(entity, (1., 1., 1.));
        self.emissive_intensity(entity, 1.);

        entity
    }
    fn use_default_material(
        &self,
        entity: ObjectID,
    ) -> &Self {

        let id = self.world().get_resource::<SingleIDBaseDefaultMaterial>().unwrap();
        self.use_material(entity, id.0);

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

        println!("emissive_color >>>>>>>>>>");

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
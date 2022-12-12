use pi_atom::Atom;
use pi_engine_shell::object::InterfaceObject;

use pi_scene_context::{object::ObjectID, materials::{material::{SingleMaterialIDCommandList, MaterialIDCommand}, material_meta::InterfaceMaterialMeta}};
use render_resource::ImageAssetKey;
use render_shader::shader::KeyShaderEffect;

use crate::shader::UnlitShader;

use super::{command::{SingleUnlitMaterialCommandList, EUnlitMaterialCommand}};


pub trait InterfaceUnlitMaterial {
    fn create_unlit_material(
        & self,
    ) -> ObjectID;
    fn emissive_texture(
        & self,
        entity: ObjectID,
        image: ImageAssetKey,
    ) -> &Self;
}

impl InterfaceUnlitMaterial for pi_engine_shell::engine_shell::EnginShell {
    fn create_unlit_material(
        & self,
    ) -> ObjectID {
        //  println!("create_unlit_material");
        let entity = self.new_object();

        self.as_material(entity, KeyShaderEffect(Atom::from(UnlitShader::KEY)));

        entity
    }
    fn emissive_texture(
        & self,
        entity: ObjectID,
        image: ImageAssetKey,
    ) -> &Self {
        let world = self.world();
        let commands = world.get_resource_mut::<SingleUnlitMaterialCommandList>().unwrap();
        commands.list.push(EUnlitMaterialCommand::EmissiveTexture(entity, image));

        self
    }
}
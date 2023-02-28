use pi_atom::Atom;
use pi_engine_shell::object::InterfaceObject;

use pi_render::{render_3d::shader::uniform_texture::UniformTextureWithSamplerParam, renderer::shader::KeyShaderMeta};
use pi_scene_context::{object::ObjectID, materials::{ interface::InterfaceMaterialMeta}, pass::EPassTag};

use crate::shader::UnlitShader;

use super::{command::{SingleUnlitMaterialCommandList, EUnlitMaterialCommand}};


pub trait InterfaceUnlitMaterial {
    fn create_unlit_material(
        & self,
        pass: EPassTag,
    ) -> ObjectID;
    fn emissive_texture(
        & self,
        entity: ObjectID,
        image: UniformTextureWithSamplerParam,
    ) -> &Self;
}

impl InterfaceUnlitMaterial for pi_engine_shell::engine_shell::EnginShell {
    fn create_unlit_material(
        & self,
        pass: EPassTag,
    ) -> ObjectID {
        //  log::debug!("create_unlit_material");
        let entity = self.new_object();

        self.as_material(entity, KeyShaderMeta::from(UnlitShader::KEY), pass);

        entity
    }
    fn emissive_texture(
        & self,
        entity: ObjectID,
        image: UniformTextureWithSamplerParam,
    ) -> &Self {
        let world = self.world();
        let commands = world.get_resource_mut::<SingleUnlitMaterialCommandList>().unwrap();
        commands.list.push(EUnlitMaterialCommand::EmissiveTexture(entity, image));

        self
    }
}
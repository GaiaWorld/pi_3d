use pi_engine_shell::prelude::*;

use pi_scene_context::prelude::*;

use crate::shader::UnlitShader;

use super::{command::{ActionListUnlitMaterial, EUnlitMaterialCommand}};

pub struct ActionUnlitMaterial;
impl ActionUnlitMaterial {
    pub fn create(
        app: &mut App,
        name: String,
        pass: EPassTag,
    ) -> Entity {
        let mut queue = CommandQueue::default();
        let mut commands = Commands::new(&mut queue, &app.world);

        let entity = commands.spawn_empty().id();
        queue.apply(&mut app.world);

        ActionMaterial::init(app, entity, KeyShaderMeta::from(UnlitShader::KEY), pass);

        entity
    }
    pub fn emissive_texture(
        app: &mut App,
        entity: ObjectID,
        image: UniformTextureWithSamplerParam,
    ) {
        let mut cmds = app.world.get_resource_mut::<ActionListUnlitMaterial>().unwrap();
        cmds.push(EUnlitMaterialCommand::EmissiveTexture(entity, image));
    }
}

// pub trait InterfaceUnlitMaterial {
//     fn create_unlit_material(
//         & self,
//         pass: EPassTag,
//     ) -> ObjectID;
//     fn emissive_texture(
//         & self,
//         entity: ObjectID,
//         image: UniformTextureWithSamplerParam,
//     ) -> &Self;
// }

// impl InterfaceUnlitMaterial for pi_engine_shell::engine_shell::EnginShell {
//     fn create_unlit_material(
//         & self,
//         pass: EPassTag,
//     ) -> ObjectID {
//         //  log::debug!("create_unlit_material");
//         let entity = self.new_object();

//         self.as_material(entity, KeyShaderMeta::from(UnlitShader::KEY), pass);

//         entity
//     }
//     fn emissive_texture(
//         & self,
//         entity: ObjectID,
//         image: UniformTextureWithSamplerParam,
//     ) -> &Self {
//         let world = self.world();
//         let commands = world.get_resource_mut::<SingleUnlitMaterialCommandList>().unwrap();
//         commands.list.push(EUnlitMaterialCommand::EmissiveTexture(entity, image));

//         self
//     }
// }
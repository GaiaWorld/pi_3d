use pi_atom::Atom;
use pi_engine_shell::prelude::*;

use pi_render::renderer::shader::KeyShaderMeta;
use pi_scene_context::prelude::*;

use super::shader::BRDFShader;


pub struct InterfaceBRDFMaterial;
impl InterfaceBRDFMaterial {
    pub fn create_brdf_material(
        app: &mut App,
    ) -> ObjectID {
        let mut queue = CommandQueue::default();
        let mut commands = Commands::new(&mut queue, &app.world);

        let entity = commands.spawn_empty().id();
        queue.apply(&mut app.world);

        ActionMaterial::init(app, entity, KeyShaderMeta::from(BRDFShader::KEY), EPassTag::Opaque);

        entity
    }
}

// impl InterfaceBRDFMaterial for pi_engine_shell::engine_shell::EnginShell {
//     fn create_brdf_material(
//         & self,
//     ) -> ObjectID {
//         let entity = self.new_object();
//         self.as_material(entity, KeyShaderMeta::from(BRDFShader::KEY), EPassTag::Opaque);

//         entity
//     }
// }
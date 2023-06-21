
use pi_engine_shell::prelude::*;
use pi_scene_context::prelude::*;

use super::shader::CloudShader;


pub struct InterfaceCloudMaterial;
impl InterfaceCloudMaterial {
    pub fn create_cloud_material(
        app: &mut App,
    ) -> Entity {
        let mut queue = CommandQueue::default();
        let mut commands = Commands::new(&mut queue, &app.world);

        let entity = commands.spawn_empty().id();
        queue.apply(&mut app.world);

        ActionMaterial::init(app, entity, KeyShaderMeta::from(CloudShader::KEY), EPassTag::Sky);

        entity
    }
}

// impl InterfaceCloudMaterial for EnginShell {
//     fn create_cloud_material(
//         &self,
//     ) -> ObjectID {
//         let entity = self.new_object();
//         self.as_material(entity, Atom::from(CloudShader::KEY), EPassTag::Sky);

//         entity
//     }
// }
use pi_engine_shell::prelude::*;
use pi_scene_context::prelude::*;
use crate::shader::AxisShader;

pub struct InterfaceAxisMaterial;
impl InterfaceAxisMaterial {
    pub fn create(
        app: &mut App
    ) -> ObjectID {
        let mut queue = CommandQueue::default();
        let mut commands = Commands::new(&mut queue, &app.world);

        let entity = commands.spawn_empty().id();
        queue.apply(&mut app.world);

        ActionMaterial::init(app, entity, KeyShaderMeta::from(AxisShader::KEY), EPassTag::Opaque);

        entity
    }
}

// impl InterfaceAxisMaterial for engine_shell::EnginShell {
//     fn create_axis_material(&self) -> ObjectID {
//         log::debug!("create_default_material");
//         let entity = self.new_object();
//         self.as_material(entity, KeyShaderMeta::from(AxisShader::KEY), EPassTag::Opaque);

//         entity
//     }
    
// }

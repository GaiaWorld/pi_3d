use pi_atom::Atom;
use pi_engine_shell::prelude::*;
use pi_scene_context::{pass::EPassTag, materials::command::ActionMaterial};

use crate::water::shader::WaterShader;




pub struct InterfaceWaterMaterial;
impl InterfaceWaterMaterial {
    pub fn create_water_material(
        app: &mut App,
    ) -> Entity {
        let mut queue = CommandQueue::default();
        let mut commands = Commands::new(&mut queue, &app.world);

        let entity = commands.spawn_empty().id();
        queue.apply(&mut app.world);

        ActionMaterial::init(app, entity, KeyShaderMeta::from(WaterShader::KEY), EPassTag::Opaque);

        entity
    }
}


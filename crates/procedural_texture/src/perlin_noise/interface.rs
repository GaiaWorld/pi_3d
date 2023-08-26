
pub trait InterfacePerlinNoiseMaterial {
    // fn create_perlinnoise_material(
    //     app: &mut App,
    // ) -> Entity {
    //     let mut queue = CommandQueue::default();
    //     let mut commands = Commands::new(&mut queue, &app.world);

    //     let entity = commands.spawn_empty().id();
    //     queue.apply(&mut app.world);

    //     ActionMaterial::init(app, entity, KeyShaderMeta::from(PerlinNoiseShader::KEY), EPassTag::Opaque);

    //     entity
    // }
}

// impl InterfacePerlinNoiseMaterial for EnginShell {
//     fn create_perlinnoise_material(
//         &self,
//     ) -> ObjectID {
//         let entity = self.new_object();
//         self.as_material(entity, Atom::from(PerlinNoiseShader::KEY), EPassTag::Opaque);

//         entity
//     }
// }
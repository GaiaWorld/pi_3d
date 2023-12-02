
pub struct ActionDefaultMaterial;
impl ActionDefaultMaterial {
    // pub fn create(
    //     app: &mut App,
    //     pass: EPassTag,
    // ) -> Entity {
    //     let mut queue = CommandQueue::default();
    //     let mut commands = Commands::new(&mut queue, &app.world);

    //     let entity = commands.spawn_empty().id();
    //     queue.apply(&mut app.world);

    //     // ActionMaterial::init(app, entity, KeyShaderMeta::from(DefaultShader::KEY), pass);

    //     entity
    // }
    // pub fn use_default_material(
    //     app: &mut App,
    //     id_mesh: Entity,
    // ) {
    //     let mat = app.world.get_resource::<SingleIDBaseDefaultMaterial>().unwrap().0;
    //     ActionMaterial::use_material(app, OpsMaterialUse::ops(id_mesh, mat));
    // }
}

// pub trait InterfaceDefaultMaterial {
//     fn create_default_material(
//         & self,
//         pass: EPassTag,
//     ) -> ObjectID;
//     fn use_default_material(
//         &self,
//         entity: ObjectID,
//     ) -> &Self;
//     fn emissive_color(
//         &self,
//         entity: ObjectID,
//         color: (f32, f32, f32),
//     ) -> &Self;
//     fn emissive_intensity(
//         &self,
//         entity: ObjectID,
//         intensity: f32,
//     ) -> &Self;
// }

// impl InterfaceDefaultMaterial for crate::engine::Engine {
//     fn create_default_material(
//         & self,
//         pass: EPassTag,
//     ) -> ObjectID {
//         //  log::debug!("create_default_material");
//         let entity = self.new_object();

//         self.as_material(entity, KeyShaderMeta::from(DefaultShader::KEY), pass);

//         entity
//     }
//     fn use_default_material(
//         &self,
//         entity: ObjectID,
//     ) -> &Self {

//         let id = self.world().get_resource::<SingleIDBaseDefaultMaterial>().unwrap();
//         self.use_material(entity, id.0.0.clone());

//         self
//     }
//     fn emissive_color(
//         &self,
//         entity: ObjectID,
//         color: (f32, f32, f32),
//     ) -> &Self {
//         let world = self.world();
//         let commands = world.get_resource_mut::<SingeDefaultMaterialCommandList>().unwrap();
//         commands.list.push(DefaultMaterialCommand::EmissiveColor(entity, color));

//         log::debug!("emissive_color >>>>>>>>>>");

//         self
//     }
//     fn emissive_intensity(
//         &self,
//         entity: ObjectID,
//         intensity: f32,
//     ) -> &Self {
//         let world = self.world();
//         let commands = world.get_resource_mut::<SingeDefaultMaterialCommandList>().unwrap();
//         commands.list.push(DefaultMaterialCommand::EmissiveIntensity(entity, intensity));

//         self
//     }
// }
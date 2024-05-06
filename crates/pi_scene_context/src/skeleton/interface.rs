use pi_scene_shell::prelude::*;


pub trait TInterfaceSkeleton {
    fn create_skeleton_ubo(
        &self,
        bone_mode: ESkinBonesPerVertex,
        bone_root: ObjectID,
        bones: Vec<ObjectID>,
    ) -> ObjectID;

    fn use_skeleton(
        &self,
        mesh: ObjectID,
        skin: ObjectID,
    ) -> &Self;
}

// impl TInterfaceSkeleton for pi_scene_shell::engine_shell::EnginShell {
//     fn create_skeleton_ubo(
//         &self,
//         bone_mode: ESkinBonesPerVertex,
//         bone_root: ObjectID,
//         bones: Vec<ObjectID>,
//     ) -> ObjectID {
//         let id = self.new_object();

//         let cmds = self.world().get_single_res_mut::<SingleSkinCreateCommands>().unwrap();
//         cmds.0.push(ESkinCreateCommand::UBO(id, bone_mode, (bone_root, bones)));

//         id
//     }

//     fn use_skeleton(
//         &self,
//         mesh: ObjectID,
//         skin: ObjectID,
//     ) -> &Self {
        
//         let cmds = self.world().get_single_res_mut::<SingleSkinModifyCommands>().unwrap();
//         cmds.0.push(ESkinModifyCommand::Use(mesh, skin));

//         self
//     }
// }
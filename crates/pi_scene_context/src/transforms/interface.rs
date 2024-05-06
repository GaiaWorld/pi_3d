
use pi_scene_shell::prelude::*;
use pi_scene_math::{Vector3, Quaternion};



pub trait InterfaceTransformNode {
    fn create_transform_node(
        & self,
        scene: ObjectID,
    ) -> ObjectID;

    fn as_transform_node(
        & self,
        object: ObjectID,
    ) -> & Self;

    fn transform_position(
        & self,
        node: ObjectID,
        position: Vector3
    ) -> & Self;

    fn transform_parent(
        & self,
        node: ObjectID,
        parent: ObjectID,
    ) -> & Self;

    fn transform_scaling(
        & self,
        node: ObjectID,
        scaling: Vector3
    ) -> & Self;

    fn transform_rotation_euler(
        & self,
        node: ObjectID,
        euler_angle: Vector3
    ) -> & Self;

    fn transform_rotation_quaternion(
        & self,
        node: ObjectID,
        quat: Quaternion
    ) -> & Self;
}
// impl InterfaceTransformNode for crate::engine::Engine {
//     fn create_transform_node(
//         & self,
//         scene: ObjectID,
//     ) -> ObjectID {

//         let entity = self.new_object();
//         // let world = self.world_mut();

//         self.add_to_scene(entity, scene);

//         self.as_transform_node(entity);

//         self.transform_parent(entity, scene);

//         entity
//     }

//     fn as_transform_node(
//         & self,
//         object: ObjectID,
//     ) -> & Self {
//         let world = self.world();

//         let commands = world.get_single_res_mut::<SingleTransformNodeCreateCommandList>().unwrap();
//         commands.list.push(ETransformNodeCreateCommand::Create(object));

//         let commands = world.get_single_res_mut::<SingleLayerMaskCommandList>().unwrap();
//         commands.list.push(LayerMaskCommand::Set(object, LayerMask::default()));
        
//         self
//     }

//     fn transform_position(
//         & self,
//         node: ObjectID,
//         position: Vector3
//     ) -> & Self {
//         let world = self.world();

//         let commands = world.get_single_res_mut::<SingleTransformNodeModifyCommandList>().unwrap();
//         commands.list.push(ETransformNodeModifyCommand::ModifyPosition(node, position));

//         self
//     }

//     fn transform_rotation_euler(
//         & self,
//         node: ObjectID,
//         euler_angle: Vector3
//     ) -> & Self {
//         let world = self.world();

//         let commands = world.get_single_res_mut::<SingleTransformNodeModifyCommandList>().unwrap();
//         commands.list.push(ETransformNodeModifyCommand::ModifyRotation(node, euler_angle));

//         self
//     }

//     fn transform_parent(
//         & self,
//         node: ObjectID,
//         parent: ObjectID,
//     ) -> & Self {
//         let world = self.world();

//         let commands = world.get_single_res_mut::<SingleTreeCommandList>().unwrap();
//         commands.list.push(TreeCommand::Remove(node));
//         commands.list.push(TreeCommand::Append(node, parent));

//         self
//     }

//     fn transform_rotation_quaternion(
//         & self,
//         node: ObjectID,
//         quat: Quaternion
//     ) -> & Self {
//         let world = self.world();

//         let commands = world.get_single_res_mut::<SingleTransformNodeModifyCommandList>().unwrap();
//         commands.list.push(ETransformNodeModifyCommand::ModifyRotationQuaternion(node, quat));

//         self
//     }

//     fn transform_scaling(
//         & self,
//         node: ObjectID,
//         scaling: Vector3
//     ) -> & Self {
//         let world = self.world();

//         let commands = world.get_single_res_mut::<SingleTransformNodeModifyCommandList>().unwrap();
//         commands.list.push(ETransformNodeModifyCommand::ModifyScaling(node, scaling));

//         self
//     }
// }


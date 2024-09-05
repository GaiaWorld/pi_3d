use pi_scene_shell::prelude::*;
use pi_scene_math::Vector4;


pub trait InterfaceMesh {
    fn create_mesh(
        & self,
        scene: ObjectID,
    ) -> ObjectID;

    fn as_mesh(
        & self,
        object: ObjectID,
    ) -> & Self;
    
    fn create_instanced_mesh(
        & self,
        scene: ObjectID,
        source: ObjectID,
    ) -> ObjectID;

    fn set_instance_color(
        & self,
        instance: ObjectID,
        color: Vector4,
    ) -> &Self;
    
    fn set_instance_tilloff(
        & self,
        instance: ObjectID,
        value: Vector4,
    ) -> &Self;

    fn cast_shadow(
        &self,
        instance: ObjectID,
        value: bool,
    ) -> &Self;
    
    fn receive_shadow(
        &self,
        instance: ObjectID,
        value: bool,
    ) -> &Self;
}

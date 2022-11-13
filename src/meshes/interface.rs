use crate::{object::ObjectID, transforms::interface::InterfaceTransformNode, scene::interface::InterfaceScene};

use super::command::{SingleMeshCommandList, MeshCommand};


pub trait InterfaceMesh {
    fn create_mesh(
        & self,
        scene: ObjectID,
    ) -> ObjectID;

    fn as_mesh(
        & self,
        object: ObjectID,
    ) -> & Self;
}
impl InterfaceMesh for crate::engine::Engine {
    fn create_mesh(
        & self,
        scene: ObjectID,
    ) -> ObjectID {

        let entity = self.new_object();
        let world = self.world();

        self.add_to_scene(entity, scene);
        self.as_transform_node(entity);
        self.transform_parent(entity, scene);

        self.as_mesh(entity);

        entity
    }

    fn as_mesh(
        & self,
        object: ObjectID,
    ) -> & Self {
        let world = self.world();

        let commands = world.get_resource_mut::<SingleMeshCommandList>().unwrap();
        commands.list.push(MeshCommand::Create(object));

        self
    }
}
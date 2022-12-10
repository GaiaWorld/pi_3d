use pi_engine_shell::object::InterfaceObject;

use crate::{object::ObjectID, transforms::interface::InterfaceTransformNode, scene::interface::InterfaceScene, renderers::{render_mode::{InterfaceRenderMode, ERenderMode}, render_sort::{InterfaceRenderSort, RenderSortParam}, render_blend::InterfaceRenderBlend, render_depth_and_stencil::InterfaceRenderDepthAndStencil, render_primitive::{InterfaceRenderPrimitive, PrimitiveState}}, layer_mask::{interface::InterfaceLayerMask, LayerMask}};

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

        self.render_sort(object, RenderSortParam::opaque());
        self.render_mode(object, ERenderMode::Opaque);
        self.disable_blend(object);
        self.disable_depth_stencil(object);
        self.layer_mask(object, LayerMask::default());
        self.primitive(object, PrimitiveState::default());

        self
    }
}
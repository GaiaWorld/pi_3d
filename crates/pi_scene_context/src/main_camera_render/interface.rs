use pi_engine_shell::object::InterfaceObject;

use crate::{renderers::render_object::RendererID, object::ObjectID};

use super::command::{SingleMainCameraRenderCommandList, MainCameraRenderCommand};


pub trait InterfaceMainCamera {
    fn active_camera(
        & self,
        object: ObjectID,
        flag: bool,
    ) -> &Self;
}
impl InterfaceMainCamera for crate::engine::Engine {
    fn active_camera(
        & self,
        object: ObjectID,
        flag: bool,
    ) -> &Self {
        if flag {
            let render_id = self.new_object();

            let world = self.world();
            let commands = world.get_resource_mut::<SingleMainCameraRenderCommandList>().unwrap();
            commands.list.push(MainCameraRenderCommand::Active(object, RendererID(render_id), None));
        } else {
            //
        }

        self
    }
}
use crate::object::ObjectID;

use super::{command::{SingleLayerMaskCommandList, LayerMaskCommand}, LayerMask};

pub trait InterfaceLayerMask {
    fn layer_mask(
        & self,
        object: ObjectID,
        layer: LayerMask,
    ) -> & Self;
}

impl InterfaceLayerMask for crate::engine::Engine {
    fn layer_mask(
        & self,
        object: ObjectID,
        layer: LayerMask,
    ) -> & Self {
        let world = self.world();

        let commands = world.get_resource_mut::<SingleLayerMaskCommandList>().unwrap();
        commands.list.push(LayerMaskCommand::Set(object, layer));

        self
    }
}

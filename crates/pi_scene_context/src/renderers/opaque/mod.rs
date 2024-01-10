

use pi_scene_shell::prelude::*;

pub trait InterfaceOpaque {
    fn as_opaque(
        &self,
        entity: ObjectID,
    ) -> &Self;

    fn not_opaque(
        &self,
        entity: ObjectID,
    ) -> &Self;
}
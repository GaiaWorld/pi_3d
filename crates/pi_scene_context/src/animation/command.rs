
use pi_scene_shell::prelude::*;

pub enum ETargetAnimationType {
    Position,
    Euler,
    Quaternion,
    Scaling,
    Fov,
    Uniform(Atom),
    Instance(Atom),
}

pub enum EPropertyAnimationValueType {
    LocalPosition,
    LocalEuler,
    LocalQuaternion,
    LocalScaling,
    IndicesRange,
    Fov,
    OrthSize,
    Enable,
}

pub struct OpsPropertyTargetAnimation(pub(crate) Entity, pub(crate) Entity, pub(crate) EPropertyAnimationValueType, pub(crate) u64);
impl OpsPropertyTargetAnimation {
    pub fn ops(target: Entity, group: Entity, vtype: EPropertyAnimationValueType, curve: u64) -> Self {
        Self(target, group, vtype, curve)
    }
}
pub type ActionListPropertyTargetAnimation = ActionList<OpsPropertyTargetAnimation>;


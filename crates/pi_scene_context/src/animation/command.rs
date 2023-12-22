use pi_animation::animation::AnimationInfo;
use pi_engine_shell::prelude::*;
use pi_slotmap::DefaultKey;

pub enum ETargetAnimationType {
    Position,
    Euler,
    Quaternion,
    Scaling,
    Fov,
    Uniform(Atom),
    Instance(Atom),
}



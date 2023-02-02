use pi_engine_shell::object::ObjectID;
use pi_scene_math::Matrix;

pub struct BoneParent(pub ObjectID);

pub struct BoneAbsolute(pub Matrix);

pub struct BoneAbsoluteInv(pub Matrix);

/// * Gets the base matrix (initial matrix which remains unchanged)
pub struct BoneBaseMatrix(pub Matrix);

pub struct BoneDifferenceMatrix(pub Matrix);

pub struct BoneMatrix(pub Matrix);
use std::ops::Mul;

use pi_engine_shell::prelude::*;
use pi_scene_math::Matrix;

use super::skeleton::*;

#[derive(Component)]
pub struct BoneParent(pub ObjectID);

#[derive(Component)]
pub struct BoneAbsolute(pub Matrix);
impl BoneAbsolute {
    pub fn update(&mut self, rootmatrix: &Matrix) {
        self.0 = self.0 * rootmatrix;
    }
}

#[derive(Component)]
pub struct BoneAbsoluteInv(pub Matrix);
impl BoneAbsoluteInv {
    pub fn update(&mut self, abs: &BoneAbsolute) {
        if abs.0.is_invertible() {
            self.0 = abs.0.try_inverse().unwrap();
        }
    }
}

/// * Gets the base matrix (initial matrix which remains unchanged)
#[derive(Component)]
pub struct BoneBaseMatrix(pub Matrix);

#[derive(Component)]
pub struct BoneDifferenceMatrix(pub Matrix);
impl BoneDifferenceMatrix {
    pub fn update(&mut self, rootmatrix: &Matrix) {
        self.0 = self.0.mul(rootmatrix);
    }
}

#[derive(Component)]
pub struct BoneMatrix(pub Matrix);

pub struct ActionBone;
impl ActionBone {
    pub fn init(
        commands: &mut EntityCommands,
        empty: &SingleEmptyEntity,
        parent: Entity,
    ) {
        commands
            .insert(BoneParent(parent))
            .insert(BoneAbsolute(Matrix::identity()))
            .insert(BoneAbsoluteInv(Matrix::identity()))
            .insert(BoneDifferenceMatrix(Matrix::identity()))
            .insert(BoneMatrix(Matrix::identity()))
            .insert(BoneBaseMatrix(Matrix::identity()))
            // .insert(SkeletonID(empty.id()))
        ;
    }
    pub fn modify_pose(
        commands: &mut EntityCommands,
        pose: Matrix,
    ) {
        commands.insert(BoneBaseMatrix(pose));
    }
    pub fn modify_skin(
        commands: &mut EntityCommands,
        id_skin: Entity,
    ) {
        commands.insert(SkeletonID(id_skin));
    }
}
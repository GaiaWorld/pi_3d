use std::ops::Mul;

use pi_scene_shell::prelude::*;
use pi_scene_math::Matrix;

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

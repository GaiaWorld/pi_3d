
use pi_scene_shell::prelude::*;
use pi_scene_math::{Vector4, Matrix, Matrix2, Vector2, Number};

pub trait Value {
    fn name(&self) -> &str;
}

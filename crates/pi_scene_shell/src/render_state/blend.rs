
use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ModelBlend {
    pub enable: bool,
    pub src_color: BlendFactor,
    pub dst_color: BlendFactor,
    pub src_alpha: BlendFactor,
    pub dst_alpha: BlendFactor,
    pub opt_color: BlendOperation,
    pub opt_alpha: BlendOperation,
}
impl Default for ModelBlend {
    fn default() -> Self {
        Self {
            enable: false,
            src_color: BlendFactor::SrcAlpha,
            dst_color: BlendFactor::OneMinusSrcAlpha,
            src_alpha: BlendFactor::One,
            dst_alpha: BlendFactor::OneMinusSrcAlpha,
            opt_color: BlendOperation::Add,
            opt_alpha: BlendOperation::Add,
        }
    }
}
impl ModelBlend {
    pub fn combine(&mut self) {
        self.enable = true;
    }
    pub fn one_one() -> Self {
        Self {
            enable: true,
            src_color: BlendFactor::One,
            dst_color: BlendFactor::One,
            src_alpha: BlendFactor::One,
            dst_alpha: BlendFactor::One,
            opt_color: BlendOperation::Add,
            opt_alpha: BlendOperation::Add,
        }
    }
    pub fn to_string(&self) -> String {
        if self.enable {
            String::from("None")
        } else { 
            String::from(format!("{:?} {:?} {:?} {:?} {:?} {:?}", self.src_color, self.dst_color, self.src_alpha, self.dst_alpha, self.opt_color, self.opt_alpha))
        }
    }
}
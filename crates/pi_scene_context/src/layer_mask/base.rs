///
/// Layer Mask
/// * 通过 layer mask 数据标识目标的层级信息
/// * 提供用户操作接口, 对应实现操作命令, 实现操作命令队列, 命令队列的执行System

use pi_scene_shell::prelude::*;

pub trait TViewerLayerMask {
    fn include(&self, _other: u32) -> bool {
        return true;
    }
}

#[derive(Debug, Clone, Copy, Component, PartialEq, Eq, PartialOrd, Ord)]
pub struct LayerMask(pub u32);
impl Default for LayerMask {
    fn default() -> Self {
        Self(0xFFFFFFFF)
    }
}
impl TViewerLayerMask for LayerMask {
    fn include(&self, other: u32) -> bool {
        return self.0 & other > 0;
    }
}

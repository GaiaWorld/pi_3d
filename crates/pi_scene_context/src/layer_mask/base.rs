///
/// Layer Mask
/// * 通过 layer mask 数据标识目标的层级信息
/// * 提供用户操作接口, 对应实现操作命令, 实现操作命令队列, 命令队列的执行System

use pi_engine_shell::prelude::*;


#[derive(Debug, Clone, Copy, Component, PartialEq, Eq, PartialOrd, Ord)]
pub struct LayerMask(pub u32);
impl Default for LayerMask {
    fn default() -> Self {
        Self(0xFFFFFFFF)
    }
}
impl LayerMask {
    pub fn include(&self, other: &Self) -> bool {
        return self.0 & other.0 > 0;
    }
}

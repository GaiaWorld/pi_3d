
use pi_scene_shell::prelude::*;

/// 标识 Mesh 的渲染排序参数
#[derive(Debug, Clone, Copy, Component, Default)]
pub struct TransparentSortParam {
    /// 同 渲染类型 中的 渲染分组
    pub group: i32,
    /// 同 渲染分组 中的 渲染顺序
    pub index: i32,
}
impl TransparentSortParam {
    pub fn opaque() -> Self {
        Self {
            group: 0,
            index: 2000,
        }
    }
    pub fn transparent() -> Self {
        Self {
            group: 0,
            index: 3000,
        }
    }
    pub fn skybox() -> Self {
        Self {
            group: 0,
            index: 2000,
        }
    }
    pub fn alpha_test() -> Self {
        Self {
            group: 0,
            index: 2450,
        }
    }
}
impl PartialEq for TransparentSortParam {
    fn eq(&self, other: &Self) -> bool {
        self.group == other.group && self.index == other.index
    }
}
impl Eq for TransparentSortParam {
    fn assert_receiver_is_total_eq(&self) {

    }
}
impl PartialOrd for TransparentSortParam {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.group.partial_cmp(&other.group) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.index.partial_cmp(&other.index)
    }
}
impl Ord for TransparentSortParam {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

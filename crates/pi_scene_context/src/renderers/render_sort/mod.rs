use std::mem::replace;

use pi_engine_shell::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Component)]
pub enum ERenderSortParam {
    Opaque(OpaqueSortParam),
    Tansparent(TransparentSortParam),
}

#[derive(Debug, Clone, Copy, PartialOrd, Component)]
pub struct OpaqueSortParam {
    pub disance: f32,
}
impl PartialEq for OpaqueSortParam {
    fn eq(&self, other: &Self) -> bool {
        self.disance == other.disance
    }
}
impl Eq for OpaqueSortParam {
    fn assert_receiver_is_total_eq(&self) {}
}

#[derive(Debug, Clone, Copy, Component)]
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

pub struct OpsRenderQueue(pub(crate) Entity, pub(crate) TransparentSortParam, pub(crate) u16);
impl OpsRenderQueue {
    pub fn ops(mesh: Entity, group: i32, index: i32) -> Self {
        Self(mesh, TransparentSortParam { group, index }, 0)
    }
}
pub type ActionListRenderQueue = ActionList<OpsRenderQueue>;

pub struct ActionRenderSort;
impl ActionRenderSort {
    pub fn modify(
        commands: &mut EntityCommands,
        val: TransparentSortParam,
    ) {
        commands.insert(val);
    }
}

pub fn sys_act_render_queue(
    mut cmds: ResMut<ActionListRenderQueue>,
    mut items: Query<&mut TransparentSortParam>,
) {
    cmds.drain().drain(..).for_each(|OpsRenderQueue(entity, val, count)| {
        if let Ok(mut item) = items.get_mut(entity) {
            *item = val;
            return;
        }

        if count < ACTION_WAIT_FRAME {
            cmds.push(OpsRenderQueue(entity, val, count + 1));
        }
    });
}

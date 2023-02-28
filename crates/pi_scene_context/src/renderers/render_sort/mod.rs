use std::mem::replace;

use pi_ecs::{prelude::{ResMut, Setup, Commands}};
use pi_ecs_macros::setup;
use pi_engine_shell::run_stage::{TSystemStageInfo, ERunStageChap};

use crate::object::{GameObject, ObjectID};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub enum ERenderSortParam {
    Opaque(OpaqueSortParam),
    Tansparent(TransparentSortParam),
}

#[derive(Debug, Clone, Copy, PartialOrd)]
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

#[derive(Debug, Clone, Copy)]
pub struct TransparentSortParam {
    /// 同 渲染类型 中的 渲染分组
    pub group: u8,
    /// 同 渲染分组 中的 渲染顺序
    pub index: u32,
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

#[derive(Debug, Default)]
pub struct SingleRenderSortCommandList {
    pub list: Vec<(ObjectID, TransparentSortParam)>
}

pub struct SysRenderSortCommand;
impl TSystemStageInfo for SysRenderSortCommand {

}
#[setup]
impl SysRenderSortCommand {
    #[system]
    pub fn cmds(
        mut cmds: ResMut<SingleRenderSortCommandList>,
        mut items: Commands<GameObject, TransparentSortParam>,
    ) {
        let mut  list = replace(&mut cmds.list, vec![]);

        list.drain(..).for_each(|(obj, value)| {
            items.insert(obj, value);
        });
    }
}

pub trait InterfaceRenderSort {
    fn render_sort(
        &self,
        entity: ObjectID,
        value: TransparentSortParam,
    ) -> &Self;
}

impl InterfaceRenderSort for crate::engine::Engine {
    fn render_sort(
        &self,
        entity: ObjectID,
        value: TransparentSortParam,
    ) -> &Self {
        let commands = self.world().get_resource_mut::<SingleRenderSortCommandList>().unwrap();
        commands.list.push((entity, value));

        self
    }
}

pub struct PluginRenderSort;
impl crate::Plugin for PluginRenderSort {
    fn init(
        &mut self,
        engine: &mut crate::engine::Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {
        let world = engine.world_mut();

        world.insert_resource(SingleRenderSortCommandList::default());
        SysRenderSortCommand::setup(world, stages.query_stage::<SysRenderSortCommand>(ERunStageChap::Initial));

        Ok(())
    }
}
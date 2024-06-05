
use std::ops::Deref;

use crate::{prelude::StageModel, renderers::prelude::StageRenderer};

pub use pi_scene_shell::prelude::*;

mod pass_object;
mod command;
mod command_sys;
mod system;

pub use pass_object::*;
pub use command::*;
pub use command_sys::*;
pub use system::*;

use crate::materials::prelude::StageMaterial;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet, PartialOrd, Ord)]
pub enum StagePassObject {
    Create,
    _CreateApply,
    Command,
    EffectModify,
}

#[derive(Clone, Component, Default)]
pub struct PassTagOrders(pub Vec<PassTag>, pub PassTagValue);
impl PassTagOrders {
    pub fn new(orders: Vec<PassTag>) -> Self {
        let mut tag = 0;
        orders.iter().for_each(|item| {
            let val = item.deref().clone();
            if tag & val == 0 {
                tag += val;
            }
        });

        Self(orders, tag)
    }
}

pub struct PluginPassObject;
impl Plugin for PluginPassObject {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActionListPassObject::default());
        
        app.configure_sets(
            Update,
            (
                StagePassObject::Create.after(StageMaterial::Command).after(StageModel::AbstructMeshCommand).after(StageMaterial::Use),
                StagePassObject::_CreateApply.after(StagePassObject::Create),
                StagePassObject::Command.after(StagePassObject::_CreateApply).before(StageRenderer::RenderStateCommand),
                StagePassObject::EffectModify.in_set(FrameDataPrepare).after(StagePassObject::_CreateApply).after(StageMaterial::Ready).before(StageRenderer::PassBindGroup),
            )
        );

        app.add_systems(
            Update, 
            (
                apply_deferred.in_set(StagePassObject::_CreateApply),
                sys_create_pass_object.in_set(StagePassObject::Create),
                sys_act_pass_object.in_set(StagePassObject::Command),
                (
                    sys_modify_pass_effect_by_material,
                    sys_modify_pass_effect_by_pass
                ).chain().in_set(StagePassObject::EffectModify),
            )
        );
    }
}
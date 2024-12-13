
use std::ops::Deref;

use crate::{prelude::StageModel, renderers::prelude::StageRenderer};

pub use pi_scene_shell::prelude::{PassTag, PassTagValue};

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
    PassCreate,
    _PassCreate,
    PassCommand,
    PassReady,
    PassDispose,
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
        app.insert_resource(ActionListRenderState::default());
        
#[cfg(feature = "use_bevy")]
        app.configure_sets(
            Update,
            (
                StagePassObject::PassCreate.after(StageMaterial::MatCommand).after(StageModel::AbstructMeshCommand).after(StageMaterial::MatUse),
                StagePassObject::_PassCreate.after(StagePassObject::PassCreate),
                StagePassObject::PassCommand.after(StagePassObject::_PassCreate).before(StageRenderer::RenderStateCommand),
                StagePassObject::PassReady.in_set(FrameDataPrepare).after(StagePassObject::_PassCreate).after(StageMaterial::MatReady).before(StageRenderer::PassBindGroup),
            )
        );

#[cfg(feature = "use_bevy")]
        app.add_systems(
            Update, 
            (
                apply_deferred.in_set(StagePassObject::_PassCreate),
                sys_create_pass_object.in_set(StagePassObject::PassCreate),
                sys_act_pass_object.in_set(StagePassObject::PassCommand),
                (
                    sys_modify_pass_effect_by_material
                ).chain().in_set(StagePassObject::PassReady),
            )
        );

#[cfg(not(feature = "use_bevy"))]
        app
        .configure_set(Update, StagePassObject::PassCreate      .in_set(ERunStageChap::Modify).after(StageMaterial::MatUse).after(StageModel::InstanceCreate))
        .configure_set(Update, StagePassObject::_PassCreate     .in_set(ERunStageChap::Modify).after(StagePassObject::PassCreate))
        .configure_set(Update, StagePassObject::PassCommand     .in_set(ERunStageChap::Modify).after(StagePassObject::_PassCreate).before(StageRenderer::RenderStateCommand))
        .configure_set(Update, StagePassObject::PassReady       .in_set(ERunStageChap::Collect).in_set(FrameDataPrepare).after(StageMaterial::MatReady).before(StageRenderer::PassBindGroup))
        .configure_set(Update, StagePassObject::PassDispose     .in_set(ERunStageChap::Dispose))
        ;

#[cfg(not(feature = "use_bevy"))]
        app
        .add_systems(Update, sys_create_pass_object
            // .run_if(runif_acts::<OpsPassObject>)  
            .in_set(StagePassObject::PassCreate))
        .add_systems(Update, sys_act_pass_object
            // .run_if(runif_acts::<OpsRenderState>)     
            .in_set(StagePassObject::PassCommand))
        .add_systems(Update, sys_modify_pass_effect_by_material  .in_set(StagePassObject::PassReady))
        ;
    }
}
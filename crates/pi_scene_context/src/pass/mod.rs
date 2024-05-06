
use std::ops::Deref;

use crate::renderers::prelude::StageRenderer;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash,  PartialOrd, Ord)]
pub enum StagePassObject {
    Create,
    _CreateApply,
    EffectModify,
}

#[derive(Clone, )]
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
        app.world.insert_single_res(ActionListPassObject::default());
        

        // app.configure_set(Update, StagePassObject::Create.after(StageMaterial::Command));
        // app.configure_set(Update, StagePassObject::_CreateApply.after(StagePassObject::Create));
        // app.configure_set(Update, StagePassObject::EffectModify.in_set(FrameDataPrepare).after(StagePassObject::_CreateApply).after(StageMaterial::Ready).before(StageRenderer::PassBindGroup));

        // app.add_system(Update, 
        //     apply_deferred.in_set(StagePassObject::_CreateApply)
        // );
        app.add_system(Update, 
            sys_create_pass_object
        );
        app.add_system(Update, 
            
                sys_modify_pass_effect_by_material
        );
        app.add_system(Update, 
                sys_modify_pass_effect_by_pass
        );
    }
}
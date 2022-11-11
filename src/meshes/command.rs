use std::mem::replace;

use pi_ecs::{prelude::{ResMut, Query, Setup}, query::Write};
use pi_ecs_macros::setup;

use crate::{object::{ObjectID, GameObject}, flags::{RenderSortParam, RenderBlend, PrimitiveState, RenderDepthAndStencil}, transforms::{interface::InterfaceTransformNode}, scene::{interface::InterfaceScene}, plugin::Plugin, layer_mask::LayerMask, resources::RenderDynUniformBuffer};

use super::model::BuildinModelBind;

#[derive(Debug)]
pub enum MeshCommand {
    Create(ObjectID),
    Destroy(ObjectID),
}

#[derive(Debug, Default)]
pub struct SingleMeshCommandList {
    pub list: Vec<MeshCommand>,
}

pub struct SysMeshCommand;
#[setup]
impl SysMeshCommand {
    #[system]
    pub fn cmd(
        mut cmds: ResMut<SingleMeshCommandList>,
        mut meshes: Query<GameObject, (Write<LayerMask>, Write<RenderSortParam>, Write<RenderBlend>, Write<PrimitiveState>, Write<RenderDepthAndStencil>, Write<BuildinModelBind>)>,
        mut dynbuffer: ResMut<RenderDynUniformBuffer>,
    ) {
        let mut list = replace(&mut cmds.list, vec![]);

        list.drain(..).for_each(|cmd| {
            match cmd {
                MeshCommand::Create(entity) => {
                    match meshes.get_mut(entity) {
                        Some(mut item) => {
                            item.0.insert_no_notify(LayerMask::default());
                            item.1.insert_no_notify(RenderSortParam::opaque());
                            item.2.insert_no_notify(RenderBlend::default());
                            item.3.insert_no_notify(PrimitiveState::default());
                            item.4.insert_no_notify(RenderDepthAndStencil::default());
                            item.5.insert_no_notify(BuildinModelBind::new(&mut dynbuffer));
                        },
                        None => {
                            
                        },
                    }
                },
                MeshCommand::Destroy(_) => todo!(),
            }
        });
    }
}

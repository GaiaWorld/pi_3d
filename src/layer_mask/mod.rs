use pi_ecs::{prelude::{ResMut, Query, Setup}, query::Write, entity};
use pi_ecs_macros::setup;

use crate::{object::{ObjectID, GameObject}, plugin::Plugin};


#[derive(Debug, Clone, Copy)]
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

#[derive(Debug)]
pub enum LayerMaskCommand {
    Set(ObjectID, LayerMask),
}
#[derive(Debug, Default)]
pub struct SingleLayerMaskCommandList {
    pub list: Vec<LayerMaskCommand>,
}

pub struct SysLayerMaskCommand;
#[setup]
impl SysLayerMaskCommand {
    #[system]
    pub fn cmd(
        mut cmds: ResMut<SingleLayerMaskCommandList>,
        mut objects: Query<GameObject, Write<LayerMask>>,
    ) {
        cmds.list.drain(..).for_each(|cmd| {
            match cmd {
                LayerMaskCommand::Set(entity, layer) => {
                    match objects.get_mut(entity) {
                        Some(mut object) => {
                            object.insert_no_notify(layer);
                        },
                        None => todo!(),
                    }
                },
            }
        });
    }
}

pub struct PluginLayerMask;
impl Plugin for PluginLayerMask {
    fn init(
        engine: &mut crate::engine::Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {
        let world = engine.world_mut();

        SysLayerMaskCommand::setup(world, stages.command_stage());

        world.insert_resource(SingleLayerMaskCommandList::default());

        Ok(())
    }
}

pub trait InterfaceLayerMask {
    fn layer_mask(
        &mut self,
        object: ObjectID,
        layer: LayerMask,
    ) -> &mut Self;
}

impl InterfaceLayerMask for crate::engine::Engine {
    fn layer_mask(
        &mut self,
        object: ObjectID,
        layer: LayerMask,
    ) -> &mut Self {
        let world = self.world_mut();

        let commands = world.get_resource_mut::<SingleLayerMaskCommandList>().unwrap();
        commands.list.push(LayerMaskCommand::Set(object, layer));

        self
    }
}

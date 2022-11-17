
use std::{marker::PhantomData, mem::replace};
use crossbeam::queue::SegQueue;
use pi_assets::{
    asset::Handle,
};
use pi_atom::Atom;

use pi_ecs::{prelude::{ResMut, Query, Setup}, query::Write};
use pi_ecs_macros::setup;
use pi_engine_shell::object::InterfaceObject;
use pi_render::rhi::{asset::TextureRes};
use pi_share::Share;

use crate::object::{ObjectID, GameObject};

use super::texture_sampler::{InterfaceTextureAddressMode};


pub type TextureKey = u128;

pub struct Texture2D {
    pub textureid: TextureKey,
}

#[derive(Debug)]
enum ECommand {
    Create(ObjectID, TextureKey),
    Destroy(ObjectID),
}

#[derive(Debug, Default)]
struct SingleCommandList {
    pub list: Vec<ECommand>,
}

struct SysCommand;
#[setup]
impl SysCommand {
    #[system]
    fn sys(
        mut cmds: ResMut<SingleCommandList>,
        mut textures: Query<GameObject, Write<Texture2D>>,
    ) {
        let mut list = replace(&mut cmds.list, vec![]);

        list.drain(..).for_each(|cmd| {
            match cmd {
                ECommand::Create(entity, texid) => {
                    if let Some(mut target) = textures.get_mut(entity) {
                        target.write(Texture2D {
                            textureid: texid,
                        });
                    }
                },
                ECommand::Destroy(entity) => {
                    if let Some(mut target) = textures.get_mut(entity) {
                        target.remove();
                    }
                },
            }
        });
    }
}

#[derive(Clone)]
pub struct SingleImageAwait<T>(Share<SegQueue<(ObjectID, Atom, Handle<TextureRes>)>>, PhantomData<T>);

impl<T> Default for SingleImageAwait<T> {
    fn default() -> Self { Self(Share::new(SegQueue::new()), PhantomData) }
}

pub trait InterfaceTexture2D {
    /// 根据名称创建纹理数据
    ///   * 创建应用层纹理数据 Texture2D
    ///      * 设置 TextureID
    ///        * 如果目标Texture不存在则创建并加载
    /// 返回纹理数据存放的 Entity
    /// * PS
    ///   * Sampler 信息通过 InterfaceTextureSampler 接口 设置
    fn new_texture_2d(
        &self,
        path: &str,
    ) -> ObjectID;
}

impl InterfaceTexture2D for crate::engine::Engine {
    fn new_texture_2d(
        &self,
        path: &str,
    ) -> ObjectID {
        let world = self.world();

        let texture = self.new_object();

        // let texid = 0;
        let texid = 0;
        let commands = world.get_resource_mut::<SingleCommandList>().unwrap();
        commands.list.push(ECommand::Create(texture, texid));

        self.with_texture_sampler(texture);

        texture
    }
}

pub struct PluginTexture2D;
impl crate::Plugin for PluginTexture2D {
    fn init(
        &mut self,
        engine: &mut crate::engine::Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {
        let world = engine.world_mut();

        SysCommand::setup(world, stages.command_stage());

        world.insert_resource(SingleCommandList::default());

        Ok(())
    }
}

use std::{marker::PhantomData, mem::replace, 
    ops::{Deref, DerefMut, Index, IndexMut, Mul},
};
use pi_assets::{
    asset::Handle, mgr::{AssetMgr, LoadResult},
};
use pi_async::rt::AsyncRuntime;
use pi_atom::Atom;

use pi_ecs::{prelude::{ResMut, Query, Setup, Res}, query::Write};
use pi_ecs_macros::setup;
use pi_engine_shell::object::InterfaceObject;
use pi_hal::{runtime::MULTI_MEDIA_RUNTIME, loader::AsyncLoader};
use pi_hash::XHashMap;
use pi_render::rhi::{asset::{TextureRes, ImageTextureDesc}, device::RenderDevice, RenderQueue, texture::{Sampler, TextureView}};
use pi_scene_math::Number;
use pi_share::Share;
use render_resource::sampler::{SamplerDesc, SamplerAssetKey, SamplerPool};

use crate::object::{ObjectID, GameObject};

use super::texture_sampler::{InterfaceTextureAddressMode, TextureSamplerDesc, TextureSamplerID};

pub mod scale_offset;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Texture2DDesc {
    pub id: ObjectID,
    pub sampler: SamplerAssetKey,
}

#[derive(Debug)]
enum ECommand {
    Create(ObjectID, Texture2DKey, SamplerDesc),
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
        device: Res<RenderDevice>,
        mut cmds: ResMut<SingleCommandList>,
        mut loadwait: ResMut<SingleTexture2DAwait>,
        mut samplerpool: ResMut<SamplerPool>,
    ) {
        let mut list = replace(&mut cmds.list, vec![]);

        list.drain(..).for_each(|cmd| {
            match cmd {
                ECommand::Create(entity, key, sampler) => {
                    samplerpool.create(&sampler, &device);
                    loadwait.add(key, Texture2DDesc { id: entity, sampler: SamplerPool::cacl_key(&sampler) } );
                }
            }
        });
    }
}

#[derive(Default)]
pub struct SingleTexture2DAwait {
    pub(crate) map: XHashMap<Texture2DKey, Vec<Texture2DDesc>>,
}
impl SingleTexture2DAwait {
    pub(crate) fn add(
        &mut self,
        key: Texture2DKey,
        obj: Texture2DDesc,
    ) {
        if self.map.contains_key(&key) {
            let list = self.map.get_mut(&key).unwrap();
            if list.contains(&obj) == false {
                list.push(obj);
            }
        } else {
            let list = vec![obj];
            self.map.insert(key, list);
        }
    }
}

pub struct SysTexture2DReady;
#[setup]
impl SysTexture2DReady {
    #[system]
    pub fn ready(
        mut wait_list: ResMut<SingleTexture2DAwait>,
        mut textures: Query<GameObject, Write<Texture2D>>,
        assets_mgr: Res<Share<AssetMgr<TextureRes>>>,
        device: Res<RenderDevice>,
        queue: Res<RenderQueue>,
        mut samplerpool: ResMut<SamplerPool>,
    ) {
        let mut map = replace(&mut wait_list.map, XHashMap::default());

        for (key, mut idlist) in map.drain() {
            let result = AssetMgr::load(&assets_mgr, &(key.get_hash() as u64));
            match result {
                LoadResult::Ok(res) => {
                    idlist.drain(..).for_each(|item| {
                        if let Some(mut texture) = textures.get_mut(item.id) {
                            texture.write(Texture2D {
                                texture: Handle::<TextureRes>::from(res.clone()),
                                sampler: samplerpool.get(item.sampler).unwrap()
                            });
                        }
                    });
                },
                _ => {
                    idlist.drain(..).for_each(|item| {
                        wait_list.add(key.clone(), item);
                    });

                    let key = key.clone();
                    let device = device.clone();
                    let queue = queue.clone();
                    MULTI_MEDIA_RUNTIME
                        .spawn(MULTI_MEDIA_RUNTIME.alloc(), async move {
                            let desc = ImageTextureDesc {
                                url: &key,
                                device: &device,
                                queue: &queue,
                            };

                            let r = TextureRes::async_load(desc, result).await;
                            match r {
                                Err(e) => {
                                    log::error!("load image fail, {:?}", e);
                                },
                                _ => {
                                    // awaits.push((id, key.clone(), r));
                                }
                            };
                        })
                        .unwrap();
                }
            }
        }
    }
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
        sampler: SamplerDesc,
    ) -> ObjectID;
}

impl InterfaceTexture2D for crate::engine::Engine {
    fn new_texture_2d(
        &self,
        path: &str,
        sampler: SamplerDesc,
    ) -> ObjectID {
        let world = self.world();

        let texture = self.new_object();

        let commands = world.get_resource_mut::<SingleCommandList>().unwrap();
        commands.list.push(ECommand::Create(texture, Atom::from(path), sampler));

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
        SysTexture2DReady::setup(world, stages.command_stage());

        world.insert_resource(SingleCommandList::default());
        world.insert_resource(SingleTexture2DAwait::default());

        Ok(())
    }
}
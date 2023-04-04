
use pi_assets::{mgr::AssetMgr, asset::GarbageEmpty};
use pi_ecs::{prelude::{ResMut, Commands}, world};
use pi_engine_shell::prelude::*;
use pi_hash::XHashMap;
use pi_render::{rhi::{device::RenderDevice}, renderer::{bind_buffer::{BindBufferAllocator}, bind_group::{BindGroupLayout, KeyBindGroup, KeyBindGroupLayout, BindGroup}}, asset::{AssetDataCenter, AssetLoader}, render_3d::bind_groups::{scene::KeyBindGroupScene, model::KeyBindGroupModel, texture_sampler::KeyBindGroupTextureSamplers}};

use crate::object::{ObjectID};


#[derive(Default, Clone, Resource)]
pub struct AssetBindGroupSceneWaits(pub XHashMap<KeyBindGroupScene, Vec<ObjectID>>);
impl AssetBindGroupSceneWaits {
    pub fn add(&mut self, key: &KeyBindGroupScene, id: ObjectID) {
        if !self.0.contains_key(key) {
            self.0.insert(key.clone(), vec![]);
        }

        self.0.get_mut(key).unwrap().push(id)
    }
}

#[derive(Default, Clone, Resource)]
pub struct AssetBindGroupModelWaits(pub XHashMap<KeyBindGroupModel, Vec<ObjectID>>);
impl AssetBindGroupModelWaits {
    pub fn add(&mut self, key: &KeyBindGroupModel, id: ObjectID) {
        if !self.0.contains_key(key) {
            self.0.insert(key.clone(), vec![]);
        }

        self.0.get_mut(key).unwrap().push(id)
    }
}

#[derive(Default, Clone, Resource)]
pub struct AssetBindGroupTextureSamplersWaits(pub XHashMap<KeyBindGroupTextureSamplers, Vec<ObjectID>>);
impl AssetBindGroupTextureSamplersWaits {
    pub fn add(&mut self, key: &KeyBindGroupTextureSamplers, id: ObjectID) {
        if !self.0.contains_key(key) {
            self.0.insert(key.clone(), vec![]);
        }

        self.0.get_mut(key).unwrap().push(id)
    }
}


pub struct PluginRenderBindGroup;
impl Plugin for PluginRenderBindGroup {
    // fn init(
    //     &mut self,
    //     engine: &mut pi_engine_shell::engine_shell::EnginShell,
    //     stages: &mut pi_engine_shell::run_stage::RunStage,
    // ) -> Result<(), pi_engine_shell::plugin::ErrorPlugin> {
    //     let world = engine.world_mut();
    //     let device = world.get_resource::<RenderDevice>().unwrap();
    //     let allocator = BindBufferAllocator::new(device);
        
    //     let world = engine.world_mut();
    //     world.insert_resource(allocator);
        
    //     world.insert_resource(AssetBindGroupSceneWaits::default());
    //     world.insert_resource(AssetBindGroupModelWaits::default());
    //     world.insert_resource(AssetBindGroupTextureSamplersWaits::default());
        
    //     // log::debug!("{:?}", device.limits());
    //     world.insert_resource(AssetMgr::<BindGroup>::new(GarbageEmpty(), false, 2 * 1024 * 1024, 60 * 1000));
    //     world.insert_resource(AssetMgr::<BindGroupLayout>::new(GarbageEmpty(), false, 2 * 1024 * 1024, 60 * 1000));

    //     Ok(())
    // }

    fn build(&self, app: &mut App) {
        let world = &mut app.world;
        let mut cfgs = world.get_resource_mut::<AssetMgrConfigs>().unwrap();

        let cfg_bind_group = if let Some(cfg) = cfgs.0.get("ASSET_BIND_GROUP") {
            cfg.clone()
        } else {
            let cfg = AssetCapacity { ty: String::from("ASSET_BIND_GROUP"), min: 1024 * 100, max: 1024 * 200, timeout: 10 * 1000 };
            cfgs.insert(cfg.clone());
            cfg
        };
        
        let cfg_bind_group_layout = if let Some(cfg) = cfgs.0.get("ASSET_BIND_GROUP_LAYOUT") {
            cfg.clone()
        } else {
            let cfg = AssetCapacity { ty: String::from("ASSET_BIND_GROUP_LAYOUT"), min: 1024 * 100, max: 1024 * 200, timeout: 10 * 1000 };
            cfgs.insert(cfg.clone());
            cfg
        };

        let device = world.get_resource::<RenderDevice>().unwrap();

        if world.get_resource::<ResBindBufferAllocator>().is_none() {
            let allocator = ResBindBufferAllocator(BindBufferAllocator::new(device));
            world.insert_resource(allocator);
        }
        
        world.insert_resource(AssetBindGroupSceneWaits::default());
        world.insert_resource(AssetBindGroupModelWaits::default());
        world.insert_resource(AssetBindGroupTextureSamplersWaits::default());
        
        // log::debug!("{:?}", device.limits());
        world.insert_resource(ShareAssetMgr::<BindGroup>::create(GarbageEmpty(), false, &cfg_bind_group));
        world.insert_resource(ShareAssetMgr::<BindGroupLayout>::create(GarbageEmpty(), false, &cfg_bind_group_layout));
    }
}

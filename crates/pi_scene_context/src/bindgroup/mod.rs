
use pi_assets::{mgr::AssetMgr, asset::GarbageEmpty};
use pi_ecs::prelude::{ResMut, Commands};
use pi_engine_shell::{plugin::Plugin, run_stage::TSystemStageInfo};
use pi_hash::XHashMap;
use pi_render::{rhi::{device::RenderDevice}, renderer::{bind_buffer::{BindBufferAllocator}, bind_group::{BindGroupLayout, KeyBindGroup, KeyBindGroupLayout, BindGroup}}, asset::{AssetDataCenter, AssetLoader}, render_3d::bind_groups::{scene::KeyBindGroupScene, model::KeyBindGroupModel, texture_sampler::KeyBindGroupTextureSamplers}};

use crate::object::{ObjectID};


#[derive(Default, Clone)]
pub struct AssetBindGroupSceneWaits(pub XHashMap<KeyBindGroupScene, Vec<ObjectID>>);
impl AssetBindGroupSceneWaits {
    pub fn add(&mut self, key: &KeyBindGroupScene, id: ObjectID) {
        if !self.0.contains_key(key) {
            self.0.insert(key.clone(), vec![]);
        }

        self.0.get_mut(key).unwrap().push(id)
    }
}

#[derive(Default, Clone)]
pub struct AssetBindGroupModelWaits(pub XHashMap<KeyBindGroupModel, Vec<ObjectID>>);
impl AssetBindGroupModelWaits {
    pub fn add(&mut self, key: &KeyBindGroupModel, id: ObjectID) {
        if !self.0.contains_key(key) {
            self.0.insert(key.clone(), vec![]);
        }

        self.0.get_mut(key).unwrap().push(id)
    }
}

#[derive(Default, Clone)]
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
    fn init(
        &mut self,
        engine: &mut pi_engine_shell::engine_shell::EnginShell,
        stages: &mut pi_engine_shell::run_stage::RunStage,
    ) -> Result<(), pi_engine_shell::plugin::ErrorPlugin> {
        let world = engine.world_mut();
        let device = world.get_resource::<RenderDevice>().unwrap();
        let allocator = BindBufferAllocator::new(device);
        
        let world = engine.world_mut();
        world.insert_resource(allocator);
        
        world.insert_resource(AssetBindGroupSceneWaits::default());
        world.insert_resource(AssetBindGroupModelWaits::default());
        world.insert_resource(AssetBindGroupTextureSamplersWaits::default());
        
        // log::info!("{:?}", device.limits());
        world.insert_resource(AssetMgr::<BindGroup>::new(GarbageEmpty(), false, 2 * 1024 * 1024, 60 * 1000));
        world.insert_resource(AssetMgr::<BindGroupLayout>::new(GarbageEmpty(), false, 2 * 1024 * 1024, 60 * 1000));

        Ok(())
    }
}



use pi_engine_shell::prelude::*;
use pi_hash::XHashMap;

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

fn sys_recycle_binds_recorder(
    mut recorder: ResMut<ResBindsRecorder>,
) {
    recorder.recycle();
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

        let device = app.world.get_resource::<PiRenderDevice>().unwrap();

        if app.world.get_resource::<ResBindBufferAllocator>().is_none() {
            let allocator = ResBindBufferAllocator(BindBufferAllocator::new(device));
            app.insert_resource(allocator);
        }
        
        
        app.insert_resource(AssetBindGroupSceneWaits::default());
        app.insert_resource(AssetBindGroupModelWaits::default());
        app.insert_resource(AssetBindGroupTextureSamplersWaits::default());
        
        // log::debug!("{:?}", device.limits());
        
        let cfg = if let Some(cfg) = app.world.get_resource::<AssetCfgBindGroup>() { cfg.0.clone() } else {
            app.insert_resource(AssetCfgBindGroup::default());
            app.world.get_resource::<AssetCfgBindGroup>().unwrap().0.clone()
        };
        app.insert_resource(ShareAssetMgr::<BindGroup>::create(GarbageEmpty(), false, &cfg));
        app.insert_resource(ShareAssetMgr::<BindGroupLayout>::create(GarbageEmpty(), false, &cfg));
        
        if app.world.get_resource::<ResBindsRecorder>().is_none() {
            let allocator = ResBindsRecorder(BindsRecorder::new());
            app.insert_resource(allocator);
            app.add_system(sys_recycle_binds_recorder.in_set(ERunStageChap::Initial));
        }
    }
}

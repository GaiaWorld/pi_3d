

use std::sync::Arc;

use pi_assets::asset::Handle;
use pi_engine_shell::prelude::*;
use pi_hash::XHashMap;


#[derive(Default, Clone, Resource)]
pub struct AssetBindGroupSceneWaits(pub XHashMap<KeyBindGroupScene, Vec<Entity>>);
impl AssetBindGroupSceneWaits {
    pub fn add(&mut self, key: &KeyBindGroupScene, id: Entity) {
        if !self.0.contains_key(key) {
            self.0.insert(key.clone(), vec![]);
        }

        self.0.get_mut(key).unwrap().push(id)
    }
}

#[derive(Default, Clone, Resource)]
pub struct AssetBindGroupModelWaits(pub XHashMap<KeyBindGroupModel, Vec<Entity>>);
impl AssetBindGroupModelWaits {
    pub fn add(&mut self, key: &KeyBindGroupModel, id: Entity) {
        if !self.0.contains_key(key) {
            self.0.insert(key.clone(), vec![]);
        }

        self.0.get_mut(key).unwrap().push(id)
    }
}

#[derive(Default, Clone, Resource)]
pub struct AssetBindGroupTextureSamplersWaits(pub XHashMap<KeyBindGroupTextureSamplers, Vec<Entity>>);
impl AssetBindGroupTextureSamplersWaits {
    pub fn add(&mut self, key: &KeyBindGroupTextureSamplers, id: Entity) {
        if !self.0.contains_key(key) {
            self.0.insert(key.clone(), vec![]);
        }

        self.0.get_mut(key).unwrap().push(id)
    }
}


#[derive(Clone)]
pub struct BindGroups3D {
    pub scene: Option<Arc<BindGroupScene>>,
    pub model: Option<Arc<BindGroupModel>>, 
    pub textures: Option<Arc<BindGroupTextureSamplers>>,
    pub lightingshadow: Option<Arc<BindGroupSetExtend>>,
}
impl BindGroups3D {
    pub fn create(
        scene: Option<Arc<BindGroupScene>>,
        model: Option<Arc<BindGroupModel>>, 
        textures: Option<Arc<BindGroupTextureSamplers>>,
        lightingshadow: Option<Arc<BindGroupSetExtend>>,
    ) -> Self {
        Self { scene, model, textures, lightingshadow }
    }
    pub fn key_set_blocks(&self) -> KeyShaderSetBlocks<4, EKeyShader3DSetBlock> {
        let mut key_set_blocks = [None, None, None, None];
        let mut setidx = 0;

        if let Some(set) = &self.scene {
            key_set_blocks[setidx] = Some(EKeyShader3DSetBlock::Scene(set.key().key_set.clone()));
            setidx += 1;
        }
        
        if let Some(set) = &self.model {
            key_set_blocks[setidx] = Some(EKeyShader3DSetBlock::Model(set.key().key.clone()));
            setidx += 1;
        }

        if let Some(set_2) = &self.textures {
            key_set_blocks[setidx] = Some(EKeyShader3DSetBlock::TextureSampler(set_2.key().asset_u64()));
            setidx += 1;
        }
        if let Some(_set_3) = &self.lightingshadow {
            key_set_blocks[setidx] = Some(EKeyShader3DSetBlock::Other(0));
            // setidx += 1;
        }

        KeyShaderSetBlocks(key_set_blocks)
    }
    pub fn bind_group_layouts(&self) -> [Option<Handle<BindGroupLayout>>; 4] {
        let mut bind_group_layouts = [None, None, None, None];
        
        let mut setidx = 0;
        if let Some(set) = &self.scene {
            bind_group_layouts[setidx] = Some(set.bind_group().layout());
            setidx += 1;
        }

        if let Some(set) = &self.model {
            bind_group_layouts[setidx] = Some(set.bind_group().layout());
            setidx += 1;
        }

        if let Some(set) = &self.textures {
            bind_group_layouts[setidx] = Some(set.bind_group().layout());
            setidx += 1;
        }

        if let Some(set) = &self.lightingshadow {
            bind_group_layouts[3] = Some(set.bind_group().layout());
            // setidx += 1;
        }

        bind_group_layouts
    }
    pub fn key_bindgroup_layouts(&self) -> [Option<u64>; 4] {
        let mut key_bindgroup_layouts = [None, None, None, None];
        
        let mut setidx = 0;
        if let Some(set) = &self.scene {
            key_bindgroup_layouts[setidx] = Some(*set.bind_group().layout().key());
            setidx += 1;
        }
        
        if let Some(set) = &self.model {
            key_bindgroup_layouts[setidx] = Some(*set.bind_group().layout().key());
            setidx += 1;
        }

        if let Some(set_2) = &self.textures {
            key_bindgroup_layouts[setidx] = Some(*set_2.bind_group().layout().key());
            setidx += 1;
        }
        if let Some(set_3) = &self.lightingshadow {
            key_bindgroup_layouts[setidx] = Some(*set_3.bind_group().layout().key());
            // setidx += 1;
        }

        key_bindgroup_layouts
    }
    pub fn groups(&self) -> DrawBindGroups {
        let mut groups = DrawBindGroups::default();
        
        let mut setidx = 0;
        if let Some(set) = &self.scene {
            groups.insert_group(setidx, DrawBindGroup::GroupUsage(set.bind_group().clone()));
            setidx += 1;
        }

        if let Some(set) = &self.model {
            groups.insert_group(setidx, DrawBindGroup::GroupUsage(set.bind_group().clone()));
            setidx += 1;
        }

        if let Some(set) = &self.textures {
            groups.insert_group(setidx, DrawBindGroup::GroupUsage(set.bind_group().clone()));
            setidx += 1;
        }
        
        if let Some(set) = &self.lightingshadow {
            groups.insert_group(setidx, DrawBindGroup::GroupUsage(set.bind_group().clone()));
            // setidx += 1;
        }

        groups
    }
}
pub type KeyPipeline3D = KeyRenderPipeline3D;
pub type Pipeline3D = RenderRes<RenderPipeline>;
pub type Pipeline3DUsage = Handle<Pipeline3D>;

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
        
        // let cfg = if let Some(cfg) = app.world.get_resource::<AssetCfgBindGroup>() { cfg.0.clone() } else {
        //     app.insert_resource(AssetCfgBindGroup::default());
        //     app.world.get_resource::<AssetCfgBindGroup>().unwrap().0.clone()
        // };
        let cfg = app.world.get_resource_mut::<AssetMgrConfigs>().unwrap().query::<BindGroup>();
        app.insert_resource(ShareAssetMgr::<BindGroup>::create(GarbageEmpty(), false, &cfg));
        let cfg = app.world.get_resource_mut::<AssetMgrConfigs>().unwrap().query::<BindGroupLayout>();
        app.insert_resource(ShareAssetMgr::<BindGroupLayout>::create(GarbageEmpty(), false, &cfg));
        
        if app.world.get_resource::<ResBindsRecorder>().is_none() {
            let allocator = ResBindsRecorder(BindsRecorder::new());
            app.insert_resource(allocator);
            app.add_systems(Update, sys_recycle_binds_recorder.in_set(ERunStageChap::Initial));
        }
    }
}

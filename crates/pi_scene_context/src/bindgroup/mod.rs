

use std::{hash::{DefaultHasher, Hasher}, sync::Arc};

use pi_scene_shell::prelude::*;

use crate::prelude::{BindModel, CommonBindModel};


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
            bind_group_layouts[setidx] = Some(set.bind_group().layout());
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

#[derive(Clone, PartialEq, Eq)]
pub struct BindGroups3DHashResource(pub u64);
impl From<&BindGroups3D> for BindGroups3DHashResource {
    fn from(value: &BindGroups3D) -> Self {
        let mut hasher = DefaultHasher::default();
        if let Some(bindgroup) = &value.scene {
            bindgroup.hash_resource(&mut hasher);
        }
        if let Some(bindgroup) = &value.model {
            bindgroup.hash_resource(&mut hasher);
        }
        if let Some(bindgroup) = &value.textures {
            bindgroup.hash_resource(&mut hasher);
        }
        if let Some(bindgroup) = &value.lightingshadow {
            bindgroup.hash_resource(&mut hasher);
        }

        Self(hasher.finish())
    }
}

pub type KeyPipeline3D = KeyRenderPipeline3D;
pub type Pipeline3D = RenderRes<RenderPipeline>;
pub type Pipeline3DUsage = Handle<Pipeline3D>;

pub struct PluginRenderBindGroup;
impl Plugin for PluginRenderBindGroup {
    fn build(&self, app: &mut App) {
        let device = app.world.get_single_res::<PiRenderDevice>().unwrap();
        if app.world.get_single_res::<ResBindBufferAllocator>().is_none() {
            let mut allocator = ResBindBufferAllocator(BindBufferAllocator::new(device));
            let commonbindmodel = CommonBindModel(BindModel::new(&mut allocator).unwrap());
            app.world.insert_single_res(commonbindmodel);
            app.world.insert_single_res(allocator);
        }
        app.world.insert_single_res(AssetBindGroupSceneWaits::default());
        app.world.insert_single_res(AssetBindGroupModelWaits::default());
        app.world.insert_single_res(AssetBindGroupTextureSamplersWaits::default());

        let cfg = app.world.get_single_res_mut::<AssetMgrConfigs>().unwrap().query::<BindGroup>();
        app.world.insert_single_res(ShareAssetMgr::<BindGroup>::create(GarbageEmpty(), false, &cfg));
        let cfg = app.world.get_single_res_mut::<AssetMgrConfigs>().unwrap().query::<BindGroupLayout>();
        app.world.insert_single_res(ShareAssetMgr::<BindGroupLayout>::create(GarbageEmpty(), false, &cfg));

    }
}

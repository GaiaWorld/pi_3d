

use std::{hash::{DefaultHasher, Hasher}, sync::Arc};

use pi_scene_shell::prelude::*;

use crate::prelude::{BindModel, BindModelMatIdx, CommonBindModel};


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


#[derive(Clone, Default)]
pub struct BindGroups3D {
    pub scene: Option<Arc<BindGroupScene>>,
    pub model: Option<Arc<BindGroupModel>>, 
    pub textures: Option<Arc<BindGroupTextureSamplers>>,
    pub matvalues: Option<Arc<BindGroupMaterial>>,
    pub hashresource: u64,
}
impl BindGroups3D {
    pub fn create(
        scene: Option<Arc<BindGroupScene>>,
        model: Option<Arc<BindGroupModel>>, 
        matvalues: Option<Arc<BindGroupMaterial>>,
        textures: Option<Arc<BindGroupTextureSamplers>>,
    ) -> Self {
        
        let mut hasher = DefaultHasher::default();
        if let Some(bindgroup) = &scene {
            bindgroup.hash_resource(&mut hasher);
        }
        if let Some(bindgroup) = &model {
            bindgroup.hash_resource(&mut hasher);
        }
        if let Some(bindgroup) = &matvalues {
            bindgroup.hash_resource(&mut hasher);
        }
        if let Some(bindgroup) = &textures {
            bindgroup.hash_resource(&mut hasher);
        }

        Self { scene, model, textures, matvalues, hashresource: hasher.finish() }
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

        if let Some(set) = &self.matvalues {
            bind_group_layouts[setidx] = Some(set.bind_group().layout());
            setidx += 1;
        }

        if let Some(set) = &self.textures {
            bind_group_layouts[setidx] = Some(set.bind_group().layout());
            setidx += 1;
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
        if let Some(set_3) = &self.matvalues {
            key_bindgroup_layouts[setidx] = Some(*set_3.bind_group().layout().key());
            setidx += 1;
        }
        if let Some(set_2) = &self.textures {
            key_bindgroup_layouts[setidx] = Some(*set_2.bind_group().layout().key());
            setidx += 1;
        }

        key_bindgroup_layouts
    }
    pub fn groups(&self) -> DrawBindGroups {
        
        let mut setidx = 0;
        match (&self.scene, &self.model, &self.matvalues, &self.textures) {
            (Some(set0), None, None, None) => {
                DrawBindGroups::from_vec(vec![
                    (DrawBindGroup::GroupUsage(set0.bind_group().clone()), 0),
                ])
            },
            (Some(set0), Some(set1), None, None) => {
                DrawBindGroups::from_vec(vec![
                    (DrawBindGroup::GroupUsage(set0.bind_group().clone()), 0),
                    (DrawBindGroup::GroupUsage(set1.bind_group().clone()), 1),
                ])
            },
            (Some(set0), Some(set1), Some(set2), None) => {
                DrawBindGroups::from_vec(vec![
                    (DrawBindGroup::GroupUsage(set0.bind_group().clone()), 0),
                    (DrawBindGroup::GroupUsage(set1.bind_group().clone()), 1),
                    (DrawBindGroup::GroupUsage(set2.bind_group().clone()), 2),
                ])
            },
            (Some(set0), Some(set1), Some(set2), Some(set3)) => {
                DrawBindGroups::from_vec(vec![
                    (DrawBindGroup::GroupUsage(set0.bind_group().clone()), 0),
                    (DrawBindGroup::GroupUsage(set1.bind_group().clone()), 1),
                    (DrawBindGroup::GroupUsage(set2.bind_group().clone()), 2),
                    (DrawBindGroup::GroupUsage(set3.bind_group().clone()), 3),
                ])
            },
            _ => {
                let mut groups = DrawBindGroups::default();
                if let Some(set) = &self.scene {
                    groups.insert_group(setidx, DrawBindGroup::GroupUsage(set.bind_group().clone()));
                    setidx += 1;
                }
        
                if let Some(set) = &self.model {
                    groups.insert_group(setidx, DrawBindGroup::GroupUsage(set.bind_group().clone()));
                    setidx += 1;
                }
                
                if let Some(set) = &self.matvalues {
                    groups.insert_group(setidx, DrawBindGroup::GroupUsage(set.bind_group().clone()));
                    setidx += 1;
                }

                if let Some(set) = &self.textures {
                    groups.insert_group(setidx, DrawBindGroup::GroupUsage(set.bind_group().clone()));
                    setidx += 1;
                }
                groups
            },
        }
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
        if let Some(bindgroup) = &value.matvalues {
            bindgroup.hash_resource(&mut hasher);
        }
        if let Some(bindgroup) = &value.textures {
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
        let device = app.world.get_resource::<PiRenderDevice>().unwrap();
        if app.world.get_resource::<ResBindBufferAllocator>().is_none() {
            let mut allocator = ResBindBufferAllocator(BindBufferAllocator::new(device));
            let commonbindmodel = CommonBindModel(BindModel::new(&mut allocator), BindModelMatIdx::new(&mut allocator));
            let bindpassindexs = BindPassIndexPool::new(&mut allocator);
            app.insert_resource(commonbindmodel);
            app.insert_resource(bindpassindexs);
            app.insert_resource(allocator);
        }
        app.insert_resource(MaterialDataMgr::default());
        app.insert_resource(AssetBindGroupSceneWaits::default());
        app.insert_resource(AssetBindGroupModelWaits::default());
        app.insert_resource(AssetBindGroupTextureSamplersWaits::default());

        let cfg = app.world.get_resource_mut::<AssetMgrConfigs>().unwrap().query::<BindGroup>();
        app.insert_resource(ShareAssetMgr::<BindGroup>::create(GarbageEmpty(), false, &cfg));
        let cfg = app.world.get_resource_mut::<AssetMgrConfigs>().unwrap().query::<BindGroupLayout>();
        app.insert_resource(ShareAssetMgr::<BindGroupLayout>::create(GarbageEmpty(), false, &cfg));

    }
}

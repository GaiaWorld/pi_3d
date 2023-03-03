
use std::sync::Arc;

use pi_assets::{asset::{Handle, GarbageEmpty}, mgr::AssetMgr};
use pi_ecs::prelude::Commands;
use pi_engine_shell::object::{ObjectID, GameObject};
use pi_hash::XHashMap;
use pi_render::{
    asset::{AssetDataCenter, AssetLoader},
    render_3d::{
        shader::{
            shader::{KeyShader3D, Shader3D, EKeyShader3DSetBlock},
            shader_effect_meta::ShaderEffectMeta,
            instance_code::EInstanceCode
        },
        bind_groups::{scene::BindGroupScene, model::BindGroupModel, texture_sampler::BindGroupTextureSamplers}
    },
    rhi::{asset::RenderRes, device::RenderDevice}
};
use pi_share::Share;

use crate::{pass::{TPassData, EPassTag, PassTag}, geometry::geometry::RenderGeometry};

use super::base::*;

/// * Set0
/// * 更新依赖: BindSceneEffect, BindViewer
#[derive(Default, Clone)]
pub struct PassBindGroups(pub Option<BindGroups3D>);
impl TPassData<Option<BindGroups3D>> for PassBindGroups {
    fn new(val: Option<BindGroups3D>) -> Self { Self(val) }
    fn val(&self) -> &Option<BindGroups3D> { &self.0 }
}

/// * Set0
/// * 更新依赖: BindSceneEffect, BindViewer
#[derive(Default, Clone)]
pub struct PassShader(pub Option<Handle<Shader3D>>);
impl TPassData<Option<Handle<Shader3D>>> for PassShader {
    fn new(val: Option<Handle<Shader3D>>) -> Self { Self(val) }
    fn val(&self) -> &Option<Handle<Shader3D>> { &self.0 }
}
impl From<(Handle<Shader3D>, Option<()>)> for PassShader {
    fn from(value: (Handle<Shader3D>, Option<()>)) -> Self {
        Self(Some(value.0))
    }
}

#[derive(Default, Clone)]
pub struct PassPipelineKey(pub Option<KeyPipeline3D>);
impl TPassData<Option<KeyPipeline3D>> for PassPipelineKey {
    fn new(val: Option<KeyPipeline3D>) -> Self { Self(val) }
    fn val(&self) -> &Option<KeyPipeline3D> { &self.0 }
}


#[derive(Default, Clone)]
pub struct PassPipeline(pub Option<Pipeline3DUsage>);
impl TPassData<Option<Pipeline3DUsage>> for PassPipeline {
    fn new(val: Option<Pipeline3DUsage>) -> Self { Self(val) }
    fn val(&self) -> &Option<Pipeline3DUsage> { &self.0 }
}
impl From<(Pipeline3DUsage, Option<()>)> for PassPipeline {
    fn from(value: (Pipeline3DUsage, Option<()>)) -> Self {
        Self(Some(value.0))
    }
}

pub struct PassDraw(pub Option<Arc<DrawObj3D>>);
impl TPassData<Option<Arc<DrawObj3D>>> for PassDraw {
    fn new(val: Option<Arc<DrawObj3D>>) -> Self { Self(val) }
    fn val(&self) -> &Option<Arc<DrawObj3D>> { &self.0 }
}

pub type AssetDataCenterShader3D = AssetDataCenter<KeyShader3D, Shader3D, ()>;
pub type AssetLoaderShader3D = AssetLoader<KeyShader3D, ObjectID, Shader3D, (), PassShader>;

pub type AssetDataCenterPipeline3D = AssetDataCenter<u64, Pipeline3D, ()>;
pub type AssetLoaderPipeline3D = AssetLoader<u64, ObjectID, Pipeline3D, (), PassPipeline>;

// #[derive(Debug, Default)]
// pub struct Shader3DLoader {
//     create_list: XHashMap<KeyShader3D, (Handle<ShaderEffectMeta>, EInstanceCode, BindGroupScene, BindGroupModel, Option<BindGroupTextureSamplers>)>,
//     pass01_wait_list: XHashMap<KeyShader3D, Vec<ObjectID>>,
// }
// impl Shader3DLoader {
//     pub fn new() -> Self {
//         Self {
//             create_list: XHashMap::default(),
//             pass01_wait_list: XHashMap::default(),
//         }
//     }
//     pub fn request(
//         &mut self,
//         idobj: ObjectID,
//         key: &KeyShader3D,
//         meta: Handle<ShaderEffectMeta>,
//         instance: EInstanceCode,
//         scene: &BindGroupScene,
//         model: &BindGroupModel, 
//         textures: Option<&BindGroupTextureSamplers>,
//     ) {
//         if self.create_list.get(key).is_none() {
//             let textures = if let Some(textures) = textures {
//                 Some(textures.clone())
//             } else {
//                 None
//             };
//             self.create_list.insert(key.clone(), (meta, instance.clone(), scene.clone(), model.clone(), textures));
//             self.pass01_wait_list.insert(key.clone(), vec![]);
//         }
//         self.pass01_wait_list.get_mut(key).unwrap().push(idobj);
//     }

//     pub fn single_create(
//         &mut self,
//         device: &RenderDevice,
//         asset_mgr: &Share<AssetMgr<Shader3D>>,
//         shader_cmd: &mut Commands<GameObject, PassShader>,
//     ) {
//         self.create_list.drain().for_each(|(key, item)| {
//             let (meta, instance, set_0, set_1, set_2) = item;
//             let data = meta.build(device, &key.key_meta, &key.key_attributes, &instance, &set_0, &set_1, set_2.as_ref(), None);
//             if let Some(shader) = asset_mgr.insert(key.clone(), data) {
//                 let data = PassShaderData::create(shader.clone(), &set_0, &set_1, set_2.as_ref());
//                 let data = Some(Arc::new(data));
//                 if let Some(mut list) = self.pass01_wait_list.remove(&key) {
//                     list.drain(..).for_each(|id| {
//                         log::info!("single_create: Shader ");
//                         shader_cmd.insert(id, PassShader(data.clone()));
//                     })
//                 }
//             }
//         });
//     }
// }

// #[derive(Debug, Default)]
// pub struct Pipeline3DLoader {
//     create_list: XHashMap<KeyPipeline3D, (Handle<Shader3D>, BindGroups3D, RenderGeometry)>,
//     pass01_wait_list: XHashMap<KeyPipeline3D, Vec<ObjectID>>,
// }
// impl Pipeline3DLoader {
//     pub fn new() -> Self {
//         Self {
//             create_list: XHashMap::default(),
//             pass01_wait_list: XHashMap::default(),
//         }
//     }
//     pub fn request(
//         &mut self,
//         idobj: ObjectID,
//         key: &KeyPipeline3D,
//         shader: Handle<Shader3D>,
//         bindgroups: BindGroups3D,
//         rendergeo: RenderGeometry,
//     ) {
//         if self.create_list.get(key).is_none() {
//             self.create_list.insert(key.clone(), (shader, bindgroups, rendergeo));
//             self.pass01_wait_list.insert(key.clone(), vec![]);
//         }
//         self.pass01_wait_list.get_mut(key).unwrap().push(idobj);
//     }

//     pub fn single_create(
//         &mut self, 
//         asset_mgr: &Share<AssetMgr<RenderRes<Pipeline3D>>>,
//         device: &RenderDevice,
//         pass01: &mut Commands<GameObject, PassDraw>,
//     ) {
//         self.create_list.drain().for_each(|(key, item)| {
//             let key_u64 = key.to_u64();

//             let (shader, bindgroups, rendergeo) = item;
//             let mut bind_group_layouts = [None, None, None, None];
//             for i in 0..4 {
//                 if let Some(val) = &bindgroups.0[i] {
//                     bind_group_layouts[i] = Some(val.layout());
//                 }
//             }
//             let pipeline = KeyPipeline3D::create(key.clone(), shader, bind_group_layouts, device);
//             if let Some(pipeline) = asset_mgr.insert(key_u64, pipeline) {
//                 let draw = DrawObj3D {
//                     pipeline: Some(pipeline),
//                     bindgroups: bindgroups.groups(),
//                     vertices: rendergeo.vertices(),
//                     instances: rendergeo.instances(),
//                     indices: rendergeo.indices.clone(),
//                 };
//                 let data = Some(Arc::new(draw));
//                 if let Some(mut list) = self.pass01_wait_list.remove(&key) {
//                     list.drain(..).for_each(|id| {
//                         pass01.insert(id, PassDraw(data.clone()));
//                     })
//                 }
//             }
//         });
//     }
// }

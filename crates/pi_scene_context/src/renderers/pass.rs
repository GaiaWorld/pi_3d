
use std::sync::Arc;

use pi_assets::asset::Handle;
use pi_render::{render_3d::{shader::shader::{KeyShader3D, Shader3D, EKeyShader3DSetBlock}}, renderer::{bind_group::BindGroupUsage, pipeline::{KeyRenderPipeline}, draw_obj::{DrawObj, DrawBindGroups, DrawBindGroup}, draw_obj_list::{DrawList}}, rhi::{pipeline::RenderPipeline, asset::RenderRes}};

use crate::{pass::TPassData};

#[derive(Debug, Clone)]
pub struct BindGroups3D(pub [Option<BindGroupUsage>;4]);
impl BindGroups3D {
    pub fn groups(&self) -> DrawBindGroups {
        let mut groups = DrawBindGroups::default();
        for i in 0..4 {
            if let Some(val) = &self.0[i as usize] {
                let val = DrawBindGroup::GroupUsage(val.clone());
                groups.insert_group(i, val);
            }
        }

        groups
    }
}
pub type KeyPipeline3D = KeyRenderPipeline<4, EKeyShader3DSetBlock>;
pub type Pipeline3D = RenderPipeline;
pub type Pipeline3DUsage = Handle<RenderRes<Pipeline3D>>;

pub type DrawObj3D = DrawObj;
pub type DrawList3D = DrawList;

/// * Set0
/// * 更新依赖: BindSceneEffect, BindViewer
#[derive(Default, Clone)]
pub struct Pass01Shader(pub Option<(KeyShader3D, Handle<Shader3D>, BindGroups3D)>);
impl TPassData<Option<(KeyShader3D, Handle<Shader3D>, BindGroups3D)>> for Pass01Shader {
    fn new(val: Option<(KeyShader3D, Handle<Shader3D>, BindGroups3D)>) -> Self { Self(val) }
    fn val(&self) -> &Option<(KeyShader3D, Handle<Shader3D>, BindGroups3D)> { &self.0 }
}
#[derive(Default, Clone)]
pub struct Pass02Shader(pub Option<(KeyShader3D, Handle<Shader3D>, BindGroups3D)>);
impl TPassData<Option<(KeyShader3D, Handle<Shader3D>, BindGroups3D)>> for Pass02Shader {
    fn new(val: Option<(KeyShader3D, Handle<Shader3D>, BindGroups3D)>) -> Self { Self(val) }
    fn val(&self) -> &Option<(KeyShader3D, Handle<Shader3D>, BindGroups3D)> { &self.0 }
}
#[derive(Default, Clone)]
pub struct Pass03Shader(pub Option<(KeyShader3D, Handle<Shader3D>, BindGroups3D)>);
impl TPassData<Option<(KeyShader3D, Handle<Shader3D>, BindGroups3D)>> for Pass03Shader {
    fn new(val: Option<(KeyShader3D, Handle<Shader3D>, BindGroups3D)>) -> Self { Self(val) }
    fn val(&self) -> &Option<(KeyShader3D, Handle<Shader3D>, BindGroups3D)> { &self.0 }
}
#[derive(Default, Clone)]
pub struct Pass04Shader(pub Option<(KeyShader3D, Handle<Shader3D>, BindGroups3D)>);
impl TPassData<Option<(KeyShader3D, Handle<Shader3D>, BindGroups3D)>> for Pass04Shader {
    fn new(val: Option<(KeyShader3D, Handle<Shader3D>, BindGroups3D)>) -> Self { Self(val) }
    fn val(&self) -> &Option<(KeyShader3D, Handle<Shader3D>, BindGroups3D)> { &self.0 }
}
#[derive(Default, Clone)]
pub struct Pass05Shader(pub Option<(KeyShader3D, Handle<Shader3D>, BindGroups3D)>);
impl TPassData<Option<(KeyShader3D, Handle<Shader3D>, BindGroups3D)>> for Pass05Shader {
    fn new(val: Option<(KeyShader3D, Handle<Shader3D>, BindGroups3D)>) -> Self { Self(val) }
    fn val(&self) -> &Option<(KeyShader3D, Handle<Shader3D>, BindGroups3D)> { &self.0 }
}
#[derive(Default, Clone)]
pub struct Pass06Shader(pub Option<(KeyShader3D, Handle<Shader3D>, BindGroups3D)>);
impl TPassData<Option<(KeyShader3D, Handle<Shader3D>, BindGroups3D)>> for Pass06Shader {
    fn new(val: Option<(KeyShader3D, Handle<Shader3D>, BindGroups3D)>) -> Self { Self(val) }
    fn val(&self) -> &Option<(KeyShader3D, Handle<Shader3D>, BindGroups3D)> { &self.0 }
}
#[derive(Default, Clone)]
pub struct Pass07Shader(pub Option<(KeyShader3D, Handle<Shader3D>, BindGroups3D)>);
impl TPassData<Option<(KeyShader3D, Handle<Shader3D>, BindGroups3D)>> for Pass07Shader {
    fn new(val: Option<(KeyShader3D, Handle<Shader3D>, BindGroups3D)>) -> Self { Self(val) }
    fn val(&self) -> &Option<(KeyShader3D, Handle<Shader3D>, BindGroups3D)> { &self.0 }
}
#[derive(Default, Clone)]
pub struct Pass08Shader(pub Option<(KeyShader3D, Handle<Shader3D>, BindGroups3D)>);
impl TPassData<Option<(KeyShader3D, Handle<Shader3D>, BindGroups3D)>> for Pass08Shader {
    fn new(val: Option<(KeyShader3D, Handle<Shader3D>, BindGroups3D)>) -> Self { Self(val) }
    fn val(&self) -> &Option<(KeyShader3D, Handle<Shader3D>, BindGroups3D)> { &self.0 }
}

#[derive(Default, Clone)]
pub struct Pass01Pipeline(pub Option<(KeyPipeline3D, Pipeline3DUsage, BindGroups3D)>);
impl TPassData<Option<(KeyPipeline3D, Pipeline3DUsage, BindGroups3D)>> for Pass01Pipeline {
    fn new(val: Option<(KeyPipeline3D, Pipeline3DUsage, BindGroups3D)>) -> Self { Self(val) }
    fn val(&self) -> &Option<(KeyPipeline3D, Pipeline3DUsage, BindGroups3D)> { &self.0 }
}
#[derive(Default, Clone)]
pub struct Pass02Pipeline(pub Option<(KeyPipeline3D, Pipeline3DUsage, BindGroups3D)>);
impl TPassData<Option<(KeyPipeline3D, Pipeline3DUsage, BindGroups3D)>> for Pass02Pipeline {
    fn new(val: Option<(KeyPipeline3D, Pipeline3DUsage, BindGroups3D)>) -> Self { Self(val) }
    fn val(&self) -> &Option<(KeyPipeline3D, Pipeline3DUsage, BindGroups3D)> { &self.0 }
}
#[derive(Default, Clone)]
pub struct Pass03Pipeline(pub Option<(KeyPipeline3D, Pipeline3DUsage, BindGroups3D)>);
impl TPassData<Option<(KeyPipeline3D, Pipeline3DUsage, BindGroups3D)>> for Pass03Pipeline {
    fn new(val: Option<(KeyPipeline3D, Pipeline3DUsage, BindGroups3D)>) -> Self { Self(val) }
    fn val(&self) -> &Option<(KeyPipeline3D, Pipeline3DUsage, BindGroups3D)> { &self.0 }
}
#[derive(Default, Clone)]
pub struct Pass04Pipeline(pub Option<(KeyPipeline3D, Pipeline3DUsage, BindGroups3D)>);
impl TPassData<Option<(KeyPipeline3D, Pipeline3DUsage, BindGroups3D)>> for Pass04Pipeline {
    fn new(val: Option<(KeyPipeline3D, Pipeline3DUsage, BindGroups3D)>) -> Self { Self(val) }
    fn val(&self) -> &Option<(KeyPipeline3D, Pipeline3DUsage, BindGroups3D)> { &self.0 }
}
#[derive(Default, Clone)]
pub struct Pass05Pipeline(pub Option<(KeyPipeline3D, Pipeline3DUsage, BindGroups3D)>);
impl TPassData<Option<(KeyPipeline3D, Pipeline3DUsage, BindGroups3D)>> for Pass05Pipeline {
    fn new(val: Option<(KeyPipeline3D, Pipeline3DUsage, BindGroups3D)>) -> Self { Self(val) }
    fn val(&self) -> &Option<(KeyPipeline3D, Pipeline3DUsage, BindGroups3D)> { &self.0 }
}
#[derive(Default, Clone)]
pub struct Pass06Pipeline(pub Option<(KeyPipeline3D, Pipeline3DUsage, BindGroups3D)>);
impl TPassData<Option<(KeyPipeline3D, Pipeline3DUsage, BindGroups3D)>> for Pass06Pipeline {
    fn new(val: Option<(KeyPipeline3D, Pipeline3DUsage, BindGroups3D)>) -> Self { Self(val) }
    fn val(&self) -> &Option<(KeyPipeline3D, Pipeline3DUsage, BindGroups3D)> { &self.0 }
}
#[derive(Default, Clone)]
pub struct Pass07Pipeline(pub Option<(KeyPipeline3D, Pipeline3DUsage, BindGroups3D)>);
impl TPassData<Option<(KeyPipeline3D, Pipeline3DUsage, BindGroups3D)>> for Pass07Pipeline {
    fn new(val: Option<(KeyPipeline3D, Pipeline3DUsage, BindGroups3D)>) -> Self { Self(val) }
    fn val(&self) -> &Option<(KeyPipeline3D, Pipeline3DUsage, BindGroups3D)> { &self.0 }
}
#[derive(Default, Clone)]
pub struct Pass08Pipeline(pub Option<(KeyPipeline3D, Pipeline3DUsage, BindGroups3D)>);
impl TPassData<Option<(KeyPipeline3D, Pipeline3DUsage, BindGroups3D)>> for Pass08Pipeline {
    fn new(val: Option<(KeyPipeline3D, Pipeline3DUsage, BindGroups3D)>) -> Self { Self(val) }
    fn val(&self) -> &Option<(KeyPipeline3D, Pipeline3DUsage, BindGroups3D)> { &self.0 }
}

pub struct Pass01Draw(pub Option<Arc<DrawObj3D>>);
impl TPassData<Option<Arc<DrawObj3D>>> for Pass01Draw {
    fn new(val: Option<Arc<DrawObj3D>>) -> Self { Self(val) }
    fn val(&self) -> &Option<Arc<DrawObj3D>> { &self.0 }
}
pub struct Pass02Draw(pub Option<Arc<DrawObj3D>>);
impl TPassData<Option<Arc<DrawObj3D>>> for Pass02Draw {
    fn new(val: Option<Arc<DrawObj3D>>) -> Self { Self(val) }
    fn val(&self) -> &Option<Arc<DrawObj3D>> { &self.0 }
}
pub struct Pass03Draw(pub Option<Arc<DrawObj3D>>);
impl TPassData<Option<Arc<DrawObj3D>>> for Pass03Draw {
    fn new(val: Option<Arc<DrawObj3D>>) -> Self { Self(val) }
    fn val(&self) -> &Option<Arc<DrawObj3D>> { &self.0 }
}
pub struct Pass04Draw(pub Option<Arc<DrawObj3D>>);
impl TPassData<Option<Arc<DrawObj3D>>> for Pass04Draw {
    fn new(val: Option<Arc<DrawObj3D>>) -> Self { Self(val) }
    fn val(&self) -> &Option<Arc<DrawObj3D>> { &self.0 }
}
pub struct Pass05Draw(pub Option<Arc<DrawObj3D>>);
impl TPassData<Option<Arc<DrawObj3D>>> for Pass05Draw {
    fn new(val: Option<Arc<DrawObj3D>>) -> Self { Self(val) }
    fn val(&self) -> &Option<Arc<DrawObj3D>> { &self.0 }
}
pub struct Pass06Draw(pub Option<Arc<DrawObj3D>>);
impl TPassData<Option<Arc<DrawObj3D>>> for Pass06Draw {
    fn new(val: Option<Arc<DrawObj3D>>) -> Self { Self(val) }
    fn val(&self) -> &Option<Arc<DrawObj3D>> { &self.0 }
}
pub struct Pass07Draw(pub Option<Arc<DrawObj3D>>);
impl TPassData<Option<Arc<DrawObj3D>>> for Pass07Draw {
    fn new(val: Option<Arc<DrawObj3D>>) -> Self { Self(val) }
    fn val(&self) -> &Option<Arc<DrawObj3D>> { &self.0 }
}
pub struct Pass08Draw(pub Option<Arc<DrawObj3D>>);
impl TPassData<Option<Arc<DrawObj3D>>> for Pass08Draw {
    fn new(val: Option<Arc<DrawObj3D>>) -> Self { Self(val) }
    fn val(&self) -> &Option<Arc<DrawObj3D>> { &self.0 }
}

use pi_assets::asset::Handle;
use pi_render::{render_3d::{shader::shader::{KeyShader3D, Shader3D, EKeyShader3DSetBlock}, bind_groups::scene::BindGroupScene}, renderer::{bind_group::BindGroupUsage, pipeline::{KeyRenderPipeline, RenderPipeline, TRenderPipeline}, draw_obj::{TBindGroups, DrawObj}, draw_obj_list::{DrawList}}};

use crate::{pass::TPassData, geometry::geometry::RenderGeometry};

#[derive(Debug, Clone)]
pub struct BindGroups3D(pub [Option<BindGroupUsage>;4]);
impl TBindGroups for BindGroups3D {
    fn bindgroups<'a>(&'a self) -> std::slice::Iter<'a, Option<BindGroupUsage>> {
        self.0.iter()
    }
}
pub type KeyPipeline3D = KeyRenderPipeline<4, EKeyShader3DSetBlock>;
pub type Pipeline3D = RenderPipeline<4, EKeyShader3DSetBlock>;
#[derive(Debug, Clone)]
pub struct Pipeline3DUsage(pub Handle<Pipeline3D>);
impl TRenderPipeline for Pipeline3DUsage {
    fn pipeline(&self) -> &pi_render::rhi::pipeline::RenderPipeline {
        &self.0.0
    }
}
pub type DrawObj3D = DrawObj<Pipeline3DUsage, BindGroups3D, RenderGeometry>;
pub type DrawList3D = DrawList<Pipeline3DUsage, BindGroups3D, RenderGeometry>;

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
pub struct Pass01Pipeline(pub Option<(KeyPipeline3D, Handle<Pipeline3D>, BindGroups3D)>);
impl TPassData<Option<(KeyPipeline3D, Handle<Pipeline3D>, BindGroups3D)>> for Pass01Pipeline {
    fn new(val: Option<(KeyPipeline3D, Handle<Pipeline3D>, BindGroups3D)>) -> Self { Self(val) }
    fn val(&self) -> &Option<(KeyPipeline3D, Handle<Pipeline3D>, BindGroups3D)> { &self.0 }
}
#[derive(Default, Clone)]
pub struct Pass02Pipeline(pub Option<(KeyPipeline3D, Handle<Pipeline3D>, BindGroups3D)>);
impl TPassData<Option<(KeyPipeline3D, Handle<Pipeline3D>, BindGroups3D)>> for Pass02Pipeline {
    fn new(val: Option<(KeyPipeline3D, Handle<Pipeline3D>, BindGroups3D)>) -> Self { Self(val) }
    fn val(&self) -> &Option<(KeyPipeline3D, Handle<Pipeline3D>, BindGroups3D)> { &self.0 }
}
#[derive(Default, Clone)]
pub struct Pass03Pipeline(pub Option<(KeyPipeline3D, Handle<Pipeline3D>, BindGroups3D)>);
impl TPassData<Option<(KeyPipeline3D, Handle<Pipeline3D>, BindGroups3D)>> for Pass03Pipeline {
    fn new(val: Option<(KeyPipeline3D, Handle<Pipeline3D>, BindGroups3D)>) -> Self { Self(val) }
    fn val(&self) -> &Option<(KeyPipeline3D, Handle<Pipeline3D>, BindGroups3D)> { &self.0 }
}
#[derive(Default, Clone)]
pub struct Pass04Pipeline(pub Option<(KeyPipeline3D, Handle<Pipeline3D>, BindGroups3D)>);
impl TPassData<Option<(KeyPipeline3D, Handle<Pipeline3D>, BindGroups3D)>> for Pass04Pipeline {
    fn new(val: Option<(KeyPipeline3D, Handle<Pipeline3D>, BindGroups3D)>) -> Self { Self(val) }
    fn val(&self) -> &Option<(KeyPipeline3D, Handle<Pipeline3D>, BindGroups3D)> { &self.0 }
}
#[derive(Default, Clone)]
pub struct Pass05Pipeline(pub Option<(KeyPipeline3D, Handle<Pipeline3D>, BindGroups3D)>);
impl TPassData<Option<(KeyPipeline3D, Handle<Pipeline3D>, BindGroups3D)>> for Pass05Pipeline {
    fn new(val: Option<(KeyPipeline3D, Handle<Pipeline3D>, BindGroups3D)>) -> Self { Self(val) }
    fn val(&self) -> &Option<(KeyPipeline3D, Handle<Pipeline3D>, BindGroups3D)> { &self.0 }
}
#[derive(Default, Clone)]
pub struct Pass06Pipeline(pub Option<(KeyPipeline3D, Handle<Pipeline3D>, BindGroups3D)>);
impl TPassData<Option<(KeyPipeline3D, Handle<Pipeline3D>, BindGroups3D)>> for Pass06Pipeline {
    fn new(val: Option<(KeyPipeline3D, Handle<Pipeline3D>, BindGroups3D)>) -> Self { Self(val) }
    fn val(&self) -> &Option<(KeyPipeline3D, Handle<Pipeline3D>, BindGroups3D)> { &self.0 }
}
#[derive(Default, Clone)]
pub struct Pass07Pipeline(pub Option<(KeyPipeline3D, Handle<Pipeline3D>, BindGroups3D)>);
impl TPassData<Option<(KeyPipeline3D, Handle<Pipeline3D>, BindGroups3D)>> for Pass07Pipeline {
    fn new(val: Option<(KeyPipeline3D, Handle<Pipeline3D>, BindGroups3D)>) -> Self { Self(val) }
    fn val(&self) -> &Option<(KeyPipeline3D, Handle<Pipeline3D>, BindGroups3D)> { &self.0 }
}
#[derive(Default, Clone)]
pub struct Pass08Pipeline(pub Option<(KeyPipeline3D, Handle<Pipeline3D>, BindGroups3D)>);
impl TPassData<Option<(KeyPipeline3D, Handle<Pipeline3D>, BindGroups3D)>> for Pass08Pipeline {
    fn new(val: Option<(KeyPipeline3D, Handle<Pipeline3D>, BindGroups3D)>) -> Self { Self(val) }
    fn val(&self) -> &Option<(KeyPipeline3D, Handle<Pipeline3D>, BindGroups3D)> { &self.0 }
}

pub struct Pass01Draw(pub Option<DrawObj3D>);
impl TPassData<Option<DrawObj3D>> for Pass01Draw {
    fn new(val: Option<DrawObj3D>) -> Self { Self(val) }
    fn val(&self) -> &Option<DrawObj3D> { &self.0 }
}
pub struct Pass02Draw(pub Option<DrawObj3D>);
impl TPassData<Option<DrawObj3D>> for Pass02Draw {
    fn new(val: Option<DrawObj3D>) -> Self { Self(val) }
    fn val(&self) -> &Option<DrawObj3D> { &self.0 }
}
pub struct Pass03Draw(pub Option<DrawObj3D>);
impl TPassData<Option<DrawObj3D>> for Pass03Draw {
    fn new(val: Option<DrawObj3D>) -> Self { Self(val) }
    fn val(&self) -> &Option<DrawObj3D> { &self.0 }
}
pub struct Pass04Draw(pub Option<DrawObj3D>);
impl TPassData<Option<DrawObj3D>> for Pass04Draw {
    fn new(val: Option<DrawObj3D>) -> Self { Self(val) }
    fn val(&self) -> &Option<DrawObj3D> { &self.0 }
}
pub struct Pass05Draw(pub Option<DrawObj3D>);
impl TPassData<Option<DrawObj3D>> for Pass05Draw {
    fn new(val: Option<DrawObj3D>) -> Self { Self(val) }
    fn val(&self) -> &Option<DrawObj3D> { &self.0 }
}
pub struct Pass06Draw(pub Option<DrawObj3D>);
impl TPassData<Option<DrawObj3D>> for Pass06Draw {
    fn new(val: Option<DrawObj3D>) -> Self { Self(val) }
    fn val(&self) -> &Option<DrawObj3D> { &self.0 }
}
pub struct Pass07Draw(pub Option<DrawObj3D>);
impl TPassData<Option<DrawObj3D>> for Pass07Draw {
    fn new(val: Option<DrawObj3D>) -> Self { Self(val) }
    fn val(&self) -> &Option<DrawObj3D> { &self.0 }
}
pub struct Pass08Draw(pub Option<DrawObj3D>);
impl TPassData<Option<DrawObj3D>> for Pass08Draw {
    fn new(val: Option<DrawObj3D>) -> Self { Self(val) }
    fn val(&self) -> &Option<DrawObj3D> { &self.0 }
}

use std::{sync::Arc, ops::Deref, slice::Iter};

use pi_scene_shell::prelude::*;
use smallvec::SmallVec;

use super::base::DrawList3D;


#[derive(Clone, Default, Resource)]
pub struct RendererHasher(pub DefaultHasher);

#[derive(Clone, Copy, Component)]
pub struct RendererEnable(pub bool);

#[derive(Clone, Copy, Component)]
pub struct RendererBlend(pub bool);

#[derive(Clone, Copy, Component)]
pub struct RenderViewport(pub f32, pub f32, pub f32, pub f32, pub f32, pub f32);
impl Default for RenderViewport {
    fn default() -> Self {
        Self(0., 0., 1., 1., 0., 1.)
    }
}
impl RenderViewport {
    pub fn val(&self) -> (f32, f32, f32, f32, f32, f32) {
        (self.0, self.1, self.2, self.3, self.4, self.5)
    }
}

#[derive(Clone, Copy, Component)]
pub struct RenderSize(pub(crate) u32, pub(crate) u32);
impl RenderSize {
    pub fn new(width: u32, height: u32) -> Self {
        Self(width, height)
    }
    pub fn width(&self) -> u32 { self.0 }
    pub fn height(&self) -> u32 { self.1 }
}

#[derive(Clone, Copy, Component)]
pub struct RenderColorFormat(pub ColorFormat);
impl Default for RenderColorFormat {
    fn default() -> Self {
        Self(ColorFormat::Rgba8Unorm)
    }
}
impl RenderColorFormat {
    pub fn desc(&self) -> SmallVec<[TextureDescriptor; 1]> {
        SmallVec::from_slice(
            &[TextureDescriptor {
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: self.0.val(),
                usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
                base_mip_level: 0,
                base_array_layer: 0,
                array_layer_count: None,
                view_dimension: Some(wgpu::TextureViewDimension::D2),
            }]
        )
    }
}

#[derive(Clone, Copy, Component)]
pub struct RenderColorClear(pub u8, pub u8, pub u8, pub u8);
impl Default for RenderColorClear {
    fn default() -> Self {
        Self(0, 0, 0, 0)
    }
}
impl RenderColorClear {
    pub fn color(&self) -> wgpu::Color {
        wgpu::Color { r: self.0 as f64 / 255.0, g: self.1 as f64 / 255.0, b: self.2 as f64 / 255.0, a: self.3 as f64 / 255.0 }
    }
}

#[derive(Clone, Copy, Component)]
pub struct RenderDepthFormat(pub DepthStencilFormat);
impl Default for RenderDepthFormat {
    fn default() -> Self {
        Self(DepthStencilFormat::Depth32Float)
    }
}
impl RenderDepthFormat {
    pub fn need_depth(&self) -> bool {
        match self.0 {
            DepthStencilFormat::None => false,
            DepthStencilFormat::Stencil8 => false,
            DepthStencilFormat::Depth16Unorm => true,
            DepthStencilFormat::Depth24Plus => true,
            DepthStencilFormat::Depth24PlusStencil8 => true,
            DepthStencilFormat::Depth32Float => true,
            DepthStencilFormat::Depth32FloatStencil8 => true,
        }
    }
    pub fn desc(&self) -> Option<TextureDescriptor> {
        if let Some(val) = self.0.val() {
            Some(
                TextureDescriptor {
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format: val,
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                    base_mip_level: 0,
                    base_array_layer: 0,
                    array_layer_count: None,
                    view_dimension: Some(wgpu::TextureViewDimension::D2),
                }
            )
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Component)]
pub struct RenderDepthClear(pub f32);
impl Default for RenderDepthClear {
    fn default() -> Self {
        Self(0.)
    }
}

#[derive(Clone, Copy, Component)]
pub struct RenderStencilClear(pub u32);
impl Default for RenderStencilClear {
    fn default() -> Self {
        Self(0)
    }
}

#[derive(Clone, Copy, Component)]
pub struct RenderAutoClearColor(pub bool);
impl Default for RenderAutoClearColor {
    fn default() -> Self {
        Self(false)
    }
}

#[derive(Clone, Copy, Component)]
pub struct RenderAutoClearDepth(pub bool);
impl Default for RenderAutoClearDepth {
    fn default() -> Self {
        Self(false)
    }
}

#[derive(Clone, Copy, Component)]
pub struct RenderAutoClearStencil(pub bool);
impl Default for RenderAutoClearStencil {
    fn default() -> Self {
        Self(false)
    }
}

#[derive(Clone, Copy, Component)]
pub enum RenderTargetMode {
    Auto,
    Window,
    Custom(u32, u32),
}
impl Default for RenderTargetMode {
    fn default() -> Self {
        Self::Auto
    }
}

#[derive(Clone, Component)]
pub enum RendererRenderTarget {
    None(Option<Arc<SafeTargetView>>),
    FinalRender,
    Custom(Arc<SafeTargetView>),
}
impl Default for RendererRenderTarget {
    fn default() -> Self {
        Self::None(None)
    }
}
impl RendererRenderTarget {
    pub fn view(&self) -> Option<&wgpu::TextureView> {
        match self {
            RendererRenderTarget::None(srt) => if let Some(srt) = srt {
                let view: &wgpu::TextureView = srt.target().colors[0].0.as_ref().deref();
                Some(view)
            } else {
                None
            },
            RendererRenderTarget::FinalRender => None,
            RendererRenderTarget::Custom(srt) => {
                let view: &wgpu::TextureView = srt.target().colors[0].0.as_ref().deref();
                Some(view)
            },
        }
    }
    pub fn depth_view(&self) -> Option<&wgpu::TextureView> {
        match self {
            RendererRenderTarget::None(srt) => if let Some(srt) = srt {
                if let Some(view) = srt.target().depth.as_ref() {
                    Some(view.0.as_ref().deref())
                } else {
                    None
                }
            } else { None },
            RendererRenderTarget::FinalRender => None,
            RendererRenderTarget::Custom(srt) => {
                if let Some(view) = srt.target().depth.as_ref() {
                    Some(view.0.as_ref().deref())
                } else {
                    None
                }
            },
        }
    }
    pub fn is_active(&self) -> bool {
        match self {
            RendererRenderTarget::None(_) => true,
            RendererRenderTarget::FinalRender => true,
            RendererRenderTarget::Custom(_) => true,
        }
    }
}

#[derive(Component)]
pub struct Renderer {
    pub ready: bool,
    pub draws: DrawList3D,
    pub vertexs: usize,
}
impl Renderer {
    pub fn new() -> Self {
        Self {
            draws: DrawList3D { list: vec![], viewport: (0., 0., 1., 1., 0., 1.) },
            ready: false,
            vertexs: 0,
        }
    }
    pub fn clear(&mut self) {
        self.draws.list.clear();
        self.ready = false;
        self.vertexs = 0;
    }

    pub fn reset(&mut self) {
        self.ready = true;
    }
}

#[derive(Clone, Default, Component)]
pub struct ViewerRenderersInfo(pub Vec<Entity>, pub Vec<PassTag>);
impl ViewerRenderersInfo {
    pub fn add(&mut self, renderer: Entity, pass: PassTag) {
        // log::error!("Add Renderer {:?}", (renderer, pass));
        match self.0.binary_search(&renderer) {
            Ok(_) => {
                // self.0.insert(idx, renderer);
            },
            Err(idx) => {
                self.0.insert(idx, renderer);
                self.1.insert(idx, pass);
            },
        }
    }
    pub fn remove(&mut self, renderer: Entity) {
        // log::error!("Remove Renderer");
        match self.0.binary_search(&renderer) {
            Ok(idx) => {
                self.0.remove(idx);
                self.1.remove(idx);
            },
            Err(_) => todo!(),
        }
    }
    pub fn renderers(&self) -> Iter<Entity> {
        self.0.iter()
    }
    pub fn passtags(&self) -> Iter<PassTag> {
        self.1.iter()
    }
    pub fn get(&self, idx: usize) -> (Option<&Entity>, Option<&PassTag>) {
        (
            self.0.get(idx),
            self.1.get(idx),
        )
    }
}

#[derive(Component)]
pub struct DirtyViewerRenderersInfo;

#[derive(Default, Resource)]
pub struct RendererDrawCallRecord(pub XHashMap<Entity, u32>);

use crate::{ecs::*, prelude::MemSize};

use pi_bevy_asset::ShareAssetMgr;
use pi_bevy_render_plugin::{constant::texture_sampler::{ColorFormat, DepthStencilFormat}, PiSafeAtlasAllocator, PiScreenTexture};
use pi_render::{components::view::target_alloc::{SafeTargetView, ShareTargetView, TargetDescriptor, TextureDescriptor}, renderer::sampler::{BindDataSampler, KeySampler, SamplerRes}, rhi::device::RenderDevice};
use pi_scene_math::Number;
use pi_share::Share;
use pi_slotmap::SlotMap;
use smallvec::SmallVec;

pub type KeyRenderTarget = u64;

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum KeyCustomRenderTarget {
    Custom(KeyRenderTarget),
    FinalRender(bool),
}

#[derive(Clone)]
pub struct CustomRenderTarget {
    pub rt: ShareTargetView,
    pub sampler: BindDataSampler,
    pub width: u32,
    pub height: u32,
    pub color_format: ColorFormat,
    pub depth_stencil_format: DepthStencilFormat,
}
impl CustomRenderTarget {
    pub fn new(
        device: &RenderDevice, sample: KeySampler,
        asset_samp: &ShareAssetMgr<SamplerRes>, atlas_allocator: &PiSafeAtlasAllocator,
        color_format: ColorFormat, depth_stencil_format: DepthStencilFormat, width: u32, height: u32, screen: &PiScreenTexture
    ) -> Option<Self> {
        let currlist: Vec<ShareTargetView> = vec![];
        if let Some(sampler) = BindDataSampler::create(sample, &device, &asset_samp) {
            
            let mut default_width = width;
            let mut default_height = height;
            if let Some(screen) = &screen.0 {
                if let Some(screen) = screen.texture() {
                    let swidth = screen.width();
                    let sheight = screen.height();
                    if width <= swidth && height <= sheight {
                        default_width = swidth;
                        default_height = sheight;
                    }
                }
            }
            default_width = ((default_width - 1) / 32 + 1) * 32;
            default_height = ((default_height - 1) / 32 + 1) * 32;

            let target_type = atlas_allocator.create_type(
                TargetDescriptor {
                    colors_descriptor: Self::color_desc(&color_format),
                    need_depth: Self::need_depth(&depth_stencil_format),
                    default_width,
                    default_height,
                    depth_descriptor: Self::depth_desc(&depth_stencil_format)
                }
            );

            // log::warn!("CustomRenderTarget Allocate. {:?}", (width, height, color_format, depth_stencil_format));
            
            // log::warn!("New RenderTarget: {:?}", (format.desc(), depth.desc()));
            let rt = atlas_allocator.allocate_alone_not_share(
                width,
                height,
                target_type.clone(),
                currlist.iter(),
                true
            );
            // log::error!("CustomTarget: {:?}", (color_format, depth_stencil_format));
            Some(
                Self { rt: Share::new(rt), sampler, width, height, color_format, depth_stencil_format }
            )
        } else {
            None
        }
    }

    pub fn from_srt(
        srt: Option<Share<SafeTargetView>>,
        device: &RenderDevice,
        asset_samp: &ShareAssetMgr<SamplerRes>
    ) -> Option<Self> {
        if let Some(srt) = srt {
            if let Some(color_format) = ColorFormat::new(srt.target().colors[0].0.texture.format()) {
                let depth_stencil_format = if let Some(depth) = &srt.target().depth {
                    depth_format(depth.0.texture.format())
                } else { Some(DepthStencilFormat::None) };

                if let Some(depth_stencil_format) = depth_stencil_format {
                    let width = srt.target().width;
                    let height = srt.target().height;
                    if let Some(sampler) = BindDataSampler::create(KeySampler::default(), &device, &asset_samp) {
                        Some(
                            Self {
                                rt: srt, sampler, width, height, color_format, depth_stencil_format
                            }
                        )
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }
    pub fn tilloff(&self, viewport: (Number, Number, Number, Number)) -> (Number, Number, Number, Number) {
        let rect = self.rt.rect();
        // log::error!("TillOff {:?}", (rect.min.x, rect.min.y, rect.max.x, rect.max.y));
        let sx = (rect.max.x - rect.min.x).abs() as Number / self.rt.target().width as Number;
        let sy = (rect.max.y - rect.min.y).abs() as Number / self.rt.target().height as Number;
        let ox = rect.min.x as Number / self.rt.target().width as Number;
        let oy = rect.min.y as Number / self.rt.target().height as Number;

        let (vox, voy, vsx, vsy) = viewport;

        // (
        //     sx,
        //     sy,
        //     ox,
        //     oy,
        // )
        // f(y) = y * sy + oy => { y = 1 - x, f(x)  = (1 - x) * sy + oy = x * (-sy) + (sy + oy) }
        // (
        //     sx,
        //     sy * -1,
        //     ox,
        //     oy + sy,
        // )
        
        #[cfg(feature = "webgl_context")]
        let result = (
                sx * vsx,
                sy * vsy,
                ox * vsx + vox,
                oy * vsy + voy,
            );
        #[cfg(not(feature = "webgl_context"))]
        let result = (
                sx * vsx,
                (sy * -1.0) * vsy,
                ox * vsx + vox,
                (oy + sy) * vsy + voy,
            );

        result
    
    }

    pub fn color_desc(format: &ColorFormat) -> SmallVec<[TextureDescriptor; 1]> {
        SmallVec::from_slice(
            &[TextureDescriptor {
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: format.val(),
                usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
                base_mip_level: 0,
                base_array_layer: 0,
                array_layer_count: None,
                view_dimension: Some(wgpu::TextureViewDimension::D2),
            }]
        )
    }

    pub fn need_depth(format: &DepthStencilFormat) -> bool {
        match format {
            DepthStencilFormat::None => false,
            DepthStencilFormat::Stencil8 => false,
            DepthStencilFormat::Depth16Unorm => true,
            DepthStencilFormat::Depth24Plus => true,
            DepthStencilFormat::Depth24PlusStencil8 => true,
            DepthStencilFormat::Depth32Float => true,
            DepthStencilFormat::Depth32FloatStencil8 => true,
        }
    }
    pub fn depth_desc(format: &DepthStencilFormat) -> Option<TextureDescriptor> {
        if let Some(val) = format.val() {
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

#[derive(Resource, Default)]
pub struct CustomRenderTargets(pub SlotMap<KeyRenderTarget, Option<CustomRenderTarget>>, pub Vec<(KeyRenderTarget, KeySampler, ColorFormat, DepthStencilFormat, u32, u32)>);
impl CustomRenderTargets {
    pub fn create_sync(
        &mut self,
        device: &RenderDevice,
        asset_samp: &ShareAssetMgr<SamplerRes>, atlas_allocator: &PiSafeAtlasAllocator,
        sample: KeySampler,
        color_format: ColorFormat, depth_stencil_format: DepthStencilFormat, width: u32, height: u32,
        screen: &PiScreenTexture,
    ) -> Option<KeyRenderTarget> {
        if let Some(rt) = CustomRenderTarget::new(device, sample, asset_samp, atlas_allocator, color_format, depth_stencil_format, width, height, screen) {
            let key = self.0.insert(Some(rt));
            Some(key)
        } else{
            None
        }

    }
    pub fn create(
        &mut self,
        sample: KeySampler,
        color_format: ColorFormat, depth_stencil_format: DepthStencilFormat, width: u32, height: u32
    ) -> Option<KeyRenderTarget> {
        let key = self.0.insert(None);
        self.1.push((key, sample, color_format, depth_stencil_format, width, height));
        Some(key)
    }
    pub fn get(&self, key: KeyRenderTarget) -> Option<CustomRenderTarget> {
        if let Some(target) = self.0.get(key) {
            if let Some(target) = target {
                Some(target.clone())
            } else {
                None
            }
        } else {
            None
        }
    }
    pub fn update(
        &mut self,
        device: &RenderDevice,
        asset_samp: &ShareAssetMgr<SamplerRes>,
        atlas_allocator: &PiSafeAtlasAllocator,
        screen: &PiScreenTexture,
    ) {
        self.1.drain(..).for_each(|(key, sample, color_format, depth_stencil_format, width, height)| {
            if let Some(item) = self.0.get_mut(key) {
                if let Some(rt) = CustomRenderTarget::new(device, sample, asset_samp, atlas_allocator, color_format, depth_stencil_format, width, height, screen) {
                    *item = Some(rt)
                }
            }
        });
    }
    pub fn insert_srt(&mut self, srt: Option<ShareTargetView>, mut key: Option<KeyRenderTarget>,
        device: &RenderDevice,
        asset_samp: &ShareAssetMgr<SamplerRes>
    ) -> Option<KeyRenderTarget> {
        if let Some(key) = key {
            self.delete(key);
        }
        let id = self.0.insert(CustomRenderTarget::from_srt(srt, device, asset_samp));
        key = Some(id);

        return key;
    }
    pub fn delete(&mut self, key: KeyRenderTarget) {
        self.0.remove(key);
    }
}
impl MemSize for CustomRenderTargets {
    fn memsize(&self) -> usize {
        self.0.capacity() * 32
        + self.1.capacity() * 48
    }
}

fn depth_format(val: wgpu::TextureFormat) -> Option<DepthStencilFormat> {
    match val {
        wgpu::TextureFormat::Stencil8 => Some(DepthStencilFormat::Stencil8),
        wgpu::TextureFormat::Depth16Unorm => Some(DepthStencilFormat::Depth16Unorm),
        wgpu::TextureFormat::Depth24Plus => Some(DepthStencilFormat::Depth24Plus),
        wgpu::TextureFormat::Depth24PlusStencil8 => Some(DepthStencilFormat::Depth24PlusStencil8),
        wgpu::TextureFormat::Depth32Float => Some(DepthStencilFormat::Depth32Float),
        wgpu::TextureFormat::Depth32FloatStencil8 => Some(DepthStencilFormat::Depth32FloatStencil8),
        _ => { None }
    }
}
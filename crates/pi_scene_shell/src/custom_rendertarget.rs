// use bevy_ecs::system::Resource;
use pi_bevy_asset::ShareAssetMgr;
use pi_bevy_render_plugin::{constant::texture_sampler::{ColorFormat, DepthStencilFormat}, PiSafeAtlasAllocator};
use pi_render::{components::view::target_alloc::{TextureDescriptor, ShareTargetView, TargetDescriptor}, renderer::{sampler::{BindDataSampler, KeySampler, SamplerRes}}, rhi::device::RenderDevice};
use pi_scene_math::Number;
use pi_share::Share;
use pi_slotmap::SlotMap;
use smallvec::SmallVec;

pub type KeyRenderTarget = u64;

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum KeyCustomRenderTarget {
    Custom(KeyRenderTarget),
    FinalRender,
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
        color_format: ColorFormat, depth_stencil_format: DepthStencilFormat, width: u32, height: u32
    ) -> Option<Self> {
        let currlist: Vec<ShareTargetView> = vec![];
        if let Some(sampler) = BindDataSampler::create(sample, &device, &asset_samp) {
            let target_type = atlas_allocator.create_type(
                TargetDescriptor {
                    colors_descriptor: Self::color_desc(&color_format),
                    need_depth: Self::need_depth(&depth_stencil_format),
                    default_width: width,
                    default_height: height,
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

    pub fn tilloff(&self, viewport: (Number, Number, Number, Number)) -> (Number, Number, Number, Number) {
        let rect = self.rt.rect();
        let sx = self.width as Number / self.rt.target().width as Number;
        let sy = self.height as Number / self.rt.target().height as Number;
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
        (
            sx * vsx,
            (sy * -1.0) * vsy,
            ox * vsx + vox,
            (oy + sy) * vsy + voy,
        )
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

#[derive( Default)]
pub struct CustomRenderTargets(pub SlotMap<KeyRenderTarget, CustomRenderTarget>);
impl CustomRenderTargets {
    pub fn create(
        &mut self,
        device: &RenderDevice, sample: KeySampler,
        asset_samp: &ShareAssetMgr<SamplerRes>, atlas_allocator: &PiSafeAtlasAllocator,
        color_format: ColorFormat, depth_stencil_format: DepthStencilFormat, width: u32, height: u32
    ) -> Option<KeyRenderTarget> {
        if let Some(rt) = CustomRenderTarget::new(device, sample, asset_samp, atlas_allocator, color_format, depth_stencil_format, width, height) {
            Some(self.0.insert(rt))
        } else {
            None
        }
    }
    pub fn get(&self, key: KeyRenderTarget) -> Option<CustomRenderTarget> {
        if let Some(target) = self.0.get(key) {
            Some(target.clone())
        } else {
            None
        }
    }
    pub fn delete(&mut self, key: KeyRenderTarget) {
        self.0.remove(key);
    }
}

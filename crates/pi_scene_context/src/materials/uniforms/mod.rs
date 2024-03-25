
use pi_scene_shell::prelude::*;
use pi_scene_math::Number;

pub mod uniform;
pub mod float;
pub mod vec2;
pub mod vec4;
// pub mod mat2;
pub mod mat4;
// pub mod int;
pub mod uint;
pub mod texture;
// pub mod sys_texture;
// pub mod sys_uniform;

#[derive(Resource)]
pub struct ResourceDefaultTextures {
    pub white1: Handle<TextureRes>,
    pub white2: Handle<TextureRes>,
    pub white3: Handle<TextureRes>,
    pub black1: Handle<TextureRes>,
    pub black2: Handle<TextureRes>,
    pub black3: Handle<TextureRes>,
}

pub(crate) fn update_data(
    data: &mut [Number],
    slot: usize,
    value: &[Number],
    num_count: usize,
) {
    if value.len() >= num_count {
        for i in 0..num_count {
            data[slot * num_count + i] = value[i];
        }
    }
}

pub fn set_up_uniforms(
    asset_tex: &ShareAssetMgr<TextureRes>,
    device: &PiRenderDevice,
    queue: &PiRenderQueue,
) -> ResourceDefaultTextures {
    let desc = wgpu::TextureViewDescriptor::default();
    // 
    let texture = DefaultTexture::create(device, queue, EDefaultTexture::White, wgpu::TextureDimension::D1);
    let texture = texture.create_view(&desc);
    let white1 = asset_tex.insert(Atom::from(DefaultTexture::WHITE_1D).asset_u64() as u64, TextureRes::new(1, 1, 4, texture, true, wgpu::TextureFormat::Rgba8Unorm)).unwrap();
    // 
    let texture = DefaultTexture::create(device, queue, EDefaultTexture::White, wgpu::TextureDimension::D2);
    let texture = texture.create_view(&desc);
    let white2 = asset_tex.insert(Atom::from(DefaultTexture::WHITE_2D).asset_u64() as u64, TextureRes::new(1, 1, 4, texture, true, wgpu::TextureFormat::Rgba8Unorm)).unwrap();
    // 
    let texture = DefaultTexture::create(device, queue, EDefaultTexture::White, wgpu::TextureDimension::D3);
    let texture = texture.create_view(&desc);
    let white3 = asset_tex.insert(Atom::from(DefaultTexture::WHITE_3D).asset_u64() as u64, TextureRes::new(1, 1, 4, texture, true, wgpu::TextureFormat::Rgba8Unorm)).unwrap();
    // 
    let texture = DefaultTexture::create(device, queue, EDefaultTexture::Black, wgpu::TextureDimension::D1);
    let texture = texture.create_view(&desc);
    let black1 = asset_tex.insert(Atom::from(DefaultTexture::BLACK_1D).asset_u64() as u64, TextureRes::new(1, 1, 4, texture, true, wgpu::TextureFormat::Rgba8Unorm)).unwrap();
    // 
    let texture = DefaultTexture::create(device, queue, EDefaultTexture::Black, wgpu::TextureDimension::D2);
    let texture = texture.create_view(&desc);
    let black2 = asset_tex.insert(Atom::from(DefaultTexture::BLACK_2D).asset_u64() as u64, TextureRes::new(1, 1, 4, texture, true, wgpu::TextureFormat::Rgba8Unorm)).unwrap();
    // 
    let texture = DefaultTexture::create(device, queue, EDefaultTexture::Black, wgpu::TextureDimension::D3);
    let texture = texture.create_view(&desc);
    let black3 = asset_tex.insert(Atom::from(DefaultTexture::BLACK_3D).asset_u64() as u64, TextureRes::new(1, 1, 4, texture, true, wgpu::TextureFormat::Rgba8Unorm)).unwrap();
    // log::warn!("DefaultTexture OK!");

    ResourceDefaultTextures {
        white1,
        white2,
        white3,
        black1,
        black2,
        black3,
    }
}
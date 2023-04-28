
use pi_atom::Atom;
use pi_engine_shell::{prelude::*};
use pi_scene_math::Number;


pub mod value_uniform;
pub mod texture_uniform;
// pub mod sys_mat4;
// pub mod sys_mat2;
// pub mod sys_float;
// pub mod sys_int;
// pub mod sys_uint;
// pub mod sys_vec2;
// pub mod sys_vec4;
pub mod uniform;
pub mod float;
pub mod vec2;
pub mod vec4;
pub mod mat2;
pub mod mat4;
pub mod int;
pub mod uint;
pub mod boolean;
pub mod byte;
pub mod texture;
pub mod sys_texture;
pub mod sys_uniform;
pub mod sys_pass;

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
    asset_tex: Res<ShareAssetMgr<TextureRes>>,
    device: Res<PiRenderDevice>,
    queue: Res<PiRenderQueue>,
) {
    let desc = wgpu::TextureViewDescriptor::default();
    // 
    let texture = DefaultTexture::create(&device, &queue, EDefaultTexture::White, wgpu::TextureDimension::D1);
    let texture = texture.create_view(&desc);
    asset_tex.insert(Atom::from(DefaultTexture::WHITE_1D).get_hash() as u64, TextureRes::new(1, 1, 4, texture, true));
    // 
    let texture = DefaultTexture::create(&device, &queue, EDefaultTexture::White, wgpu::TextureDimension::D2);
    let texture = texture.create_view(&desc);
    asset_tex.insert(Atom::from(DefaultTexture::WHITE_2D).get_hash() as u64, TextureRes::new(1, 1, 4, texture, true));
    // 
    let texture = DefaultTexture::create(&device, &queue, EDefaultTexture::White, wgpu::TextureDimension::D3);
    let texture = texture.create_view(&desc);
    asset_tex.insert(Atom::from(DefaultTexture::WHITE_3D).get_hash() as u64, TextureRes::new(1, 1, 4, texture, true));
    // 
    let texture = DefaultTexture::create(&device, &queue, EDefaultTexture::Black, wgpu::TextureDimension::D1);
    let texture = texture.create_view(&desc);
    asset_tex.insert(Atom::from(DefaultTexture::BLACK_1D).get_hash() as u64, TextureRes::new(1, 1, 4, texture, true));
    // 
    let texture = DefaultTexture::create(&device, &queue, EDefaultTexture::Black, wgpu::TextureDimension::D2);
    let texture = texture.create_view(&desc);
    asset_tex.insert(Atom::from(DefaultTexture::BLACK_2D).get_hash() as u64, TextureRes::new(1, 1, 4, texture, true));
    // 
    let texture = DefaultTexture::create(&device, &queue, EDefaultTexture::Black, wgpu::TextureDimension::D3);
    let texture = texture.create_view(&desc);
    asset_tex.insert(Atom::from(DefaultTexture::BLACK_3D).get_hash() as u64, TextureRes::new(1, 1, 4, texture, true));
}
use naga::Handle;
use pi_engine_shell::object::ObjectID;
use pi_render::rhi::{asset::TextureRes, texture::Sampler};
use pi_scene_context::{texture::{texture_sampler::TextureSamplerDesc}, shaders::{FragmentUniformBindTexture, FragmentUniformBindTextureSampler}};


pub struct EmissiveTexture {
    pub id: ObjectID,
}
impl FragmentUniformBindTexture for EmissiveTexture {
    const TEXTURE_BIND: u8 = 0;

    const TEXTURE_SAMPLER_TYPE: wgpu::TextureSampleType = wgpu::TextureSampleType::Uint;

    const DIM: wgpu::TextureViewDimension = wgpu::TextureViewDimension::D2;

    const MULTI: bool = false;
}
impl FragmentUniformBindTextureSampler for EmissiveTexture {
    const SAMPLER_BIND: u8 = 1;

    const SAMPLER_TYPE: wgpu::SamplerBindingType = wgpu::SamplerBindingType::Filtering;

}


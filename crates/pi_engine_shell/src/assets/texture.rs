use pi_render::renderer::texture::*;
use wgpu::{TextureView, TextureFormat};


pub trait ITexture {
    fn view(&self) -> &TextureView;
    fn format(&self) -> TextureFormat;
    fn key(&self) -> KeyTextureViewUsage;
}
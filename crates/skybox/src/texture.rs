// use pi_hal::image;
// use pi_render::rhi::{device::RenderDevice, texture::Texture, RenderQueue};

// pub struct DefaultTexture(Vec<u8>, u32, u32);

// impl DefaultTexture {
//     pub fn new(device: &RenderDevice, queue: &RenderQueue, path: &str) -> Self {
//         let image = image::from_path(path).unwrap();

//         // let size = wgpu::Extent3d {
//         //     width: image.1,
//         //     height: image.2,
//         //     depth_or_array_layers: 1,
//         // };

//         // let texture = device.create_texture(&wgpu::TextureDescriptor {
//         //     label: None,
//         //     size,
//         //     mip_level_count: 1,
//         //     sample_count: 1,
//         //     dimension: wgpu::TextureDimension::D2,
//         //     format: wgpu::TextureFormat::Rgba8Unorm,
//         //     usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING,
//         // });

//         // queue.write_texture(
//         //     texture.as_image_copy(),
//         //     &image.0,
//         //     wgpu::ImageDataLayout {
//         //         offset: 0,
//         //         bytes_per_row: std::num::NonZeroU32::new(image.1 * 4),
//         //         rows_per_image: None,
//         //     },
//         //     size,
//         // );

//         DefaultTexture(image.0, image.1, image.2)
//     }
// }

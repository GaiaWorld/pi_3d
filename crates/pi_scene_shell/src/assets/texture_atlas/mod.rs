mod texture_atlas;
mod base;

use std::mem::replace;

use pi_hash::XHashMap;
use pi_render::renderer::texture::ImageTextureFrame;
use pi_world::single_res::{SingleRes, SingleResMut};
pub use texture_atlas::*;
pub use base::*;
use crate::prelude::ResMut;


pub struct DataTextureSubData {
	pub data: Option<Vec<u8>>,
	pub dataoffset: u64,
	pub xoffset: u32,
	pub yoffset: u32,
	pub width: u32,
	pub height: u32,
	pub aspect: Option<wgpu::TextureAspect>,
	pub depth_or_array_layers: u32,
}

#[derive(crate::prelude::Resource, Default)]
pub struct DataTextureCmds {
    pub createdata: XHashMap<pi_atom::Atom, (DataTextureSubData, wgpu::TextureFormat, wgpu::TextureViewDimension, crate::prelude::KeyImageTextureFrame)>,
    pub updatedata: XHashMap<pi_atom::Atom, Vec<DataTextureSubData>>,
    pub record: XHashMap<pi_atom::Atom, crate::prelude::Handle<crate::prelude::ImageTextureFrame>>,
}

pub fn sys_update_data_texture(
    mut refs: ResMut<DataTextureCmds>,
    device: SingleRes<crate::prelude::PiRenderDevice>,
    queue: SingleRes<crate::prelude::PiRenderQueue>,
    imgtex_asset: SingleRes<crate::prelude::ShareAssetMgr<crate::prelude::ImageTextureFrame>>,
) {
    let mut createdata = replace(&mut refs.createdata, XHashMap::default());
    createdata.drain().for_each(|(key, (data, format, dimension, texkey))| {
        if let Some(res) = refs.record.get(&key) {
			if res.texture().format != format || res.texture().view_dimension != dimension {
				// 
			} else {
				if let Some(d) = &data.data {
					let origin = wgpu::Origin3d { x: data.xoffset, y: data.yoffset, z: 0 };
					ImageTextureFrame::update_sub(&res.texture().texture, &queue, origin, data.width, data.height, data.depth_or_array_layers, data.aspect, d, data.dataoffset);
					// res.update(&queue, data.xoffset, data.yoffset, data.width, data.height, data.depth_or_array_layers, data.aspect, d, data.dataoffset);
				}
			}
        } else {
            if let Some(res) = imgtex_asset.get(&texkey) {
				if res.texture().format != format || res.texture().view_dimension != dimension {
					// 
				} else {
					if let Some(d) = &data.data {
						let origin = wgpu::Origin3d { x: data.xoffset, y: data.yoffset, z: 0 };
						ImageTextureFrame::update_sub(&res.texture().texture, &queue, origin, data.width, data.height, data.depth_or_array_layers, data.aspect, d, data.dataoffset);
						// res.update(&queue, data.xoffset, data.yoffset, data.width, data.height, data.depth_or_array_layers, data.aspect, d, data.dataoffset);
					}
					refs.record.insert(key, res);
				}
            } else {
				let d = if let Some(data) = &data.data {
					Some(data.as_slice())
				} else { None };
                let texture = crate::prelude::ImageTextureFrame::create_data_texture(
					&device, &queue, &texkey.url, data.width, data.height, format, dimension, true, 0, data.aspect, d, data.dataoffset
				);
                match imgtex_asset.insert(texkey, ImageTextureFrame::new(texture)) {
                    Ok(data) => refs.record.insert(key, data),
                    Err(_) => None,
                };
            }
		}
    });
    let mut updatedata = replace(&mut refs.updatedata, XHashMap::default());
	updatedata.drain().for_each(|(key, mut data)| {
        if let Some(res) = refs.record.get(&key) {
			data.drain(..).for_each(|data| {
				if let Some(d) = &data.data {
					let origin = wgpu::Origin3d { x: data.xoffset, y: data.yoffset, z: 0 };
					ImageTextureFrame::update_sub(&res.texture().texture, &queue, origin, data.width, data.height, data.depth_or_array_layers, data.aspect, d, data.dataoffset);
					// res.update(&queue, data.xoffset, data.yoffset, data.width, data.height, data.depth_or_array_layers, data.aspect, d, data.dataoffset);
				}
			});
        }
	});
}
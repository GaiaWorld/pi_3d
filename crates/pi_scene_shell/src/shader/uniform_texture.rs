use std::{hash::Hash, sync::Arc};

use derive_deref::Deref;
use pi_atom::Atom;

use pi_render::{
    renderer::{
        buildin_data::{DefaultTexture, EDefaultTexture},
        sampler::KeySampler, shader_stage::EShaderStage, texture::EKeyTexture
    },
    rhi::sampler::EAddressMode
};

use crate::{binds::BindEffectTextureInfo, prelude::EngineCustomPlugins};

use super::{UniformPropertyName, ShaderSetBind, TUnifromShaderProperty};

#[derive(Clone, Hash)]
pub struct UniformTexture2DDesc {
    pub slotname: UniformPropertyName,
    pub tex_sampler_type: wgpu::TextureSampleType,
    pub dimision: wgpu::TextureViewDimension,
    pub multisampled: bool,
    pub stage: EShaderStage,
    pub initial: EDefaultTexture,
}
impl Default for UniformTexture2DDesc {
    fn default() -> Self {
        Self {
            slotname: UniformPropertyName::from("_MainTex"),
            tex_sampler_type: wgpu::TextureSampleType::Float { filterable: true },
            dimision: wgpu::TextureViewDimension::D2,
            multisampled: false,
            stage: EShaderStage::FRAGMENT,
            initial: EDefaultTexture::White,
        }
    }
}
impl UniformTexture2DDesc {
    pub fn new(
        slotname: UniformPropertyName,
        tex_sampler_type: wgpu::TextureSampleType,
        dimision: wgpu::TextureViewDimension,
        multisampled: bool,
        stage: EShaderStage,
        initial: EDefaultTexture,
    ) -> Self {
        Self {
            slotname,
            tex_sampler_type,
            dimision,
            multisampled,
            stage,
            initial
        }
    }
    pub fn new2d(
        slotname: UniformPropertyName,
        stage: EShaderStage,
    ) -> Arc<Self> {
        Arc::new(
            Self {
                slotname,
                tex_sampler_type: wgpu::TextureSampleType::Float { filterable: true },
                dimision: wgpu::TextureViewDimension::D2,
                multisampled: false,
                stage,
                initial: EDefaultTexture::White,
            }
        )
    }
    pub fn size(&self) -> usize {
        self.slotname.as_bytes().len() + 1 + 1 + 1 + 1
    }
    pub fn sampler_type(&self) -> wgpu::SamplerBindingType {
        match self.tex_sampler_type {
            wgpu::TextureSampleType::Float { filterable } => if filterable { wgpu::SamplerBindingType::Filtering } else { wgpu::SamplerBindingType::NonFiltering } ,
            wgpu::TextureSampleType::Depth => wgpu::SamplerBindingType::Filtering,
            wgpu::TextureSampleType::Sint => wgpu::SamplerBindingType::NonFiltering,
            wgpu::TextureSampleType::Uint => wgpu::SamplerBindingType::NonFiltering,
        }
    }
}
impl TUnifromShaderProperty for UniformTexture2DDesc {
    fn tag(&self) -> &UniformPropertyName {
        &self.slotname
    }
}
impl PartialEq for UniformTexture2DDesc {
    fn eq(&self, other: &Self) -> bool {
        self.tag().eq(other.tag())
    }
}
impl Eq for UniformTexture2DDesc {
    fn assert_receiver_is_total_eq(&self) {}
}
impl PartialOrd for UniformTexture2DDesc {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.tag().partial_cmp(other.tag())
    }
}
impl Ord for UniformTexture2DDesc {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

// #[derive(Clone, Hash, PartialEq, Eq)]
// pub struct RuntimeUniformTexture2DDesc {
//     pub slotname: UniformPropertyName,
//     pub runtimeindex: u16,
//     pub tex_sampler_type: wgpu::TextureSampleType,
//     pub dimision: ETextureViewDimension,
//     pub stage: EShaderStage,
// }

// #[derive(Clone, Hash, PartialEq, Eq)]
// pub struct RuntimeEffectUniformTexture2DDescs {
//     pub textures: Vec<RuntimeUniformTexture2DDesc>,
// }
// impl TBindDescToShaderCode for RuntimeEffectUniformTexture2DDescs {
//     fn vs_code(&self, set: u32, bind: u32) -> String {
//         let mut result = String::from("");
//         for item in self.textures.iter() {
//             if item.stage.mode() & wgpu::ShaderStages::VERTEX == wgpu::ShaderStages::VERTEX {
//                 result += &texture_bind_code_mat(&item.tex_sampler_type, item.dimision.mode(), &item.slotname, &item.slotname, set, bind);
//             }
//         }
//         result
//     }

//     fn fs_code(&self, set: u32, bind: u32) -> String {
//         let mut result = String::from("");
//         for item in self.textures.iter() {
//             if item.stage.mode() & wgpu::ShaderStages::FRAGMENT == wgpu::ShaderStages::FRAGMENT {
//                 result += &texture_bind_code_mat(&item.tex_sampler_type, item.dimision.mode(), &item.slotname, &item.slotname, set, bind);
//             }
//         }
//         result
//     }
// }

/// * 材质的纹理设置参数
#[derive(Clone, Hash)]
pub struct UniformTextureWithSamplerParam {
    pub slotname: UniformPropertyName,
    pub wrapu: EAddressMode,
    pub wrapv: EAddressMode,
    pub wrapw: EAddressMode,
    pub url: EKeyTexture,
    pub sample: KeySampler,
    pub texture_sample: wgpu::TextureSampleType,
    pub sampler_bind_type: wgpu::SamplerBindingType,
}
impl Default for UniformTextureWithSamplerParam {
    fn default() -> Self {
        Self {
            slotname: UniformPropertyName::from(DefaultTexture::WHITE_2D),
            wrapu: EAddressMode::ClampToEdge,
            wrapv: EAddressMode::ClampToEdge,
            wrapw: EAddressMode::ClampToEdge,
            url: EKeyTexture::Tex(Atom::from(DefaultTexture::path(EDefaultTexture::White, wgpu::TextureDimension::D2))),
            sample: KeySampler::default(),
            texture_sample: wgpu::TextureSampleType::Float { filterable: true },
            sampler_bind_type: wgpu::SamplerBindingType::Filtering,
        }
    }
}
impl PartialEq for UniformTextureWithSamplerParam {
    fn eq(&self, other: &Self) -> bool {
        self.slotname.eq(&other.slotname) && self.url.eq(&other.url)
    }
}
impl Eq for UniformTextureWithSamplerParam {
    fn assert_receiver_is_total_eq(&self) {}
}
impl PartialOrd for UniformTextureWithSamplerParam {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.slotname.partial_cmp(&other.slotname)
    }
}
impl Ord for UniformTextureWithSamplerParam {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

/// * 从 shader 描述生成的 纹理描述数组,
/// * 能通过 纹理属性名称 获取 纹理槽位序号
/// * 能通过 纹理的使用信息 生成 纹理的Uniform描述数组(数组序号对应纹理槽位序号)
/// * 如果某个槽位没有设置 则 根据 shader 描述中对应声明使用默认纹理设置
#[derive(Clone, Deref)]
pub struct EffectUniformTexture2DDescs(pub Vec<Arc<UniformTexture2DDesc>>);
impl From<Vec<Arc<UniformTexture2DDesc>>> for EffectUniformTexture2DDescs {
    fn from(mut value: Vec<Arc<UniformTexture2DDesc>>) -> Self {
        value.sort_by(|a, b| { a.slotname.cmp(&b.slotname) });

        Self (value)
    }
}
impl EffectUniformTexture2DDescs {
    
}


pub fn texture_type_code(tex_sampler_type: &wgpu::TextureSampleType, dimision: wgpu::TextureViewDimension) -> String {
    match tex_sampler_type {
        wgpu::TextureSampleType::Float { .. } => match dimision {
            wgpu::TextureViewDimension::D1          => String::from(" texture1D "),
            wgpu::TextureViewDimension::D2          => String::from(" texture2D "),
            wgpu::TextureViewDimension::D2Array     => String::from(" texture2DArray "),
            wgpu::TextureViewDimension::Cube        => String::from(" textureCube "),
            wgpu::TextureViewDimension::CubeArray   => String::from(" textureCubeArray "),
            wgpu::TextureViewDimension::D3          => String::from(" texture3D "),
        },
        wgpu::TextureSampleType::Depth => String::from(" texture2DShadow "),
        wgpu::TextureSampleType::Sint => String::from(" itexture2D "),
        wgpu::TextureSampleType::Uint => String::from(" utexture2D "),
    }
}
pub fn texture_bind_code(tex_sampler_type: &wgpu::TextureSampleType, dimision: wgpu::TextureViewDimension, name: &str, set: u32, bind: u32) -> String {

    // layout(set = 2, binding = 0) uniform texture2D _MainTex;
    let mut result = ShaderSetBind::code_set_bind_head(set, bind);
    result += texture_type_code(tex_sampler_type, dimision).as_str();
    result += name;
    result += ";"; result += crate::prelude::S_BREAK;

    result
}

pub fn texture_bind_code_mat(engineopt: &EngineCustomPlugins, tex_sampler_type: &wgpu::TextureSampleType, dimension: wgpu::TextureViewDimension, name: &str, bindname: &str, set: u32, bind: u32) -> String {

    // layout(set = 2, binding = 0) uniform texture2D _MainTex;
    let mut result = ShaderSetBind::code_set_bind_head(set, bind);
    result += texture_type_code(tex_sampler_type, dimension).as_str();
    result += name;
    result += ";"; result += crate::prelude::S_BREAK;
    result += texture_code(name, bindname, tex_sampler_type, dimension, engineopt).as_str();

    result
}

fn texture_code(slotname: &str, bindname: &str, tex_sampler_type: &wgpu::TextureSampleType, dimension: wgpu::TextureViewDimension, engineopt: &EngineCustomPlugins) -> String {
    let mut uv = String::from("uvAtlas(uv * tilloff.xy + tilloff.zw + os, ");
    uv += slotname;
    uv += BindEffectTextureInfo::SUFFIX_TILLOFF;
    // if engineopt.disenable_material_array == false {
        uv += "[vMatIdx]";
    // }
    uv += ",";
    uv += slotname;
    uv += BindEffectTextureInfo::SUFFIX_ADDRESS;
    // if engineopt.disenable_material_array == false {
        uv += "[vMatIdx]";
    // }
    uv += ")";
    
    let mut coord = String::from("");
    coord += slotname;
    coord += BindEffectTextureInfo::SUFFIX_ADDRESS;
    // if engineopt.disenable_material_array == false {
        coord += "[vMatIdx]";
    // };
    coord += ".w";

    // let uv = String::from("uv * tilloff.xy + tilloff.zw + os");
    let uvatlas = &uv;
    match tex_sampler_type {
        wgpu::TextureSampleType::Float { .. } => match dimension {
            wgpu::TextureViewDimension::D1          => String::from("vec4 Get") + slotname + "(float uv){ return texture(sampler1D(" + bindname + ", sampler" + bindname + "), uv); }\n",
            wgpu::TextureViewDimension::D2          => String::from("vec4 Get") + slotname + "(vec2 uv, vec2 os, vec4 tilloff){ return texture(sampler2D(" + bindname + ", sampler" + bindname + "), " + uvatlas +"); }\n",
            wgpu::TextureViewDimension::D2Array     => String::from("vec4 Get") + slotname + "(vec2 uv, vec2 os, vec4 tilloff){ return texture(sampler2DArray(" + bindname + ", sampler" + bindname + "), vec3(" + uvatlas +", " + &coord + ")); }\n",
            wgpu::TextureViewDimension::Cube        => String::from(""),
            wgpu::TextureViewDimension::CubeArray   => String::from(""),
            wgpu::TextureViewDimension::D3          => String::from("vec4 Get") + slotname + "(vec2 uv, vec2 os, float layer, vec4 tilloff){ return texture(sampler3D(" + bindname + ", sampler" + bindname + "), vec3(" + uvatlas + ", layer)); }\n",
        },
        wgpu::TextureSampleType::Depth => String::from("vec4 Get") + slotname + "(vec2 uv, vec2 os, vec4 tilloff){ return texture(sampler2D(" + bindname + ", sampler" + bindname + "), " + uvatlas +"); }\n",
        wgpu::TextureSampleType::Sint => String::from("ivec4 Get") + slotname + "(vec2 uv, vec2 os, vec4 tilloff){ return texture(sampler2D(" + bindname + ", sampler" + bindname + "), " + uvatlas +"); }\n",
        wgpu::TextureSampleType::Uint => String::from("uvec4 Get") + slotname + "(vec2 uv, vec2 os, vec4 tilloff){ return texture(sampler2D(" + bindname + ", sampler" + bindname + "), " + uvatlas +"); }\n",
    }
}

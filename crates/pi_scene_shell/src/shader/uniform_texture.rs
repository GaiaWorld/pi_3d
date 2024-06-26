use std::{sync::Arc, hash::Hash};

use derive_deref::Deref;
use pi_atom::Atom;

use pi_render::renderer::{buildin_data::{EDefaultTexture, DefaultTexture}, sampler::KeySampler, shader_stage::EShaderStage, texture::EKeyTexture};

use super::{UniformPropertyName, TBindDescToShaderCode, ShaderSetBind, TUnifromShaderProperty};


#[derive(Clone, Hash, PartialEq, Eq)]
pub struct UniformSamplerDesc {
    pub slotname: UniformPropertyName,
    pub ty: wgpu::SamplerBindingType,
    pub stage: EShaderStage,
}
impl UniformSamplerDesc {
    pub fn base(texture: &UniformTexture2DDesc) -> Arc<Self> {
        Arc::new(
            Self {
                slotname: UniformPropertyName::from(String::from(crate::prelude::S_SAMPLER) + texture.slotname.as_str()),
                ty: wgpu::SamplerBindingType::Filtering,
                stage: texture.stage
            }
        )
    }
    fn _ty_code(&self) -> String {
        match self.ty {
            wgpu::SamplerBindingType::Filtering     => String::from(crate::prelude::S_SPACE) + crate::prelude::S_SAMPLER + crate::prelude::S_SPACE,
            wgpu::SamplerBindingType::NonFiltering  => String::from(crate::prelude::S_SPACE) + crate::prelude::S_SAMPLER + crate::prelude::S_SPACE,
            wgpu::SamplerBindingType::Comparison    => String::from(crate::prelude::S_SPACE) + "sampler_comparison" + crate::prelude::S_SPACE,
        }
    }
    fn _code(&self, set: u32, bind: u32) -> String {

        // layout(set = 2, binding = 0) uniform texture2D _MainTex;
        let mut result = ShaderSetBind::code_set_bind_head(set, bind);
        result += self._ty_code().as_str();
        result += self.slotname.as_str();
        result += ";"; result += crate::prelude::S_BREAK;

        result
    }
}
impl TBindDescToShaderCode for UniformSamplerDesc {
    fn vs_code(&self, set: u32, bind: u32) -> String {
        if self.stage.mode() & wgpu::ShaderStages::VERTEX == wgpu::ShaderStages::VERTEX {
            self._code(set, bind)
        } else {
            String::from("")
        }
    }

    fn fs_code(&self, set: u32, bind: u32) -> String {
        if self.stage.mode() & wgpu::ShaderStages::FRAGMENT == wgpu::ShaderStages::FRAGMENT {
            self._code(set, bind)
        } else {
            String::from("")
        }
    }
}

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
impl TBindDescToShaderCode for UniformTexture2DDesc {
    fn vs_code(&self, set: u32, bind: u32) -> String {
        if self.stage.mode() & wgpu::ShaderStages::VERTEX == wgpu::ShaderStages::VERTEX {
            texture_bind_code(&self.tex_sampler_type, self.dimision, &self.slotname, set, bind)
        } else {
            String::from("")
        }
    }

    fn fs_code(&self, set: u32, bind: u32) -> String {
        if self.stage.mode() & wgpu::ShaderStages::FRAGMENT == wgpu::ShaderStages::FRAGMENT {
            texture_bind_code(&self.tex_sampler_type, self.dimision, &self.slotname, set, bind)
        } else {
            String::from("")
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

/// * 材质的纹理设置参数
#[derive(Clone, Hash)]
pub struct UniformTextureWithSamplerParam {
    pub slotname: UniformPropertyName,
    pub filter: bool,
    pub sample: KeySampler,
    pub url: EKeyTexture,
}
impl Default for UniformTextureWithSamplerParam {
    fn default() -> Self {
        Self {
            slotname: UniformPropertyName::from(DefaultTexture::WHITE_2D),
            filter: true,
            sample: KeySampler::linear_clamp(),
            url: EKeyTexture::Tex(Atom::from(DefaultTexture::path(EDefaultTexture::White, wgpu::TextureDimension::D2))),
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

/// * 根据 shader 描述 & 设置的效果纹理参数 构建的纹理使用信息
/// * 数据放在渲染物体上
#[derive(Clone, Default, Hash, PartialEq, Eq)]
pub struct EffectUniformTextureWithSamplerUseinfo(pub Vec<(Arc<UniformTextureWithSamplerParam>, Arc<UniformTexture2DDesc>, Arc<UniformSamplerDesc>)>);

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
    /// * 根据用户的纹理设置数组, 填补未设置的槽, 以补全所有需要的纹理设置
    /// * 允许用户不设置纹理,自动使用默认纹理
    pub fn use_info(&self, mut param: Vec<Arc<UniformTextureWithSamplerParam>>) -> EffectUniformTextureWithSamplerUseinfo {
        param.sort_by(|a, b| { a.slotname.cmp(&b.slotname) });

        let mut result = EffectUniformTextureWithSamplerUseinfo::default();
        // 某个槽位没有设置 则 根据 shader 描述中对应声明使用默认纹理设置
        self.0.iter().for_each(|item| {
            let slotname = &item.slotname;
            let useinfo = match param.binary_search_by(|probe| probe.slotname.cmp(slotname)) {
                Ok(index) => {
                    let param = param.get(index).unwrap();
                    let sampler = Arc::new(
                        UniformSamplerDesc {
                            slotname: UniformPropertyName::from(String::from(crate::prelude::S_SAMPLER) + slotname.as_str()),
                            ty: if param.filter { wgpu::SamplerBindingType::Filtering } else { wgpu::SamplerBindingType::NonFiltering },
                            stage: item.stage,
                        }
                    );
                    (param.clone(), item.clone(), sampler)
                },
                Err(_) => {
                    let param = UniformTextureWithSamplerParam {
                        slotname: slotname.clone(),
                        filter: true,
                        sample: KeySampler::default(),
                        url: EKeyTexture::Tex(Atom::from(DefaultTexture::path(item.initial, wgpu::TextureDimension::D2))),
                    };
                    (Arc::new(param), item.clone(), UniformSamplerDesc::base(item))
                },
            };

            result.0.push(useinfo);
        });

        result
    }
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

pub fn texture_bind_code_mat(tex_sampler_type: &wgpu::TextureSampleType, dimision: wgpu::TextureViewDimension, name: &str, set: u32, bind: u32, idx: u32) -> String {

    // layout(set = 2, binding = 0) uniform texture2D _MainTex;
    let mut result = ShaderSetBind::code_set_bind_head(set, bind);
    result += texture_type_code(tex_sampler_type, dimision).as_str();
    result += name;
    result += ";"; result += crate::prelude::S_BREAK;
    result += texture_code(name, idx, tex_sampler_type, dimision).as_str();

    result
}

fn texture_code(name: &str, idx: u32, tex_sampler_type: &wgpu::TextureSampleType, dimision: wgpu::TextureViewDimension) -> String {
    let temp = idx.to_string();
    let idx = temp.as_str();
    // let uv = String::from("uvAtlas(uv * tilloff.xy + tilloff.zw + os, " + idx + ")");
    let uv = String::from("uv * tilloff.xy + tilloff.zw + os");
    let uvatlas = &uv;
    match tex_sampler_type {
        wgpu::TextureSampleType::Float { .. } => match dimision {
            wgpu::TextureViewDimension::D1          => String::from("vec4 Get") + name + "(float uv){ return texture(sampler1D(" + name + ", sampler" + name + "), uv); }\n",
            wgpu::TextureViewDimension::D2          => String::from("vec4 Get") + name + "(vec2 uv, vec2 os, vec4 tilloff){ return texture(sampler2D(" + name + ", sampler" + name + "), " + uvatlas +"); }\n",
            wgpu::TextureViewDimension::D2Array     => String::from("vec4 Get") + name + "(vec2 uv, vec2 os, float layer, vec4 tilloff){ return texture(sampler2DArray(" + name + ", sampler" + name + "), vec3(" + uvatlas +", layer)); }\n",
            wgpu::TextureViewDimension::Cube        => String::from(""),
            wgpu::TextureViewDimension::CubeArray   => String::from(""),
            wgpu::TextureViewDimension::D3          => String::from("vec4 Get") + name + "(vec2 uv, vec2 os, float layer, vec4 tilloff){ return texture(sampler3D(" + name + ", sampler" + name + "), vec3(" + uvatlas + ", layer)); }\n",
        },
        wgpu::TextureSampleType::Depth => String::from("vec4 Get") + name + "(vec2 uv, vec2 os, vec4 tilloff){ return texture(sampler2D(" + name + ", sampler" + name + "), " + uvatlas +"); }\n",
        wgpu::TextureSampleType::Sint => String::from("ivec4 Get") + name + "(vec2 uv, vec2 os, vec4 tilloff){ return texture(sampler2D(" + name + ", sampler" + name + "), " + uvatlas +"); }\n",
        wgpu::TextureSampleType::Uint => String::from("uvec4 Get") + name + "(vec2 uv, vec2 os, vec4 tilloff){ return texture(sampler2D(" + name + ", sampler" + name + "), " + uvatlas +"); }\n",
    }
}

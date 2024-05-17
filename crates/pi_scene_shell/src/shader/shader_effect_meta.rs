use std::sync::Arc;

use pi_assets::asset::{Asset, Size};
use pi_atom::Atom;
use pi_bevy_asset::{TAssetCapacity, AssetCapacity};
use wgpu::ShaderSource;

use pi_render::{
    renderer::{
        shader::*,
        buildin_data::EDefaultTexture,
        shader_stage::EShaderStage,
        attributes::*,
    },
    rhi::device::RenderDevice
};

use super::{
    block_code::{BlockCode, BlockCodeAtom, TToBlockCodeAtom},
    varying_code::{VaryingCode, Varyings},
    shader_defines::ShaderDefinesSet,
    uniform_value::{MaterialValueBindDesc, UniformPropertyVec4, UniformPropertyVec2, UniformPropertyFloat,  UniformPropertyUint}, 
    uniform_texture::{UniformTexture2DDesc, EffectUniformTexture2DDescs},
    shader::*, Varying, TUnifromShaderProperty,
    buildin_var::*
};

pub type BindDefine = u32;

/// 材质代码
/// * vs
///   * 定义和声明
///     * Shader Name
///     * #defines
///     * Attributes VS Define Code
///     * BindGroups VS Define Code
///     * Effect VS Define Code
///     * Varyings Define
///     * ERenderAlignment Define Code
///   * main 函数内部代码
///     * 基础 Attributes 引入
///     * Instance Running Code (Shader 自带定义块代码 - 常量 、 方法, 不包含 Uniform 定义)
///     * ERenderAlignment Running Code
///     * Skin Running Code
///     * VS Before Effect Snippets
///     * Effect VS Running Code
///     * VS After Effect Snippets
///     * Effect Varying Code
/// * fs
///   * 定义和声明
///     * Shader Name
///     * #defines
///     * BindGroups FS Define Code
///     * Effect FS Define Code (Shader 自带定义块代码 - 常量 、 方法, 不包含 Uniform 定义)
///     * Varyings Define
///   * main 函数内部代码
///     * Skin Running Code
///     * FS Before Effect Snippets
///     * Effect VS Running Code
///     * FS After Effect Snippets
#[derive(Clone)]
pub struct ShaderEffectMeta {
    pub uniforms: Arc<MaterialValueBindDesc>,
    pub textures: Arc<EffectUniformTexture2DDescs>,
    // pub samplers: Vec<UniformSamplerDesc>,
    pub varyings: Varyings,
    pub material_instance_code: String,
    /// 顶点代码片段
    vs: BlockCodeAtom,
    /// 像素代码片段
    fs: BlockCodeAtom,
    pub size: usize,
    pub defines: ShaderDefinesSet,
    pub binddefines: BindDefine,
}

impl From<(pi_render::rhi::shader::ShaderMeta, Vec<Atom>, Vec<Atom>)> for ShaderEffectMeta {
    fn from(
        value: (pi_render::rhi::shader::ShaderMeta, Vec<Atom>, Vec<Atom>),
    ) -> Self {
        let (value, vs_defines, fs_defines) = value;
        
        let mut uniforms: MaterialValueBindDesc = MaterialValueBindDesc::default();
        let mut textures: Vec<UniformTexture2DDesc> = vec![];
        // let mut samplers: Vec<Arc<UniformSamplerDesc>> = vec![];

        let len = value.bindings.buffer_uniform_expands.len();
        for index in 0..len {
            let bindinfo = value.bindings.buffer_uniform_expands.get(index);
            let layout = value.bindings.bind_group_entrys.get(index);

            if let (Some(layout), Some(bindinfo)) = (layout, bindinfo) {
                let len = layout.len();

                for j in 0..len {
                    let entry = layout.get(j);
                    let info = bindinfo.get(j);
                    if let (Some(entry), Some(info)) = (entry, info) {
                        match entry.ty {
                            wgpu::BindingType::Buffer { ty: _, has_dynamic_offset: _, min_binding_size: _ } => {
                                info.list.iter().for_each(|uniform| {
                                    if let Some(value) = &uniform.buffer_expand {
                                        match value.ty.ty {
                                            pi_render::rhi::shader::TypeKind::Float => {
                                                match value.ty.size {
                                                    pi_render::rhi::shader::TypeSize::Mat {columns: _, .. } => {
                                                        // if rows == 4 {
                                                        //     uniforms.mat4_list.push(UniformPropertyMat4(uniform.name.clone(), crate::vec_u8_to_f32_16(&value.default_value)));
                                                        // } else if rows == 2 {
                                                        //     uniforms.mat2_list.push(UniformPropertyMat2(uniform.name.clone(), crate::vec_u8_to_f32_4(&value.default_value)));
                                                        // }
                                                    },
                                                    pi_render::rhi::shader::TypeSize::Vec(v) => {
                                                        if v == 4 {
                                                            uniforms.vec4_list.push(UniformPropertyVec4(uniform.name.clone(), crate::vec_u8_to_f32_4(&value.default_value), false));
                                                        } else if v == 2 {
                                                            uniforms.vec2_list.push(UniformPropertyVec2(uniform.name.clone(), crate::vec_u8_to_f32_2(&value.default_value), false));
                                                        }
                                                    },
                                                    pi_render::rhi::shader::TypeSize::Scalar => {
                                                        uniforms.float_list.push(UniformPropertyFloat(uniform.name.clone(), crate::vec_u8_to_f32(&value.default_value), false));
                                                    },
                                                }
                                            },
                                            pi_render::rhi::shader::TypeKind::Sint => {
                                                // uniforms.int_list.push(UniformPropertyInt(uniform.name.clone(), crate::vec_u8_to_i32(&value.default_value)));
                                            },
                                            pi_render::rhi::shader::TypeKind::Uint => {
                                                uniforms.uint_list.push(UniformPropertyUint(uniform.name.clone(), crate::vec_u8_to_u32(&value.default_value), false));
                                            },
                                        }
                                    }
                                });
                            },
                            wgpu::BindingType::Sampler(_) => {
                                // let val = UniformSamplerDesc {
                                //     slotname: info.list.get(0).unwrap().name.clone(),
                                //     ty: val,
                                //     stage: entry.visibility,
                                // };
                                // samplers.push(val);
                            },
                            wgpu::BindingType::Texture { sample_type, view_dimension, multisampled } => {
                                match view_dimension {
                                    wgpu::TextureViewDimension::D1 => todo!(),
                                    wgpu::TextureViewDimension::D2 => {
                                        let val = UniformTexture2DDesc::new(
                                            info.list.get(0).unwrap().name.clone(),
                                            sample_type,
                                            wgpu::TextureViewDimension::D2,
                                            multisampled,
                                            EShaderStage::new(entry.visibility),
                                            EDefaultTexture::White,
                                        );
                                        textures.push(val);
                                    },
                                    wgpu::TextureViewDimension::D2Array => todo!(),
                                    wgpu::TextureViewDimension::Cube => todo!(),
                                    wgpu::TextureViewDimension::CubeArray => todo!(),
                                    wgpu::TextureViewDimension::D3 => todo!(),
                                }
                            },
                            wgpu::BindingType::StorageTexture { access: _, format: _, view_dimension: _ } => {
                                
                            },
                            wgpu::BindingType::AccelerationStructure => todo!(),
                        }
                    }
                }
            }
        }
        let defines = ShaderDefinesSet::from((&vs_defines, &fs_defines));
        let vs = value.vs.to_block_code();
        let fs = value.fs.to_block_code();
        let varyings = Varyings::from(&value.varyings);

        Self::new(uniforms, textures, varyings, String::from(""), vs, fs, defines)
    }
}
impl Asset for ShaderEffectMeta {
    type Key = KeyShaderMeta;
    // const TYPE: &'static str = "KeyShaderMeta";
}
impl TAssetCapacity for ShaderEffectMeta {
	const ASSET_TYPE: &'static str = "SHADER_EFFECT_META";
	fn capacity() -> AssetCapacity {
        AssetCapacity { flag: false, min: 4 * 1024 * 1024, max: 6 * 1024 * 1024, timeout: 60 * 60 * 1000 }
	}
}
impl Size for ShaderEffectMeta {
    fn size(&self) -> usize {
        self.size
    }
}
impl ShaderEffectMeta {
    pub fn new(
        mut uniforms: MaterialValueBindDesc,
        mut textures: Vec<UniformTexture2DDesc>,
        // samplers: Vec<UniformSamplerDesc>,
        mut varyings: Varyings,
        material_instance_code: String,
        vs: BlockCodeAtom,
        fs: BlockCodeAtom,
        defines: ShaderDefinesSet,
    ) -> Self {
        let mut arc_textures = vec![];
        textures.drain(..).for_each(|item| {
            arc_textures.push(Arc::new(item));
        });
        arc_textures.sort_by(|a, b| { a.slotname.cmp(&b.slotname) });
        // let len = arc_textures.len();
        // for idx in 0..len {
        //     uniforms.vec4_list.push(UniformPropertyVec4(Atom::from(String::from("uTexST") + &idx.to_string()), [1., 1., 0., 0.], false));
        // }

        uniforms.sort();

        // varyings.0.push(Varying { format: Atom::from(EBuildinVertexAtribute::TextureIDs.kind()), name: Atom::from(ShaderVarVarying::TEXTURE_IDS) });
        uniforms.vec4_list.iter().for_each(|item| { if item.instance() { varyings.0.push(Varying { format: Atom::from(crate::static_string::S_VEC4), name: item.tag().clone() }) } });
        uniforms.vec3_list.iter().for_each(|item| { if item.instance() { varyings.0.push(Varying { format: Atom::from(crate::static_string::S_VEC3), name: item.tag().clone() }) } });
        uniforms.vec2_list.iter().for_each(|item| { if item.instance() { varyings.0.push(Varying { format: Atom::from(crate::static_string::S_VEC2), name: item.tag().clone() }) } });
        uniforms.float_list.iter().for_each(|item| { if item.instance() { varyings.0.push(Varying { format: Atom::from(crate::static_string::S_FLOAT), name: item.tag().clone() }) } });
        uniforms.uint_list.iter().for_each(|item| { if item.instance() { varyings.0.push(Varying { format: Atom::from(crate::static_string::S_UINT), name: item.tag().clone() }) } });

        let size = varyings.size() + vs.size() + fs.size();

        Self {
            uniforms: Arc::new(uniforms),
            textures: Arc::new(EffectUniformTexture2DDescs::from(arc_textures)),
            // samplers,
            varyings,
            material_instance_code,
            vs,
            fs,
            size,
            defines,
            binddefines: 0,
        }
    }
    pub fn uniform_count(&self) -> usize {
        0
        // + self.uniforms.mat4_list.len()
        // + self.uniforms.mat2_list.len()
        + self.uniforms.vec4_list.len()
        + self.uniforms.vec3_list.len()
        + self.uniforms.vec2_list.len()
        + self.uniforms.float_list.len()
        // + self.uniforms.int_list.len()
        + self.uniforms.uint_list.len()
    }
    pub fn vs_blocks_2(
        &self,
        name: &str,
        // vertex_layouts: &KeyShaderFromAttributes,
        running_model_snippets: &[String],
        // instance: &EVerticeExtendCode,
        // render_alignment: &ERenderAlignment,
        // skin: &ESkinCode,
        defined_snippets: &[String],
        running_before_effect_snippets: &[String],
        running_after_effect_snippets: &[String],
    ) -> String {
        // Start
        let mut code = String::from("#version 450\r\n");

        // DEFINES
        // TODO

        // Shader Name
        code += "#define SHADER_NAME vertex:"; code += name; code += crate::prelude::S_BREAK;

        // Shader 定义 Varying 代码
        code += &VaryingCode::vs_code(&self.varyings);

        // 功能块的定义代码 - 功能块的 Uniform 、常量 、 方法
        defined_snippets.iter().for_each(|val| {
            code += val;
        });

        // Shader 自带定义块代码 - 常量 、 方法, 不包含 Uniform 定义
        code += self.vs.define.as_str();

        // Running Start
        code += "void main() {";    code += crate::prelude::S_BREAK;

        // 预制内容
        code += EVertexDataKind::Color4.kind();     code += crate::prelude::S_SPACE; code += ShaderVarVertices::COLOR4 ;    code += " = vec4(1., 1., 1., 1.);"; code += crate::prelude::S_BREAK;
        code += EVertexDataKind::Normal.kind();     code += crate::prelude::S_SPACE; code += ShaderVarVertices::NORMAL ;    code += " = vec3(0., 1., 0.);";     code += crate::prelude::S_BREAK;
        code += EVertexDataKind::UV.kind();         code += crate::prelude::S_SPACE; code += ShaderVarVertices::UV ;        code += " = vec2(0., 0.);";         code += crate::prelude::S_BREAK;

        code += self.material_instance_code.as_str();
        
        // 功能块的 运行代码
        running_model_snippets.iter().for_each(|val| {
            code += val;
        });

        // 功能块的 运行代码
        running_before_effect_snippets.iter().for_each(|val| {
            code += val;
        });

        // Shader 的运行代码
        code += self.vs.running.as_str();
        
        // 功能块的 运行代码
        running_after_effect_snippets.iter().for_each(|val| {
            code += val;
        });

        code += "}"; code += crate::prelude::S_BREAK;

        return code;
    }
    pub fn fs_blocks_2(
        &self,
        name: &str,
        defined_snippets: &[String],
        running_before_effect_snippets: &[String],
        running_after_effect_snippets: &[String],
    ) -> String {
        // Start
        let mut code = String::from("#version 450"); code += crate::prelude::S_BREAK;

        // DEFINES
        // TODO

        // Shader Name
        code += "#define SHADER_NAME fragment:"; code += name; code += crate::prelude::S_BREAK;

        // Shader 定义 Varying 代码
        code += &VaryingCode::fs_code(&self.varyings);

        // 功能块的定义代码 - 功能块的 Uniform 、常量 、 方法
        defined_snippets.iter().for_each(|val| {
            code += val;
        });

        // Shader 自带定义块代码 - 常量 、 方法, 不包含 Uniform 定义
        code += self.fs.define.as_str();

        // Running Start
        code += "void main() {"; code += crate::prelude::S_BREAK;

        // 功能块的 运行代码
        running_before_effect_snippets.iter().for_each(|val| {
            code += val;
        });

        // Shader 的运行代码
        code += self.fs.running.as_str();
        
        // 功能块的 运行代码
        running_after_effect_snippets.iter().for_each(|val| {
            code += val;
        });
        
        code += "}"; code += crate::prelude::S_BREAK;

        return code;
    }
    pub fn build_2(
        &self,
        device: &RenderDevice,
        key_meta: &KeyShaderMeta,
        // vertex_layouts: &KeyShaderFromAttributes,
        // instance: &EVerticeExtendCode,
        // render_alignment: &ERenderAlignment,
        // skin: &ESkinCode,
        vs_defined_snippets: &[String],
        vs_running_model_snippets: &[String],
        vs_running_before_effect_snippets: &[String],
        vs_running_after_effect_snippets: &[String],
        fs_defined_snippets: &[String],
        fs_running_before_effect_snippets: &[String],
        fs_running_after_effect_snippets: &[String],
    ) -> Shader3D {
        let vs = self.vs_blocks_2(key_meta.as_str(), vs_running_model_snippets, vs_defined_snippets, vs_running_before_effect_snippets, vs_running_after_effect_snippets);
        let fs = self.fs_blocks_2(key_meta.as_str(), fs_defined_snippets, fs_running_before_effect_snippets, fs_running_after_effect_snippets);

        #[cfg(not(target_arch = "wasm32"))]
        {
            // log::warn!("Shader: {:?}", key_meta);
            let temp = String::from("temp/");
            let root_dir = std::env::current_dir().unwrap();
            let mut file_name: String = key_meta.to_string() + ".vert";
            file_name = temp.clone() + file_name.as_str();
            let _ = std::fs::write(root_dir.join(file_name), vs.as_str());
            
            let mut file_name = key_meta.to_string() + ".frag";
            file_name = temp + file_name.as_str();
            let _ = std::fs::write(root_dir.join(file_name), fs.as_str());
        }
        // log::error!("Shader: {:?}", key_meta);
        // log::error!("VS: {:?}", vs.as_str());
        // log::error!("FS: {:?}", fs.as_str());

        let vs = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some((key_meta.to_string() + "-VS").as_str()),
            source: ShaderSource::Glsl {
                shader: std::borrow::Cow::Borrowed(vs.as_str()),
                stage: naga::ShaderStage::Vertex,
                defines: naga::FastHashMap::default(),
            },
        });

        let fs = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some((key_meta.to_string() + "-FS").as_str()),
            source: ShaderSource::Glsl {
                shader: std::borrow::Cow::Borrowed(fs.as_str()),
                stage: naga::ShaderStage::Fragment,
                defines: naga::FastHashMap::default(),
            },
        });

        Shader3D { vs, vs_point: "main", fs, fs_point: "main" }
    }
    
    pub fn define_code(
        list: &Vec<BlockCode>,
    ) -> String {
        let mut result = String::from("");
        list.iter().for_each(|item| {
            result += item.define.as_str();
        });

        result
    }
    pub fn running_code(
        list: &Vec<BlockCode>,
    ) -> String {
        let mut result = String::from("");
        list.iter().for_each(|item| {
            result += item.running.as_str();
        });

        result
    }

    pub fn lighting_about(mut result: String) -> String {
        result += crate::prelude::S_VEC3;  result += crate::prelude::S_SPACE; result += ShaderVarSurface::POSITION;   result += " = "; result += ShaderVarVarying::POSITION;  result += ";";    result += crate::prelude::S_BREAK;
        result += crate::prelude::S_VEC3;  result += crate::prelude::S_SPACE; result += ShaderVarSurface::NORMAL;     result += " = "; result += ShaderVarVarying::NORMAL;  result += ";";    result += crate::prelude::S_BREAK;
        result += crate::prelude::S_VEC3;  result += crate::prelude::S_SPACE; result += ShaderVarSurface::VIEW;       result += " = WorldSpaceViewDir("; result += ShaderVarSurface::POSITION;  result += ");"; result += crate::prelude::S_BREAK;
        result += crate::prelude::S_FLOAT; result += crate::prelude::S_SPACE; result += ShaderVarSurface::N_DOT_V;    result += " = dot("; result += ShaderVarSurface::NORMAL; result += ", "; result += ShaderVarSurface::VIEW; result += ");"; result += crate::prelude::S_BREAK;
        result += crate::prelude::S_FLOAT; result += crate::prelude::S_SPACE; result += ShaderVarSurface::GLOSSINESS; result += " = 1.0;";    result += crate::prelude::S_BREAK;
        result += crate::prelude::S_VEC3;  result += crate::prelude::S_SPACE; result += ShaderVarSurface::LIGHTMAP;   result += " = vec3(1., 1., 1.);"; result += crate::prelude::S_BREAK;
        result
    }
}

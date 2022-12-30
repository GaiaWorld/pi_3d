use pi_ecs::prelude::Query;
use pi_engine_shell::object::GameObject;
use render_shader::{skin_code::ESkinCode, unifrom_code::{MaterialTextureBindDesc, UniformPropertyName, UniformTextureDesc}};

use super::shader_effect::{ResShaderEffectMeta, ShaderEffectMeta, UniformPropertyFloat, AssetResShaderEffectMeta};

pub trait TWithUniformDesc {
    fn tex_uniform(&self) -> UniformTextureDesc;
    fn val_uniform_float(&self) -> Vec<UniformPropertyFloat>;
}

impl TWithUniformDesc for ESkinCode {
    fn tex_uniform(&self) -> UniformTextureDesc {
        UniformTextureDesc {
            slotname: UniformPropertyName::from(Self::SKIN_TEX_UNIFORM_NAME),
            sampler_binding_type: wgpu::SamplerBindingType::NonFiltering,
            tex_sampler_type: wgpu::TextureSampleType::Float { filterable: false },
            dimension: wgpu::TextureViewDimension::D2,
            multisampled: false,
            stage: wgpu::ShaderStages::VERTEX,
        }
    }

    fn val_uniform_float(&self) -> Vec<UniformPropertyFloat> {
        vec![
            UniformPropertyFloat(UniformPropertyName::from(Self::SKIN_TEX_WIDTH_UNIFORM_NAME), 4),
            UniformPropertyFloat(UniformPropertyName::from(Self::SKIN_TEX_HEIGHT_UNIFORM_NAME), 1),
        ]
    }
}

pub trait TTextureBindDescModify {
    fn append_tex(&mut self, desc: UniformTextureDesc);
    fn remove_tex(&mut self, name: UniformPropertyName);
    fn append_float(&mut self, desc: UniformPropertyFloat);
    fn remove_float(&mut self, name: UniformPropertyName);
}
impl TTextureBindDescModify for ShaderEffectMeta {
    fn append_tex(&mut self, desc: UniformTextureDesc) {
        if let Some(textures) = &mut self.textures {
            let len = textures.list.len();
            let mut flag = true;
            for i in 0..len {
                if textures.list[i].slotname == desc.slotname {
                    flag = false;
                    break;
                }
            }

            if flag {
                textures.list.push(desc);
            }
        } else {
            self.textures = Some(MaterialTextureBindDesc {
                set: 1,
                list: vec![desc],
            });
        }
    }

    fn remove_tex(&mut self, name: UniformPropertyName) {
        if let Some(textures) = &mut self.textures {
            let len = textures.list.len();
            let mut index = usize::MAX;
            for i in 0..len {
                if textures.list[i].slotname == name {
                    index = i;
                    break;
                }
            }

            if index < len {
                for i in index..(len - 1) {
                    textures.list[i] = textures.list[i + 1].clone();
                }
                textures.list.pop();
            }
        }
    }

    fn append_float(&mut self, desc: UniformPropertyFloat) {
        let list = &mut self.uniforms.float_list;
        let len = list.len();
        let mut flag = true;
        for i in 0..len {
            if list[i].0 == desc.0 {
                flag = false;
                break;
            }
        }

        if flag {
            list.push(desc);
        }
    }

    fn remove_float(&mut self, name: UniformPropertyName) {
        let list = &mut self.uniforms.float_list;
        let len = list.len();
        let mut index = usize::MAX;
        for i in 0..len {
            if list[i].0 == name {
                index = i;
                break;
            }
        }

        if index < len {
            for i in index..(len - 1) {
                list[i] = list[i + 1].clone();
            }
            list.pop();
        }
    }
}

pub struct MaterialMeta(pub ShaderEffectMeta);
impl MaterialMeta {
    pub fn from_effect(effect: &ShaderEffectMeta) -> Self {
        Self(effect.clone())
    }
    pub fn update(&mut self, skin: ESkinCode) {
        let mut tex_uniform = skin.tex_uniform();
        let mut val_uniforms = skin.val_uniform_float();
        match skin {
            ESkinCode::None => {
                self.0.remove_tex(tex_uniform.slotname);
                self.0.remove_float(val_uniforms.pop().unwrap().0);
                self.0.remove_float(val_uniforms.pop().unwrap().0);
            },
            ESkinCode::RowTexture => {
                self.0.append_tex(tex_uniform);
                self.0.append_float(val_uniforms.pop().unwrap());
                self.0.append_float(val_uniforms.pop().unwrap());
            },
            ESkinCode::FramesTextureInstance => {
                self.0.append_tex(tex_uniform);
                self.0.append_float(val_uniforms.pop().unwrap());
                self.0.append_float(val_uniforms.pop().unwrap());
            },
        }
    }
}


pub struct SysMaterialMetaUpdate;
impl SysMaterialMetaUpdate {
    pub fn sys(
        mut items: Query<
            GameObject,
            (&AssetResShaderEffectMeta, )
        >
    ) {

    }
}
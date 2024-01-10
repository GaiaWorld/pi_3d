use pi_scene_shell::prelude::*;
use pi_hash::{XHashMap, XHashSet};

pub type TShaderLanguageDefine = u8;
pub struct ShaderLanguageDefine;
impl ShaderLanguageDefine {
    pub const FLOATS_TO_VEC: TShaderLanguageDefine = 0b0000_0001;
    pub fn apply(val: TShaderLanguageDefine) -> String {
        let mut result = String::from("");
        if val & Self::FLOATS_TO_VEC == Self::FLOATS_TO_VEC {
            result += "
#define float4 vec4
#define float3 vec3
#define float2 vec2
#define float4x4 mat4
";
        }

        result
    }
}

pub trait TNodeMaterialBlock {
    const KEY: &'static str;
    const FS_DEFINED: &'static str = "";
    const VS_DEFINED: &'static str = "";
    const BIND_DEFINES: BindDefine = 0;
    const SHADER_LANGUAGE_DEFINES: TShaderLanguageDefine = 0;
    const VS_SURFACE: &'static str = "";
    const FS_SURFACE: &'static str = "";
    fn mat4() -> Vec<UniformPropertyMat4> { vec![] }
    // fn mat2() -> Vec<UniformPropertyMat2> { vec![] }
    fn vec4() -> Vec<UniformPropertyVec4> { vec![] }
    fn vec3() -> Vec<UniformPropertyVec3> { vec![] }
    fn vec2() -> Vec<UniformPropertyVec2> { vec![] }
    fn float() -> Vec<UniformPropertyFloat> { vec![] }
    // fn int() -> Vec<UniformPropertyInt> { vec![] }
    fn uint() -> Vec<UniformPropertyUint> { vec![] }
    fn textures() -> Vec<UniformTexture2DDesc> { vec![] }
    fn varyings() -> Vec<Varying> { vec![] }
    fn depends() -> Vec<Atom> { vec![] }
    fn info() -> NodeMaterialBlockInfo {
        NodeMaterialBlockInfo {
            fs_define: String::from(Self::FS_DEFINED),
            vs_define: String::from(Self::VS_DEFINED),
            fs_surface: String::from(Self::FS_SURFACE),
            vs_surface: String::from(Self::VS_SURFACE),
            mat4: Self::mat4(),
            // mat2: Self::mat2(),
            vec4: Self::vec4(),
            vec3: Self::vec3(),
            vec2: Self::vec2(),
            float: Self::float(),
            // int: Self::int(),
            uint: Self::uint(),
            textures: Self::textures(),
            varyings: Self::varyings(),
            depends: Self::depends(),
            binddefines: Self::BIND_DEFINES,
            shader_language_defines: Self::SHADER_LANGUAGE_DEFINES,
        }
    }
}

/// 节点材质块
#[derive(Default, Clone)]
pub struct NodeMaterialBlockInfo {
    /// 像素着色器 声明代码
    pub fs_define: String,
    /// 顶点着色器 声明代码
    pub vs_define: String,
    /// 像素着色器 执行代码
    pub fs_surface: String,
    /// 顶点着色器 执行代码
    pub vs_surface: String,
    /// 声明的 Matrix4 uniform
    pub mat4: Vec<UniformPropertyMat4>,
    // pub mat2: Vec<UniformPropertyMat2>,
    /// 声明的 vec4 uniform
    pub vec4: Vec<UniformPropertyVec4>,
    /// 声明的 vec3 uniform
    pub vec3: Vec<UniformPropertyVec3>,
    /// 声明的 vec2 uniform
    pub vec2: Vec<UniformPropertyVec2>,
    /// 声明的 f32 uniform
    pub float: Vec<UniformPropertyFloat>,
    // pub int: Vec<UniformPropertyInt>,
    /// 声明的 u32 uniform
    pub uint: Vec<UniformPropertyUint>,
    /// 声明的 纹理 uniform
    pub textures: Vec<UniformTexture2DDesc>,
    /// 声明的 varying
    pub varyings: Vec<Varying>,
    /// 依赖的 其他块的key
    pub depends: Vec<Atom>,
    /// 依赖的 功能性bind的key组合
    pub binddefines: BindDefine,
    /// 依赖的 语言转换块 - 一般不使用
    pub shader_language_defines: TShaderLanguageDefine,
}

pub struct NodeMaterialBuilder {
    pub blocks: XHashSet<Atom>,
    pub values: MaterialValueBindDesc,
    pub textures: Vec<UniformTexture2DDesc>,
    pub varyings: Varyings,
    pub material_instance_code: String,
    // pub check_instance: EVerticeExtendCode,
    pub fs_define: String,
    pub vs_define: String,
    pub fs: String,
    pub vs: String,
    pub defines: ShaderDefinesSet,
    pub binddefines: BindDefine,
    pub shader_language_defines: TShaderLanguageDefine,
}
impl NodeMaterialBuilder {
    pub fn new() -> Self {
        Self {
            blocks: XHashSet::default(),
            values: MaterialValueBindDesc::default(),
            textures: Vec::default(),
            varyings: Varyings(vec![]),
            material_instance_code: String::from(""),
            // check_instance: EVerticeExtendCode::default(),
            fs_define: String::from(""),
            vs_define: String::from(""),
            fs:  String::from(""),
            vs:  String::from(""),
            defines: ShaderDefinesSet::default(),
            binddefines: BindDefines::VIEWER | BindDefines::MODEL_BIND | BindDefines::EFFECT_VALUE_BIND,
            shader_language_defines: 0,
        }
    }
    pub fn include(&mut self, key: &Atom, infos: &XHashMap<Atom, NodeMaterialBlockInfo>) {
        let mut keys: Vec<Atom> = vec![key.clone()];
        let mut tempkeys = vec![key.clone()];
        let mut layer = 0;
        let mut single = vec![];

        loop {
            if tempkeys.len() == 0 {
                break;
            }

            if layer > 128 {
                log::error!("NodeMaterialBlockInfo Error");
                return;
            }

            layer += 1;

            let mut temp = vec![];
            tempkeys.drain(..).for_each(|key| {
                if let Some(info) = infos.get(&key) {
                    // log::warn!("{:?}", key);
                    info.depends.iter().for_each(|v| {
                        // log::warn!(" Depend {:?}", v);
                        temp.push(v.clone());
                        keys.push(v.clone());
                    });
                    if info.depends.len() == 0 {
                        single.push(key.clone());
                    }
                } else {
                    log::error!("Node MaterialBlock Not Found: {:?}", key);
                }
            });
            tempkeys = temp;
        }

        single.drain(..).for_each(|v| {
            keys.push(v);
        });

        let len = keys.len();
        for i in 0..len {
            let key: &Atom = keys.get(len - i - 1).unwrap();
            if !self.blocks.contains(&key) {
                if let Some(info) = infos.get(key) {
    
                    self.blocks.insert(key.clone());
    
                    info.mat4.iter().for_each(|v| {
                        self.values.mat4_list.push(v.clone());
                    });
                    // info.mat2.iter().for_each(|v| {
                    //     self.values.mat2_list.push(v.clone());
                    // });
                    info.vec4.iter().for_each(|v| {
                        self.values.vec4_list.push(v.clone());
                    });
                    info.vec3.iter().for_each(|v| {
                        self.values.vec3_list.push(v.clone());
                    });
                    info.vec2.iter().for_each(|v| {
                        self.values.vec2_list.push(v.clone());
                    });
                    info.float.iter().for_each(|v| {
                        self.values.float_list.push(v.clone());
                    });
                    // info.int.iter().for_each(|v| {
                    //     self.values.int_list.push(v.clone());
                    // });
                    info.uint.iter().for_each(|v| {
                        self.values.uint_list.push(v.clone());
                    });
                    
                    info.textures.iter().for_each(|v| {
                        self.textures.push(v.clone());
                    });
        
                    info.varyings.iter().for_each(|v| {
                        self.varyings.0.push(v.clone());
                    });
        
                    self.fs_define      += info.fs_define.as_str();
                    self.vs_define      += info.vs_define.as_str();
                    self.binddefines    = self.binddefines | info.binddefines;
                    self.shader_language_defines    = self.shader_language_defines | info.shader_language_defines;
                }
            }
        }
    }
    pub fn apply<T: TNodeMaterialBlock>(&mut self) {
        let key = Atom::from(T::KEY);
        if !self.blocks.contains(&key) {
            self.blocks.insert(key);

            // T::mat4().drain(..).for_each(|v| {
            //     self.values.mat4_list.push(v);
            // });
            // T::mat2().drain(..).for_each(|v| {
            //     self.values.mat2_list.push(v);
            // });
            T::vec4().drain(..).for_each(|v| {
                self.values.vec4_list.push(v);
            });
            T::vec3().drain(..).for_each(|v| {
                self.values.vec3_list.push(v);
            });
            T::vec2().drain(..).for_each(|v| {
                self.values.vec2_list.push(v);
            });
            T::float().drain(..).for_each(|v| {
                self.values.float_list.push(v);
            });
            // T::int().drain(..).for_each(|v| {
            //     self.values.int_list.push(v);
            // });
            T::uint().drain(..).for_each(|v| {
                self.values.uint_list.push(v);
            });
            
            T::textures().drain(..).for_each(|v| {
                self.textures.push(v);
            });

            T::varyings().drain(..).for_each(|v| {
                self.varyings.0.push(v);
            });

            self.fs_define      += T::FS_DEFINED;
            self.vs_define      += T::VS_DEFINED;
            self.binddefines    = self.binddefines | T::BIND_DEFINES;
            self.shader_language_defines    = self.shader_language_defines | T::SHADER_LANGUAGE_DEFINES;
        }
    }
    pub fn meta(mut self) -> ShaderEffectMeta {
        // log::warn!("{:?}", self.shader_language_defines);

        let predefines = ShaderLanguageDefine::apply(self.shader_language_defines);
        self.vs_define = predefines.clone() + self.vs_define.as_str();
        self.fs_define = predefines.clone() + self.fs_define.as_str();

        // log::warn!("{:?}", predefines);

        let mut result = ShaderEffectMeta::new(
            self.values, 
            self.textures, 
            self.varyings, 
            self.material_instance_code, 
            // self.check_instance, 
            BlockCodeAtom { define: Atom::from(self.vs_define), running: Atom::from(self.vs) }, 
            BlockCodeAtom { define: Atom::from(self.fs_define), running: Atom::from(self.fs) }, 
            self.defines,
        );

        result.binddefines = result.binddefines | self.binddefines;

        result
    }
}
use pi_engine_shell::prelude::*;
use pi_hash::{XHashMap, XHashSet};

pub trait TNodeMaterialBlock {
    const KEY: &'static str;
    const FS_DEFINED: &'static str;
    const VS_DEFINED: &'static str;
    fn mat4() -> Vec<UniformPropertyMat4> { vec![] }
    // fn mat2() -> Vec<UniformPropertyMat2> { vec![] }
    fn vec4() -> Vec<UniformPropertyVec4> { vec![] }
    fn vec2() -> Vec<UniformPropertyVec2> { vec![] }
    fn float() -> Vec<UniformPropertyFloat> { vec![] }
    // fn int() -> Vec<UniformPropertyInt> { vec![] }
    fn uint() -> Vec<UniformPropertyUint> { vec![] }
    fn textures() -> Vec<UniformTexture2DDesc> { vec![] }
    fn varyings() -> Vec<Varying> { vec![] }
    fn depends() -> Vec<Atom> { vec![] }
    fn info() -> NodeMaterialBlockInfo {
        NodeMaterialBlockInfo {
            fs: String::from(Self::FS_DEFINED),
            vs: String::from(Self::VS_DEFINED),
            mat4: Self::mat4(),
            // mat2: Self::mat2(),
            vec4: Self::vec4(),
            vec2: Self::vec2(),
            float: Self::float(),
            // int: Self::int(),
            uint: Self::uint(),
            textures: Self::textures(),
            varyings: Self::varyings(),
            depends: Self::depends()
        }
    }
}

#[derive(Clone)]
pub struct NodeMaterialBlockInfo {
    pub fs: String,
    pub vs: String,
    pub mat4: Vec<UniformPropertyMat4>,
    // pub mat2: Vec<UniformPropertyMat2>,
    pub vec4: Vec<UniformPropertyVec4>,
    pub vec2: Vec<UniformPropertyVec2>,
    pub float: Vec<UniformPropertyFloat>,
    // pub int: Vec<UniformPropertyInt>,
    pub uint: Vec<UniformPropertyUint>,
    pub textures: Vec<UniformTexture2DDesc>,
    pub varyings: Vec<Varying>,
    pub depends: Vec<Atom>,
}

pub struct NodeMaterialBuilder {
    pub blocks: XHashSet<Atom>,
    pub values: MaterialValueBindDesc,
    pub textures: Vec<UniformTexture2DDesc>,
    pub varyings: Varyings,
    pub fs_define: String,
    pub vs_define: String,
    pub fs: String,
    pub vs: String,
    pub defines: ShaderDefinesSet,
}
impl NodeMaterialBuilder {
    pub fn new() -> Self {
        Self {
            blocks: XHashSet::default(),
            values: MaterialValueBindDesc::default(),
            textures: Vec::default(),
            varyings: Varyings(vec![]),
            fs_define: String::from(""),
            vs_define: String::from(""),
            fs:  String::from(""),
            vs:  String::from(""),
            defines: ShaderDefinesSet::default(),
        }
    }
    pub fn include(&mut self, key: &Atom, infos: &XHashMap<Atom, NodeMaterialBlockInfo>) {
        let mut keys: Vec<Atom> = vec![key.clone()];
        let mut tempkeys = vec![key.clone()];

        loop {
            if tempkeys.len() == 0 {
                break;
            }

            let mut temp = vec![];
            tempkeys.drain(..).for_each(|key| {
                if let Some(info) = infos.get(&key) {
                    info.depends.iter().for_each(|v| {
                        temp.push(v.clone());
                        keys.push(v.clone());
                    });
                }
            });
            tempkeys = temp;
        }

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
        
                    self.fs_define += info.fs.as_str();
                    self.vs_define += info.vs.as_str();
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

            self.fs_define += T::FS_DEFINED;
            self.vs_define += T::VS_DEFINED;

        }
    }
    pub fn meta(self) -> ShaderEffectMeta {
        ShaderEffectMeta::new(
            self.values, 
            self.textures, 
            self.varyings, 
            BlockCodeAtom { define: Atom::from(self.vs_define), running: Atom::from(self.vs) }, 
            BlockCodeAtom { define: Atom::from(self.fs_define), running: Atom::from(self.fs) }, 
            self.defines
        )
    }
}
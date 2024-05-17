use std::hash::Hash;

use pi_atom::Atom;

use super::{TUnifromShaderProperty, UniformPropertyName, TBindDescToShaderCode};


// pub enum UniformValueKind {
//     Mat4,
//     Mat2,
//     Vec4,
//     Vec2,
//     Float,
//     Int,
//     Uint,
//     TextureD1,
//     TextureD2,
//     TextureD3,
// }

// impl UniformValueKind {
//     pub fn code(&self) -> String {
//         match self {
//             UniformValueKind::Mat4              => String::from(crate::prelude::S_MAT4),
//             UniformValueKind::Mat2              => String::from("mat2"),
//             UniformValueKind::Vec4              => String::from(crate::prelude::S_VEC4),
//             UniformValueKind::Vec2              => String::from(crate::prelude::S_VEC2),
//             UniformValueKind::Float             => String::from(crate::prelude::S_FLOAT),
//             UniformValueKind::Int               => String::from(crate::prelude::S_INT),
//             UniformValueKind::Uint              => String::from(crate::prelude::S_UINT),
//             UniformValueKind::TextureD1         => String::from(crate::prelude::S_TEXTURE2D),
//             UniformValueKind::TextureD2         => String::from(crate::prelude::S_TEXTURE2D),
//             UniformValueKind::TextureD3         => String::from("textureCube"),
//         }
//     }
// }

#[derive(Clone)]
pub struct UniformPropertyMat4(pub UniformPropertyName, pub [f32;16], pub bool);
impl TUnifromShaderProperty for UniformPropertyMat4 {
    fn tag(&self) -> &UniformPropertyName {
        &self.0
    }
}
impl Hash for UniformPropertyMat4 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.tag().hash(state);
    }
}
impl PartialEq for UniformPropertyMat4 {
    fn eq(&self, other: &Self) -> bool {
        self.tag().eq(other.tag())
    }
}
impl Eq for UniformPropertyMat4 {
    fn assert_receiver_is_total_eq(&self) {}
}
impl PartialOrd for UniformPropertyMat4 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.tag().partial_cmp(other.tag())
    }
}
impl Ord for UniformPropertyMat4 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

// #[derive(Clone, Debug)]
// pub struct UniformPropertyMat2(pub UniformPropertyName, pub [f32;4], pub bool);
// impl TUnifromShaderProperty for UniformPropertyMat2 {
//     fn tag(&self) -> &UniformPropertyName {
//         &self.0
//     }
// }
// impl Hash for UniformPropertyMat2 {
//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//         self.tag().hash(state);
//     }
// }
// impl PartialEq for UniformPropertyMat2 {
//     fn eq(&self, other: &Self) -> bool {
//         self.tag().eq(other.tag())
//     }
// }
// impl Eq for UniformPropertyMat2 {
//     fn assert_receiver_is_total_eq(&self) {}
// }
// impl PartialOrd for UniformPropertyMat2 {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         self.tag().partial_cmp(other.tag())
//     }
// }
// impl Ord for UniformPropertyMat2 {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         self.partial_cmp(other).unwrap()
//     }
// }

#[derive(Clone)]
pub struct UniformPropertyVec4(pub UniformPropertyName, pub [f32;4], pub bool);
impl UniformPropertyVec4 {
    pub fn instance(&self) -> bool { self.2 }
}
impl TUnifromShaderProperty for UniformPropertyVec4 {
    fn tag(&self) -> &UniformPropertyName {
        &self.0
    }
}
impl Hash for UniformPropertyVec4 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.tag().hash(state);
    }
}
impl PartialEq for UniformPropertyVec4 {
    fn eq(&self, other: &Self) -> bool {
        self.tag().eq(other.tag())
    }
}
impl Eq for UniformPropertyVec4 {
    fn assert_receiver_is_total_eq(&self) {}
}
impl PartialOrd for UniformPropertyVec4 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.tag().partial_cmp(other.tag())
    }
}
impl Ord for UniformPropertyVec4 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Clone)]
pub struct UniformPropertyVec3(pub UniformPropertyName, pub [f32;3], pub bool);
impl UniformPropertyVec3 {
    pub fn instance(&self) -> bool { self.2 }
}
impl TUnifromShaderProperty for UniformPropertyVec3 {
    fn tag(&self) -> &UniformPropertyName {
        &self.0
    }
}
impl Hash for UniformPropertyVec3 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.tag().hash(state);
    }
}
impl PartialEq for UniformPropertyVec3 {
    fn eq(&self, other: &Self) -> bool {
        self.tag().eq(other.tag())
    }
}
impl Eq for UniformPropertyVec3 {
    fn assert_receiver_is_total_eq(&self) {}
}
impl PartialOrd for UniformPropertyVec3 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.tag().partial_cmp(other.tag())
    }
}
impl Ord for UniformPropertyVec3 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Clone)]
pub struct UniformPropertyVec2(pub UniformPropertyName, pub [f32;2], pub bool);
impl UniformPropertyVec2 {
    pub fn instance(&self) -> bool { self.2 }
}
impl TUnifromShaderProperty for UniformPropertyVec2 {
    fn tag(&self) -> &UniformPropertyName {
        &self.0
    }
}
impl Hash for UniformPropertyVec2 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.tag().hash(state);
    }
}
impl PartialEq for UniformPropertyVec2 {
    fn eq(&self, other: &Self) -> bool {
        self.tag().eq(other.tag())
    }
}
impl Eq for UniformPropertyVec2 {
    fn assert_receiver_is_total_eq(&self) {}
}
impl PartialOrd for UniformPropertyVec2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.tag().partial_cmp(other.tag())
    }
}
impl Ord for UniformPropertyVec2 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Clone)]
pub struct UniformPropertyFloat(pub UniformPropertyName, pub f32, pub bool);
impl UniformPropertyFloat {
    pub fn instance(&self) -> bool { self.2 }
}
impl TUnifromShaderProperty for UniformPropertyFloat {
    fn tag(&self) -> &UniformPropertyName {
        &self.0
    }
}
impl Hash for UniformPropertyFloat {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.tag().hash(state);
    }
}
impl PartialEq for UniformPropertyFloat {
    fn eq(&self, other: &Self) -> bool {
        self.tag().eq(other.tag())
    }
}
impl Eq for UniformPropertyFloat {
    fn assert_receiver_is_total_eq(&self) {}
}
impl PartialOrd for UniformPropertyFloat {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.tag().partial_cmp(other.tag())
    }
}
impl Ord for UniformPropertyFloat {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Clone)]
pub struct UniformPropertyInt(pub UniformPropertyName, pub i32, pub bool);
impl UniformPropertyInt {
    pub fn instance(&self) -> bool { self.2 }
}
impl TUnifromShaderProperty for UniformPropertyInt {
    fn tag(&self) -> &UniformPropertyName {
        &self.0
    }
}
impl Hash for UniformPropertyInt {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.tag().hash(state);
    }
}
impl PartialEq for UniformPropertyInt {
    fn eq(&self, other: &Self) -> bool {
        self.tag().eq(other.tag())
    }
}
impl Eq for UniformPropertyInt {
    fn assert_receiver_is_total_eq(&self) {}
}
impl PartialOrd for UniformPropertyInt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.tag().partial_cmp(other.tag())
    }
}
impl Ord for UniformPropertyInt {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Clone)]
pub struct UniformPropertyUint(pub UniformPropertyName, pub u32, pub bool);
impl UniformPropertyUint {
    pub fn instance(&self) -> bool { self.2 }
}
impl TUnifromShaderProperty for UniformPropertyUint {
    fn tag(&self) -> &UniformPropertyName {
        &self.0
    }
}
impl Hash for UniformPropertyUint {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.tag().hash(state);
    }
}
impl PartialEq for UniformPropertyUint {
    fn eq(&self, other: &Self) -> bool {
        self.tag().eq(other.tag())
    }
}
impl Eq for UniformPropertyUint {
    fn assert_receiver_is_total_eq(&self) {}
}
impl PartialOrd for UniformPropertyUint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.tag().partial_cmp(other.tag())
    }
}
impl Ord for UniformPropertyUint {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct MaterialValueBindDesc {
    pub stage: wgpu::ShaderStages,
    pub mat4_list: Vec<UniformPropertyMat4>,
    // pub mat2_list: Vec<UniformPropertyMat2>,
    pub vec4_list: Vec<UniformPropertyVec4>,
    pub vec3_list: Vec<UniformPropertyVec3>,
    pub vec2_list: Vec<UniformPropertyVec2>,
    pub float_list: Vec<UniformPropertyFloat>,
    // pub int_list: Vec<UniformPropertyInt>,
    pub uint_list: Vec<UniformPropertyUint>,
}
impl Default for MaterialValueBindDesc {
    fn default() -> Self {
        Self {
            stage: wgpu::ShaderStages::VERTEX_FRAGMENT, 
            mat4_list: vec![],
            // mat2_list: vec![],
            vec4_list: vec![], vec3_list: vec![], vec2_list: vec![], float_list: vec![],
            // int_list: vec![],
            uint_list: vec![]
        }
    }
}
impl MaterialValueBindDesc {
    pub const PRE_KEY_FOR_INSTANCE_UNIFORM: &str = "_I";
    pub fn none(stage: wgpu::ShaderStages) -> Self {
        Self { stage, 
            mat4_list: vec![],
            // mat2_list: vec![],
            vec4_list: vec![], vec3_list: vec![], vec2_list: vec![], float_list: vec![],
            // int_list: vec![],
            uint_list: vec![]
        }
    }
    pub fn sort(&mut self) {
        self.mat4_list.sort_by(|a, b| { a.0.cmp(&b.0) });
        // self.mat2_list.sort_by(|a, b| { a.0.cmp(&b.0) });
        self.vec4_list.sort_by(|a, b| { a.0.cmp(&b.0) });
        self.vec3_list.sort_by(|a, b| { a.0.cmp(&b.0) });
        self.vec2_list.sort_by(|a, b| { a.0.cmp(&b.0) });
        self.float_list.sort_by(|a, b| { a.0.cmp(&b.0) });
        // self.int_list.sort_by(|a, b| { a.0.cmp(&b.0) });
        self.uint_list.sort_by(|a, b| { a.0.cmp(&b.0) });
    }
    pub fn size(&self) -> usize {
        let mut size = 0;
        self.mat4_list.iter().for_each(|item| {
            size += item.0.as_bytes().len();
        });
        
        // self.mat2_list.iter().for_each(|item| {
        //     size += item.0.as_bytes().len();
        // });
        
        self.vec4_list.iter().for_each(|item| {
            size += item.0.as_bytes().len();
        });

        self.vec3_list.iter().for_each(|item| {
            size += item.0.as_bytes().len();
        });

        self.vec2_list.iter().for_each(|item| {
            size += item.0.as_bytes().len();
        });
        
        self.float_list.iter().for_each(|item| {
            size += item.0.as_bytes().len();
        });
        
        // self.int_list.iter().for_each(|item| {
        //     size += item.0.as_bytes().len();
        // });
        
        self.uint_list.iter().for_each(|item| {
            size += item.0.as_bytes().len();
        });

        size
    }
    pub fn query_instance(&self, key: &Atom) -> bool {
        match self.vec4_list.binary_search_by(|v| { v.0.cmp(key) }) {
            Ok(idx) => return self.vec4_list[idx].2,
            Err(_) => {},
        }
        match self.vec3_list.binary_search_by(|v| { v.0.cmp(key) }) {
            Ok(idx) => return self.vec3_list[idx].2,
            Err(_) => {},
        }
        match self.vec2_list.binary_search_by(|v| { v.0.cmp(key) }) {
            Ok(idx) => return self.vec2_list[idx].2,
            Err(_) => {},
        }
        match self.float_list.binary_search_by(|v| { v.0.cmp(key) }) {
            Ok(idx) => return self.float_list[idx].2,
            Err(_) => {},
        }
        match self.uint_list.binary_search_by(|v| { v.0.cmp(key) }) {
            Ok(idx) => return self.uint_list[idx].2,
            Err(_) => {},
        }
        return  false;
    }
    pub fn label(&self) -> String {
        let mut result = String::from("");

        self.mat4_list.iter().for_each(|name| {
            result += "#";
            result += name.0.as_str();
        });
        
        // self.mat2_list.iter().for_each(|name| {
        //     result += "#";
        //     result += name.0.as_str();
        // });

        self.vec4_list.iter().for_each(|name| {
            result += "#";
            result += name.0.as_str();
        });

        self.vec3_list.iter().for_each(|name| {
            result += "#";
            result += name.0.as_str();
        });

        self.vec2_list.iter().for_each(|name| {
            result += "#";
            result += name.0.as_str();
        });

        self.float_list.iter().for_each(|name| {
            result += "#";
            result += name.0.as_str();
        });

        self.uint_list.iter().for_each(|name| {
            result += "#";
            result += name.0.as_str();
        });

        result
    }
    fn _code(&self, set: u32, index: u32) -> String {
        let mut result = String::from("");

        if self.size() == 0 {

        } else {
            let mut total_num = 0;
    
            result += "layout(set = ";
            result += set.to_string().as_str();
            result += ", binding = ";
            result += index.to_string().as_str();
            result += ") uniform MatParam {"; result += crate::prelude::S_BREAK;
    
            self.mat4_list.iter().for_each(|name| {
                result += crate::prelude::S_MAT4; result += crate::prelude::S_SPACE;
                result += &name.0;
                result += ";"; result += crate::prelude::S_BREAK;
            });
            total_num += self.mat4_list.len();
            
            // self.mat2_list.iter().for_each(|name| {
            //     result += "mat2 ";
            //     result += &name.0;
            //     result += ";\r\n";
            // });
            // total_num += self.mat2_list.len();
            
            self.vec4_list.iter().for_each(|name| {
                result += crate::prelude::S_VEC4; result += crate::prelude::S_SPACE;
                result += &name.0;
                if name.2 { result += Self::PRE_KEY_FOR_INSTANCE_UNIFORM; }
                result += ";"; result += crate::prelude::S_BREAK;
            });
            total_num += self.vec4_list.len();

            self.vec3_list.iter().for_each(|name| {
                result += crate::prelude::S_VEC4; result += crate::prelude::S_SPACE;
                result += &name.0;
                if name.2 { result += Self::PRE_KEY_FOR_INSTANCE_UNIFORM; }
                result += ";"; result += crate::prelude::S_BREAK;
            });
            total_num += self.vec3_list.len();
            
            self.vec2_list.iter().for_each(|name| {
                result += crate::prelude::S_VEC2; result += crate::prelude::S_SPACE;
                result += &name.0;
                if name.2 { result += Self::PRE_KEY_FOR_INSTANCE_UNIFORM; }
                result += ";"; result += crate::prelude::S_BREAK;
            });
            total_num += self.vec2_list.len();
            let fill_vec2_count    = self.vec2_list.len() % 2;
            if fill_vec2_count > 0 {
                result += "vec2 _placeholder_vec2_0;"; result += crate::prelude::S_BREAK;
            }
            
            self.float_list.iter().for_each(|name| {
                result += crate::prelude::S_FLOAT; result += crate::prelude::S_SPACE;
                result += &name.0;
                if name.2 { result += Self::PRE_KEY_FOR_INSTANCE_UNIFORM; }
                result += ";"; result += crate::prelude::S_BREAK;
            });
            total_num += self.float_list.len();
            
            // self.int_list.iter().for_each(|name| {
            //     result += "int ";
            //     result += &name.0;
            //     result += ";\r\n";
            // });
            // total_num += self.int_list.len();
            
            self.uint_list.iter().for_each(|name| {
                result += crate::prelude::S_UINT; result += crate::prelude::S_SPACE;
                result += &name.0;
                if name.2 { result += Self::PRE_KEY_FOR_INSTANCE_UNIFORM; }
                result += ";"; result += crate::prelude::S_BREAK;
            });
            total_num += self.uint_list.len();
            let fill_int_count    = (self.float_list.len() /* + self.int_list.len()*/ + self.uint_list.len()) % 4;
            if fill_int_count > 0 {
                for i in fill_int_count..4 {
                    result += "uint _placeholder_int_";
                    result += &i.to_string();
                    result += ";"; result += crate::prelude::S_BREAK;
                }
            // } else {
            //     // 4 个 占位u32; 对应 ShaderBindEffectValue 中也有处理
            //     if total_num == 0 {
            //         for i in 0..4 {
            //             result += "uint _placeholder_int_";
            //             result += &i.to_string();
            //             result += ";\r\n";
            //         }
            //     }
            }
    
            result += "};"; result += crate::prelude::S_BREAK;
            // log::info!("Uniform Count: {}", total_num);
    
        }

        result
    }
    pub fn vs_running_code(&self) -> String {
        let mut result = String::from("");
        self.vec4_list.iter().for_each(|name| {
            if name.2 { 
                result += &name.0; result += crate::prelude::S_EQUAL; result += &name.0; result += Self::PRE_KEY_FOR_INSTANCE_UNIFORM; result += ";"; result += crate::prelude::S_BREAK;
            }
        });
        self.vec3_list.iter().for_each(|name| {
            if name.2 { 
                result += &name.0; result += crate::prelude::S_EQUAL; result += &name.0; result += Self::PRE_KEY_FOR_INSTANCE_UNIFORM; result += ".xyz;"; result += crate::prelude::S_BREAK;
            }
        });
        self.vec2_list.iter().for_each(|name| {
            if name.2 { 
                result += &name.0; result += crate::prelude::S_EQUAL; result += &name.0; result += Self::PRE_KEY_FOR_INSTANCE_UNIFORM; result += ";"; result += crate::prelude::S_BREAK;
            }
        });
        self.float_list.iter().for_each(|name| {
            if name.2 { 
                result += &name.0; result += crate::prelude::S_EQUAL; result += &name.0; result += Self::PRE_KEY_FOR_INSTANCE_UNIFORM; result += ";"; result += crate::prelude::S_BREAK;
            }
        });
        self.uint_list.iter().for_each(|name| {
            if name.2 { 
                result += &name.0; result += crate::prelude::S_EQUAL; result += &name.0; result += Self::PRE_KEY_FOR_INSTANCE_UNIFORM; result += ";"; result += crate::prelude::S_BREAK;
            }
        });

        result
    }
}
impl TBindDescToShaderCode for MaterialValueBindDesc {
    fn vs_code(&self, set: u32, bind: u32) -> String {
        if self.stage & wgpu::ShaderStages::VERTEX == wgpu::ShaderStages::VERTEX {
            self._code(set, bind)
        } else {
            String::from("")
        }
    }

    fn fs_code(&self, set: u32, bind: u32) -> String {
        if self.stage & wgpu::ShaderStages::FRAGMENT == wgpu::ShaderStages::FRAGMENT {
            self._code(set, bind)
        } else {
            String::from("")
        }
    }
}
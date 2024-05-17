use crate::shader::ShaderVarVarying;


pub const S_VEC2: &'static str = "vec2";
pub const S_VEC3: &'static str = "vec3";
pub const S_VEC4: &'static str = "vec4";
pub const S_MAT4: &'static str = "mat4";
pub const S_UVEC4: &'static str = "uvec4";
pub const S_SPACE: &'static str = " ";
pub const S_FLOAT: &'static str = "float";
pub const S_UINT: &'static str = "uint";
pub const S_INT: &'static str = "int";
pub const S_BREAK: &'static str = "\n";
pub const S_EQUAL: &'static str = "=";
pub const S_POSITION: &'static str = "position";
pub const S_NORMAL: &'static str = "normal";
pub const S_V_POSITION: &'static str = "v_position";
pub const S_V_POS: &'static str = ShaderVarVarying::POSITION;
pub const S_V_NORMAL: &'static str = ShaderVarVarying::NORMAL;
pub const S_V_COLOR: &'static str = ShaderVarVarying::COLOR;
pub const S_V_UV: &'static str = ShaderVarVarying::UV;
pub const S_EMISSION: &'static str = "emission";
pub const S_TEXTURE2D: &'static str = "texture2D";
pub const S_SAMPLER: &'static str = "sampler";